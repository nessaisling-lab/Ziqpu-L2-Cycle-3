//! The root component: sets up the session + context, renders the header/stepper, switches on the
//! current phase, and mounts the persistent guardrail from the Ranked step onward.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use agents::{demo_choices, demo_seeker, Recommendation};
use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::components::{Briefing, Checkpoint, Guardrail, Ranked, Setup};
use crate::state::{build_session, AppCtx, Phase};

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
    // The Raw ⇄ Live display toggle. Pulled out here (rather than inline in the `AppCtx` literal) so
    // the header button can both read it for its label and flip it on click. Default Live.
    let mut live_mode = use_signal(|| true);
    // The Ask box's answer lives here (rather than inline in the `AppCtx` literal) so the
    // `ask_reader` coroutine below — a hook, which cannot be created inside an event handler — can
    // capture it and fill an in-flight measured reading once the off-thread prose arrives.
    let mut answer = use_signal(|| None::<crate::state::AnswerView>);

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
                        }));
                    }
                }
            }
        },
    );

    let ctx = AppCtx {
        session,
        phase: use_signal(|| Phase::Setup),
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
        signals: use_signal(|| None),
        briefing: use_signal(|| None),
        live_mode,
        raw_readings: use_signal(HashMap::new),
        answer,
        calls: use_signal(Vec::new),
        pending,
        sources,
        reader,
        ask_reader,
    };
    use_context_provider(|| ctx.clone());

    let phase = *ctx.phase.read();
    let step = match phase {
        Phase::Setup => 0,
        Phase::Ranked => 1,
        Phase::Checkpoint => 2,
        Phase::Briefing => 3,
    };

    rsx! {
        // Inlined stylesheet, baked into the binary — see `CSS` above for why this isn't `asset!()`.
        style { dangerous_inner_html: CSS }

        // The faint cuneiform watermark — a fixed, rotated ground layer behind everything. Rendered
        // in the bundled Noto Sans Cuneiform face (see ziqpu.css) so the glyphs actually draw.
        div { class: "cuni-wm", "aria-hidden": "true",
            "𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺 𒀭 𒉀 𒁾 𒊺"
        }

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
                    // Raw ⇄ Live display toggle — reads the same computed readings both ways, so
                    // flipping is instant (no re-fetch). Live = streamed model prose; Raw = template.
                    button {
                        class: if *live_mode.read() { "mode-toggle mode-toggle--live" } else { "mode-toggle mode-toggle--raw" },
                        r#type: "button",
                        title: "Toggle between the live model reading and the raw local template",
                        "aria-pressed": if *live_mode.read() { "true" } else { "false" },
                        onclick: move |_| {
                            let now = *live_mode.read();
                            live_mode.set(!now);
                        },
                        if *live_mode.read() { "✦ live" } else { "○ raw" }
                        span { class: "mode-toggle__hint", "raw ⇄ live" }
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
        }
    }
}
