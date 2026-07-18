//! The in-app **Settings** surface — a downloader can paste their own hosted-provider API key here
//! and get live readings with **no env vars and no `.env` file**. Keys are stored in the OS
//! credential vault (Windows Credential Manager / macOS Keychain / Linux Secret Service — see
//! [`crate::vault`]) and applied to the environment live on Save, so they take effect without a
//! restart.
//!
//! ## Why this is a page, not a modal
//!
//! It was a modal, twice, and it fought the owner both times: the header scrolled out of reach, then
//! the whole frame collapsed into a ~40px strip in the header row — no backdrop, no centering, the
//! controls crushed. The cause is structural. The trigger lives inside the header's flex cluster, so
//! the overlay was born deep in the tree and depended on `position:fixed` escaping every ancestor,
//! plus grid centering, plus `max-height`, plus nested scroll containers all behaving at once. Each
//! fix moved the failure somewhere else.
//!
//! A page has none of those moving parts. It renders at the app root, in normal flow, and scrolls
//! like any other page — no fixed positioning to be trapped, no backdrop, no centering to overflow,
//! no height cap to clip, no z-index. There is nothing left to go wrong, which is the whole point:
//! the owner's standard was "reliably accessible, does not fight me".
//!
//! Two pieces: [`SettingsButton`] is just the header trigger; [`SettingsPage`] is the surface.

use dioxus::prelude::*;

use crate::components::{KeyField, ModelPanel, ModelPicker};
use crate::settings::{
    active_mode_label, built_in_available, load_settings, save_local_url, save_model_for,
    save_provider,
};
use crate::vault::Provider;

/// The provider cards offered in Settings: `(slug, label, sub-label)`. The built-in tier is only
/// listed when this build ships a configured proxy.
fn provider_choices() -> Vec<(&'static str, &'static str, &'static str)> {
    let mut v = Vec::new();
    if built_in_available() {
        v.push(("built_in", "Ziqpu built-in", "free · no key"));
    }
    v.push((Provider::Anthropic.slug(), "Anthropic (Claude)", "your key"));
    v.push((Provider::OpenRouter.slug(), "OpenRouter", "your key"));
    v
}

/// The header trigger. Owns no state — it just asks `app.rs` to show the page, so the surface can
/// render at the app root instead of being born inside the header's flex cluster.
#[component]
pub fn SettingsButton(on_open: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "gear",
            r#type: "button",
            title: "Settings — provider, model, and keys (kept in this device's keychain)",
            "aria-label": "Open settings",
            onclick: move |_| on_open.call(()),
            "⚙ settings"
        }
    }
}

/// The Settings surface, rendered as a full page in normal flow.
#[component]
pub fn SettingsPage(on_close: EventHandler<()>) -> Element {
    // Seed the fields once, from `settings.json` only. **No key is read here** — a key that reaches
    // a signal is a key that has been shown. `KeyField` owns that surface and asks the vault for
    // presence, never for a value.
    let initial = use_hook(load_settings);
    let mut local_url = use_signal(|| initial.local_url.clone().unwrap_or_default());
    // The explicit provider choice (slug), or None when the seeker never picked one.
    let mut provider = use_signal(|| initial.provider.clone());
    // Per-provider model picks, seeded from the saved settings.
    let anthropic_model = use_signal(|| initial.anthropic_model.clone().unwrap_or_default());
    let openrouter_model = use_signal(|| initial.openrouter_model.clone().unwrap_or_default());
    // Bumped after a key is saved: the Anthropic catalog only becomes reachable once a key exists,
    // so the picker needs a nudge to try again.
    let mut catalog_reload = use_signal(|| 0u32);
    // The raw key fields live behind this — the picker is the everyday control.
    let mut advanced = use_signal(|| false);
    // So does the local-model benchmark panel, which is the tallest thing in here.
    let mut local_open = use_signal(|| false);

    // UI-only state: whether Save landed. Keys are saved by `KeyField` the moment they're pasted,
    // so Save no longer has a credential path — and there are no reveal toggles, by design.
    let mut saved = use_signal(|| false);

    // Clear "Saved ✓" when a model pick changes. The provider card and the local-URL field reset it
    // inline, but `ModelPicker` writes `chosen` directly and has no way to tell us — so a stale ✓
    // used to sit in the sticky footer next to Save, asserting a pick was persisted when it wasn't,
    // and "← done" then unmounted the page and dropped it. Save-then-pick isn't contrived: the
    // catalog fetch is a network call, so Save is clickable long before the picker fills in.
    use_effect(move || {
        // Subscribe to both picks; the read is the subscription.
        let _ = anthropic_model.read();
        let _ = openrouter_model.read();
        saved.set(false);
    });

    // Recomputed each render — reads only env-var *presence*, so it reflects the live mode after Save.
    let mode_label = active_mode_label();

    let save = move |_| {
        let u = local_url.read().trim().to_string();
        let am = anthropic_model.read().trim().to_string();
        let om = openrouter_model.read().trim().to_string();

        // **Non-secrets only.** Keys go straight from the paste field to the vault in [`KeyField`],
        // so Save has no credential path left to get wrong.
        //
        // Each call below is load-modify-save on the file plus set-or-remove on the live env. That
        // shape is deliberate, and replaced a `SettingsFile { .. }` literal that had two bugs a
        // release audit caught:
        //
        // 1. A literal must restate every field it isn't changing, and each restatement is a chance
        //    to reset one. `openrouter_key: None` looked like a no-op and was an active **delete** —
        //    erasing a legacy plaintext key that `migrate_plaintext_keys_to_vault` had deliberately
        //    left on disk when the keystore was unreachable ("we never drop a key we can't
        //    re-home"). It was unrecoverable, and silent until the next launch. Loading first makes
        //    preservation the default, so a field nobody remembers here simply survives.
        // 2. `apply_settings_live` never *removes* a var — an empty field must not wipe a live key —
        //    so clearing a model back to "Ziqpu's default", or clearing the local URL, said
        //    "Saved ✓" while the old value stayed in the environment until a restart. These helpers
        //    clear.
        if let Some(p) = provider.read().clone() {
            save_provider(&p);
        }
        save_model_for(Provider::Anthropic, &am);
        save_model_for(Provider::OpenRouter, &om);
        save_local_url(&u);
        saved.set(true);
    };

    rsx! {
        // A page: normal flow, no overlay, no fixed positioning, scrolls with the document. Nothing
        // here can be trapped by an ancestor or clipped by a height cap, because none are involved.
        section {
            class: "settings-page",
            "aria-label": "Settings",
            // Esc still leaves, for the keyboard-first.
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    on_close.call(());
                }
            },

            div { class: "settings-head",
                h2 { class: "settings-title", "Settings" }
                button {
                    class: "btn btn--ghost",
                    r#type: "button",
                    "aria-label": "Close settings",
                    onclick: move |_| on_close.call(()),
                    "← done"
                }
            }

            div { class: "settings-body",

                    p { class: "settings-lede",
                        "Choose who writes your readings, and which model. Keys are optional — "
                        "they live under Advanced, in this device's secure keychain."
                    }

                    // ---- Preferred provider ----
                    // An explicit pick wins over whatever key merely happens to be present, so a key
                    // exported for another provider can't silently hijack Live readings.
                    span { class: "settings-label", "Preferred provider" }
                    div { class: "provider-grid",
                        for (slug, label, sub) in provider_choices() {
                            button {
                                key: "{slug}",
                                class: if provider.read().as_deref() == Some(slug) {
                                    "provider-card provider-card--on"
                                } else {
                                    "provider-card"
                                },
                                r#type: "button",
                                "aria-pressed": if provider.read().as_deref() == Some(slug) { "true" } else { "false" },
                                onclick: move |_| {
                                    provider.set(Some(slug.to_string()));
                                    saved.set(false);
                                },
                                span { class: "pc-name", "{label}" }
                                span { class: "pc-sub", "{sub}" }
                            }
                        }
                    }

                    // ---- Model (live catalog) ----
                    // The everyday control. `built_in` serves Anthropic ids through the proxy, so
                    // it writes the same scoped setting Anthropic does.
                    if provider.read().as_deref() == Some(Provider::OpenRouter.slug()) {
                        ModelPicker {
                            provider,
                            chosen: openrouter_model,
                            reload: catalog_reload,
                        }
                    } else {
                        ModelPicker {
                            provider,
                            chosen: anthropic_model,
                            reload: catalog_reload,
                        }
                    }

                    // ---- Advanced: raw keys + local endpoint ----
                    // Pasting a key is the exception, not the everyday path, so it lives behind a
                    // disclosure — and a key is what makes the Anthropic catalog reachable, so
                    // saving one reloads the picker above.
                    button {
                        class: "settings-reveal settings-advanced",
                        r#type: "button",
                        "aria-expanded": if *advanced.read() { "true" } else { "false" },
                        onclick: move |_| advanced.toggle(),
                        if *advanced.read() { "▾ Advanced — API keys" } else { "▸ Advanced — API keys" }
                    }

                    if *advanced.read() {

                    // ---- Provider keys: presence only, never the value ----
                    // Each row reports whether a key exists and where it came from — including one
                    // exported in the environment, which Ziqpu detects and uses but didn't store.
                    // Adding, replacing, and removing all happen inside `KeyField`, which saves on
                    // paste; nothing here can read a key back out.
                    span { class: "settings-label", "API keys" }
                    KeyField {
                        provider: Provider::Anthropic,
                        on_change: move |_| catalog_reload.with_mut(|n| *n += 1),
                    }
                    KeyField {
                        provider: Provider::OpenRouter,
                        on_change: move |_| catalog_reload.with_mut(|n| *n += 1),
                    }

                    // ---- Optional local model URL ----
                    label { class: "settings-field",
                        span { class: "settings-label", "Local model URL " span { class: "settings-opt", "(optional)" } }
                        input {
                            class: "settings-input",
                            r#type: "text",
                            autocomplete: "off",
                            spellcheck: "false",
                            placeholder: "http://localhost:1234/v1",
                            value: "{local_url}",
                            oninput: move |e| {
                                local_url.set(e.value());
                                saved.set(false);
                            },
                        }
                    }

                    p { class: "settings-hint",
                        "Keys live in your OS keychain (Credential Manager / Keychain / Secret Service), "
                        "outside the app folder. Sent only to the model API you configure — never logged, "
                        "never shared."
                    }

                    } // end Advanced

                    // ---- Local model (folded) ----
                    // The benchmark/download panel is taller than everything else combined, and
                    // almost nobody opens Settings to use it. Folded, the page opens on just the
                    // controls you came for.
                    button {
                        class: "settings-reveal settings-advanced",
                        r#type: "button",
                        "aria-expanded": if *local_open.read() { "true" } else { "false" },
                        onclick: move |_| local_open.toggle(),
                        if *local_open.read() {
                            "▾ Local model — run readings on this machine"
                        } else {
                            "▸ Local model — run readings on this machine"
                        }
                    }
                    if *local_open.read() {
                        div { class: "settings-drawer", ModelPanel {} }
                    }

            } // end body

            // Sticks to the bottom of the window while the page scrolls behind it, so Save is
            // always one click away. `position:sticky` (unlike `fixed`) is laid out by its own
            // scroll container, so no ancestor can trap it — the trap that broke the modal.
            div { class: "settings-foot",
                span { class: "settings-active",
                    "Active: "
                    strong { "{mode_label}" }
                }
                div { class: "settings-actions",
                    if *saved.read() {
                        span { class: "settings-saved", "Saved ✓" }
                    }
                    button {
                        class: "btn btn--go",
                        r#type: "button",
                        onclick: save,
                        "Save"
                    }
                }
            }
        }
    }
}
