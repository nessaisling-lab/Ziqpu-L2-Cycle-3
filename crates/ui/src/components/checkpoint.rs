//! Checkpoint — the graded human-in-the-loop gate, rendered as a held "seal" moment. It shows the
//! proposal, *proves* the gate blocks without approval, and only on Approve mints a token, pulls
//! grounded signals, and briefs.

use dioxus::prelude::*;

use crate::state::{pretty_call, AppCtx, Phase};

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

    // The prompt is one sentence ("Ground this read for TSLA? I'll pull …"); split it into the
    // seal's question (heading) and its costed-call explanation (body).
    let (question, explanation) = match prompt.split_once('?') {
        Some((q, rest)) => (format!("{q}?"), rest.trim().to_string()),
        None => (prompt.clone(), String::new()),
    };

    rsx! {
        p { class: "eyebrow", "Human-in-the-loop · the costed step" }
        div { class: "gate",
            svg {
                class: "seal",
                view_box: "0 0 54 54",
                "aria-hidden": "true",
                circle {
                    cx: "27",
                    cy: "27",
                    r: "24",
                    fill: "none",
                    stroke: "var(--gold)",
                    "stroke-width": "1.2",
                }
                circle {
                    cx: "27",
                    cy: "27",
                    r: "17",
                    fill: "none",
                    stroke: "var(--line)",
                    "stroke-width": "1",
                }
                path {
                    d: "M27 12v9m0 12v9m-15-15h9m12 0h9",
                    fill: "none",
                    stroke: "var(--gold)",
                    "stroke-width": "1.4",
                }
                circle { cx: "27", cy: "27", r: "3.4", fill: "var(--gold)" }
            }
            h3 { "{question}" }
            if !explanation.is_empty() {
                p { "{explanation}" }
            }
            div { class: "blocked", "↳ attempt without approval → blocked: {proof}" }

            div { class: "actions",
                button {
                    class: "btn",
                    r#type: "button",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            ctx.request.set(None);
                            ctx.gate_proof.set(None);
                            ctx.phase.set(Phase::Ranked);
                        }
                    },
                    "Keep the symbolic read"
                }
                button {
                    class: "btn btn--go",
                    r#type: "button",
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
                                session.calls().iter().map(pretty_call).collect()
                            };

                            ctx.signals.set(Some(signals));
                            ctx.briefing.set(Some(briefing));
                            ctx.calls.set(calls);
                            ctx.phase.set(Phase::Briefing);
                        }
                    },
                    "Approve & ground"
                }
            }
        }
    }
}
