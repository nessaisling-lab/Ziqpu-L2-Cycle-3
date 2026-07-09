//! Setup — the seeded demo. A fixed seeker and five real US IPOs, measured offline. "Read the
//! fits" runs OBSERVE → DECIDE on the graded session and moves to the ranked view.

use dioxus::prelude::*;

use crate::state::{measures_for, AppCtx, Phase};

#[component]
pub fn Setup() -> Element {
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
        section { class: "card setup",
            h2 { "Seeded demo" }
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

            div { class: "actions",
                button {
                    class: "primary",
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            let seeker = ctx.seeker.read().clone();
                            let choices = ctx.choices.read().clone();
                            let recs = {
                                let mut session = ctx.session.borrow_mut();
                                session.recommend(&seeker, &choices)
                            };
                            let calls: Vec<String> = {
                                let session = ctx.session.borrow();
                                session.calls().iter().map(|c| format!("{c:?}")).collect()
                            };
                            ctx.measures.set(measures_for(&seeker, &choices));
                            ctx.recs.set(recs);
                            ctx.calls.set(calls);
                            ctx.selected.set(0);
                            ctx.phase.set(Phase::Ranked);
                        }
                    },
                    "Read the fits →"
                }
            }
        }
    }
}
