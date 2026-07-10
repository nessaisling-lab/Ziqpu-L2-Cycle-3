//! Setup — a two-mode entry point. **Seeded demo** is the original one-click default (a fixed
//! seeker + five real US IPOs, measured offline). **Your birth moment** is the form that lets the
//! seeker enter their own details. Both converge on the same graded `run_recommend` path, so the
//! loop, the checkpoint, and the guardrail are identical whichever mode is used.

use dioxus::prelude::*;

use crate::components::BirthInputForm;
use crate::state::{run_recommend, AppCtx};

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Seeded,
    Custom,
}

#[component]
pub fn Setup() -> Element {
    let mut mode = use_signal(|| Mode::Seeded);
    let current = *mode.read();

    rsx! {
        div { class: "setup-modes",
            button {
                class: if current == Mode::Seeded { "mode-tab current" } else { "mode-tab" },
                r#type: "button",
                onclick: move |_| mode.set(Mode::Seeded),
                "Seeded demo"
            }
            button {
                class: if current == Mode::Custom { "mode-tab current" } else { "mode-tab" },
                r#type: "button",
                onclick: move |_| mode.set(Mode::Custom),
                "Enter my own birth details"
            }
        }

        {match current {
            Mode::Seeded => rsx! { SeededPanel {} },
            Mode::Custom => rsx! { BirthInputForm {} },
        }}
    }
}

/// The original seeded demo card, verbatim in behavior — a fixed seeker and five real US IPOs,
/// measured offline. "Read the fits" runs OBSERVE → DECIDE on the graded session via `run_recommend`.
#[component]
fn SeededPanel() -> Element {
    let ctx = use_context::<AppCtx>();

    let seeker = ctx.seeker.read().clone();
    let choices = ctx.choices.read().clone();

    let date = seeker.date.to_string();
    let time = seeker
        .time
        .map(|t| t.to_string())
        .unwrap_or_else(|| "time unknown".to_string());
    let tz = seeker.tz.to_string();
    let lat = seeker.lat;
    let lon = seeker.lon;

    rsx! {
        section { class: "setup",
            p { class: "eyebrow", "Begin · a seeded reading" }
            h2 { class: "setup-title", "Seeded demo" }
            p { class: "muted",
                "A fixed seeker and five real US IPOs, measured entirely offline — no keys, no network. "
                "Hamun-ana measures the charts; Ungasaga reads the fit."
            }

            div { class: "seeker",
                span { class: "seeker-label", "Seeker" }
                span { class: "seeker-detail", "{date} · {time} · {tz} · {lat}, {lon}" }
            }

            div { class: "choice-chips",
                {choices.iter().map(|c| {
                    let ticker = c.ticker.clone();
                    let name = c.name.clone();
                    rsx! {
                        span { key: "{ticker}", class: "chip",
                            strong { "{ticker}" }
                            " · {name}"
                        }
                    }
                })}
            }

            div { class: "actions", style: "justify-content:flex-start",
                button {
                    class: "btn btn--go",
                    r#type: "button",
                    onclick: {
                        let ctx = ctx.clone();
                        move |_| run_recommend(ctx.clone())
                    },
                    "Read the fits →"
                }
            }
        }
    }
}
