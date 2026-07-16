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
    active_mode_label, apply_settings_live, built_in_available, load_settings, save_settings,
    SettingsFile,
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

    // Recomputed each render — reads only env-var *presence*, so it reflects the live mode after Save.
    let mode_label = active_mode_label();

    let save = move |_| {
        let u = local_url.read().trim().to_string();
        let am = anthropic_model.read().trim().to_string();
        let om = openrouter_model.read().trim().to_string();

        // **Non-secrets only.** Keys go straight from the paste field to the vault in [`KeyField`],
        // so Save has no credential path left to get wrong. It used to have one, and it was a trap:
        // the key fields were seeded from the vault, so an unreachable keystore rendered them blank
        // — and a blank field meant "clear this provider", making an innocent Save silently delete
        // a working key.
        let file = SettingsFile {
            openrouter_key: None,
            // Legacy shared field — the picker writes the scoped ones below. Preserve whatever a
            // pre-picker install had rather than silently dropping it.
            model: load_settings().model,
            local_url: (!u.is_empty()).then_some(u),
            anthropic_model: (!am.is_empty()).then_some(am),
            openrouter_model: (!om.is_empty()).then_some(om),
            provider: provider.read().clone(),
            // Preserve the developer-build switch — this modal only edits credentials, and a bare
            // literal would silently reset the entitlement to its default on Save.
            dev_build: load_settings().dev_build,
        };
        save_settings(&file);
        apply_settings_live(&file);
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
