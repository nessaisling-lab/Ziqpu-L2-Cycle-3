//! Ranked — the DECIDE output. Recommendations are already best-fit first; each renders as a
//! FitCard. Selecting one and grounding it proposes the checkpoint (and proves the gate blocks).

use dioxus::prelude::*;

use crate::components::FitCard;
use crate::state::{AppCtx, Phase};

#[component]
pub fn Ranked() -> Element {
    let ctx = use_context::<AppCtx>();
    let count = ctx.recs.read().len();

    rsx! {
        section { class: "card ranked",
            h2 { "Ranked fits" }
            p { class: "muted", "Best-fit first. Pick one to ground against real signals." }

            div { class: "cards",
                {(0..count).map(|i| rsx! { FitCard { key: "{i}", index: i } })}
            }

            div { class: "actions",
                button {
                    class: "primary",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            let selected = *ctx.selected.read();
                            let ticker = ctx.recs.read().get(selected).map(|r| r.choice.clone());
                            let Some(ticker) = ticker else { return };
                            let choice = ctx
                                .choices
                                .read()
                                .iter()
                                .find(|c| c.ticker == ticker)
                                .cloned();
                            let Some(choice) = choice else { return };

                            let request = ctx.session.borrow().propose_grounding(&choice);
                            // Prove the gate: without a token, the costed pull is refused *before*
                            // any external call — and this early-return never touches the calls log.
                            let proof = {
                                let mut session = ctx.session.borrow_mut();
                                match session.pull_grounded(&choice, None) {
                                    Err(e) => e.to_string(),
                                    Ok(_) => "BUG: pulled without approval".to_string(),
                                }
                            };

                            ctx.request.set(Some(request));
                            ctx.gate_proof.set(Some(proof));
                            ctx.phase.set(Phase::Checkpoint);
                        }
                    },
                    "Ground this read →"
                }
            }
        }
    }
}
