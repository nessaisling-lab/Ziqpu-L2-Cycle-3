//! The root component: sets up the session + context, renders the header/stepper, switches on the
//! current phase, and mounts the persistent guardrail from the Ranked step onward.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{demo_choices, demo_seeker};
use dioxus::prelude::*;

use crate::components::{Briefing, Checkpoint, Guardrail, Ranked, Setup};
use crate::state::{build_session, AppCtx, Phase};

const STEPS: [&str; 4] = [
    "Seeded demo",
    "Ranked fits",
    "Checkpoint",
    "Grounded briefing",
];

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

        header { class: "app-header",
            div { class: "brand",
                h1 { "Ziqpu" }
                span { class: "tagline", "the ledger of the sky — measured, not fate" }
            }
            nav { class: "stepper",
                {STEPS.iter().enumerate().map(|(i, label)| {
                    let cls = if i == step {
                        "step current"
                    } else if i < step {
                        "step done"
                    } else {
                        "step"
                    };
                    let num = i + 1;
                    rsx! {
                        span { key: "{i}", class: "{cls}",
                            span { class: "step-num", "{num}" }
                            span { class: "step-label", "{label}" }
                        }
                    }
                })}
            }
        }

        main { class: "app-main",
            {match phase {
                Phase::Setup => rsx! { Setup {} },
                Phase::Ranked => rsx! { Ranked {} },
                Phase::Checkpoint => rsx! { Checkpoint {} },
                Phase::Briefing => rsx! { Briefing {} },
            }}

            if phase != Phase::Setup {
                Guardrail {}
            }
        }
    }
}
