//! First-run onboarding gate — **welcome → live readings → birth chart → reveal handle → local
//! model → enter**.
//!
//! Shown only for a brand-new seeker (no saved profile); returning seekers skip it entirely (see the
//! gate in [`crate::app`]). The first thing asked is which hosted provider powers live readings
//! (Anthropic or OpenRouter) — paste a key into the OS credential vault, or skip. It then reuses
//! [`BirthInputForm`] in `reveal_mode` (which advances instead of running the loop), the [`Identity`]
//! card for the reveal, and [`ModelPanel`] for the optional local-model setup — so the wizard adds a
//! flow, not new forms. On "Enter Ziqpu" it fires [`on_done`], dropping the gate.

use dioxus::prelude::*;

use crate::components::{BirthInputForm, Identity, ModelPanel};
use crate::settings::{apply_provider_key_live, built_in_available, save_provider};
use crate::vault::{self, Provider};

/// The beats of the gate.
#[derive(Clone, Copy, PartialEq)]
enum Step {
    Welcome,
    /// Choose a hosted provider + paste a key (or skip) — the first thing asked.
    Connect,
    Birth,
    Reveal,
    Model,
}

#[component]
pub fn Onboarding(on_done: EventHandler<()>) -> Element {
    let mut step = use_signal(|| Step::Welcome);

    // Connect-step state: the chosen provider (None until a card is picked), the masked key field,
    // whether the key is revealed, and a short, key-free status line (error or "saved").
    let mut provider = use_signal(|| None::<Provider>);
    let mut key_field = use_signal(String::new);
    let mut reveal = use_signal(|| false);
    let mut status = use_signal(|| None::<String>);

    // Save the pasted key to the OS vault and apply it live, then advance. On a vault failure (no
    // keystore) the key is still applied to the session so Live works now — we just warn that it
    // won't persist, and the "Continue" path below still lets them proceed.
    let save_key = move |_| {
        let Some(p) = *provider.read() else { return };
        let k = key_field.read().trim().to_string();
        if k.is_empty() {
            status.set(Some("Paste a key first, or skip.".to_string()));
            return;
        }
        match vault::set_key(p, &k) {
            Ok(()) => {
                apply_provider_key_live(p, &k);
                // Record the CHOICE too, not just the key — otherwise a key that merely happens to
                // be exported for the other provider would keep winning the interpreter's ordering.
                save_provider(p.slug());
                step.set(Step::Birth);
            }
            Err(_) => {
                // Couldn't reach the keystore — use the key for this session only.
                apply_provider_key_live(p, &k);
                save_provider(p.slug());
                status.set(Some(
                    "Saved for this session only — this device's keychain wasn't reachable. \
                     Use “Continue” to go on."
                        .to_string(),
                ));
            }
        }
    };

    rsx! {
        div { class: "onboarding",
            div { class: "onboarding-card",
                {match *step.read() {
                    Step::Welcome => rsx! {
                        p { class: "eyebrow", "Welcome" }
                        h1 { class: "onboarding-title", "Ziqpu" }
                        p { class: "onboarding-lede",
                            "You're about to get a chart-self — an anonymous reading identity drawn from "
                            "your birth moment. No email, no account, no password. Just a handle and a chart, "
                            "kept on this machine."
                        }
                        p { class: "onboarding-sub", "the ledger of the sky · measured, not fate" }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--go",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Connect),
                                "Begin →"
                            }
                        }
                    },
                    Step::Connect => rsx! {
                        p { class: "onboarding-step", "Step 1 of 4 · live readings" }
                        h2 { class: "onboarding-title", style: "font-size:27px;margin:2px 0 12px",
                            "Power your readings"
                        }
                        p { class: "onboarding-lede",
                            "Ziqpu can write each reading with a hosted model. "
                            if built_in_available() {
                                "Use the built-in reader — free, no key, no setup — or connect your own provider below."
                            } else {
                                "Choose a provider and paste your API key — it's kept in this device's secure keychain, never in a file. Prefer to explore first? Skip — Ziqpu still runs offline, and you can add a key any time from Settings."
                            }
                        }
                        // The recommended zero-setup path: Ziqpu's built-in free tier (the key proxy).
                        // Shown only when this build ships a configured proxy.
                        if built_in_available() {
                            button {
                                class: "btn btn--go",
                                style: "width:100%;margin:2px 0 6px",
                                r#type: "button",
                                onclick: move |_| {
                                    save_provider("built_in");
                                    step.set(Step::Birth);
                                },
                                "Use Ziqpu's built-in reader — free ✦"
                            }
                            p { class: "onboarding-lede onboarding-lede--muted", style: "margin:2px 0 14px",
                                "Recommended. Or bring your own key:"
                            }
                        }
                        div { class: "provider-grid",
                            for p in [Provider::Anthropic, Provider::OpenRouter] {
                                button {
                                    key: "{p.label()}",
                                    class: if *provider.read() == Some(p) {
                                        "provider-card provider-card--on"
                                    } else {
                                        "provider-card"
                                    },
                                    r#type: "button",
                                    "aria-pressed": if *provider.read() == Some(p) { "true" } else { "false" },
                                    onclick: move |_| {
                                        provider.set(Some(p));
                                        status.set(None);
                                    },
                                    span { class: "pc-name", "{p.label()}" }
                                    span { class: "pc-sub",
                                        if p == Provider::Anthropic { "Claude · recommended" } else { "Many models, one key" }
                                    }
                                }
                            }
                        }
                        if let Some(p) = *provider.read() {
                            label { class: "settings-field",
                                span { class: "settings-label", "{p.label()} API key" }
                                div { class: "settings-keyrow",
                                    input {
                                        class: "settings-input",
                                        r#type: if *reveal.read() { "text" } else { "password" },
                                        autocomplete: "off",
                                        spellcheck: "false",
                                        placeholder: "{p.key_hint()}",
                                        value: "{key_field}",
                                        oninput: move |e| {
                                            key_field.set(e.value());
                                            status.set(None);
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
                            if let Some(msg) = status.read().clone() {
                                p { class: "provider-err", "{msg}" }
                            }
                        }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Welcome),
                                "← back"
                            }
                            if provider.read().is_some() {
                                button {
                                    class: "btn btn--go",
                                    r#type: "button",
                                    onclick: save_key,
                                    "Save & continue →"
                                }
                            }
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Birth),
                                if provider.read().is_some() { "Continue →" } else { "Skip for now →" }
                            }
                        }
                    },
                    Step::Birth => rsx! {
                        p { class: "onboarding-step", "Step 2 of 4 · your birth moment" }
                        // reveal_mode: the form saves the chart + sets the seeker, then advances here
                        // rather than running the graded loop.
                        BirthInputForm { reveal_mode: true, on_continue: move |_| step.set(Step::Reveal) }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Connect),
                                "← back"
                            }
                        }
                    },
                    Step::Reveal => rsx! {
                        p { class: "onboarding-step", "Step 3 of 4 · meet your chart-self" }
                        // The Identity card reads the just-set seeker: it shows the chart-derived handle
                        // with the re-roll / reset controls, so the reveal *is* the identity surface.
                        Identity {}
                        p { class: "onboarding-lede onboarding-lede--muted",
                            "This is your anonymous handle. Re-roll until it fits — you can change it any "
                            "time from Setup. Nothing here leaves your machine."
                        }
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Birth),
                                "← edit chart"
                            }
                            button {
                                class: "btn btn--go",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Model),
                                "Next: local model →"
                            }
                        }
                    },
                    Step::Model => rsx! {
                        p { class: "onboarding-step", "Step 4 of 4 · your local model (optional)" }
                        p { class: "onboarding-lede",
                            "Ziqpu can run readings on your own machine — private, offline, free. "
                            "Benchmark to see the best model for your hardware and set it up now, or skip "
                            "and use the offline template (Raw) or your hosted key (Live). You can always do "
                            "this later from Settings."
                        }
                        ModelPanel {}
                        div { class: "onboarding-actions",
                            button {
                                class: "btn btn--ghost",
                                r#type: "button",
                                onclick: move |_| step.set(Step::Reveal),
                                "← back"
                            }
                            button {
                                class: "btn btn--go",
                                r#type: "button",
                                onclick: move |_| on_done.call(()),
                                "Enter Ziqpu →"
                            }
                        }
                    },
                }}
            }
        }
    }
}
