//! The root component: sets up the session + context, renders the header/stepper, switches on the
//! current phase, and mounts the persistent guardrail from the Ranked step onward.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{demo_choices, demo_seeker};
use dioxus::prelude::*;

use crate::components::{Briefing, Checkpoint, Guardrail, Ranked, Setup};
use crate::state::{build_session, AppCtx, Phase};

/// The real 4-beat sequence, in order. Rendered as the `.steps` rail with `aria-current` on the
/// active phase (the guardrail is a *persistent* surface, not a beat, so it is not a step here).
const STEPS: [&str; 4] = ["Setup", "Ranked fits", "Checkpoint", "Grounded briefing"];

#[component]
pub fn App() -> Element {
    // The graded session lives for the whole app, interior-mutable and single-threaded.
    let ctx = AppCtx {
        session: use_hook(|| Rc::new(RefCell::new(build_session()))),
        phase: use_signal(|| Phase::Setup),
        seeker: use_signal(demo_seeker),
        choices: use_signal(demo_choices),
        recs: use_signal(Vec::new),
        measures: use_signal(HashMap::new),
        selected: use_signal(|| 0usize),
        request: use_signal(|| None),
        gate_proof: use_signal(|| None),
        signals: use_signal(|| None),
        briefing: use_signal(|| None),
        answer: use_signal(|| None),
        calls: use_signal(Vec::new),
    };
    use_context_provider(|| ctx.clone());

    let phase = *ctx.phase.read();
    let step = match phase {
        Phase::Setup => 0,
        Phase::Ranked => 1,
        Phase::Checkpoint => 2,
        Phase::Briefing => 3,
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/ziqpu.css") }

        div { class: "wrap",
            header {
                div { class: "brand",
                    // The brand mark — a bronze star in a ruled circle, inline so it inherits tokens.
                    svg {
                        class: "mark",
                        view_box: "0 0 40 40",
                        "aria-hidden": "true",
                        circle {
                            cx: "20",
                            cy: "20",
                            r: "18.5",
                            fill: "none",
                            stroke: "var(--gold)",
                            "stroke-width": "1",
                        }
                        path {
                            d: "M20 2.5V37.5M2.5 20H37.5",
                            stroke: "var(--line)",
                            "stroke-width": "1",
                        }
                        path {
                            d: "M20 6 L23.4 17 L34 17 L25.3 23.8 L28.6 34.5 L20 27.8 L11.4 34.5 L14.7 23.8 L6 17 L16.6 17 Z",
                            fill: "var(--gold)",
                        }
                        circle { cx: "20", cy: "20", r: "2.2", fill: "var(--bg)" }
                    }
                    div {
                        h1 { class: "word", "Ziqpu" }
                        div { class: "tagline", "the ledger of the sky · measured, not fate" }
                    }
                }
                button {
                    class: "theme",
                    r#type: "button",
                    onclick: move |_| {
                        // Flip data-theme on the document root, mirroring the mockup's toggle.
                        let _ = document::eval(
                            "const r=document.documentElement;\
                             const e=r.getAttribute('data-theme')||(window.matchMedia('(prefers-color-scheme:dark)').matches?'dark':'light');\
                             r.setAttribute('data-theme', e==='dark'?'light':'dark');",
                        );
                    },
                    "◐ theme"
                }
            }

            nav { class: "steps", role: "tablist", "aria-label": "Reading flow",
                {STEPS.iter().enumerate().map(|(i, label)| {
                    let cls = if i < step { "step done" } else { "step" };
                    let num = format!("{:02}", i + 1);
                    let current = if i == step { "true" } else { "false" };
                    rsx! {
                        div { key: "{i}", class: "{cls}", role: "tab", "aria-current": "{current}",
                            span { class: "n", "{num}" }
                            span { class: "t", "{label}" }
                        }
                    }
                })}
            }

            section { class: "phase", key: "{step}",
                {match phase {
                    Phase::Setup => rsx! { Setup {} },
                    Phase::Ranked => rsx! { Ranked {} },
                    Phase::Checkpoint => rsx! { Checkpoint {} },
                    Phase::Briefing => rsx! { Briefing {} },
                }}
            }

            if phase != Phase::Setup {
                Guardrail {}
            }
        }
    }
}
