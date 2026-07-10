//! FitCard — one recommendation as a ledger entry: a band-colored left stripe, a bronze score
//! meter, the vizier's serif reading (the *meaning*), a mono measured line (the *data*), and the
//! collapsible Backstage. Reads its recommendation from the shared context by index so it stays
//! reactive to selection without needing `PartialEq` props.

use dioxus::prelude::*;

use crate::components::Backstage;
use crate::state::{fit_band_var, AppCtx};

#[component]
pub fn FitCard(index: usize) -> Element {
    let ctx = use_context::<AppCtx>();

    let rec = ctx.recs.read().get(index).cloned();
    let Some(rec) = rec else {
        return rsx! {};
    };

    let selected = *ctx.selected.read();
    let is_selected = selected == index;

    let band = fit_band_var(rec.fit);
    let label = rec.fit.label();
    let name = rec.name.clone();
    let ticker = rec.choice.clone();
    let reading = rec.reading.clone();
    let score = rec.score;
    let card_cls = if is_selected {
        "card card--fit card--selected"
    } else {
        "card card--fit"
    };
    let choice = rec.choice.clone();

    // The measured line binds to the tightest contacts (Measures.top), flowing in lapis / friction
    // in terracotta — the "data" half of the card's typographic duality.
    let tops = ctx
        .measures
        .read()
        .get(&ticker)
        .map(|m| m.top.clone())
        .unwrap_or_default();

    rsx! {
        article {
            class: "{card_cls}",
            style: "--band:var({band});--pct:{score}%",
            onclick: {
                let mut ctx = ctx.clone();
                move |_| ctx.selected.set(index)
            },

            div { class: "card__top",
                h2 { class: "card__name", "{name}" }
                span { class: "ticker", "{ticker}" }
                span { class: "badge", "{label}" }
            }

            div { class: "meter", i {} }
            div { class: "score",
                b { "{score}" }
                " / 100"
            }

            p { class: "reading", "{reading}" }

            p { class: "measured",
                if tops.is_empty() {
                    span { class: "orb", "no close contacts between the two charts" }
                } else {
                    {tops.iter().take(3).enumerate().map(|(i, a)| {
                        let cls = if a.harmonious { "flow" } else { "fric" };
                        let contact = format!("{} {} {}", a.body_a, a.aspect, a.body_b);
                        let orb = format!("{:.1}°", a.orb);
                        let sep = if i > 0 { " · " } else { "" };
                        rsx! {
                            span { key: "{i}",
                                "{sep}"
                                span { class: "{cls}", "{contact}" }
                                " "
                                span { class: "orb", "{orb}" }
                            }
                        }
                    })}
                }
            }

            Backstage { choice }
        }
    }
}
