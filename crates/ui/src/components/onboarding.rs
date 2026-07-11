//! First-run onboarding gate — **welcome → birth chart → reveal your anonymous handle → enter**.
//!
//! Shown only for a brand-new seeker (no saved profile); returning seekers skip it entirely (see the
//! gate in [`crate::app`]). It reuses [`BirthInputForm`] in `reveal_mode` (which advances instead of
//! running the loop) and the [`Identity`] card for the reveal, so the wizard adds a flow, not a new
//! form. On "Enter Ziqpu" it fires [`on_done`], which drops the gate and reveals the main app.

use dioxus::prelude::*;

use crate::components::{BirthInputForm, Identity};

/// The three beats of the gate.
#[derive(Clone, Copy, PartialEq)]
enum Step {
    Welcome,
    Birth,
    Reveal,
}

#[component]
pub fn Onboarding(on_done: EventHandler<()>) -> Element {
    let mut step = use_signal(|| Step::Welcome);
    let current = *step.read();

    rsx! {
        div { class: "onboarding",
            div { class: "onboarding-card",
                {match current {
                    Step::Welcome => rsx! {
                        p { class: "eyebrow", "Welcome" }
                        h1 { class: "onboarding-title", "Ziqpu" }
                        p { class: "onboarding-lede",
                            "You're about to get a chart-self — an anonymous reading identity drawn from "
                            "your birth moment. No email, no account, no password. Just a handle and a chart, "
                            "kept on this machine."
                        }
                        p { class: "onboarding-sub", "the ledger of the sky · measured, not fate" }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--go",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Birth),
                                "Begin →"
                            }
                        }
                    },
                    Step::Birth => rsx! {
                        p { class: "onboarding-step", "Step 1 of 2 · your birth moment" }
                        // reveal_mode: the form saves the chart + sets the seeker, then advances here
                        // rather than running the graded loop.
                        BirthInputForm { reveal_mode: true, on_continue: move |_| step.set(Step::Reveal) }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Welcome),
                                "← back"
                            }
                        }
                    },
                    Step::Reveal => rsx! {
                        p { class: "onboarding-step", "Step 2 of 2 · meet your chart-self" }
                        // The Identity card reads the just-set seeker: it shows the chart-derived handle
                        // with the re-roll / reset controls, so the reveal *is* the identity surface.
                        Identity {}
                        p { class: "onboarding-lede onboarding-lede--muted",
                            "This is your anonymous handle. Re-roll until it fits — you can change it any "
                            "time from Setup. Nothing here leaves your machine."
                        }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Birth),
                                "← edit chart"
                            }
                            button {
                                class: "btn btn--go",
                                r#type: "button",
                                onclick: move |_| on_done.call(()),
                                "Enter Ziqpu →"
                            }
                        }
                    },
                }}
            }
        }
    }
}
