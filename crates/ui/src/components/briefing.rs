//! Briefing — the ACT output. Sets the vizier's grounded reading beside the real (pulled) signals,
//! as one ledger card. A rung badge (`GROUNDED · LIVE` / `GROUNDED · LOCAL` / `LOCAL · UNSOURCED` /
//! `GROUNDED`) says truthfully how much reality backs the words; for an **unsourced** read the raw
//! signals panel is hidden — the reading stands on the charts alone and says so. "Start over"
//! rebuilds a fresh session and clears the reactive state to Setup.

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

    // Which rung produced this read — drives the badge and whether the raw-signals panel shows.
    // `None` (pre-pipeline state) defaults to the sourced "GROUNDED" behavior for back-compat.
    let rung = *ctx.rung.read();
    let rung_badge = rung.map(|r| r.badge()).unwrap_or("GROUNDED");
    let sourced = rung.map(|r| r.is_sourced()).unwrap_or(true);

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

    // The card header already shows name + band + score, so strip a leading redundant
    // "FIT: <band> (score) — name" line if the interpreter emitted one; keep the warm body, the
    // grounded/reality beat (or the unsourced note), and the REMINDER intact.
    let reading = match reading.split_once('\n') {
        Some((first, rest)) if first.trim_start().starts_with("FIT:") => {
            rest.trim_start().to_string()
        }
        _ => reading,
    };

    // The rung badge's tone: unsourced reads out in a cautionary hue, the rest in the grounded gold.
    let rung_cls = if sourced {
        "badge badge--rung"
    } else {
        "badge badge--rung badge--unsourced"
    };

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
                span { class: "{rung_cls}", "{rung_badge}" }
            }
            div { class: "meter", i {} }

            p { class: "reading", "{reading}" }

            // The raw pulled signals — the receipts — only when the read is actually sourced. An
            // unsourced read hides this panel entirely rather than show an empty/placeholder source
            // under a "LOCAL · UNSOURCED" badge (which would contradict it).
            if signals.is_some() && sourced {
                p { class: "measured grounded-line", "GROUNDED · {source}" }
                ul { class: "grounded-items", {items.into_iter()} }
            }
            // No separate REMINDER line here: the reading is self-contained and already ends on the
            // rung-correct REMINDER (an unsourced read's carries "— and unsourced").
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
                        ctx.grounding.set(false);
                        ctx.answer.set(None);
                        ctx.calls.set(Vec::new());
                        // Clear the layered-pipeline state too, so a fresh run starts clean.
                        ctx.draft.set(None);
                        ctx.draft_pending.set(false);
                        ctx.rung.set(None);
                        ctx.phase.set(Phase::Setup);
                    }
                },
                "↺ Start over"
            }
        }
    }
}
