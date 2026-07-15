//! The Ziqpu loading indicator — a stand of wheat, drawn in SVG + animated in CSS (no runtime
//! dependency, works offline). Two beats, chosen by the owner over four other styles:
//!
//! - **Download** ([`WheatPhase::Download`]) — stalks grow in **left → right**, tracking the real
//!   percent; a grown stalk turns gold.
//! - **Loading into VRAM** ([`WheatPhase::Loading`]) — the whole field **sways** and its color washes
//!   **red → green → gold** (the "working" beat; gold is reserved for done).
//! - **Done** ([`WheatPhase::Done`]) — every stalk solid gold, still. Gold = resident in VRAM.
//!
//! The animation is pure CSS keyed off the container class (`.wload--loading` / `.wload--done`); this
//! component only decides how many stalks are grown (for the download percent) and the stagger delay.

use dioxus::prelude::*;

/// How many stalks make up the field. Enough to read as a stand of wheat, few enough to stay light.
const STALKS: usize = 17;

/// Which beat of the loader to draw.
#[derive(Clone, Copy, PartialEq)]
pub enum WheatPhase {
    /// First-run download, `0..=100` %. Stalks grow left→right to match.
    Download(u8),
    /// Indeterminate load into VRAM — sway + red→green→gold wash.
    Loading,
    /// Loaded / ready — solid gold, still.
    Done,
}

/// One wheat stalk: a stem, four pairs of grains, and a tip. `grown` drives the sprout-in transition
/// (the download beat); in Loading/Done the container class overrides it so all stalks stand.
#[component]
fn Stalk(index: usize, grown: bool) -> Element {
    let grow_class = if grown {
        "wload-grow wload-grown"
    } else {
        "wload-grow"
    };
    // Stagger the sway so the field ripples rather than moving as one board.
    let delay = format!("animation-delay:{:.2}s", index as f64 * 0.13);
    rsx! {
        span { key: "{index}", class: "wload-stalk", style: "{delay}",
            svg {
                view_box: "0 0 20 80",
                preserve_aspect_ratio: "none",
                g { class: "{grow_class}",
                    path { class: "wload-stem", d: "M10 80 L10 26" }
                    {(0..4).map(|k| {
                        let y = 30 + k * 8;
                        let base = y + 6;
                        let ctl = y + 2;
                        rsx! {
                            path { key: "l{k}", class: "wload-grain", d: "M10 {base} Q4 {ctl} 5 {y}" }
                            path { key: "r{k}", class: "wload-grain", d: "M10 {base} Q16 {ctl} 15 {y}" }
                        }
                    })}
                    path { class: "wload-grain", d: "M10 26 L10 15" }
                }
            }
        }
    }
}

/// The loader. Drop it in with a [`WheatPhase`]; the CSS in `ziqpu.css` (`.wload*`) does the motion.
#[component]
pub fn WheatLoader(phase: WheatPhase) -> Element {
    let container = match phase {
        WheatPhase::Download(_) => "wload",
        WheatPhase::Loading => "wload wload--loading",
        WheatPhase::Done => "wload wload--done",
    };
    // How many stalks are grown: the download percent → a left→right fill; full otherwise.
    let grown_count = match phase {
        WheatPhase::Download(pct) => (pct as usize * STALKS).div_ceil(100).min(STALKS),
        _ => STALKS,
    };
    rsx! {
        div { class: "{container}", "aria-hidden": "true",
            {(0..STALKS).map(|i| rsx! { Stalk { key: "{i}", index: i, grown: i < grown_count } })}
        }
    }
}
