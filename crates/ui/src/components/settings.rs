//! The in-app **Settings** panel — the whole point is that a downloader can paste their own
//! OpenRouter API key here and get live readings with **no env vars and no `.env` file**. The key is
//! stored locally (in the OS user-data dir, outside the repo), masked in the UI by default, and
//! applied to the environment live on Save so it takes effect without a restart.
//!
//! One self-contained component: it renders the gear button that belongs in the header cluster and,
//! when open, a modal over a dim backdrop. It owns its own open/field/`saved` signals, so `app.rs`
//! only has to drop `SettingsButton {}` into the header.

use dioxus::prelude::*;

use crate::settings::{
    active_mode_label, apply_settings_live, load_settings, save_settings, SettingsFile,
};

#[component]
pub fn SettingsButton() -> Element {
    let mut open = use_signal(|| false);

    // Seed the fields once from the persisted settings (empty strings when unset).
    let initial = use_hook(load_settings);
    let mut key = use_signal(|| initial.openrouter_key.clone().unwrap_or_default());
    let mut model = use_signal(|| initial.model.clone().unwrap_or_default());
    let mut local_url = use_signal(|| initial.local_url.clone().unwrap_or_default());

    // UI-only state: whether the key is revealed (default masked) and whether Save just landed.
    let mut reveal = use_signal(|| false);
    let mut saved = use_signal(|| false);

    // Recomputed each render — reads only env-var *presence*, so it reflects the live mode after Save.
    let mode_label = active_mode_label();

    let save = move |_| {
        let k = key.read().trim().to_string();
        let m = model.read().trim().to_string();
        let u = local_url.read().trim().to_string();
        let file = SettingsFile {
            openrouter_key: (!k.is_empty()).then_some(k),
            model: (!m.is_empty()).then_some(m),
            local_url: (!u.is_empty()).then_some(u),
            // Preserve the developer-build switch — this modal only edits credentials, and a bare
            // literal would silently reset the entitlement to its default on Save.
            dev_build: crate::settings::load_settings().dev_build,
        };
        // Persist to disk (0600 on Unix) AND apply to the live environment so the next reading uses
        // it immediately — no restart. The user chose these by hand, so live apply overrides env.
        save_settings(&file);
        apply_settings_live(&file);
        saved.set(true);
    };

    rsx! {
        button {
            class: "gear",
            r#type: "button",
            title: "Settings — your OpenRouter key, model, and local URL (stored on this machine)",
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
                    onclick: move |e| e.stop_propagation(),

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
                        "Paste your own OpenRouter API key to get live readings — no environment "
                        "variables, no files to edit."
                    }

                    // ---- OpenRouter API key (masked) ----
                    label { class: "settings-field",
                        span { class: "settings-label", "OpenRouter API key" }
                        div { class: "settings-keyrow",
                            input {
                                class: "settings-input",
                                r#type: if *reveal.read() { "text" } else { "password" },
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: "sk-or-v1-…",
                                value: "{key}",
                                oninput: move |e| {
                                    key.set(e.value());
                                    saved.set(false);
                                },
                            }
                            button {
                                class: "settings-reveal",
                                r#type: "button",
                                "aria-label": if *reveal.read() { "Hide key" } else { "Show key" },
                                onclick: move |_| reveal.toggle(),
                                if *reveal.read() { "hide" } else { "show" }
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
                            placeholder: "anthropic/claude-sonnet-4.6",
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
                        "Stored only on this machine, in Ziqpu's user-data folder (outside the app). "
                        "Sent only to the model API you configure — never logged, never shared."
                    }

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
    }
}
