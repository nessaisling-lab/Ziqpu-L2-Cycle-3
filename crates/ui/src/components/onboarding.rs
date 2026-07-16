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

use crate::components::{BirthInputForm, Identity, KeyField, ModelPanel, ModelPicker};
use crate::settings::{built_in_available, save_model_for, save_provider};
use crate::vault::{self, KeySource, Provider};

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

    // Connect-step state: the chosen provider (None until a card is picked). The key itself is
    // [`KeyField`]'s business — it saves on paste and never hands a value back, so nothing here
    // holds one.
    let mut provider = use_signal(|| None::<Provider>);
    // The provider as a slug, mirroring `provider` — the picker keys off this, and it's set in the
    // same click that sets `provider`, so the two can't drift.
    let mut slug = use_signal(|| None::<String>);
    // Per-provider model pick from the live catalog, and a nudge to refetch once a key exists
    // (Anthropic's catalog needs one).
    let model_pick = use_signal(String::new);
    let mut catalog_reload = use_signal(|| 0u32);

    // What's already on this machine, detected once when the wizard mounts: a key Ziqpu saved on a
    // previous run, or one exported by a shell/CI/launcher. Presence and origin only — this never
    // touches a key's value. Someone who already has a key set up should be told so, not asked to
    // dig it out and paste it again.
    let detected = use_hook(|| {
        [Provider::Anthropic, Provider::OpenRouter]
            .into_iter()
            .map(|p| (p, vault::key_source(p)))
            .filter(|(_, src)| src.present())
            .collect::<Vec<_>>()
    });

    // Record the provider CHOICE + model pick, then advance. The key (if any) has already saved
    // itself in [`KeyField`]. Saving the choice matters independently of the key: without it, a key
    // that merely happens to be exported for the *other* provider keeps winning the interpreter's
    // ordering — which is the bug where picking Anthropic still read through OpenRouter.
    let save_choice = move |_| {
        let Some(p) = *provider.read() else { return };
        save_provider(p.slug());
        save_model_for(p, &model_pick.read());
        step.set(Step::Birth);
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
                        // Anything Ziqpu can already see. Presence and origin, never a value — and
                        // a reason not to make someone hunt down a key they've already set up.
                        if !detected.is_empty() {
                            div { class: "detected",
                                p { class: "detected-head", "Found on this machine" }
                                for (p, src) in detected.iter().copied() {
                                    div { class: "detected-row",
                                        key: "{p.slug()}",
                                        span {
                                            class: if src == KeySource::Env { "key-dot key-dot--env" } else { "key-dot key-dot--on" },
                                            "aria-hidden": "true",
                                        }
                                        span { class: "detected-name", "{p.label()}" }
                                        span { class: "detected-line", "{src.line()}" }
                                    }
                                }
                                p { class: "settings-hint", style: "margin:8px 0 0",
                                    "Pick that provider below and continue — no pasting needed."
                                }
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
                                        slug.set(Some(p.slug().to_string()));
                                    },
                                    span { class: "pc-name", "{p.label()}" }
                                    span { class: "pc-sub",
                                        if p == Provider::Anthropic { "Claude · recommended" } else { "Many models, one key" }
                                    }
                                }
                            }
                        }
                        if let Some(p) = *provider.read() {
                            // The live catalog for the chosen provider. OpenRouter's is public, so
                            // it fills in immediately; Anthropic's needs the key below first, and
                            // saying so is the picker's job (it shows the reason inline).
                            ModelPicker {
                                provider: slug,
                                chosen: model_pick,
                                reload: catalog_reload,
                            }
                            // Saves on paste, straight to the keychain — and a key already present
                            // shows as installed rather than as an empty box demanding it again.
                            // The reload nudge matters here: Anthropic's catalog is unreachable
                            // until a key exists, so the picker above must retry once one lands.
                            KeyField {
                                provider: p,
                                on_change: move |_| catalog_reload.with_mut(|n| *n += 1),
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
                                    onclick: save_choice,
                                    "Continue →"
                                }
                            } else {
                                button {
                                    class: "btn btn--ghost",
                                    r#type: "button",
                                    onclick: move |_| step.set(Step::Birth),
                                    "Skip for now →"
                                }
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
