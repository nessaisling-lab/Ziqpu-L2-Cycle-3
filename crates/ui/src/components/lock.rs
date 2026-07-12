//! PremiumLock — the little 🔒 chip that marks a paywalled affordance when the app is in
//! "preview as customer" mode (the developer-build switch is off). A pure presentational marker;
//! whether it shows at all is the caller's entitlement decision ([`AppCtx::premium`]).

use dioxus::prelude::*;

/// A small lock chip with a label, e.g. `🔒 Premium`. Rendered next to (or around) a feature a free
/// customer can't use, so the gate is visible rather than silent. Inline SVG padlock so it recolors
/// with the theme; `title`/`aria-label` carry the reason.
#[component]
pub fn PremiumLock(label: String) -> Element {
    rsx! {
        span {
            class: "lock",
            title: "Premium — unlocked in the developer build",
            "aria-label": "Premium feature, locked: {label}",
            svg {
                class: "lock__i",
                view_box: "0 0 24 24",
                width: "12",
                height: "12",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "1.7",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                rect { x: "5", y: "11", width: "14", height: "9", rx: "1.6" }
                path { d: "M8 11V8a4 4 0 0 1 8 0v3" }
            }
            span { class: "lock__t", "{label}" }
        }
    }
}
