//! The root component: sets up the session + context, renders the header/stepper, switches on the
//! current phase, and mounts the persistent guardrail from the Ranked step onward.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{demo_choices, demo_seeker};
use dioxus::prelude::*;

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

#[component]
pub fn App() -> Element {
    // The graded session lives for the whole app, interior-mutable and single-threaded.
    let ctx = AppCtx {
        session: use_hook(|| Rc::new(RefCell::new(build_session()))),
        phase: use_signal(|| Phase::Setup),
        seeker: use_signal(demo_seeker),
        choices: use_signal(demo_choices),
        recs: use_signal(Vec::new),
        measures: use_signal(HashMap::new),
        selected: use_signal(|| 0usize),
        request: use_signal(|| None),
        gate_proof: use_signal(|| None),
        signals: use_signal(|| None),
        briefing: use_signal(|| None),
        answer: use_signal(|| None),
        calls: use_signal(Vec::new),
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

            nav { class: "steps", role: "tablist", "aria-label": "Reading flow",
                {STEPS.iter().enumerate().map(|(i, label)| {
                    let cls = if i < step { "step done" } else { "step" };
                    let num = format!("{:02}", i + 1);
                    let current = if i == step { "true" } else { "false" };
                    rsx! {
                        div { key: "{i}", class: "{cls}", role: "tab", "aria-current": "{current}",
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
