//! Reading a product code with the camera — phase 2 of scanning, behind the `camera` feature.
//!
//! Phase 1 ([`crate::gs1`]) can already turn a code *string* into an individual item's manufacture
//! date. This closes the last gap: getting that string off the physical object without the seeker
//! typing it. The output feeds the same parser and the same
//! [`Gs1Source`](crate::Gs1Source) — nothing downstream knows or cares whether the code was scanned
//! or pasted.
//!
//! # Why this is a native path and not a web one
//!
//! The obvious approach — `getUserMedia` in the UI — is unavailable: the desktop shell renders in a
//! system webview (WRY/WebView2), which does not grant camera access the way a browser does. So
//! capture happens in Rust, on the native platform API, and the decoded string is handed to the UI.
//!
//! # The split, and why decoding is separate from capture
//!
//! [`decode_luma`] is a pure function over a greyscale image: bytes in, code out. It has no camera in
//! it, so it can be exercised in an ordinary test on a synthetic image — the part most likely to be
//! subtly wrong is the part that is verifiable without hardware.
//! [`scan_once`] is the thin, unavoidably platform-bound loop around it.
//!
//! # Frame formats
//!
//! The camera is asked for an **uncompressed** frame, and the luma plane is read directly out of it.
//! That is both the cheapest path for a decoder (it wants greyscale anyway) and the reason this
//! module can exist at all: nokhwa's JPEG decoding pulls a dependency licensed outside this
//! project's allowlist, so the MJPEG path is compiled out entirely. A camera that offers *only*
//! MJPEG is reported as unsupported rather than silently returning nothing.

use std::time::{Duration, Instant};

use nokhwa::pixel_format::LumaFormat;
use nokhwa::utils::{ApiBackend, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use std::borrow::Cow;

use rxing::{
    common::HybridBinarizer, BinaryBitmap, DecodeHints, LuminanceSource, MultiFormatReader, Reader,
};

/// A greyscale frame as rxing wants to see it.
///
/// Written here rather than taken from the library because rxing 0.9's own raw sources
/// (`PlanarYUVLuminanceSource`, `RGBLuminanceSource`) leave `get_column` as `unimplemented!()` and
/// panic the moment a 1D reader takes its rotated pass — and the source that *does* implement it is
/// backed by the `image` crate, which drags in an AVIF encoder (`ravif` → `rav1e` → `libfuzzer-sys`,
/// NCSA-licensed) that this repo's allowlist rightly refuses. Thirty lines here buys a working
/// rotated pass, a much smaller dependency tree, and no licence argument.
struct LumaPlane {
    luma: Vec<u8>,
    width: usize,
    height: usize,
    inverted: bool,
}

impl LumaPlane {
    fn new(luma: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            luma,
            width,
            height,
            inverted: false,
        }
    }
    /// One byte, honouring an inversion request. Codes are usually dark-on-light, but a decoder may
    /// retry inverted for light-on-dark printing.
    fn at(&self, x: usize, y: usize) -> u8 {
        let v = self.luma[y * self.width + x];
        if self.inverted {
            255 - v
        } else {
            v
        }
    }
}

impl LuminanceSource for LumaPlane {
    fn get_row(&self, y: usize) -> Option<Cow<'_, [u8]>> {
        if y >= self.height {
            return None;
        }
        if self.inverted {
            return Some(Cow::Owned((0..self.width).map(|x| self.at(x, y)).collect()));
        }
        let start = y * self.width;
        Some(Cow::Borrowed(&self.luma[start..start + self.width]))
    }

    /// The method the library's own raw sources leave unimplemented — and the reason a 1D scan of a
    /// rotated frame used to abort the process instead of just missing.
    fn get_column(&self, x: usize) -> Vec<u8> {
        if x >= self.width {
            return Vec::new();
        }
        (0..self.height).map(|y| self.at(x, y)).collect()
    }

    fn get_matrix(&self) -> Cow<'_, [u8]> {
        if self.inverted {
            return Cow::Owned(
                (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .map(|(x, y)| self.at(x, y))
                    .collect(),
            );
        }
        Cow::Borrowed(&self.luma)
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    /// Bounds-checked rather than indexing blind: a decoder probing near an edge must get a defined
    /// value, not a panic in the middle of a scan.
    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.at(x, y)
    }
}

/// What a successful scan read off the object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    /// The code's text, exactly as encoded — hand this straight to [`crate::gs1::parse_gs1`].
    pub text: String,
    /// Which symbology it came from (`QR_CODE`, `DATA_MATRIX`, `CODE_128`, `EAN_13`, …). Worth
    /// surfacing: a 1D retail symbology tells the seeker up front that no date is coming.
    pub symbology: String,
}

/// Why a scan produced nothing. Each variant is a different thing to tell the person holding the
/// item, which is the reason they aren't collapsed into one error string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanError {
    /// No camera the platform will open.
    NoCamera(String),
    /// A camera exists but will not give an uncompressed frame. See the module note on MJPEG.
    UnsupportedFormat(String),
    /// The camera worked and nothing decodable came into view before the deadline. Not a failure of
    /// the machinery — usually the code was out of frame, too far, or glared out.
    NoCodeFound,
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::NoCamera(why) => write!(f, "no camera available: {why}"),
            ScanError::UnsupportedFormat(why) => {
                write!(f, "the camera only offers a compressed format: {why}")
            }
            ScanError::NoCodeFound => write!(f, "no code came into view"),
        }
    }
}

impl std::error::Error for ScanError {}

/// Decode a **greyscale** image into the first code found in it.
///
/// Pure: `luma` is one byte per pixel, row-major, `width * height` long. Returns `None` when the
/// image holds no readable code — which is the ordinary case for most frames of a live scan, not an
/// error.
pub fn decode_luma(luma: &[u8], width: u32, height: u32) -> Option<Scan> {
    let (w, h) = (width as usize, height as usize);
    if w == 0 || h == 0 || luma.len() < w * h {
        return None;
    }
    // Take exactly the plane; a camera row may be padded beyond width*height.
    let source = LumaPlane::new(luma[..w * h].to_vec(), w, h);
    // Worth the extra passes: a hand-held frame is rarely square-on, and a missed frame costs the
    // seeker another second of holding the item up.
    let hints = DecodeHints {
        TryHarder: Some(true),
        ..Default::default()
    };
    let result = MultiFormatReader::default()
        .decode_with_hints(&mut BinaryBitmap::new(HybridBinarizer::new(source)), &hints)
        .ok()?;
    Some(Scan {
        text: result.getText().to_string(),
        symbology: format!("{:?}", result.getBarcodeFormat()),
    })
}

/// Open the default camera and read frames until a code decodes or `timeout` elapses.
///
/// Blocking, and platform-bound — call it off the UI thread. The camera is opened for the duration
/// of the scan and closed on the way out, including on failure: a scanner that leaves the lamp on is
/// its own kind of bug.
pub fn scan_once(timeout: Duration) -> Result<Scan, ScanError> {
    // Ask for greyscale explicitly. nokhwa negotiates the closest uncompressed format the device
    // offers and hands back the luma plane — no JPEG path, which is compiled out (see module note).
    let format = RequestedFormat::new::<LumaFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(CameraIndex::Index(0), format)
        .map_err(|e| ScanError::NoCamera(e.to_string()))?;

    if camera.frame_format() == FrameFormat::MJPEG {
        return Err(ScanError::UnsupportedFormat(
            "this build decodes uncompressed frames only".to_string(),
        ));
    }
    camera
        .open_stream()
        .map_err(|e| ScanError::NoCamera(e.to_string()))?;

    let deadline = Instant::now() + timeout;
    let mut found = None;
    while Instant::now() < deadline {
        let Ok(frame) = camera.frame() else {
            continue; // a dropped frame is normal; keep looking until the deadline
        };
        let resolution = frame.resolution();
        if let Some(scan) = decode_luma(frame.buffer(), resolution.width(), resolution.height()) {
            found = Some(scan);
            break;
        }
    }
    // Release the device whatever happened.
    let _ = camera.stop_stream();

    found.ok_or(ScanError::NoCodeFound)
}

/// Whether the platform reports any camera at all — so a UI can hide or disable the scan affordance
/// instead of offering something that will fail.
pub fn camera_available() -> bool {
    nokhwa::query(ApiBackend::Auto)
        .map(|c| !c.is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Render an rxing `BitMatrix` into a greyscale buffer, the way a camera would see it printed:
    /// dark modules on a light field.
    fn render(matrix: &rxing::common::BitMatrix) -> (Vec<u8>, u32, u32) {
        let (w, h) = (matrix.getWidth(), matrix.getHeight());
        let mut luma = vec![255u8; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                if matrix.get(x, y) {
                    luma[(y * w + x) as usize] = 0;
                }
            }
        }
        (luma, w, h)
    }

    /// A frame with nothing in it decodes to nothing — and must not panic or invent a read. This is
    /// the common case during a live scan, so it is the one that has to be cheap and quiet.
    #[test]
    fn a_blank_frame_yields_no_code() {
        let luma = vec![255u8; 320 * 240];
        assert!(decode_luma(&luma, 320, 240).is_none());
        let noise: Vec<u8> = (0..320 * 240).map(|i| (i * 37 % 256) as u8).collect();
        assert!(decode_luma(&noise, 320, 240).is_none());
    }

    /// Malformed input is refused rather than read past the end of the buffer.
    #[test]
    fn a_short_or_empty_buffer_is_refused() {
        assert!(decode_luma(&[], 0, 0).is_none());
        assert!(
            decode_luma(&[0u8; 10], 320, 240).is_none(),
            "buffer shorter than the frame"
        );
        assert!(decode_luma(&[255u8; 100], 10, 0).is_none());
    }

    /// **The decode proof, without a camera.** Encode a payload with rxing's own writer, render it to
    /// a greyscale frame, and read it back — so the whole luma path (plane layout, binarizer, reader)
    /// is exercised end to end. Using the library's encoder rather than a hand-written module table
    /// means a failure here is a real defect in our path, not a typo in a symbology chart.
    #[test]
    fn a_rendered_code_round_trips_through_the_luma_path() {
        use rxing::{BarcodeFormat, MultiFormatWriter, Writer};

        // A GS1-shaped payload in a symbology that actually carries them, and a QR for the 2D path.
        for (format, payload, w, h) in [
            (BarcodeFormat::CODE_128, "0100614141000012", 400, 120),
            (
                BarcodeFormat::QR_CODE,
                "(01)00614141000012(11)200315",
                300,
                300,
            ),
        ] {
            let matrix = MultiFormatWriter
                .encode(payload, &format, w, h)
                .unwrap_or_else(|e| panic!("{format:?} should encode: {e}"));
            let (luma, width, height) = render(&matrix);

            let scan = decode_luma(&luma, width, height)
                .unwrap_or_else(|| panic!("{format:?} must decode back out of the luma plane"));
            assert_eq!(
                scan.text, payload,
                "{format:?} round-tripped wrong: {scan:?}"
            );
        }
    }

    /// The scanned string is what phase 1 already knows how to read — this is the seam between the
    /// camera and the GS1 parser, and the reason the camera needed no changes below it.
    #[test]
    fn a_scanned_string_feeds_straight_into_the_gs1_parser() {
        use rxing::{BarcodeFormat, MultiFormatWriter, Writer};
        let payload = "(01)00614141000012(11)200315";
        let matrix = MultiFormatWriter
            .encode(payload, &BarcodeFormat::QR_CODE, 300, 300)
            .expect("encodes");
        let (luma, w, h) = render(&matrix);
        let scan = decode_luma(&luma, w, h).expect("decodes");

        let elements = crate::gs1::parse_gs1(&scan.text);
        assert_eq!(crate::gs1::gtin(&elements), Some("00614141000012"));
        assert_eq!(
            crate::gs1::production_date(&elements, 2026),
            Some(crate::gs1::ScannedDate::Day(
                chrono::NaiveDate::from_ymd_opt(2020, 3, 15).unwrap()
            )),
            "a scanned code must reach the same individual-item date a pasted one does"
        );
    }

    /// LIVE — needs a real webcam and a code held up to it. Ignored by default.
    /// `cargo test -p agents --features camera camera -- --ignored --nocapture live_scan`
    #[test]
    #[ignore = "needs a webcam and a physical code"]
    fn live_scan_reads_a_held_up_code() {
        eprintln!("camera present: {}", camera_available());
        match scan_once(Duration::from_secs(15)) {
            Ok(scan) => {
                eprintln!("scanned {}: {}", scan.symbology, scan.text);
                let elements = crate::gs1::parse_gs1(&scan.text);
                eprintln!("parsed {} GS1 element(s): {elements:?}", elements.len());
            }
            Err(e) => eprintln!("no scan: {e}"),
        }
    }
}
