//! FitCard — one recommendation as a ledger entry: a band-colored left stripe, a bronze score
//! meter, the vizier's serif reading (the *meaning*), a mono measured line (the *data*), and the
//! collapsible Backstage. Reads its recommendation from the shared context by index so it stays
//! reactive to selection without needing `PartialEq` props.

use agents::Fit;
use dioxus::prelude::*;

use crate::components::Backstage;
use crate::state::{fit_band_var, AppCtx};

/// The card's rune tile — a small band-colored glyph that reads at a glance: a star for a good fit,
/// a clock for a mixed one, an X for a misaligned one. Stroked in `currentColor` so the `.rune`
/// tile's `--band` color flows through.
fn rune(fit: Fit) -> Element {
    match fit {
        Fit::StronglyAligned | Fit::Aligned => rsx! {
            svg {
                class: "i",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M11.5 2.3a.53.53 0 0 1 .95 0l2.31 4.68a2.1 2.1 0 0 0 1.6 1.16l5.16.75a.53.53 0 0 1 .3.91l-3.74 3.64a2.1 2.1 0 0 0-.61 1.88l.88 5.14a.53.53 0 0 1-.77.56l-4.62-2.43a2.1 2.1 0 0 0-1.97 0L6.4 21a.53.53 0 0 1-.77-.56l.88-5.14a2.1 2.1 0 0 0-.61-1.88L2.16 9.79a.53.53 0 0 1 .29-.9l5.17-.76a2.1 2.1 0 0 0 1.6-1.16z" }
            }
        },
        Fit::Mixed => rsx! {
            svg {
                class: "i",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M12 6v6l4 2" }
            }
        },
        Fit::Misaligned => rsx! {
            svg {
                class: "i",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                path { d: "M18 6 6 18M6 6l12 12" }
            }
        },
    }
}

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
                div { class: "rune", "aria-hidden": "true", {rune(rec.fit)} }
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
