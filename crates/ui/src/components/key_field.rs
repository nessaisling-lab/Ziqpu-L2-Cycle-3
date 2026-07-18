//! The API-key surface — **proof of presence, never the key**.
//!
//! ## The rule
//!
//! The owner's requirement is absolute and worth quoting, because it is the whole design:
//! *"it should not be able to be seen whatsoever. It should only show proof of the key being
//! present. Simply that. You're not allowed to see it."* This app ships to other people; a key
//! visible in a shipped build is a key that leaks.
//!
//! ## Why deleting the "show" button wasn't enough
//!
//! The previous surface seeded its field with `vault::get_key(...)` and rendered it as a password
//! input. Masking is a *rendering* choice: the plaintext still sat in a Dioxus signal and in the
//! DOM's `value` attribute, one devtools inspection (or one `r#type: "text"`) from being read. The
//! fix is that this component **never calls [`vault::get_key`]**. It asks [`vault::key_source`]
//! instead — presence and origin — which cannot carry a secret because it never holds one.
//!
//! A key therefore travels exactly one way: seeker → paste field → vault → process env → the model
//! API. There is no read path back to any screen. Losing a key means pasting a new one, which is
//! the intended trade and why [`Pending::Replace`] confirms first.
//!
//! ## Detection
//!
//! A key exported by a shell, CI, or a launcher is **detected and used** but is not Ziqpu's to
//! manage ([`KeySource::Env`]) — so this surface reports it and says how to take control, rather
//! than offering buttons that would be overridden on the next restart.

use dioxus::prelude::*;

use crate::settings::apply_provider_key_live;
use crate::vault::{self, KeySource, Provider};

/// A destructive action awaiting confirmation. Both paths delete a key that **cannot be recovered**
/// — Ziqpu can't read it back to show you, and it never had a second copy — so neither happens on a
/// single click.
#[derive(Clone, Copy, PartialEq)]
enum Pending {
    /// Delete the stored key, then open the paste field for its replacement.
    Replace,
    /// Delete the stored key and leave the provider unconfigured.
    Remove,
}

/// The key surface for one provider: a status line plus the actions its state actually allows.
///
/// `on_change` fires whenever the key state changes, so the caller can refetch anything that
/// depends on it — the Anthropic catalog is unreachable without a key, so the model picker needs
/// the nudge.
#[component]
pub fn KeyField(provider: Provider, on_change: EventHandler<()>) -> Element {
    // Presence + origin only. Re-read after every mutation so the line is never stale — and note
    // what is *absent* here: any signal that could hold a key.
    let mut source = use_signal(|| vault::key_source(provider));
    // The paste field is transient: it exists only while adding, and is wiped the moment it's used.
    let mut adding = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut pending = use_signal(|| None::<Pending>);
    let mut err = use_signal(|| None::<String>);

    // Commit the pasted key: vault first (so it survives a restart), then the live env (so the very
    // next reading uses it without one). The draft is cleared unconditionally — a key must not
    // outlive the paste that delivered it, even when the keystore write failed.
    let save = move |_| {
        let key = draft.read().trim().to_string();
        if key.is_empty() {
            err.set(Some("Paste a key first.".to_string()));
            return;
        }
        let vault_err = vault::set_key(provider, &key).err();
        apply_provider_key_live(provider, &key);
        draft.set(String::new());
        adding.set(false);
        // A keystore failure is a *persistence* failure, not a usability one: the key is live for
        // this session either way, so say what's true rather than refusing the key.
        err.set(vault_err.map(|_| {
            "Working now, but it couldn't be saved — this device's keychain wasn't reachable, \
             so you'll need to paste it again next launch."
                .to_string()
        }));
        source.set(vault::key_source(provider));
        on_change.call(());
    };

    // Delete the stored key, then either open the paste field (Replace) or stop (Remove).
    let mut confirm = move |what: Pending| {
        let was = *source.read();
        let del_err = vault::delete_key(provider).err();
        // Clear the live env var only when the key we just deleted is the one in it. Under
        // `EnvOverridesVault` the env holds the seeker's *own* export — not ours to revoke — and
        // clearing it would stop readings for this session and then silently resume them on the
        // next launch, when startup no longer overwrites it. Deleting our inert copy must do
        // exactly what the confirm said: nothing visible.
        if was == KeySource::Vault {
            apply_provider_key_live(provider, "");
        }
        pending.set(None);
        err.set(del_err);
        adding.set(matches!(what, Pending::Replace));
        source.set(vault::key_source(provider));
        on_change.call(());
    };

    let src = *source.read();

    rsx! {
        div { class: "keyfield",

            div { class: "keyfield-status",
                span {
                    class: match src {
                        KeySource::Vault => "key-dot key-dot--on",
                        KeySource::Env | KeySource::EnvOverridesVault => "key-dot key-dot--env",
                        KeySource::None => "key-dot",
                    },
                    "aria-hidden": "true",
                }
                span { class: "keyfield-name", "{provider.label()}" }
                span {
                    class: if src.present() { "keyfield-line keyfield-line--on" } else { "keyfield-line" },
                    "{src.line()}"
                }

                // Actions, gated on what each state can honestly offer.
                if pending.read().is_none() && !*adding.read() {
                    match src {
                        KeySource::Vault => rsx! {
                            div { class: "keyfield-actions",
                                button {
                                    class: "settings-reveal",
                                    r#type: "button",
                                    onclick: move |_| pending.set(Some(Pending::Replace)),
                                    "replace"
                                }
                                button {
                                    class: "settings-reveal",
                                    r#type: "button",
                                    onclick: move |_| pending.set(Some(Pending::Remove)),
                                    "remove"
                                }
                            }
                        },
                        // Nothing of ours to manage, and we can't outlast the export — the startup
                        // fill lets the environment win, so any button here would lie next launch.
                        KeySource::Env => rsx! {},
                        // We DO hold a key, but it's inert. Removing it is honest and does exactly
                        // what it says. Replace is deliberately absent: pasting would appear to work
                        // (it's applied live for the session) and then silently revert on the next
                        // launch when the export wins again — a button that lies on a delay.
                        KeySource::EnvOverridesVault => rsx! {
                            div { class: "keyfield-actions",
                                button {
                                    class: "settings-reveal",
                                    r#type: "button",
                                    onclick: move |_| pending.set(Some(Pending::Remove)),
                                    "remove saved key"
                                }
                            }
                        },
                        KeySource::None => rsx! {
                            div { class: "keyfield-actions",
                                button {
                                    class: "settings-reveal settings-reveal--add",
                                    r#type: "button",
                                    onclick: move |_| {
                                        adding.set(true);
                                        err.set(None);
                                    },
                                    "+ add key"
                                }
                            }
                        },
                    }
                }
            }

            if src == KeySource::Env {
                p { class: "settings-hint keyfield-hint",
                    "Ziqpu found this key in your environment and is using it. It isn't stored here, "
                    "so there's nothing on this screen to change — unset "
                    code { "{provider.env_var()}" }
                    " in your shell if you'd rather save a key to this device instead."
                }
            }

            if src == KeySource::EnvOverridesVault {
                p { class: "settings-hint keyfield-hint",
                    "You have a key saved on this device, but "
                    code { "{provider.env_var()}" }
                    " is set in your environment and wins — that's the key writing your readings. "
                    "Unset it to fall back to your saved one, or remove the saved one if it's stale."
                }
            }

            // The destructive confirm. The key is gone for good: Ziqpu can't show it to you, and
            // this was the only copy it kept. What happens *next* depends on the state — promising
            // "readings will stop" would be false when an environment key is standing behind it.
            if let Some(what) = *pending.read() {
                div { class: "keyfield-confirm",
                    p { class: "keyfield-warn",
                        "This deletes the API key stored on this device. "
                        if what == Pending::Replace {
                            "You'll need to paste a new one."
                        } else if src == KeySource::EnvOverridesVault {
                            "Readings will keep working — they're already running on the key in your environment, not this one."
                        } else {
                            "Live readings for this provider will stop until you add a new one."
                        }
                        " It can't be recovered — Ziqpu never shows a key back to you."
                    }
                    div { class: "keyfield-actions",
                        button {
                            class: "btn btn--ghost",
                            r#type: "button",
                            onclick: move |_| pending.set(None),
                            "cancel"
                        }
                        button {
                            class: "btn btn--danger",
                            r#type: "button",
                            onclick: move |_| confirm(what),
                            if what == Pending::Replace { "delete & paste new" } else { "delete key" }
                        }
                    }
                }
            }

            // The paste field — write-only, and the one place a key is ever on screen, as the
            // seeker types it. No reveal control exists, deliberately.
            if *adding.read() {
                div { class: "keyfield-add",
                    input {
                        class: "settings-input",
                        r#type: "password",
                        autocomplete: "off",
                        spellcheck: "false",
                        placeholder: "{provider.key_hint()}",
                        value: "{draft}",
                        oninput: move |e| {
                            draft.set(e.value());
                            err.set(None);
                        },
                    }
                    div { class: "keyfield-actions",
                        button {
                            class: "btn btn--ghost",
                            r#type: "button",
                            onclick: move |_| {
                                draft.set(String::new());
                                adding.set(false);
                                err.set(None);
                            },
                            "cancel"
                        }
                        button {
                            class: "btn btn--go",
                            r#type: "button",
                            onclick: save,
                            "save key"
                        }
                    }
                }
            }

            if let Some(msg) = err.read().clone() {
                p { class: "provider-err", "{msg}" }
            }
        }
    }
}
