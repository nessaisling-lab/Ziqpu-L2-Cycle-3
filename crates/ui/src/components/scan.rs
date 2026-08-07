//! The Scan control — read a product code with the webcam or from a picture, and say honestly what
//! it found.
//!
//! Compiled only with `--features camera`. Without it the component renders nothing, so the call
//! site needs no `cfg` and an affordance that cannot work never reaches the screen.
//!
//! # Why there is a live preview
//!
//! A scanner you cannot aim is a guessing game: press a button, wait, and be told "no code found"
//! with no idea whether the problem was the framing, the light, or the feature itself. The webview
//! cannot open the camera, so the preview works the other way round — the worker encodes small
//! greyscale frames and pushes them up as data URIs. It costs a little throughput and buys the
//! difference between aiming and hoping.
//!
//! # What this deliberately does *not* do
//!
//! It does not turn a scan into a chartable choice. A code can give a day-precise manufacture date,
//! but a natal chart also needs a **place**, and a product's code carries none. Every other choice in
//! Ziqpu gets its coordinates from something real — an exchange floor, a CSV column, an assembly
//! plant. Inventing a location to make the scan chartable would be the same failure as the January-1
//! dates this project deleted, one field over. So the scan reports what the code actually says, and
//! the question of where a manufactured object is "born" stays open rather than being answered by a
//! default nobody chose.

use dioxus::prelude::*;

/// Without the `camera` feature there is no scanner, so the control renders nothing at all — the
/// call site stays free of `cfg`, and a button that could only fail never appears.
#[cfg(not(feature = "camera"))]
#[component]
pub fn ScanButton() -> Element {
    rsx! {}
}

/// What the worker sends up: either a picture of what the camera sees, or the finished outcome.
///
/// Both travel one channel so they stay ordered — a result can never be overtaken by a stale frame
/// arriving after it and leaving a frozen image on screen.
#[cfg(feature = "camera")]
pub enum ScanMsg {
    Frame(String),
    Done(ScanState),
}

/// Where the scan currently stands, as far as the UI is concerned.
#[cfg(feature = "camera")]
#[derive(Clone, PartialEq)]
pub enum ScanState {
    Idle,
    /// The camera is open and frames are being read.
    Looking,
    /// A code was read. `lines` is the honest record — identity, and a date only if the code had one.
    Found {
        symbology: String,
        lines: Vec<String>,
    },
    /// Nothing came back. Carries the reason, because "no camera" and "no code in view" are
    /// different problems for the person holding the item.
    Failed(String),
}

/// A control that reads a code — from the camera, with a live preview, or from a picture.
#[cfg(feature = "camera")]
#[component]
pub fn ScanButton() -> Element {
    let mut state = use_signal(|| ScanState::Idle);
    let mut preview = use_signal(|| None::<String>);

    // The scan blocks for as long as its timeout, so it cannot run on the event loop — the window
    // would freeze exactly the way the grounded pull used to. The worker streams frames and the
    // final outcome back through one coroutine, the same pattern `run_grounding` uses.
    let worker = use_coroutine(move |mut rx: UnboundedReceiver<ScanMsg>| async move {
        use futures_util::StreamExt;
        while let Some(msg) = rx.next().await {
            match msg {
                ScanMsg::Frame(png) => preview.set(Some(png)),
                ScanMsg::Done(done) => {
                    preview.set(None);
                    state.set(done);
                }
            }
        }
    });

    let current = state.read().clone();
    let busy = matches!(current, ScanState::Looking);
    let frame = preview.read().clone();

    rsx! {
        div { class: "scan",
            div { class: "scan__actions",
                button {
                    class: "btn scan__btn",
                    r#type: "button",
                    disabled: busy,
                    onclick: move |_| {
                        if busy {
                            return;
                        }
                        state.set(ScanState::Looking);
                        let tx = worker.tx();
                        std::thread::spawn(move || {
                            let frames = tx.clone();
                            let outcome = scan_and_read(move |png| {
                                let _ = frames.unbounded_send(ScanMsg::Frame(png));
                            });
                            let _ = tx.unbounded_send(ScanMsg::Done(outcome));
                        });
                    },
                    if busy { "Looking for a code…" } else { "⛶ Scan with the camera" }
                }

                // No camera, no product to hand, or a code easier to screenshot than to hold up — a
                // picture goes through the very same decoder, so both agree on what a code says.
                label { class: "scan__file",
                    span { "…or read an image" }
                    input {
                        r#type: "file",
                        accept: "image/png,image/jpeg,.png,.jpg,.jpeg",
                        onchange: move |evt| async move {
                            let Some(engine) = evt.files() else { return };
                            let Some(name) = engine.files().first().cloned() else { return };
                            match engine.read_file(&name).await {
                                Some(bytes) => state.set(read_bytes(&bytes)),
                                None => {
                                    state.set(ScanState::Failed("Couldn't read that file.".to_string()))
                                }
                            }
                        },
                    }
                }
            }

            // The preview is what makes this aimable: it shows the camera's own view while the scan
            // runs, so a miss reads as a framing problem rather than a mystery.
            if let Some(png) = frame {
                img {
                    class: "scan__preview",
                    src: "data:image/png;base64,{png}",
                    alt: "What the camera is seeing right now",
                }
            }

            match current {
                ScanState::Idle => rsx! {
                    p { class: "scan__hint",
                        "Hold a product's barcode or QR code up to the camera, or pick an image of one. "
                        "Most retail barcodes carry only an identity — a manufacture date appears on the "
                        "2D codes used for medicines and perishables."
                    }
                },
                ScanState::Looking => rsx! {
                    p { class: "scan__hint", "Hold the code steady in view…" }
                },
                ScanState::Failed(why) => rsx! {
                    p { class: "scan__hint scan__hint--fail", "{why}" }
                },
                ScanState::Found { symbology, lines } => rsx! {
                    div { class: "scan__result",
                        p { class: "scan__sym", "Read a {symbology}" }
                        ul { class: "scan__lines",
                            {lines.iter().map(|l| rsx! { li { key: "{l}", "{l}" } })}
                        }
                    }
                },
            }
        }
    }
}

/// Scan with the camera, reporting frames as they arrive. Runs on the worker thread.
#[cfg(feature = "camera")]
fn scan_and_read(on_frame: impl FnMut(String)) -> ScanState {
    use agents::camera::{scan_with_preview, ScanError};
    use std::time::Duration;

    match scan_with_preview(Duration::from_secs(20), on_frame) {
        Ok(scan) => report(scan),
        Err(ScanError::NoCamera(_)) => {
            ScanState::Failed("No camera this app can open.".to_string())
        }
        Err(ScanError::UnsupportedFormat(_)) => ScanState::Failed(
            "This camera only offers a compressed video format, which this build can't read. \
             Reading an image of the code still works."
                .to_string(),
        ),
        Err(ScanError::NoCodeFound) => ScanState::Failed(
            "No code came into view. Try holding it closer, flatter, or in better light."
                .to_string(),
        ),
    }
}

/// Decode a picked image through the same decoder the lens uses, so a picture and a lens agree.
#[cfg(feature = "camera")]
fn read_bytes(bytes: &[u8]) -> ScanState {
    use agents::camera::ScanError;
    match agents::camera::decode_image_bytes(bytes) {
        Ok(scan) => report(scan),
        Err(ScanError::NoCodeFound) => {
            ScanState::Failed("No code found in that image.".to_string())
        }
        Err(e) => ScanState::Failed(e.to_string()),
    }
}

/// Turn a read code into the honest record: identity always, a date only when the code carried one.
#[cfg(feature = "camera")]
fn report(scan: agents::camera::Scan) -> ScanState {
    let elements = agents::parse_gs1(&scan.text);
    let mut lines = Vec::new();

    match agents::gtin(&elements) {
        Some(code) => lines.push(format!("Identity: GTIN {code}")),
        // Readable, but not a GS1 element string — a plain URL in a QR, say.
        None => lines.push(format!("Code: {}", scan.text)),
    }

    // The whole point of scanning — on the codes that happen to carry it.
    match agents::production_date(&elements, current_year()) {
        Some(agents::ScannedDate::Day(d)) => {
            lines.push(format!("Made on {d} — this individual item"));
        }
        Some(agents::ScannedDate::Month(y, m)) => lines.push(format!(
            "Made in {y}-{m:02} — the code names a month, not a day, so it isn't a chartable moment"
        )),
        None => lines.push(
            "No manufacture date — this code carries identity only, which is normal for retail \
             barcodes."
                .to_string(),
        ),
    }
    ScanState::Found {
        symbology: scan.symbology,
        lines,
    }
}

/// The current year, for resolving a code's two-digit one. The only clock in this path.
#[cfg(feature = "camera")]
fn current_year() -> i32 {
    use chrono::Datelike;
    chrono::Local::now().year()
}
