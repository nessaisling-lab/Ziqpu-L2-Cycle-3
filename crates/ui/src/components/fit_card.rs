//! FitCard — one recommendation as a ledger entry: a band-colored left stripe, a bronze score
//! meter, the vizier's serif reading (the *meaning*), a persistent living wheat plot whose color and
//! form encode the compatibility's *health*, and the collapsible Backstage. Reads its recommendation
//! from the shared context by index so it stays reactive to selection without needing `PartialEq`
//! props.

use agents::{Fit, ReadMode};
use dioxus::prelude::*;

use crate::components::Backstage;
use crate::state::{fit_band_var, AppCtx};

/// The two-stop health palette for a card's living wheat, driven by the compatibility *band* on a
/// red → green → gold ramp. Returned as `(base, tip)`: `base` colors the stems (set as
/// `--wheat-color`), `tip` colors the grain heads (`--wheat-tip`), so every stalk ripens from its
/// stem up into its head. Misaligned wilts to a dry reddish-brown; Mixed is healthy malachite green;
/// Aligned greens toward gold; StronglyAligned is a radiant, warm harvest gold. Deliberately distinct
/// from the semantic band stripe (which reddens a misalign as an *alarm*) — grain wilts, it doesn't alarm.
fn wheat_colors(fit: Fit) -> (&'static str, &'static str) {
    match fit {
        Fit::Misaligned => ("#a34a2c", "#7c3320"), // wilted, dry reddish-brown → drier tip
        Fit::Mixed => ("#2f7d57", "#3f9a52"),      // healthy malachite green
        Fit::Aligned => ("#6f8f34", "#b79a2a"),    // green tending to gold — ripening
        Fit::StronglyAligned => ("#b07d12", "#d8a63a"), // radiant, warm harvest gold
    }
}

/// Form parameters for a card's living wheat plot, derived from its Fit band. Health is encoded in
/// FORM as well as color: a Misaligned plot is sparse, short, drooping and near-headless (wilting); a
/// StronglyAligned plot is many, tall, upright and heavy-headed (a radiant harvest). Read by
/// [`wheat_field`] to lay out the stalks.
struct WheatHealth {
    /// How many stalks fill the plot (sparse when wilted, a full stand when radiant).
    stalks: usize,
    /// Stalk height as a fraction of the plot's max (short = starved/wilted).
    height: f32,
    /// Static lean/bow in degrees applied per stalk (high = flopping over, wilting).
    droop: f32,
    /// Grain-kernel rows up each head (head fullness).
    rows: usize,
    /// Kernel size — heavy, full heads when radiant; small when wilted.
    kernel_ry: f32,
    /// Grain opacity — faded/sparse when wilted, solid when radiant.
    kernel_op: f32,
}

impl WheatHealth {
    fn from_fit(fit: Fit) -> Self {
        match fit {
            Fit::Misaligned => Self {
                stalks: 5,
                height: 0.64,
                droop: 26.0,
                rows: 1,
                kernel_ry: 3.5,
                kernel_op: 0.5,
            },
            Fit::Mixed => Self {
                stalks: 6,
                height: 0.86,
                droop: 9.0,
                rows: 2,
                kernel_ry: 4.4,
                kernel_op: 0.9,
            },
            Fit::Aligned => Self {
                stalks: 7,
                height: 0.95,
                droop: 5.0,
                rows: 3,
                kernel_ry: 4.8,
                kernel_op: 0.95,
            },
            Fit::StronglyAligned => Self {
                stalks: 8,
                height: 1.0,
                droop: 2.5,
                rows: 4,
                kernel_ry: 5.3,
                kernel_op: 1.0,
            },
        }
    }
}

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

/// One swaying wheat stalk in the plot — a curved stem with a grain head of paired kernels — laid
/// out from the field's baseline at horizontal slot `i` of `n`. Its height, lean/droop, head fullness
/// and kernel size come from the card's [`WheatHealth`]; its colors come from the card's
/// `--wheat-color` (stem) and `--wheat-tip` (grain) custom properties. The inner `<g>` applies the
/// static wilt lean; the outer `.stalk` group is what the CSS `sway` animation rotates about its base
/// (Nisaba, goddess of grain).
fn wheat_stalk(i: usize, n: usize, h: &WheatHealth) -> Element {
    let base = "var(--wheat-color,var(--band-strong))";
    let tip = "var(--wheat-tip,var(--gold))";
    // Base point along the field's floor; stalks spread evenly across the plot.
    let bx = if n <= 1 {
        52.0
    } else {
        14.0 + 76.0 * (i as f32 / (n as f32 - 1.0))
    };
    let by = 86.0_f32;
    // Deterministic per-stalk jitter so the stand looks natural without an RNG.
    let jitter = (((i * 37 + 11) % 13) as f32 / 13.0) - 0.5; // -0.5..0.5
    let height = 66.0 * h.height * (0.92 + 0.16 * (jitter + 0.5));
    let ty = by - height;
    // A gentle natural S-curve in the stem; the static lean below adds the wilt bow.
    let cx1 = bx + jitter * 1.6;
    let cy1 = by - height * 0.5;
    let cx2 = bx - jitter * 1.6;
    let cy2 = by - height * 0.85;
    let stem = format!("M{bx:.1} {by:.1} C{cx1:.1} {cy1:.1} {cx2:.1} {cy2:.1} {bx:.1} {ty:.1}");
    // Wilt lean: alternating sign for a natural spread; magnitude is the health's droop.
    let even = i.is_multiple_of(2);
    let lean = h.droop * if even { 1.0 } else { -0.85 };
    let rot = format!("rotate({lean:.1} {bx:.1} {by:.1})");
    // A blade leaf mid-stem — longer on healthier plants (a lushness cue), gone-short when wilted.
    let dir = if even { 1.0 } else { -1.0 };
    let leaf_y = by - height * 0.42;
    let leaf_len = 11.0 * h.height * dir;
    let leaf = format!(
        "M{bx:.1} {leaf_y:.1} q {q1:.1} -4 {q2:.1} 4",
        q1 = leaf_len * 0.6,
        q2 = leaf_len
    );
    // The grain head: a crown kernel plus `rows` paired teardrops stepping down from the tip.
    let rx = h.kernel_ry * 0.5;
    let ry = h.kernel_ry;
    let op = h.kernel_op;
    let mut head: Vec<Element> = Vec::new();
    head.push(rsx! {
        ellipse { key: "crown", cx: "{bx:.1}", cy: "{ty:.1}", rx: "{rx:.1}", ry: "{ry:.1}", fill: "{tip}", opacity: "{op}" }
    });
    for r in 0..h.rows {
        let ky = ty + 4.0 + r as f32 * (ry * 1.15);
        let lx = bx - 2.6;
        let rxx = bx + 2.6;
        let lt = format!("rotate(-32 {lx:.1} {ky:.1})");
        let rt = format!("rotate(32 {rxx:.1} {ky:.1})");
        head.push(rsx! {
            ellipse { key: "l{r}", cx: "{lx:.1}", cy: "{ky:.1}", rx: "{rx:.1}", ry: "{ry:.1}", fill: "{tip}", opacity: "{op}", transform: "{lt}" }
        });
        head.push(rsx! {
            ellipse { key: "r{r}", cx: "{rxx:.1}", cy: "{ky:.1}", rx: "{rx:.1}", ry: "{ry:.1}", fill: "{tip}", opacity: "{op}", transform: "{rt}" }
        });
    }
    let cls = ["stalk--a", "stalk--b", "stalk--c"][i % 3];
    rsx! {
        g { key: "{i}", class: "stalk {cls}",
            g { transform: "{rot}",
                path { d: "{leaf}", fill: "none", stroke: "{base}", "stroke-width": "1.4", "stroke-linecap": "round", opacity: "0.7" }
                path { d: "{stem}", fill: "none", stroke: "{base}", "stroke-width": "1.8", "stroke-linecap": "round" }
                {head.into_iter()}
            }
        }
    }
}

/// The card's living wheat plot — a small stand of ~5–8 stalks whose color and form encode the fit's
/// *health* (see [`wheat_colors`] + [`WheatHealth`]), swaying continuously in the wind. Rendered
/// persistently on every card (a living health indicator, not just a loader); while the reading is
/// pending it doubles as the loader with the "Consulting the viziers…" caption beside it. Pure inline
/// SVG; each `.stalk` sways via the CSS `sway` keyframes (always-on, ungated from OS reduce-motion).
fn wheat_field(fit: Fit, pending: bool) -> Element {
    let h = WheatHealth::from_fit(fit);
    let n = h.stalks;
    let stalks = (0..n).map(|i| wheat_stalk(i, n, &h)).collect::<Vec<_>>();
    // While the reading is being fetched, the plot doubles as the loader: it washes red → green →
    // gold (the owner-chosen loading beat) over the top of the sway, then settles to its health color.
    let cls = if pending {
        "wheat wheat--pending"
    } else {
        "wheat"
    };
    rsx! {
        svg {
            class: "{cls}",
            view_box: "0 0 104 92",
            "preserveAspectRatio": "xMidYMax meet",
            fill: "none",
            "aria-hidden": "true",
            {stalks.into_iter()}
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

    let band = fit_band_var(rec.fit);
    let (wheat_base, wheat_tip) = wheat_colors(rec.fit);
    let label = rec.fit.label();
    let name = rec.name.clone();
    let ticker = rec.choice.clone();
    let score = rec.score;

    // Resolve the reading + its provenance badge by the active display mode. Each mode keeps its own
    // cache, so switching is a pure display swap:
    //   Raw   → the deterministic local template (always present the instant the ranking paints).
    //   Local → the user's LM Studio cache; the wheat loader shows while its off-thread fetch runs.
    //   Live  → the streamed hosted-model prose in `rec.reading`; loader while pending.
    let mode = *ctx.mode.read();
    let (source_text, is_pending, source_badge) = match mode {
        ReadMode::Raw => {
            let text = ctx
                .raw_readings
                .read()
                .get(&ticker)
                .cloned()
                .unwrap_or_else(|| rec.reading.clone());
            let badge = rsx! {
                span { class: "src-badge src-badge--tmpl", "○ raw · local template" }
            };
            (text, false, badge)
        }
        ReadMode::Local => match ctx.local_readings.read().get(&ticker).cloned() {
            Some(text) => {
                // Source is `Some("local · <model>")` on success, `None` on template fallback.
                let model = ctx.local_sources.read().get(&ticker).cloned().flatten();
                let badge = match model {
                    Some(m) => rsx! {
                        span { class: "src-badge src-badge--live",
                            span { class: "spark", "◐" }
                            "{m}"
                        }
                    },
                    None => rsx! {
                        span { class: "src-badge src-badge--tmpl", "◐ local · template" }
                    },
                };
                (text, false, badge)
            }
            // Not cached yet → loader (its fetch was kicked off when the toggle reached Local).
            None => (String::new(), true, rsx! {}),
        },
        ReadMode::Live => {
            let is_pending = ctx.pending.read().contains(&ticker);
            let model = ctx.sources.read().get(&ticker).cloned().flatten();
            let badge = match &model {
                Some(m) => rsx! {
                    span { class: "src-badge src-badge--live",
                        span { class: "spark", "✦" }
                        "{m} · live"
                    }
                },
                None => rsx! {
                    span { class: "src-badge src-badge--tmpl", "○ offline · template" }
                },
            };
            (rec.reading.clone(), is_pending, badge)
        }
    };

    // Strip the display chrome: the redundant leading "FIT: … — name" line (the header shows it) and
    // the trailing "REMINDER: … not financial advice" (now shown once in the persistent footer, not
    // on every card). The disclaimer stays in the reading data + the guardrail — see strip_reading_chrome.
    let reading = crate::state::strip_reading_chrome(&source_text);
    let card_cls = if is_selected {
        "card card--fit card--selected"
    } else {
        "card card--fit"
    };
    let choice = rec.choice.clone();

    rsx! {
        article {
            class: "{card_cls}",
            style: "--band:var({band});--pct:{score}%;--wheat-color:{wheat_base};--wheat-tip:{wheat_tip}",
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

            // The living wheat plot + the reading share a row: the prose (or, while pending, the
            // "Consulting the viziers…" caption) on the left, and a persistent stand of health-colored
            // wheat filling the card's spare rightward space on the right. The plot is ALWAYS shown — it
            // is a living health indicator, not just a loader — and its continuous sway doubles as the
            // pending state. Both color and form track `rec.fit` (see `wheat_colors`/`WheatHealth`): a
            // dry, sparse, drooping rust wilt for a Misaligned fit through to a tall, full, upright
            // radiant-gold harvest for a Strongly Aligned one.
            div { class: "card__mid",
                div { class: "card__read",
                    if is_pending {
                        span { class: "wheat-caption", "Consulting the viziers…" }
                    } else {
                        p { class: "reading", "{reading}" }
                        // Provenance badge (computed above): Raw mode is always the muted "○ raw · local"
                        // chip; Live is the lapis/gold "✦ {model} · live" or the "○ offline · template".
                        {source_badge}
                    }
                }
                aside { class: "wheat-field", "aria-hidden": "true", {wheat_field(rec.fit, is_pending)} }
            }

            Backstage { choice }
        }
    }
}
