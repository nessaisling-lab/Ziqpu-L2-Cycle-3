//! The root component: sets up the session + context, renders the header/stepper, switches on the
//! current phase, and mounts the persistent guardrail from the Ranked step onward.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use agents::{demo_choices, demo_seeker, GroundedSignals, ReadMode, Recommendation};
use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::components::{Briefing, Checkpoint, Guardrail, Legend, Ranked, Setup};
use crate::state::{build_session, ensure_local_readings, next_mode, seeded_stars, AppCtx, Phase};

/// The full stylesheet, baked into the binary. Inlined as a raw `<style>` element (below) rather
/// than linked via `asset!()`: the Dioxus asset server only resolves under `dx bundle`, so a plain
/// `cargo build --release` exe would 404 the linked stylesheet and render unstyled. Inlining makes
/// the brand always apply.
const CSS: &str = include_str!("../assets/ziqpu.css");

/// The real 4-beat sequence, in order. Rendered as the `.steps` rail with `aria-current` on the
/// active phase (the guardrail is a *persistent* surface, not a beat, so it is not a step here).
const STEPS: [&str; 4] = ["Setup", "Ranked fits", "Checkpoint", "Grounded briefing"];

/// The `Phase` each step index maps to, so a clicked step (index `i`) can drive `ctx.phase`. Kept
/// in lockstep with [`STEPS`] and with the `step` derivation below.
const STEP_PHASES: [Phase; 4] = [
    Phase::Setup,
    Phase::Ranked,
    Phase::Checkpoint,
    Phase::Briefing,
];

#[component]
pub fn App() -> Element {
    // The graded session lives for the whole app, interior-mutable and single-threaded.
    let session = use_hook(|| Rc::new(RefCell::new(build_session())));

    // These two are pulled out of the struct literal so the reader coroutine (a hook, which cannot
    // be created inside an event handler) can capture them at component top level. `Signal` is
    // `Copy`, so capturing them by move here still leaves copies for the `AppCtx` below.
    let mut recs = use_signal(Vec::<Recommendation>::new);
    let mut pending = use_signal(HashSet::<String>::new);
    let mut sources = use_signal(HashMap::<String, Option<String>>::new);
    // The Raw → Local → Live display mode. Pulled out here (rather than inline in the `AppCtx`
    // literal) so the header button can both read it for its label and cycle it on click. Default
    // Live. The Local-mode reading store + its off-thread fill live alongside it.
    let mut mode = use_signal(|| ReadMode::Live);
    let mut local_readings = use_signal(HashMap::<String, String>::new);
    let mut local_sources = use_signal(HashMap::<String, Option<String>>::new);
    let mut local_pending = use_signal(HashSet::<String>::new);
    // The Ask box's answer lives here (rather than inline in the `AppCtx` literal) so the
    // `ask_reader` coroutine below — a hook, which cannot be created inside an event handler — can
    // capture it and fill an in-flight measured reading once the off-thread prose arrives.
    let mut answer = use_signal(|| None::<crate::state::AnswerView>);
    // The phase machine and the grounded outputs are pulled out here so the `grounder` coroutine
    // below can capture them: when the off-thread grounded pull lands, it commits `signals`+
    // `briefing`, clears `grounding`, and advances `phase` to Briefing. All are `Copy`, so copies
    // remain for the `AppCtx` literal.
    let mut phase = use_signal(|| Phase::Setup);
    let mut signals = use_signal(|| None::<GroundedSignals>);
    let mut briefing = use_signal(|| None::<agents::Briefing>);
    let mut grounding = use_signal(|| false);

    // The event-loop-thread half of the off-thread fill: receive `(ticker, prose, live_model)` from
    // the worker thread and splice each reading back into `recs`, record its provenance in `sources`
    // (Some(model) = live API, None = template), and clear that ticker from `pending`.
    let reader = use_coroutine(
        move |mut rx: UnboundedReceiver<(String, String, Option<String>)>| async move {
            while let Some((ticker, prose, model)) = rx.next().await {
                if let Some(r) = recs.write().iter_mut().find(|r| r.choice == ticker) {
                    r.reading = prose;
                }
                sources.write().insert(ticker.clone(), model);
                pending.write().remove(&ticker);
            }
        },
    );

    // The Ask box's off-thread reader: receive `(ticker, prose)` from the guardrail's worker thread
    // and splice the prose into the in-flight measured reading — but only if the current answer is
    // still that same pending reading (a newer question may have replaced it in the meantime).
    let ask_reader = use_coroutine(
        move |mut rx: UnboundedReceiver<(String, String)>| async move {
            while let Some((ticker, prose)) = rx.next().await {
                let current = answer.read().clone();
                if let Some(crate::state::AnswerView::Reading {
                    name,
                    ticker: t,
                    label,
                    pending: true,
                    redirect,
                    note,
                    ..
                }) = current
                {
                    if t == ticker {
                        answer.set(Some(crate::state::AnswerView::Reading {
                            name,
                            ticker: t,
                            label,
                            pending: false,
                            text: prose,
                            redirect,
                            note,
                        }));
                    }
                }
            }
        },
    );

    // The grounded pull's off-thread half: receive `(signals, briefing)` from the worker thread and
    // commit them — set `signals`+`briefing`, drop the "grounding…" loading state, and advance to
    // the Briefing phase. Only this channel crosses the thread boundary (never a `Signal`).
    let grounder = use_coroutine(
        move |mut rx: UnboundedReceiver<(GroundedSignals, agents::Briefing)>| async move {
            while let Some((s, b)) = rx.next().await {
                signals.set(Some(s));
                briefing.set(Some(b));
                grounding.set(false);
                phase.set(Phase::Briefing);
            }
        },
    );

    // The Local-mode fill's event-loop-thread half: receive `(ticker, prose, model)` from the
    // worker thread and cache it — write the reading into `local_readings`, its provenance into
    // `local_sources`, and clear the ticker from `local_pending`. Only this channel crosses the
    // thread boundary (never a `Signal`). Mirrors `reader`, but for the on-demand Local mode.
    let local_reader = use_coroutine(
        move |mut rx: UnboundedReceiver<(String, String, Option<String>)>| async move {
            while let Some((ticker, prose, model)) = rx.next().await {
                local_readings.write().insert(ticker.clone(), prose);
                local_sources.write().insert(ticker.clone(), model);
                local_pending.write().remove(&ticker);
            }
        },
    );

    let ctx = AppCtx {
        session,
        phase,
        // Restore the last-entered birth chart if one was saved and still validates; fall back to the
        // demo seeker on a missing/corrupt/invalid profile. So the ranked readings compute against the
        // SAVED chart, not the demo default. `load_profile`/`draft_to_moment` never panic.
        seeker: use_signal(|| {
            crate::profile::load_profile()
                .and_then(|p| {
                    crate::components::draft_to_moment(
                        &p.date_str,
                        &p.time_str,
                        p.time_unknown,
                        &p.place(),
                        None,
                    )
                    .ok()
                })
                .unwrap_or_else(demo_seeker)
        }),
        choices: use_signal(demo_choices),
        recs,
        measures: use_signal(HashMap::new),
        selected: use_signal(|| 0usize),
        request: use_signal(|| None),
        gate_proof: use_signal(|| None),
        signals,
        briefing,
        grounding,
        mode,
        raw_readings: use_signal(HashMap::new),
        local_readings,
        local_sources,
        local_pending,
        local_reader,
        answer,
        calls: use_signal(Vec::new),
        pending,
        sources,
        reader,
        ask_reader,
        grounder,
    };
    use_context_provider(|| ctx.clone());

    let phase = *ctx.phase.read();
    let step = match phase {
        Phase::Setup => 0,
        Phase::Ranked => 1,
        Phase::Checkpoint => 2,
        Phase::Briefing => 3,
    };

    // The grounded pull intensifies the starfield behind everything into a "searching the stars"
    // state, so the (off-thread) grounded wait clearly reads as activity.
    let searching = *ctx.grounding.read();

    // The header display-mode pill — glyph + word + the active source's model, cycling on click.
    let cur_mode = *mode.read();
    let (mode_glyph, mode_word, mode_class) = match cur_mode {
        ReadMode::Raw => ("○", "raw", "mode-toggle mode-toggle--raw"),
        ReadMode::Local => ("◐", "local", "mode-toggle mode-toggle--local"),
        ReadMode::Live => ("✦", "live", "mode-toggle mode-toggle--live"),
    };
    // The model hint under the mode word: the template for Raw; the first resolved local/live model
    // otherwise (per-card provenance still shows on each card). Falls back to a neutral word.
    let model_hint = match cur_mode {
        ReadMode::Raw => "local template".to_string(),
        ReadMode::Local => local_sources
            .read()
            .values()
            .flatten()
            .next()
            .map(|s| s.trim_start_matches("local · ").to_string())
            .unwrap_or_else(|| "your machine".to_string()),
        ReadMode::Live => sources
            .read()
            .values()
            .flatten()
            .next()
            .cloned()
            .unwrap_or_else(|| "hosted model".to_string()),
    };

    // The fixed, seeded star field drawn behind the whole app — identical every launch.
    let stars = seeded_stars();
    let sky_class = if searching {
        "sky sky--searching"
    } else {
        "sky"
    };

    rsx! {
        // Inlined stylesheet, baked into the binary — see `CSS` above for why this isn't `asset!()`.
        style { dangerous_inner_html: CSS }

        // The faint cuneiform watermark — a fixed, rotated ground layer behind everything. Rendered
        // in the bundled Noto Sans Cuneiform face (see ziqpu.css) so the glyphs actually draw.
        div { class: "cuni-wm", "aria-hidden": "true",
            "𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺"
        }

        // The star field — a fixed, pointer-inert layer behind the whole app. The coordinates are a
        // seeded, deterministic list (identical every launch); each star twinkles (opacity/scale) on
        // its own staggered cycle. During the grounded pull the `.sky--searching` class intensifies
        // it into the "searching the stars" affordance. Dimmed in dark theme; twinkles at full
        // regardless of the OS reduce-motion flag (deliberate product choice — see ziqpu.css).
        div { class: "{sky_class}", "aria-hidden": "true",
            {stars.iter().enumerate().map(|(i, &(left, top, size, delay, dur))| {
                let style = format!(
                    "left:{left:.2}%;top:{top:.2}%;width:{size:.2}px;height:{size:.2}px;\
                     animation-delay:{delay:.2}s;animation-duration:{dur:.2}s"
                );
                rsx! { span { key: "{i}", class: "star", style: "{style}" } }
            })}
        }

        // A faint stand of wheat along the very bottom of the page — atmospheric only, behind
        // everything (like `.sky`). Purely decorative, pointer-inert, swaying slowly. Ties the card
        // wheat plots to the whole surface. See ziqpu.css `.wheat-horizon`.
        div { class: "wheat-horizon", "aria-hidden": "true" }

        div { class: "wrap",
            header {
                div { class: "brand",
                    // The brand mark — the gold 8-ray star, drawn inline (simple line rays + a
                    // center dot, stroked in --gold) so it recolors with the theme.
                    div { class: "mark", "aria-hidden": "true",
                        svg {
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "var(--gold)",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            line { x1: "12", y1: "12", x2: "12", y2: "2.3" }
                            line { x1: "12", y1: "12", x2: "12", y2: "21.7" }
                            line { x1: "12", y1: "12", x2: "2.3", y2: "12" }
                            line { x1: "12", y1: "12", x2: "21.7", y2: "12" }
                            line { x1: "12", y1: "12", x2: "5.5", y2: "5.5" }
                            line { x1: "12", y1: "12", x2: "18.5", y2: "5.5" }
                            line { x1: "12", y1: "12", x2: "5.5", y2: "18.5" }
                            line { x1: "12", y1: "12", x2: "18.5", y2: "18.5" }
                            circle { cx: "12", cy: "12", r: "1.6", fill: "var(--gold)", stroke: "none" }
                        }
                    }
                    div {
                        h1 { class: "word", "Ziqpu" }
                        div { class: "tagline", "the ledger of the sky · measured, not fate" }
                    }
                }
                div { class: "header-actions",
                    // Raw → Local → Live display toggle. Each mode's prose is cached in its own store,
                    // so cycling is instant when cached; reaching Local for the first time kicks off an
                    // off-thread fetch (the card shows the wheat loader meanwhile). The pill shows the
                    // active mode + its model.
                    button {
                        class: "{mode_class}",
                        r#type: "button",
                        title: "Cycle the reading source: raw template → your local model → the live model",
                        "aria-label": "Reading source: {mode_word} · {model_hint}. Click to cycle.",
                        onclick: {
                            let ctx = ctx.clone();
                            move |_| {
                                let nm = next_mode(*mode.read());
                                mode.set(nm);
                                if nm == ReadMode::Local {
                                    ensure_local_readings(ctx.clone());
                                }
                            }
                        },
                        "{mode_glyph} {mode_word}"
                        span { class: "mode-toggle__hint", "· {model_hint}" }
                    }
                    button {
                        class: "theme",
                        r#type: "button",
                        onclick: move |_| {
                            // Flip data-theme on the document root, mirroring the mockup's toggle.
                            let _ = document::eval(
                                "const r=document.documentElement;\
                                 const e=r.getAttribute('data-theme')||(window.matchMedia('(prefers-color-scheme:dark)').matches?'dark':'light');\
                                 r.setAttribute('data-theme', e==='dark'?'light':'dark');",
                            );
                        },
                        "◐ theme"
                    }
                }
            }

            nav { class: "steps", role: "tablist", "aria-label": "Reading flow",
                {STEPS.iter().enumerate().map(|(i, label)| {
                    // A step is reachable once you have advanced at least to it (i <= step). Reachable
                    // steps are clickable — they jump back (or to the current) phase without wiping any
                    // signal (seeker/choices/recs all live on). Steps *after* the current one stay
                    // non-interactive: you can never skip ahead past the checkpoint.
                    let reachable = i <= step;
                    let done = i < step;
                    let cls = match (done, reachable) {
                        (true, _) => "step done clickable",
                        (false, true) => "step clickable",
                        (false, false) => "step",
                    };
                    let num = format!("{:02}", i + 1);
                    let current = if i == step { "true" } else { "false" };
                    // Only a reachable step gets a target phase + interactive affordances.
                    let target = reachable.then(|| STEP_PHASES[i]);
                    let mut phase_sig = ctx.phase;
                    rsx! {
                        div {
                            key: "{i}",
                            class: "{cls}",
                            role: "tab",
                            "aria-current": "{current}",
                            tabindex: if target.is_some() { "0" } else { "-1" },
                            onclick: move |_| {
                                if let Some(p) = target {
                                    phase_sig.set(p);
                                }
                            },
                            onkeydown: move |e| {
                                // Enter/Space activate a tab, matching the tablist keyboard contract.
                                let key = e.key();
                                let activate = key == Key::Enter
                                    || matches!(&key, Key::Character(c) if c == " ");
                                if activate {
                                    if let Some(p) = target {
                                        e.prevent_default();
                                        phase_sig.set(p);
                                    }
                                }
                            },
                            span { class: "n", "{num}" }
                            span { class: "t", "{label}" }
                        }
                    }
                })}
            }

            section { class: "phase", key: "{step}",
                {match phase {
                    Phase::Setup => rsx! { Setup {} },
                    Phase::Ranked => rsx! { Ranked {} },
                    Phase::Checkpoint => rsx! { Checkpoint {} },
                    Phase::Briefing => rsx! { Briefing {} },
                }}
            }

            if phase != Phase::Setup {
                Guardrail {}
            }

            // The dictionary — a self-contained, always-available collapsible glossary of the
            // planets, aspects, flowing/friction, and the fit bands the engine speaks in.
            Legend {}
        }
    }
}
