//! The in-app **Settings** panel — a downloader can paste their own hosted-provider API key here and
//! get live readings with **no env vars and no `.env` file**. Keys are stored in the OS credential
//! vault (Windows Credential Manager / macOS Keychain / Linux Secret Service — see [`crate::vault`]),
//! masked in the UI by default, and applied to the environment live on Save so they take effect
//! without a restart.
//!
//! One self-contained component: it renders the gear button that belongs in the header cluster and,
//! when open, a modal over a dim backdrop. It owns its own open/field/`saved` signals, so `app.rs`
//! only has to drop `SettingsButton {}` into the header.

use dioxus::prelude::*;

use crate::components::ModelPanel;
use crate::settings::{
    active_mode_label, apply_provider_key_live, apply_settings_live, built_in_available,
    load_settings, save_settings, SettingsFile,
};
use crate::vault::{self, Provider};

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

#[component]
pub fn SettingsButton() -> Element {
    let mut open = use_signal(|| false);

    // Seed the fields once. Provider keys come from the vault; model + local URL from settings.json.
    let initial = use_hook(load_settings);
    let mut anthropic_key = use_signal(|| vault::get_key(Provider::Anthropic).unwrap_or_default());
    let mut openrouter_key =
        use_signal(|| vault::get_key(Provider::OpenRouter).unwrap_or_default());
    let mut model = use_signal(|| initial.model.clone().unwrap_or_default());
    let mut local_url = use_signal(|| initial.local_url.clone().unwrap_or_default());
    // The explicit provider choice (slug), or None when the seeker never picked one.
    let mut provider = use_signal(|| initial.provider.clone());

    // UI-only state: per-key reveal toggles (default masked), whether Save landed, and a key-free
    // error line if the keystore couldn't be reached.
    let mut reveal_anthropic = use_signal(|| false);
    let mut reveal_openrouter = use_signal(|| false);
    let mut saved = use_signal(|| false);
    let mut save_err = use_signal(|| None::<String>);

    // Recomputed each render — reads only env-var *presence*, so it reflects the live mode after Save.
    let mode_label = active_mode_label();

    let save = move |_| {
        let ak = anthropic_key.read().trim().to_string();
        let ok = openrouter_key.read().trim().to_string();
        let m = model.read().trim().to_string();
        let u = local_url.read().trim().to_string();

        // Provider keys → OS vault (an emptied field clears that provider). Keep the first keystore
        // error, if any, to surface as a soft warning — the key is still applied live below so Live
        // works this session regardless.
        let mut err = None;
        if let Err(e) = vault::set_key(Provider::Anthropic, &ak) {
            err = Some(e);
        }
        if let Err(e) = vault::set_key(Provider::OpenRouter, &ok) {
            err.get_or_insert(e);
        }
        apply_provider_key_live(Provider::Anthropic, &ak);
        apply_provider_key_live(Provider::OpenRouter, &ok);

        // Non-secret prefs → settings.json (+ live env). Keys never touch the JSON now.
        let file = SettingsFile {
            openrouter_key: None,
            model: (!m.is_empty()).then_some(m),
            local_url: (!u.is_empty()).then_some(u),
            provider: provider.read().clone(),
            // Preserve the developer-build switch — this modal only edits credentials, and a bare
            // literal would silently reset the entitlement to its default on Save.
            dev_build: load_settings().dev_build,
        };
        save_settings(&file);
        apply_settings_live(&file);

        save_err.set(err);
        saved.set(true);
    };

    rsx! {
        button {
            class: "gear",
            r#type: "button",
            title: "Settings — your Anthropic / OpenRouter key, model, and local URL (kept in this device's keychain)",
            "aria-label": "Open settings",
            onclick: move |_| {
                saved.set(false);
                open.toggle();
            },
            "⚙ settings"
        }

        if *open.read() {
            // The dim backdrop — click it (outside the card) to dismiss.
            div {
                class: "settings-backdrop",
                onclick: move |_| open.set(false),

                // The card itself — stop clicks from bubbling to the backdrop's dismiss.
                div {
                    class: "settings-modal",
                    role: "dialog",
                    "aria-modal": "true",
                    "aria-label": "Settings",
                    tabindex: "-1",
                    autofocus: true,
                    onclick: move |e| e.stop_propagation(),
                    // Esc closes — a keyboard escape hatch alongside the × and the backdrop click.
                    onkeydown: move |e| {
                        if e.key() == Key::Escape {
                            open.set(false);
                        }
                    },

                    div { class: "settings-head",
                        h2 { class: "settings-title", "Settings" }
                        button {
                            class: "settings-x",
                            r#type: "button",
                            "aria-label": "Close settings",
                            onclick: move |_| open.set(false),
                            "×"
                        }
                    }

                    p { class: "settings-lede",
                        "Paste a hosted-provider API key to get live readings — no environment variables, "
                        "no files to edit. Keys are kept in this device's secure keychain."
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

                    // ---- Anthropic API key (masked) ----
                    label { class: "settings-field",
                        span { class: "settings-label", "Anthropic API key " span { class: "settings-opt", "(Claude · recommended)" } }
                        div { class: "settings-keyrow",
                            input {
                                class: "settings-input",
                                r#type: if *reveal_anthropic.read() { "text" } else { "password" },
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: "sk-ant-…",
                                value: "{anthropic_key}",
                                oninput: move |e| {
                                    anthropic_key.set(e.value());
                                    saved.set(false);
                                },
                            }
                            button {
                                class: "settings-reveal",
                                r#type: "button",
                                "aria-label": if *reveal_anthropic.read() { "Hide key" } else { "Show key" },
                                onclick: move |_| reveal_anthropic.toggle(),
                                if *reveal_anthropic.read() { "hide" } else { "show" }
                            }
                        }
                    }

                    // ---- OpenRouter API key (masked) ----
                    label { class: "settings-field",
                        span { class: "settings-label", "OpenRouter API key " span { class: "settings-opt", "(optional)" } }
                        div { class: "settings-keyrow",
                            input {
                                class: "settings-input",
                                r#type: if *reveal_openrouter.read() { "text" } else { "password" },
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: "sk-or-v1-…",
                                value: "{openrouter_key}",
                                oninput: move |e| {
                                    openrouter_key.set(e.value());
                                    saved.set(false);
                                },
                            }
                            button {
                                class: "settings-reveal",
                                r#type: "button",
                                "aria-label": if *reveal_openrouter.read() { "Hide key" } else { "Show key" },
                                onclick: move |_| reveal_openrouter.toggle(),
                                if *reveal_openrouter.read() { "hide" } else { "show" }
                            }
                        }
                    }

                    // ---- Model ----
                    label { class: "settings-field",
                        span { class: "settings-label", "Model" }
                        input {
                            class: "settings-input",
                            r#type: "text",
                            autocomplete: "off",
                            spellcheck: "false",
                            placeholder: "claude-opus-4-8",
                            value: "{model}",
                            oninput: move |e| {
                                model.set(e.value());
                                saved.set(false);
                            },
                        }
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

                    hr { class: "settings-sep" }
                    ModelPanel {}

                    div { class: "settings-foot",
                        span { class: "settings-active",
                            "Active: "
                            strong { "{mode_label}" }
                        }
                        div { class: "settings-actions",
                            if let Some(err) = save_err.read().clone() {
                                span { class: "provider-err", "{err}" }
                            } else if *saved.read() {
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
    }
}
