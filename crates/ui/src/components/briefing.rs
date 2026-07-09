//! Briefing — the ACT output. Sets the chart read beside the real (grounded) signals. "Start over"
//! rebuilds a fresh session and clears the reactive state back to Setup.

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::state::{build_session, AppCtx, Phase};

#[component]
pub fn Briefing() -> Element {
    let ctx = use_context::<AppCtx>();

    let reading = ctx
        .briefing
        .read()
        .as_ref()
        .map(|b| b.reading.clone())
        .unwrap_or_default();

    let grounded = {
        let signals = ctx.signals.read();
        match signals.as_ref() {
            Some(s) => {
                let source = s.source.clone();
                let items = s
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let item = item.clone();
                        rsx! { li { key: "{i}", "{item}" } }
                    })
                    .collect::<Vec<_>>();
                rsx! {
                    div { class: "grounded",
                        div { class: "grounded-source", "Grounded source: {source}" }
                        ul { class: "grounded-items", {items.into_iter()} }
                    }
                }
            }
            None => rsx! {},
        }
    };

    rsx! {
        section { class: "card briefing",
            h2 { "Grounded briefing" }
            pre { class: "reading", "{reading}" }
            {grounded}

            div { class: "actions",
                button {
                    class: "ghost",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            *ctx.session.borrow_mut() = build_session();
                            ctx.recs.set(Vec::new());
                            ctx.measures.set(HashMap::new());
                            ctx.selected.set(0);
                            ctx.request.set(None);
                            ctx.gate_proof.set(None);
                            ctx.signals.set(None);
                            ctx.briefing.set(None);
                            ctx.answer.set(None);
                            ctx.calls.set(Vec::new());
                            ctx.phase.set(Phase::Setup);
                        }
                    },
                    "Start over"
                }
            }
        }
    }
}
