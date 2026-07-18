//! Checkpoint — the graded human-in-the-loop gate, rendered as a held "seal" moment. It shows the
//! proposal, *proves* the gate blocks without approval, and only on Approve mints a token, pulls
//! grounded signals, and briefs.

use agents::ReadMode;
use dioxus::prelude::*;

use crate::components::PremiumLock;
use crate::state::{run_grounding, AppCtx, Phase};

#[component]
pub fn Checkpoint() -> Element {
    let ctx = use_context::<AppCtx>();

    // While the approved pull runs off-thread, hold the checkpoint on a "grounding…" loading view so
    // the window stays responsive (the fetch + brief no longer run on the event loop). The `grounder`
    // coroutine flips this off and advances to Briefing when the worker lands.
    if *ctx.grounding.read() {
        return rsx! {
            p { class: "eyebrow", "Human-in-the-loop · grounding the read" }
            div { class: "gate gate--grounding",
                div { class: "ask-reading ask-reading--pending",
                    span { class: "wheat-caption",
                        "Pulling the real record (SEC EDGAR + Wikipedia) and writing the grounded briefing… the window stays live."
                    }
                }
            }
        };
    }

    let prompt = ctx
        .request
        .read()
        .as_ref()
        .map(|r| r.prompt.clone())
        .unwrap_or_default();
    let proof = ctx.gate_proof.read().clone().unwrap_or_default();

    // The layered pipeline's "interpreter's brief", drafted by the local model during this pause.
    // It only guides the Live grounded read, so the panel is shown only in Live mode. In the dev
    // build the brief is editable before it's sent; a free customer sees it read-only with a 🔒.
    let mode = *ctx.mode.read();
    let brief_pending = *ctx.draft_pending.read();
    let brief = ctx.draft.read().clone();
    let premium = ctx.premium();
    let mut draft_sig = ctx.draft;

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

            // The interpreter's brief — drafted locally, for free, while you decide. Only the Live
            // grounded read consumes it, so it appears only in Live mode.
            if mode == ReadMode::Live {
                div { class: "brief",
                    p { class: "brief__eyebrow", "Prepared during the pause · the interpreter's brief" }
                    if brief_pending {
                        span { class: "wheat-caption", "Your local model is drafting the brief…" }
                    } else if let Some(text) = brief {
                        if premium {
                            textarea {
                                class: "brief__edit",
                                rows: "4",
                                value: "{text}",
                                oninput: move |e| draft_sig.set(Some(e.value())),
                            }
                            p { class: "brief__hint",
                                "Edit before it sends — or clear it to let the frontier frame the reading itself."
                            }
                        } else {
                            pre { class: "brief__text", "{text}" }
                            div { class: "brief__locked",
                                PremiumLock { label: "Edit the brief".to_string() }
                                span { class: "brief__hint", "Free readings send the brief as drafted." }
                            }
                        }
                    } else {
                        span { class: "wheat-caption",
                            "No local model reachable — the frontier will frame the reading itself."
                        }
                    }
                }
            }

            div { class: "actions",
                button {
                    class: "btn",
                    r#type: "button",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            ctx.request.set(None);
                            ctx.gate_proof.set(None);
                            // Drop the drafted brief — declining ends this grounding attempt.
                            ctx.draft.set(None);
                            ctx.draft_pending.set(false);
                            ctx.phase.set(Phase::Ranked);
                        }
                    },
                    "Keep the symbolic read"
                }
                button {
                    class: "btn btn--go",
                    r#type: "button",
                    onclick: {
                        let ctx = ctx.clone();
                        // Approve mints the token and records the gated PullGrounded on the real
                        // session synchronously (tool-order log + gate preserved); the blocking fetch
                        // + grounded-brief then run off-thread, so the window never freezes here.
                        move |_| run_grounding(ctx.clone())
                    },
                    "Approve & ground"
                }
            }
        }
    }
}
