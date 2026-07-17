//! Setup — a three-mode entry point. **Seeded demo** is the original one-click default (a fixed
//! seeker + five real US IPOs, measured offline). **Search the market** opens the whole listed
//! universe: search tickers, drop several into a basket, and rank them together. **Your birth
//! moment** is the form that lets the seeker enter their own details. All three converge on the
//! same graded `run_recommend` path, so the loop, the checkpoint, and the guardrail are identical
//! whichever mode is used.

use agents::Choice;
use dioxus::prelude::*;
use tickers::{TickerRow, Universe};

use crate::components::{BirthInputForm, Identity, YourSky};
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
        // The anonymous identity — the first thing the seeker sees, their cosmic handle (no login).
        Identity {}

        // The seeker's own sky — today's beat + this week's summary (the "you" half of the product).
        YourSky {}

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

/// The three searchable universes, in picker order. Stocks first (the original default), then the
/// two industries the product owner wants weighed alongside them — airlines and insurers.
const UNIVERSES: [Universe; 3] = [Universe::Stocks, Universe::Airlines, Universe::Insurance];

/// The namespaced persistence token for a basket entry — `"<slug>:<id>"` (e.g. `"stocks:AAPL"`,
/// `"airlines:AAL"`). Namespacing is what lets a mixed basket hold an airline "AAL" *and* a stock
/// "AAL" without the two colliding on disk or when deduping.
fn basket_token(u: Universe, id: &str) -> String {
    format!("{}:{}", u.slug(), id)
}

/// Rebuild one basket entry from a persisted token. Splits on the first `:` into `<slug>:<id>`;
/// a legacy bare token with no `:` (written before universes existed) is read as a stock. Returns
/// `None` for an unknown slug or an id no longer present in that universe's table.
fn choice_from_token(token: &str) -> Option<(Universe, Choice)> {
    let (slug, id) = match token.split_once(':') {
        Some((slug, id)) => (slug, id),
        None => ("stocks", token), // back-compat: bare token → stocks
    };
    let u = Universe::from_slug(slug)?;
    tickers::choice_in(u, id).map(|c| (u, c))
}

/// Snapshot the whole basket as namespaced tokens, in pick order, and persist it. Called on every
/// add and remove so the mixed basket survives a relaunch exactly as built.
fn persist_basket(basket: &[(Universe, Choice)]) {
    let tokens: Vec<String> = basket
        .iter()
        .map(|(u, c)| basket_token(*u, &c.ticker))
        .collect();
    crate::profile::save_basket(&tokens);
}

/// Search a universe — the full-universe entry point across **Stocks · Airlines · Insurers**. A
/// picker selects the universe; the offline `tickers` index resolves the query synchronously (no
/// async, no network) via `search_in`. The seeker drops several entities — possibly from different
/// universes — into one mixed basket, and "Read the fits" ranks the whole basket against the current
/// chart via the same graded path. Each pick becomes an `agents::Choice` dated by its founding/IPO
/// moment (`tickers::choice_in`). The basket persists as namespaced `"<slug>:<id>"` tokens so a
/// stock "AAL" and an airline "AAL" never collide.
#[component]
fn MarketPanel() -> Element {
    let ctx = use_context::<AppCtx>();

    // The selected universe. Drives search, add, and each new entry's namespace.
    let mut universe = use_signal(|| Universe::Stocks);

    let mut query = use_signal(String::new);
    let mut results = use_signal(Vec::<TickerRow>::new);
    // The chosen basket — universe-tagged so each entry knows how to persist and dedupe. `Choice`
    // isn't `PartialEq`, so we dedupe by the full namespaced token. Seeded once from the saved
    // basket (rebuilt via `choice_from_token`) so the seeker's mixed picks survive a relaunch.
    let mut basket = use_signal(|| {
        crate::profile::load_basket()
            .iter()
            .filter_map(|t| choice_from_token(t))
            .collect::<Vec<(Universe, Choice)>>()
    });
    // `choice_in()` is no longer total: it returns `None` for the 764 rows with no real birth date,
    // rather than charting them on an invented one. Those rows never render as a button (see the
    // `!row.chartable` arm below), so this stays a rare path — but it is now a real one, not a
    // theoretical one, and a click that silently did nothing would be the worst of both worlds.
    let mut add_error = use_signal(|| None::<String>);

    let current_universe = *universe.read();
    let basket_now = basket.read().clone();
    let can_read = !basket_now.is_empty();

    rsx! {
        section { class: "setup market",
            p { class: "eyebrow", "Begin · search a universe" }
            h2 { class: "setup-title", "Search a universe" }
            p { class: "muted",
                "Search a universe — Stocks, Airlines, or Insurers — by ticker, id, or company name, "
                "resolved offline from committed tables (no keys, no network). Mix as many as you like "
                "into one basket, then weigh them together. Each is dated by its founding or IPO moment."
            }

            div { class: "universe-picker", role: "tablist", "aria-label": "Choose a universe",
                {UNIVERSES.iter().map(|u| {
                    let u = *u;
                    let is_current = current_universe == u;
                    rsx! {
                        button {
                            key: "{u.slug()}",
                            class: if is_current { "universe-pill current" } else { "universe-pill" },
                            r#type: "button",
                            role: "tab",
                            "aria-selected": if is_current { "true" } else { "false" },
                            onclick: move |_| {
                                // Switching universes: adopt it and clear the stale query/results so
                                // the seeker never adds from the universe they just left.
                                universe.set(u);
                                query.set(String::new());
                                results.set(Vec::new());
                                add_error.set(None);
                            },
                            "{u.label()}"
                        }
                    }
                })}
            }

            div { class: "ticker-search",
                input {
                    "aria-label": "Search the {current_universe.label()} universe",
                    placeholder: "Search {current_universe.label()} — e.g. a ticker, id, or company",
                    value: "{query}",
                    oninput: move |e| {
                        let q = e.value();
                        let u = *universe.read();
                        results.set(tickers::search_in(u, &q));
                        query.set(q);
                    },
                }
            }

            if !query.read().trim().is_empty() {
                if results.read().is_empty() {
                    p { class: "ticker-empty", "No {current_universe.label()} match that search." }
                } else {
                    div { class: "ticker-results",
                        {results.read().iter().map(|row| {
                            let id = row.ticker.clone();
                            // We know this one exists but have no birth moment for it: no listing
                            // date in Polygon, no Form 8-A on record. Show it — a seeker who typed
                            // its name deserves an answer, and "we don't know when it was born" IS
                            // the answer — but it cannot be picked, because the only way to chart it
                            // would be to invent the date. (It used to be pickable: it got
                            // 2000-01-01 and a reading built on a day that never happened.)
                            if !row.chartable {
                                return rsx! {
                                    div {
                                        key: "{row.ticker}",
                                        class: "ticker-result ticker-result--unchartable",
                                        "aria-disabled": "true",
                                        title: "Ziqpu has no birth moment for this one, so it can't be charted.",
                                        span { class: "sym", "{row.ticker}" }
                                        span { class: "co", "{row.name}" }
                                        span { class: "univ univ--none", "date unknown — can't chart" }
                                    }
                                };
                            }
                            rsx! {
                                button {
                                    key: "{row.ticker}",
                                    class: "ticker-result",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let u = *universe.read();
                                        // Resolve to a datable Choice and add it once (dedupe by the
                                        // full namespaced token, so cross-universe id collisions stay
                                        // distinct — airline "AAL" and stock "AAL" are two entries).
                                        if let Some(choice) = tickers::choice_in(u, &id) {
                                            let token = basket_token(u, &choice.ticker);
                                            let already = basket
                                                .read()
                                                .iter()
                                                .any(|(bu, c)| basket_token(*bu, &c.ticker) == token);
                                            if !already {
                                                basket.write().push((u, choice));
                                                persist_basket(&basket.read());
                                            }
                                            add_error.set(None);
                                        } else {
                                            add_error.set(Some(id.clone()));
                                        }
                                        query.set(String::new());
                                        results.set(Vec::new());
                                    },
                                    span { class: "sym", "{row.ticker}" }
                                    span { class: "co", "{row.name}" }
                                    span { class: "univ", "{current_universe.label()}" }
                                }
                            }
                        })}
                    }
                }
            }

            if let Some(t) = add_error.read().clone() {
                p { class: "ticker-empty", "Couldn't add {t} — that id isn't in the table. Try another." }
            }

            div { class: "basket",
                span { class: "basket-label", "Basket · {basket_now.len()} chosen" }
                if basket_now.is_empty() {
                    p { class: "basket-empty", "Nothing chosen yet — search above and add a few." }
                } else {
                    div { class: "basket-chips",
                        {basket_now.iter().map(|(u, c)| {
                            let u = *u;
                            let ticker = c.ticker.clone();
                            let name = c.name.clone();
                            let token = basket_token(u, &ticker);
                            let remove = token.clone();
                            rsx! {
                                span { key: "{token}", class: "basket-chip",
                                    strong { "{ticker}" }
                                    " · {name}"
                                    span { class: "chip-univ", "{u.label()}" }
                                    button {
                                        class: "chip-x",
                                        r#type: "button",
                                        "aria-label": "Remove {ticker}",
                                        onclick: move |_| {
                                            basket
                                                .write()
                                                .retain(|(bu, c)| basket_token(*bu, &c.ticker) != remove);
                                            persist_basket(&basket.read());
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
                            // The ranker is universe-agnostic — hand it the bare Choices, mixed.
                            let chosen: Vec<Choice> =
                                basket.read().iter().map(|(_, c)| c.clone()).collect();
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
