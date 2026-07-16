//! The **model picker** — a dropdown of the models a provider will actually serve, fetched live.
//!
//! Shared by onboarding and Settings so the two can't drift. No model list is hardcoded here or in
//! [`agents::models`]: the provider's own catalog is the source of truth, so what's offered is what
//! is callable today with the seeker's key.
//!
//! Badges are earned from live data, never decoration:
//! - **✦ best for readings** — clears every fit rule (honors our length cap, doesn't force
//!   chain-of-thought into the prose, guardrail intact) *and* ranks near the top of the catalog's
//!   published benchmark. See `agents::models::Fit`.
//! - **★ top quality** — the catalog's own highest-scoring model, whatever it is. Deliberately a
//!   *different* badge: the strongest model is often a mandatory-reasoning one, which is a poor fit
//!   for a short narrative reading. Seeing both is the point.
//!
//! (There is no "most popular" badge: OpenRouter's catalog publishes no usage or ranking data — its
//! `?order=` parameter is ignored and no rank field exists — so a popularity badge could only be
//! invented. We don't ship invented signal.)
//!
//! The catalog fetch is a blocking HTTPS call, so it runs on a worker thread and reports back
//! through a coroutine — the same shape `model_panel` uses for its Hub search.

use dioxus::prelude::*;
use futures_util::StreamExt;

use agents::models::{Fit, ModelOption};

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

/// A live model dropdown for `provider` (a slug: `anthropic` / `openrouter` / `built_in`).
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
        std::thread::spawn(move || {
            let _ = tx.unbounded_send(agents::models::list_for_provider(&slug));
        });
    });

    let current = status.read().clone();

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
                    select { class: "settings-input", disabled: true,
                        option { "Loading…" }
                    }
                },
                Status::Failed(why) => rsx! {
                    select { class: "settings-input", disabled: true,
                        option { "—" }
                    }
                    p { class: "provider-err", "{why}" }
                },
                Status::Ready => rsx! {
                    select {
                        class: "settings-input",
                        value: "{chosen}",
                        onchange: move |e| chosen.set(e.value()),
                        // An explicit "use the default" entry, so a seeker who never picks isn't
                        // silently bound to whatever happens to sort first.
                        option { value: "", "Ziqpu's default" }
                        for m in options.read().iter() {
                            option {
                                key: "{m.id}",
                                value: "{m.id}",
                                // Listed so you can see it exists and that support is coming —
                                // but not pickable, because choosing it today would just fail.
                                disabled: !m.fit.selectable(),
                                "{option_text(m)}"
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
                                " Greyed-out models can't be driven yet — support is planned."
                            }
                        }
                    }
                },
            }
        }
    }
}

/// One dropdown line: badges, name, then the sub-note. A native `<option>` renders text only — no
/// markup — so the badges ride as glyphs and the whole line stays type-ahead searchable.
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
    // The reason a model is caveated or greyed out ("always shows its work", "image/audio — not
    // supported yet") is the one thing a seeker most needs before choosing, so it rides in the line
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
