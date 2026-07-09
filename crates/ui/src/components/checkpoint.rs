//! Checkpoint — the graded human-in-the-loop gate. It shows the proposal, *proves* the gate blocks
//! without approval, and only on Approve mints a token, pulls grounded signals, and briefs.

use dioxus::prelude::*;

use crate::state::{AppCtx, Phase};

#[component]
pub fn Checkpoint() -> Element {
    let ctx = use_context::<AppCtx>();

    let prompt = ctx
        .request
        .read()
        .as_ref()
        .map(|r| r.prompt.clone())
        .unwrap_or_default();
    let proof = ctx.gate_proof.read().clone().unwrap_or_default();

    rsx! {
        section { class: "card checkpoint",
            h2 { "Checkpoint" }
            p { class: "prompt", "{prompt}" }

            div { class: "gate-proof",
                span { class: "gate-label", "Gate check — pull attempted with no approval:" }
                code { "blocked: {proof}" }
            }

            div { class: "actions",
                button {
                    class: "ghost",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            ctx.request.set(None);
                            ctx.gate_proof.set(None);
                            ctx.phase.set(Phase::Ranked);
                        }
                    },
                    "Decline — keep the symbolic read"
                }
                button {
                    class: "primary",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            // Move the non-Copy request out of the signal.
                            let request = ctx.request.write().take();
                            let Some(request) = request else { return };
                            let ticker = request.choice.clone();
                            let choice = ctx
                                .choices
                                .read()
                                .iter()
                                .find(|c| c.ticker == ticker)
                                .cloned();
                            let Some(choice) = choice else { return };

                            // Approve mints the token; the costed pull is now reachable.
                            let token = ctx.session.borrow().approve(request);
                            let seeker = ctx.seeker.read().clone();
                            let pulled = {
                                let mut session = ctx.session.borrow_mut();
                                session.pull_grounded(&choice, Some(&token))
                            };
                            let Ok(signals) = pulled else { return };
                            let briefing = ctx.session.borrow().brief(&seeker, &choice, &signals);
                            let calls: Vec<String> = {
                                let session = ctx.session.borrow();
                                session.calls().iter().map(|c| format!("{c:?}")).collect()
                            };

                            ctx.signals.set(Some(signals));
                            ctx.briefing.set(Some(briefing));
                            ctx.calls.set(calls);
                            ctx.phase.set(Phase::Briefing);
                        }
                    },
                    "Approve — pull grounded signals"
                }
            }
        }
    }
}
