//! FitCard — one recommendation: a four-band badge, a 0–100 score meter, the reading prose, and a
//! collapsible Backstage. Reads its recommendation from the shared context by index so it stays
//! reactive to selection without needing `PartialEq` props.

use dioxus::prelude::*;

use crate::components::Backstage;
use crate::state::{fit_class, AppCtx};

#[component]
pub fn FitCard(index: usize) -> Element {
    let ctx = use_context::<AppCtx>();

    let rec = ctx.recs.read().get(index).cloned();
    let Some(rec) = rec else {
        return rsx! {};
    };

    let selected = *ctx.selected.read();
    let is_selected = selected == index;

    let band = fit_class(rec.fit);
    let label = rec.fit.label();
    let name = rec.name.clone();
    let ticker = rec.choice.clone();
    let reading = rec.reading.clone();
    let score = rec.score;
    let card_cls = if is_selected {
        "fit-card selected"
    } else {
        "fit-card"
    };
    let choice = rec.choice.clone();

    rsx! {
        div {
            class: "{card_cls}",
            onclick: {
                let mut ctx = ctx.clone();
                move |_| ctx.selected.set(index)
            },

            div { class: "fit-head",
                div { class: "fit-title",
                    span { class: "fit-name", "{name}" }
                    span { class: "fit-ticker", "{ticker}" }
                }
                span { class: "badge {band}", "{label}" }
            }

            div { class: "meter",
                div { class: "meter-track",
                    div { class: "meter-fill {band}", style: "width: {score}%;" }
                }
                span { class: "meter-num", "{score} / 100" }
            }

            pre { class: "fit-reading", "{reading}" }

            Backstage { choice }
        }
    }
}
