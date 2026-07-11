//! The seeker's **anonymous identity** — a generated two-word cosmic handle (e.g. "Lapis Scribe"),
//! no email and no login. Rendered at the top of Setup as the "this is you" beat.
//!
//! The default handle derives from the seeker's birth chart (stable — see [`agents::anon_handle_for`]).
//! A **re-roll** gives the gaming-avatar feel: each click mixes a fresh nonce and *persists* the new
//! name via [`crate::profile::save_handle`], so it survives relaunches. A kept name can be dropped
//! back to the chart-derived default. Empty-on-disk means "derive", which is why the default is never
//! written until the seeker actually chooses to re-roll.

use dioxus::prelude::*;

use crate::state::AppCtx;

#[component]
pub fn Identity() -> Element {
    let ctx = use_context::<AppCtx>();
    let seeker = ctx.seeker.read().clone();
    // The chart seed is Copy (u64), so the re-roll closure can hold it without cloning the moment.
    let seed = agents::handle_seed(&seeker);

    // The shown handle lives in a signal so re-roll / reset repaint. Seeded from persistence: a kept
    // (re-rolled) handle, else the chart-derived default. The component remounts when the phase rail
    // changes (keyed in `App`), so returning to Setup after editing the chart re-derives the default.
    let mut handle = use_signal(|| crate::profile::handle_or_default(&seeker));
    // Re-roll nonce — a plain click counter (deterministic, no clock, no rng). 0 == the chart default.
    let mut nonce = use_signal(|| 0u64);
    // Whether the shown handle is a kept custom one (vs the chart default) — drives the reset control.
    let mut custom = use_signal(|| !crate::profile::load_handle().is_empty());

    let name = handle.read().clone();
    let is_custom = *custom.read();
    // The reset closure needs the moment (to re-derive); the re-roll closure needs only `seed`.
    let seeker_reset = seeker.clone();

    rsx! {
        section { class: "identity",
            p { class: "eyebrow", "This is you · anonymous" }
            div { class: "identity-row",
                div { class: "identity-sigil", "aria-hidden": "true", "✦" }
                div { class: "identity-body",
                    span { class: "identity-handle", "{name}" }
                    span { class: "identity-sub",
                        if is_custom {
                            "a name you rolled — no email, no account, just this handle"
                        } else {
                            "drawn from your chart — no email, no account, just this handle"
                        }
                    }
                }
                div { class: "identity-actions",
                    button {
                        class: "btn btn--ghost",
                        r#type: "button",
                        title: "Roll a new anonymous name",
                        "aria-label": "Re-roll your anonymous handle",
                        onclick: move |_| {
                            let n = nonce.read().wrapping_add(1);
                            nonce.set(n);
                            let h = agents::anon_handle_reroll(seed, n);
                            crate::profile::save_handle(&h);
                            custom.set(true);
                            handle.set(h);
                        },
                        "⟳ re-roll"
                    }
                    if is_custom {
                        button {
                            class: "btn btn--ghost",
                            r#type: "button",
                            title: "Return to the name drawn from your chart",
                            "aria-label": "Reset to the chart-derived handle",
                            onclick: move |_| {
                                // Clearing the saved handle means "derive" again — the default is
                                // never stored, so the chart drives it once more.
                                crate::profile::save_handle("");
                                nonce.set(0);
                                custom.set(false);
                                handle.set(agents::anon_handle_for(&seeker_reset));
                            },
                            "↺ cosmic default"
                        }
                    }
                }
            }
        }
    }
}
