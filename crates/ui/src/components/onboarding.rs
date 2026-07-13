//! First-run onboarding gate — **welcome → birth chart → reveal handle → local model → enter**.
//!
//! Shown only for a brand-new seeker (no saved profile); returning seekers skip it entirely (see the
//! gate in [`crate::app`]). It reuses [`BirthInputForm`] in `reveal_mode` (which advances instead of
//! running the loop), the [`Identity`] card for the reveal, and [`ModelPanel`] for the optional
//! local-model setup — so the wizard adds a flow, not new forms. On "Enter Ziqpu" it fires
//! [`on_done`], which drops the gate and reveals the main app.

use dioxus::prelude::*;

use crate::components::{BirthInputForm, Identity, ModelPanel};

/// The beats of the gate.
#[derive(Clone, Copy, PartialEq)]
enum Step {
    Welcome,
    Birth,
    Reveal,
    Model,
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
                        p { class: "onboarding-step", "Step 1 of 3 · your birth moment" }
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
                        p { class: "onboarding-step", "Step 2 of 3 · meet your chart-self" }
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
                                onclick: move |_| step.set(Step::Model),
                                "Next: local model →"
                            }
                        }
                    },
                    Step::Model => rsx! {
                        p { class: "onboarding-step", "Step 3 of 3 · your local model (optional)" }
                        p { class: "onboarding-lede",
                            "Ziqpu can run readings on your own machine — private, offline, free. "
                            "Benchmark to see the best model for your hardware and set it up now, or skip "
                            "and use the offline template (Raw) or a hosted key (Live). You can always do "
                            "this later from Settings."
                        }
                        ModelPanel {}
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Reveal),
                                "← back"
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
