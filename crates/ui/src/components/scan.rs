//! The Scan button — read a product code with the webcam, and say honestly what it found.
//!
//! Compiled only with `--features camera`. Without it the button does not exist, which is the
//! honest default: an affordance that cannot work should not be on screen.
//!
//! # What this deliberately does *not* do
//!
//! It does not turn a scan into a chartable choice. A code can give a day-precise manufacture date,
//! but a natal chart also needs a **place**, and a product's code carries none. Every other choice in
//! Ziqpu gets its coordinates from something real — an exchange floor, a CSV column, an assembly
//! plant. Inventing a location to make the scan chartable would be the same failure as the January-1
//! dates this project deleted, one field over. So the scan reports what the code actually says, and
//! the question of where a manufactured object is "born" stays open rather than being answered by a
//! default.

use dioxus::prelude::*;

/// Without the `camera` feature there is no scanner, so the button renders nothing at all — the
/// call site stays free of `cfg`, and a control that could only fail never appears.
#[cfg(not(feature = "camera"))]
#[component]
pub fn ScanButton() -> Element {
    rsx! {}
}

#[cfg(feature = "camera")]
/// What the scan worker sends back to the UI thread.
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

/// A button that opens the camera, reads one code, and shows what it said.
#[cfg(feature = "camera")]
#[component]
pub fn ScanButton() -> Element {
    let mut state = use_signal(|| ScanState::Idle);

    // The scan blocks for as long as its timeout, so it cannot run on the event loop — the window
    // would freeze exactly like the grounded pull used to. The worker sends its result back through
    // a coroutine, the same pattern `run_grounding` uses.
    let worker = use_coroutine(move |mut rx: UnboundedReceiver<ScanState>| async move {
        use futures_util::StreamExt;
        while let Some(next) = rx.next().await {
            state.set(next);
        }
    });

    let current = state.read().clone();
    let busy = matches!(current, ScanState::Looking);

    rsx! {
        div { class: "scan",
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
                        let _ = tx.unbounded_send(scan_and_read());
                    });
                },
                if busy { "Looking for a code…" } else { "⛶ Scan a code" }
            }

            match current {
                ScanState::Idle => rsx! {
                    p { class: "scan__hint",
                        "Hold a product's barcode or QR code up to the camera. Most retail barcodes \
                         carry only an identity — a manufacture date appears on the 2D codes used \
                         for medicines and perishables."
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

#[cfg(feature = "camera")]
/// Scan, then read the code's own record through the same parser a pasted code goes through.
///
/// Runs on the worker thread. Everything it returns is either what the code said or an honest
/// account of why nothing came back.
fn scan_and_read() -> ScanState {
    use agents::camera::{scan_once, ScanError};
    use std::time::Duration;

    match scan_once(Duration::from_secs(15)) {
        Err(ScanError::NoCamera(_)) => {
            ScanState::Failed("No camera this app can open.".to_string())
        }
        Err(ScanError::UnsupportedFormat(_)) => ScanState::Failed(
            "This camera only offers a compressed video format, which this build can't read."
                .to_string(),
        ),
        Err(ScanError::NoCodeFound) => ScanState::Failed(
            "No code came into view. Try holding it closer, flatter, or in better light."
                .to_string(),
        ),
        Ok(scan) => {
            let elements = agents::parse_gs1(&scan.text);
            let mut lines = Vec::new();

            match agents::gtin(&elements) {
                Some(code) => lines.push(format!("Identity: GTIN {code}")),
                // A code we can read but that carries no GS1 structure — a plain URL in a QR, say.
                None => lines.push(format!("Code: {}", scan.text)),
            }

            // The whole point of scanning, when the code happens to carry it.
            match agents::production_date(&elements, current_year()) {
                Some(agents::ScannedDate::Day(d)) => {
                    lines.push(format!("Made on {d} — this individual item"));
                }
                Some(agents::ScannedDate::Month(y, m)) => lines.push(format!(
                    "Made in {y}-{m:02} — the code names a month, not a day, so it isn't a chartable moment"
                )),
                None => lines.push(
                    "No manufacture date — this code carries identity only, which is normal for \
                     retail barcodes."
                        .to_string(),
                ),
            }
            ScanState::Found {
                symbology: scan.symbology,
                lines,
            }
        }
    }
}

#[cfg(feature = "camera")]
/// The current year, for resolving a code's two-digit one. The only clock in this path.
fn current_year() -> i32 {
    use chrono::Datelike;
    chrono::Local::now().year()
}
