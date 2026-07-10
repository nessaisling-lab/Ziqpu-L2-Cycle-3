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

/// One swaying wheat stalk — a vertical stem with paired grain kernels up the top third, stroked and
/// filled in the malachite/green brand token (`--band-strong`), so the loader reads as a stalk of
/// green grain bending in the wind. Wrapped in an outer `<g>` that statically translates it into
/// place; the inner `.stalk` group is what the CSS `sway` animation rotates about its base (Nisaba,
/// goddess of grain).
fn wheat_stalk(cls: &str, dx: f32) -> Element {
    // Kernel row centers down the stem's top third; each row gets a left + right teardrop.
    let kernels = [21.0_f32, 28.0, 35.0, 42.0];
    let pods = kernels
        .iter()
        .enumerate()
        .flat_map(|(i, &y)| {
            let lt = format!("rotate(-34 20 {y})");
            let rt = format!("rotate(34 28 {y})");
            [
                rsx! { ellipse { key: "l{i}", cx: "20", cy: "{y}", rx: "2.6", ry: "5.1", fill: "var(--band-strong)", opacity: "0.9", transform: "{lt}" } },
                rsx! { ellipse { key: "r{i}", cx: "28", cy: "{y}", rx: "2.6", ry: "5.1", fill: "var(--band-strong)", opacity: "0.9", transform: "{rt}" } },
            ]
        })
        .collect::<Vec<_>>();
    let translate = format!("translate({dx} 0)");
    rsx! {
        g { transform: "{translate}",
            g { class: "stalk {cls}",
                // the stem
                path {
                    d: "M24 74 C24 60 24 44 24 18",
                    fill: "none",
                    stroke: "var(--band-strong)",
                    "stroke-width": "1.6",
                    "stroke-linecap": "round",
                }
                // crowning kernel at the tip
                ellipse { cx: "24", cy: "14", rx: "2.6", ry: "5.4", fill: "var(--band-strong)" }
                {pods.into_iter()}
            }
        }
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
    // Live vs Raw display mode (a pure display switch — both readings are already computed).
    let live_mode = *ctx.live_mode.read();
    // While this ticker's live prose is still being fetched off-thread, show the swaying loader in
    // place of the (empty) reading. In Raw mode there is nothing to wait for — the template reading is
    // always present — so the loader never shows.
    let is_pending = live_mode && ctx.pending.read().contains(&rec.choice);

    let band = fit_band_var(rec.fit);
    let label = rec.fit.label();
    let name = rec.name.clone();
    let ticker = rec.choice.clone();
    // Pick the source text: Live streams the model prose into `rec.reading`; Raw uses the deterministic
    // local template from `raw_readings` (always present the instant the ranking paints).
    let raw_text = ctx.raw_readings.read().get(&ticker).cloned();
    let source_text = if live_mode {
        rec.reading.clone()
    } else {
        raw_text.unwrap_or_else(|| rec.reading.clone())
    };
    // The card header already shows band + score + name, so strip a leading redundant
    // "FIT: <band> (score) — name" line from the prose if the interpreter emitted one; keep the
    // warm reading + why + REMINDER intact.
    let reading = match source_text.split_once('\n') {
        Some((first, rest)) if first.trim_start().starts_with("FIT:") => {
            rest.trim_start().to_string()
        }
        _ => source_text,
    };
    let score = rec.score;
    // Provenance of the finished reading: Some(model) => live API, None => template fallback. Only
    // meaningful once the fill has landed (i.e. not pending).
    let source_model = ctx.sources.read().get(&ticker).cloned().flatten();
    // The "who wrote this" chip. Raw mode: always "○ raw · local" (deterministic template, no model).
    // Live mode: "✦ {model} · live" when the API produced the prose, else the template-fallback chip.
    let source_badge = if !live_mode {
        rsx! { span { class: "src-badge src-badge--tmpl", "○ raw · local" } }
    } else {
        match &source_model {
            Some(model) => rsx! {
                span { class: "src-badge src-badge--live",
                    span { class: "spark", "✦" }
                    "{model} · live"
                }
            },
            None => rsx! {
                span { class: "src-badge src-badge--tmpl", "○ offline · template" }
            },
        }
    };
    let card_cls = if is_selected {
        "card card--fit card--selected"
    } else {
        "card card--fit"
    };
    let choice = rec.choice.clone();

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

            if is_pending {
                // The grain-in-the-wind loader — a small field of gold wheat stalks swaying while the
                // vizier's reading is fetched. Static upright fallback under prefers-reduced-motion.
                div { class: "wheat-loader",
                    svg {
                        class: "wheat",
                        view_box: "0 0 96 84",
                        fill: "none",
                        "aria-hidden": "true",
                        {wheat_stalk("stalk--a", 4.0)}
                        {wheat_stalk("stalk--b", 24.0)}
                        {wheat_stalk("stalk--c", 44.0)}
                    }
                    span { class: "wheat-caption", "Consulting the viziers…" }
                }
            } else {
                p { class: "reading", "{reading}" }
                // Provenance badge (computed above): Raw mode is always the muted "○ raw · local" chip;
                // Live mode is the lapis/gold "✦ {model} · live" or the "○ offline · template" fallback.
                {source_badge}
            }

            Backstage { choice }
        }
    }
}
