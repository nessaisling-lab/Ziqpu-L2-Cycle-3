//! The **model picker** — a dropdown of the models a provider will actually serve, fetched live.
//!
//! Shared by onboarding and Settings so the two can't drift. No model list is hardcoded here or in
//! [`agents::models`]: the provider's own catalog is the source of truth, so what's offered is what
//! is callable today with the seeker's key.
//!
//! ## Why this isn't a `<select>`
//!
//! It was, and it was unusable. A native `<select>` is an **operating-system** control: the browser
//! hands the popup to the OS, which draws it *outside* the page. With ~320 models that meant a
//! detached list dumped over the whole app — ignoring the theme, ignoring the modal's bounds, and
//! impossible to scroll sensibly. No amount of CSS reaches it, because it isn't in the DOM.
//!
//! So the list is built from ordinary elements instead, and expands **inline, in normal flow**:
//! - It cannot escape its container or cover the app — it's just markup in the panel.
//! - It cannot be clipped by the scrolling body, and needs no anchoring, portals, or z-index games
//!   (the traps that come with an absolutely-positioned popover inside an `overflow:auto` parent).
//! - It's themed, because it's ours.
//!
//! A **filter** carries the weight a 320-row list can't: type two letters and the list is short.
//! The rendered list is capped at [`MAX_VISIBLE`] so the DOM stays small and the filter stays the
//! way through — a long scroll is the thing being fixed, not the thing to rebuild.
//!
//! ## Badges (earned from live data, never decoration)
//! - **✦ best for readings** — clears every fit rule (honors our length cap, doesn't force
//!   chain-of-thought into the prose, guardrail intact) *and* ranks near the top of the catalog's
//!   published benchmark. See `agents::models::Fit`.
//! - **✧ best free** — the strongest that costs nothing.
//! - **★ top quality** — the catalog's own highest scorer. Deliberately a *different* badge: the
//!   strongest model is often a mandatory-reasoning one, a poor fit for a short narrative reading.
//!
//! (There is no "most popular" badge: OpenRouter publishes no usage or ranking data — its `?order=`
//! parameter is ignored and no rank field exists — so it could only be invented.)
//!
//! The catalog fetch is a blocking HTTPS call, so it runs on a worker thread and reports back
//! through a coroutine — the same shape `model_panel` uses for its Hub search.

use dioxus::prelude::*;
use futures_util::StreamExt;

use agents::models::{Fit, ModelOption};

/// Most rows rendered at once. Past this the filter is the way through, not the scrollbar — which
/// also keeps the DOM (and Dioxus diffing) small on a ~320-model catalog.
const MAX_VISIBLE: usize = 40;

/// What the picker is doing right now.
#[derive(Clone, PartialEq)]
enum Status {
    /// Nothing fetched yet for the current provider.
    Idle,
    Loading,
    Ready,
    /// A short, key-free reason (e.g. "Add your Anthropic key to see its models.").
    Failed(String),
}

/// A live model list for `provider` (a slug: `anthropic` / `openrouter` / `built_in`).
///
/// `chosen` is two-way: seeded with the saved id and written on pick, so the parent can persist it.
/// The picker refetches whenever the provider changes or `reload` is bumped — the parent bumps it
/// after a key is entered, since a key is what makes the Anthropic catalog reachable.
#[component]
pub fn ModelPicker(
    provider: Signal<Option<String>>,
    chosen: Signal<String>,
    reload: Signal<u32>,
) -> Element {
    let mut options = use_signal(Vec::<ModelOption>::new);
    let mut status = use_signal(|| Status::Idle);
    // List state: whether it's expanded, the type-to-narrow text, and the keyboard cursor.
    let mut open = use_signal(|| false);
    let mut filter = use_signal(String::new);
    let mut cursor = use_signal(|| 0usize);

    // Results come back here from the worker thread.
    let results = use_coroutine(
        move |mut rx: UnboundedReceiver<Result<Vec<ModelOption>, String>>| async move {
            while let Some(res) = rx.next().await {
                match res {
                    Ok(list) => {
                        options.set(list);
                        status.set(Status::Ready);
                    }
                    Err(why) => {
                        options.set(Vec::new());
                        status.set(Status::Failed(why));
                    }
                }
            }
        },
    );

    // Refetch on provider change or an explicit reload bump. `use_effect` reruns when either signal
    // it reads changes, which keeps the fetch out of the render path (writing signals mid-render is
    // what produced the Dioxus warning spew we fixed once already).
    use_effect(move || {
        let slug = provider.read().clone();
        let _ = reload.read(); // subscribe: a bump means "a key landed, try again"
        let Some(slug) = slug else {
            status.set(Status::Idle);
            return;
        };
        status.set(Status::Loading);
        let tx = results.tx();
        // The seeker's data dir doubles as the traction cache, so the Hugging Face sweep behind the
        // OpenRouter catalog is paid once a week rather than on every open of this panel.
        let cache = crate::profile::data_dir();
        std::thread::spawn(move || {
            let _ = tx.unbounded_send(agents::models::list_for_provider(&slug, cache.as_deref()));
        });
    });

    let current = status.read().clone();

    // The rows the filter leaves, capped. Recomputed per render — cheap next to a network fetch,
    // and it keeps the list honest against whatever's typed right now.
    let matches: Vec<ModelOption> = {
        let needle = filter.read().to_ascii_lowercase();
        options
            .read()
            .iter()
            .filter(|m| {
                needle.is_empty()
                    || m.label.to_ascii_lowercase().contains(&needle)
                    || m.id.to_ascii_lowercase().contains(&needle)
            })
            .take(MAX_VISIBLE)
            .cloned()
            .collect()
    };
    let total_matching = {
        let needle = filter.read().to_ascii_lowercase();
        options
            .read()
            .iter()
            .filter(|m| {
                needle.is_empty()
                    || m.label.to_ascii_lowercase().contains(&needle)
                    || m.id.to_ascii_lowercase().contains(&needle)
            })
            .count()
    };
    let hidden = total_matching.saturating_sub(matches.len());

    // What the closed trigger reads. An empty choice is the app default, said plainly rather than
    // left blank.
    let trigger_label = {
        let id = chosen.read().clone();
        if id.is_empty() {
            "Ziqpu's default".to_string()
        } else {
            options
                .read()
                .iter()
                .find(|m| m.id == id)
                .map(option_text)
                // A saved id the current catalog doesn't list (provider switched, model retired)
                // still shows *something* truthful rather than reading as unset.
                .unwrap_or(id)
        }
    };

    // Commit a choice and collapse.
    let mut pick = move |id: String| {
        chosen.set(id);
        open.set(false);
        filter.set(String::new());
    };

    rsx! {
        div { class: "picker",
            div { class: "picker-head",
                span { class: "settings-label", "Model" }
                match &current {
                    Status::Loading => rsx! { span { class: "picker-status", "finding models…" } },
                    Status::Ready => rsx! {
                        span { class: "picker-status", "{options.read().len()} available" }
                    },
                    _ => rsx! {},
                }
            }

            match &current {
                Status::Idle => rsx! {
                    p { class: "settings-hint", "Choose a provider to see its models." }
                },
                Status::Loading => rsx! {
                    div { class: "combo-trigger combo-trigger--busy", "Loading…" }
                },
                Status::Failed(why) => rsx! {
                    div { class: "combo-trigger combo-trigger--busy", "—" }
                    p { class: "provider-err", "{why}" }
                },
                Status::Ready => rsx! {
                    div { class: "combo",
                        // Closed state: one button showing the current pick.
                        button {
                            class: "combo-trigger",
                            r#type: "button",
                            "aria-haspopup": "listbox",
                            "aria-expanded": if *open.read() { "true" } else { "false" },
                            onclick: move |_| {
                                let now = !*open.read();
                                open.set(now);
                                if now {
                                    filter.set(String::new());
                                    cursor.set(0);
                                }
                            },
                            span { class: "combo-value", "{trigger_label}" }
                            span { class: "combo-caret", if *open.read() { "▾" } else { "▸" } }
                        }

                        if *open.read() {
                            // Expands INLINE — ordinary markup in the panel, so it can't escape,
                            // can't be clipped, and can't cover the app the way the OS popup did.
                            div { class: "combo-panel",
                                input {
                                    class: "combo-filter",
                                    r#type: "text",
                                    autocomplete: "off",
                                    spellcheck: "false",
                                    autofocus: true,
                                    placeholder: "Type to narrow — name or id",
                                    value: "{filter}",
                                    oninput: move |e| {
                                        filter.set(e.value());
                                        cursor.set(0);
                                    },
                                    onkeydown: move |e| {
                                        let len = matches.len();
                                        match e.key() {
                                            // Esc closes the LIST, not the whole panel — stop it
                                            // reaching the dialog's own Escape handler.
                                            Key::Escape => {
                                                e.stop_propagation();
                                                open.set(false);
                                            }
                                            // Read into a local before writing: holding a signal's
                                            // read guard across its own `set` is a borrow error.
                                            Key::ArrowDown => {
                                                e.prevent_default();
                                                if len > 0 {
                                                    let at = *cursor.read();
                                                    cursor.set((at + 1).min(len - 1));
                                                }
                                            }
                                            Key::ArrowUp => {
                                                e.prevent_default();
                                                let at = *cursor.read();
                                                cursor.set(at.saturating_sub(1));
                                            }
                                            Key::Enter => {
                                                e.prevent_default();
                                                if let Some(m) = matches.get(*cursor.read()) {
                                                    if m.fit.selectable() {
                                                        pick(m.id.clone());
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    },
                                }

                                ul { class: "combo-list", role: "listbox",
                                    // The default always sits at the top, so "put it back" is never
                                    // a hunt through the catalog.
                                    li {
                                        class: if chosen.read().is_empty() { "combo-opt combo-opt--on" } else { "combo-opt" },
                                        role: "option",
                                        "aria-selected": if chosen.read().is_empty() { "true" } else { "false" },
                                        onclick: move |_| pick(String::new()),
                                        "Ziqpu's default"
                                    }
                                    for (i, m) in matches.iter().enumerate() {
                                        li {
                                            key: "{m.id}",
                                            class: {
                                                let mut c = String::from("combo-opt");
                                                if !m.fit.selectable() { c.push_str(" combo-opt--off"); }
                                                if *cursor.read() == i { c.push_str(" combo-opt--cursor"); }
                                                if *chosen.read() == m.id { c.push_str(" combo-opt--on"); }
                                                c
                                            },
                                            role: "option",
                                            "aria-selected": if *chosen.read() == m.id { "true" } else { "false" },
                                            "aria-disabled": if m.fit.selectable() { "false" } else { "true" },
                                            onclick: {
                                                let m = m.clone();
                                                move |_| {
                                                    if m.fit.selectable() {
                                                        pick(m.id.clone());
                                                    }
                                                }
                                            },
                                            "{option_text(m)}"
                                        }
                                    }
                                    if matches.is_empty() {
                                        li { class: "combo-empty", "No model matches that." }
                                    }
                                }

                                if hidden > 0 {
                                    p { class: "combo-more", "+{hidden} more — keep typing to narrow" }
                                }
                            }
                        }
                    }

                    // Legend for whatever badges are actually present, so the marks aren't cryptic.
                    if options.read().iter().any(|m| m.fit.is_pick()) {
                        p { class: "settings-hint",
                            "✦ best for readings — follows a careful brief, respects our length "
                            "limit, and won't write its reasoning into your reading."
                            if options.read().iter().any(|m| m.fit == Fit::BestFree) {
                                " ✧ best free — the strongest that costs nothing."
                            }
                            if options.read().iter().any(|m| m.top_quality) {
                                " ★ the catalog's highest-scoring model, which isn't always the best fit here."
                            }
                            if options.read().iter().any(|m| !m.fit.selectable()) {
                                " Dimmed models can't be driven yet — support is planned."
                            }
                        }
                    }
                },
            }
        }
    }
}

/// One row's text: badges, name, then the sub-note.
fn option_text(m: &ModelOption) -> String {
    let mut marks = String::new();
    match m.fit {
        Fit::Best => marks.push_str("✦ "),
        Fit::BestFree => marks.push_str("✧ "),
        _ => {}
    }
    if m.top_quality {
        marks.push_str("★ ");
    }
    // The reason a model is caveated or dimmed ("always shows its work", "image/audio — not
    // supported yet") is the one thing a seeker most needs before choosing, so it rides in the row
    // rather than a tooltip they'd never open. The two pick badges already speak via their glyph.
    let why = m.fit.badge().filter(|_| !m.fit.is_pick());
    let tail = match (why, m.note.is_empty()) {
        (Some(why), true) => format!(" — {why}"),
        (Some(why), false) => format!(" — {} · {why}", m.note),
        (None, true) => String::new(),
        (None, false) => format!(" — {}", m.note),
    };
    format!("{marks}{}{tail}", m.label)
}
