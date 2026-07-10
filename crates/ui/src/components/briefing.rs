//! Briefing — the ACT output. Sets the vizier's grounded reading beside the real (pulled) signals,
//! as one ledger card. "Start over" rebuilds a fresh session and clears the reactive state to Setup.

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::state::{build_session, fit_band_var, AppCtx, Phase};

#[component]
pub fn Briefing() -> Element {
    let ctx = use_context::<AppCtx>();

    let reading = ctx
        .briefing
        .read()
        .as_ref()
        .map(|b| b.reading.clone())
        .unwrap_or_default();

    let signals = ctx.signals.read().clone();

    // Recover the graded choice's fit band + score from the ranked recs, so the grounded card wears
    // the same stripe, badge, and meter it did in the ranking.
    let header = signals.as_ref().and_then(|s| {
        ctx.recs
            .read()
            .iter()
            .find(|r| r.choice == s.choice)
            .map(|r| {
                (
                    r.name.clone(),
                    r.choice.clone(),
                    fit_band_var(r.fit),
                    r.fit.label(),
                    r.score,
                )
            })
    });

    let (name, ticker, band, label, score) = header.unwrap_or_else(|| {
        (
            "Grounded".to_string(),
            String::new(),
            "--band-strong",
            "",
            50,
        )
    });

    let items = signals
        .as_ref()
        .map(|s| {
            s.items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let item = item.clone();
                    rsx! { li { key: "{i}", "— {item}" } }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let source = signals
        .as_ref()
        .map(|s| s.source.clone())
        .unwrap_or_default();

    rsx! {
        p { class: "eyebrow", "Act · grounded, still reflection" }
        article { class: "card", style: "--band:var({band});--pct:{score}%",
            div { class: "card__top",
                h2 { class: "card__name", "{name}" }
                if !ticker.is_empty() {
                    span { class: "ticker", "{ticker}" }
                }
                if !label.is_empty() {
                    span { class: "badge", "{label}" }
                }
            }
            div { class: "meter", i {} }

            p { class: "reading", "{reading}" }

            if signals.is_some() {
                p { class: "measured grounded-line", "GROUNDED · {source}" }
                ul { class: "grounded-items", {items.into_iter()} }
            }
            p { class: "measured grounded-reminder",
                "REMINDER: measured, not fate — not financial advice."
            }
        }

        div { class: "actions", style: "justify-content:flex-start;margin-top:4px",
            button {
                class: "btn",
                r#type: "button",
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
                "↺ Start over"
            }
        }
    }
}
