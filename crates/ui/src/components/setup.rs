//! Setup — a three-mode entry point. **Seeded demo** is the original one-click default (a fixed
//! seeker + five real US IPOs, measured offline). **Search the market** opens the whole listed
//! universe: search tickers, drop several into a basket, and rank them together. **Your birth
//! moment** is the form that lets the seeker enter their own details. All three converge on the
//! same graded `run_recommend` path, so the loop, the checkpoint, and the guardrail are identical
//! whichever mode is used.

use agents::Choice;
use dioxus::prelude::*;
use tickers::TickerRow;

use crate::components::BirthInputForm;
use crate::state::{run_recommend, AppCtx};

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Seeded,
    Market,
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
                class: if current == Mode::Market { "mode-tab current" } else { "mode-tab" },
                r#type: "button",
                onclick: move |_| mode.set(Mode::Market),
                "Search the market"
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
            Mode::Market => rsx! { MarketPanel {} },
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

/// Search the market — the full-universe entry point. The offline `tickers` index resolves a query
/// synchronously (no async, no network); the seeker drops several symbols into a basket, and
/// "Read the fits" ranks the whole basket against the current chart via the same graded path. Each
/// picked symbol becomes an `agents::Choice` dated by its IPO (`tickers::choice`).
#[component]
fn MarketPanel() -> Element {
    let ctx = use_context::<AppCtx>();

    let mut query = use_signal(String::new);
    let mut results = use_signal(Vec::<TickerRow>::new);
    // The chosen-choices basket. `Choice` isn't `PartialEq`, so we dedupe by ticker. Seeded once
    // from the saved basket (persisted across launches) so the seeker's picks survive a relaunch.
    let mut basket = use_signal(|| {
        crate::profile::load_basket()
            .iter()
            .filter_map(|t| tickers::choice(t))
            .collect::<Vec<Choice>>()
    });

    let basket_now = basket.read().clone();
    let can_read = !basket_now.is_empty();

    rsx! {
        section { class: "setup market",
            p { class: "eyebrow", "Begin · search the whole market" }
            h2 { class: "setup-title", "Search the market" }
            p { class: "muted",
                "Search the full listed universe by ticker or company name — resolved offline from a "
                "committed table (no keys, no network). Add as many as you like to the basket, then "
                "weigh them together. Each is dated by its IPO."
            }

            div { class: "ticker-search",
                input {
                    "aria-label": "Search a ticker or company",
                    placeholder: "Search a ticker or company — e.g. AAPL or Tesla",
                    value: "{query}",
                    oninput: move |e| {
                        let q = e.value();
                        results.set(tickers::search(&q));
                        query.set(q);
                    },
                }
            }

            if !query.read().trim().is_empty() {
                if results.read().is_empty() {
                    p { class: "ticker-empty", "No symbols match that search." }
                } else {
                    div { class: "ticker-results",
                        {results.read().iter().map(|row| {
                            let ticker = row.ticker.clone();
                            rsx! {
                                button {
                                    key: "{row.ticker}",
                                    class: "ticker-result",
                                    r#type: "button",
                                    onclick: move |_| {
                                        // Resolve to a datable Choice and add it once (dedupe by ticker).
                                        if let Some(choice) = tickers::choice(&ticker) {
                                            let already = basket.read().iter().any(|c| c.ticker == choice.ticker);
                                            if !already {
                                                basket.write().push(choice);
                                                // Persist the basket so the picks survive a relaunch.
                                                let picks: Vec<String> = basket.read().iter().map(|c| c.ticker.clone()).collect();
                                                crate::profile::save_basket(&picks);
                                            }
                                        }
                                        query.set(String::new());
                                        results.set(Vec::new());
                                    },
                                    span { class: "sym", "{row.ticker}" }
                                    span { class: "co", "{row.name}" }
                                }
                            }
                        })}
                    }
                }
            }

            div { class: "basket",
                span { class: "basket-label", "Basket · {basket_now.len()} chosen" }
                if basket_now.is_empty() {
                    p { class: "basket-empty", "Nothing chosen yet — search above and add a few." }
                } else {
                    div { class: "basket-chips",
                        {basket_now.iter().map(|c| {
                            let ticker = c.ticker.clone();
                            let name = c.name.clone();
                            let remove = ticker.clone();
                            rsx! {
                                span { key: "{ticker}", class: "basket-chip",
                                    strong { "{ticker}" }
                                    " · {name}"
                                    button {
                                        class: "chip-x",
                                        r#type: "button",
                                        "aria-label": "Remove {ticker}",
                                        onclick: move |_| {
                                            basket.write().retain(|c| c.ticker != remove);
                                            // Persist the trimmed basket too.
                                            let picks: Vec<String> = basket.read().iter().map(|c| c.ticker.clone()).collect();
                                            crate::profile::save_basket(&picks);
                                        },
                                        "×"
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            div { class: "actions", style: "justify-content:flex-start",
                button {
                    class: "btn btn--go",
                    r#type: "button",
                    disabled: !can_read,
                    onclick: {
                        let mut ctx = ctx.clone();
                        move |_| {
                            let chosen = basket.read().clone();
                            if chosen.is_empty() {
                                return;
                            }
                            ctx.choices.set(chosen);
                            run_recommend(ctx.clone());
                        }
                    },
                    "Read the fits →"
                }
            }
        }
    }
}
