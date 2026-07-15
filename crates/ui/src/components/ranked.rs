//! Ranked — the DECIDE output. Recommendations are already best-fit first; each renders as a
//! FitCard ledger entry. Selecting one and grounding it proposes the checkpoint (and proves the
//! gate blocks the costed pull without a human's approval).

use agents::ReadMode;
use dioxus::prelude::*;

use crate::components::FitCard;
use crate::state::{run_draft, AppCtx, Phase};

#[component]
pub fn Ranked() -> Element {
    let ctx = use_context::<AppCtx>();
    let count = ctx.recs.read().len();

    // In Local mode the readings are fetched off-thread (after the model loads), so show how many of
    // the cards have landed — a global "it's working" cue while the per-card wheat loaders spin.
    let local_ready = ctx.local_readings.read().len();
    let local_working = *ctx.mode.read() == ReadMode::Local && count > 0 && local_ready < count;

    rsx! {
        p { class: "eyebrow", "Hamun-ana measured · Ungasaga read" }
        p { class: "lead",
            "{count} choices, weighed against your chart and ranked best-fit first. "
            em { "Pick one to ground against real signals." }
        }

        if local_working {
            div { class: "local-progress",
                span { class: "local-progress-label",
                    "Reading on your machine — {local_ready} of {count} ready"
                }
                div { class: "progress",
                    div { class: "progress-fill progress-fill--indeterminate" }
                }
            }
        }

        {(0..count).map(|i| rsx! { FitCard { key: "{i}", index: i } })}

        div { class: "actions", style: "justify-content:flex-start;margin-top:4px",
            button {
                class: "btn btn--go",
                r#type: "button",
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
                        // Put the pause to work: the local model drafts the interpreter's framing
                        // brief off-thread while the human decides whether to approve. It sees only
                        // the measures, never external data, so it can't touch the gated pull early.
                        run_draft(ctx.clone(), choice.clone());
                        ctx.phase.set(Phase::Checkpoint);
                    }
                },
                "Ground this read →"
            }
        }
    }
}
