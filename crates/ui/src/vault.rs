//! OS credential-vault storage for hosted-provider API keys.
//!
//! The security requirement (owner): a live-reading API key must be **secure, always available, under
//! lock and key — not shared**. This module keeps hosted-provider keys in the platform credential
//! store — Windows Credential Manager (DPAPI), macOS Keychain, Linux Secret Service — instead of the
//! plaintext `settings.json` the OpenRouter key used to live in. A key here is never written to the
//! repo, never logged, and — via the `ureq` live path (`agents::llm_http`) — never placed on a
//! process command line.
//!
//! Everything is **best-effort**. On a headless Linux with no Secret Service, or on any keystore
//! error, the ops return `Err`/`None` and callers degrade gracefully (the key is applied to the
//! session's environment only, or Live is simply unavailable) — the app still builds and runs on
//! every platform. The keystore is the *store*; the process environment is still what the `agents`
//! crate reads at reading time (see [`crate::settings::apply_settings_to_env`]).

use keyring::Entry;

/// The keystore "service" namespace — one bucket for all Ziqpu secrets.
const SERVICE: &str = "ziqpu";

/// A hosted LLM provider whose key the vault can hold. Each maps to the env var the `agents`
/// interpreter reads and to a stable keystore account name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Provider {
    /// Anthropic (Claude) — `ANTHROPIC_API_KEY`. The default provider (owner set up first).
    Anthropic,
    /// OpenRouter — `OPENROUTER_API_KEY`.
    OpenRouter,
}

impl Provider {
    /// The keystore account under [`SERVICE`]. **Stable** — renaming it orphans stored keys.
    fn account(self) -> &'static str {
        match self {
            Provider::Anthropic => "anthropic-api-key",
            Provider::OpenRouter => "openrouter-api-key",
        }
    }

    /// The environment variable the `agents` interpreter reads for this provider.
    pub fn env_var(self) -> &'static str {
        match self {
            Provider::Anthropic => "ANTHROPIC_API_KEY",
            Provider::OpenRouter => "OPENROUTER_API_KEY",
        }
    }

    /// The stable slug persisted as `SettingsFile.provider` and exported as `ZIQPU_PROVIDER` — the
    /// value `agents`' `prefers_anthropic()` matches on. Keep in sync with that reader.
    pub fn slug(self) -> &'static str {
        match self {
            Provider::Anthropic => "anthropic",
            Provider::OpenRouter => "openrouter",
        }
    }

    /// Human label for the UI.
    pub fn label(self) -> &'static str {
        match self {
            Provider::Anthropic => "Anthropic (Claude)",
            Provider::OpenRouter => "OpenRouter",
        }
    }

    /// The key-format placeholder shown in the paste field (illustrative prefix, not a real key).
    pub fn key_hint(self) -> &'static str {
        match self {
            Provider::Anthropic => "sk-ant-…",
            Provider::OpenRouter => "sk-or-v1-…",
        }
    }
}

/// Build the keystore entry for a provider, or `None` if the platform keystore can't be reached at
/// all (no backend compiled, or — on Linux — no Secret Service running). Never logs the account.
fn entry(provider: Provider) -> Option<Entry> {
    Entry::new(SERVICE, provider.account()).ok()
}

/// Store `key` for `provider` in the OS credential vault. `Ok(())` on success; `Err(msg)` carries a
/// short, **key-free** reason (so the UI can say "couldn't save securely" without leaking anything).
/// An empty `key` **deletes** the stored key instead (clearing the provider).
pub fn set_key(provider: Provider, key: &str) -> Result<(), String> {
    let key = key.trim();
    if key.is_empty() {
        return delete_key(provider);
    }
    let entry = entry(provider).ok_or("OS credential store unavailable")?;
    entry.set_password(key).map_err(|e| e.to_string())
}

/// Where a provider's key comes from — the **only** thing the UI is allowed to learn about a key.
///
/// The owner's rule is absolute: *"it should not be able to be seen whatsoever. It should only show
/// proof of the key being present. Simply that. You're not allowed to see it."* So the surfaces ask
/// this instead of [`get_key`] — presence and origin, never the value. Nothing here can leak a
/// secret, because nothing here carries one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeySource {
    /// Saved by Ziqpu in this device's OS credential vault, and the key readings actually use.
    /// Ours — so it can be replaced or removed.
    Vault,
    /// Exported in the process environment by something that isn't Ziqpu — a shell, CI, a launcher —
    /// and we hold nothing of our own. Detected and used, but not ours to manage.
    Env,
    /// An environment key is winning **over** one we have saved. Both exist; the environment's is
    /// the one driving readings, because startup only lifts the vault into the env when the env is
    /// empty ([`crate::settings::fill_env_from_vault`]).
    ///
    /// This is its own state rather than a flavour of [`Self::Env`] because the two need opposite
    /// affordances. `Env` offers nothing to manage. Here there *is* something of ours to manage —
    /// an inert saved key — and if we called it `Vault` instead, Replace would look like it worked
    /// (the pasted key is applied to the live env for the session) and then silently revert on the
    /// next launch when the export wins again.
    EnvOverridesVault,
    /// No key for this provider anywhere.
    None,
}

impl KeySource {
    /// Whether a key exists at all.
    pub fn present(self) -> bool {
        !matches!(self, KeySource::None)
    }

    /// The status line shown to the seeker. States *that* a key is present and where it lives —
    /// never a value, never a prefix, never a length.
    pub fn line(self) -> &'static str {
        match self {
            KeySource::Vault => "Key installed — this device's keychain",
            KeySource::Env => "Key detected in your environment",
            KeySource::EnvOverridesVault => "Environment key in use — overriding your saved one",
            KeySource::None => "No key",
        }
    }
}

/// Report where the key that **actually drives readings** comes from, without reading its value.
///
/// The environment is checked **first**, and that order is the whole correctness of this function.
///
/// It reads backwards — the vault is "ours", so surely ask it first? That was the original bug. The
/// interpreter reads the key from the *process environment*
/// ([`agents::AnthropicInterpreter::from_env`]), and startup only copies the vault into the env when
/// the env is **empty** ([`crate::settings::fill_env_from_vault`] — an exported key wins, on
/// purpose, so a shell or CI can override the app). So when both exist, the environment's key is the
/// one in use and the vault's is inert. Answering "Vault" there would name a key that drives
/// nothing — and [`KeyField`](crate::components::KeyField) would offer replace/remove buttons that
/// silently fail to change anything, which is precisely the lie the env branch was written to avoid.
///
/// Checking the env first makes this report the *effective* key by construction: whatever the
/// interpreter would pick up, this names. That the vault also holds one is then irrelevant to the
/// question being asked.
pub fn key_source(provider: Provider) -> KeySource {
    if std::env::var_os(provider.env_var()).is_some_and(|v| !v.is_empty()) {
        // The env is populated — but that's also true of a key startup lifted *out of* the vault,
        // which would mislabel our own key as a stranger's. Distinguish by asking whether the vault
        // holds the very same key. Cheap, and no value leaves this function.
        match get_key(provider) {
            Some(vaulted) if env_matches(provider, &vaulted) => KeySource::Vault,
            Some(_) => KeySource::EnvOverridesVault,
            None => KeySource::Env,
        }
    } else if get_key(provider).is_some() {
        // In the vault but not in the env: a keystore that became unreachable, or a key saved after
        // startup's fill. Not driving readings this instant, but it is ours and it will be next
        // launch — report it as ours rather than as absent.
        KeySource::Vault
    } else {
        KeySource::None
    }
}

/// Whether `provider`'s env var currently holds exactly `key`. Compares in-process; the value is
/// never logged, returned, or rendered.
fn env_matches(provider: Provider, key: &str) -> bool {
    std::env::var(provider.env_var()).is_ok_and(|v| v == key)
}

/// Fetch `provider`'s stored key, or `None` when absent/empty/unavailable. Never logs the value.
///
/// **Not for display.** The only legitimate callers are [`key_source`] (which discards the value)
/// and the startup env fill. A UI surface wanting to show key state must call [`key_source`] — a
/// key that reaches a signal or a DOM `value` attribute is a key that has been shown.
pub fn get_key(provider: Provider) -> Option<String> {
    let entry = entry(provider)?;
    match entry.get_password() {
        Ok(k) if !k.is_empty() => Some(k),
        _ => None,
    }
}

/// Remove `provider`'s stored key. Idempotent — `Ok(())` also when there was nothing to delete.
pub fn delete_key(provider: Provider) -> Result<(), String> {
    let Some(entry) = entry(provider) else {
        return Ok(());
    };
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's rule, enforced: *"It should only show proof of the key being present. Simply
    /// that. You're not allowed to see it."*
    ///
    /// [`get_key`] is the only function that can hand a caller a key, so the invariant reduces to a
    /// single checkable fact: **no UI surface calls it.** This is a source-level test because that
    /// is exactly the shape of the invariant — Rust's type system can't say "this module may not
    /// call that function", and the failure it guards against is a quiet one. The old surface
    /// looked fine (the field was a masked `password` input) while the plaintext sat in a signal
    /// and in the DOM, one devtools peek from being read.
    #[test]
    fn no_ui_surface_reads_a_key_back() {
        let surfaces = [
            (
                "components/key_field.rs",
                include_str!("components/key_field.rs"),
            ),
            (
                "components/settings.rs",
                include_str!("components/settings.rs"),
            ),
            (
                "components/onboarding.rs",
                include_str!("components/onboarding.rs"),
            ),
        ];
        for (name, src) in surfaces {
            for (n, line) in src.lines().enumerate() {
                // Prose may discuss `get_key` — only executable code is in scope.
                let code = line.split("//").next().unwrap_or("");
                assert!(
                    !code.contains("get_key"),
                    "{name}:{} calls vault::get_key. A key that reaches a component is a key that \
                     can be shown. Ask key_source() for presence + origin instead.",
                    n + 1
                );
            }
        }
    }

    /// A key exists in two of the three states, and only the vault-held one is Ziqpu's to manage.
    #[test]
    fn presence_covers_vault_and_env_but_not_none() {
        assert!(KeySource::Vault.present());
        assert!(KeySource::Env.present());
        assert!(!KeySource::None.present());
    }

    /// The status lines are the *entire* public surface of a key. None of them may carry a value,
    /// a prefix, or a length — the three things that leak one.
    #[test]
    fn status_lines_describe_state_not_secrets() {
        for src in [KeySource::Vault, KeySource::Env, KeySource::None] {
            let line = src.line();
            assert!(!line.is_empty(), "{src:?} must say something");
            assert!(
                !line.contains("sk-"),
                "{src:?} line leaks a key shape: {line}"
            );
        }
        // Present states must read as present, and the absent one must not.
        assert!(KeySource::Vault.line().contains("installed"));
        assert!(KeySource::Env.line().contains("detected"));
        assert_eq!(KeySource::None.line(), "No key");
    }

    /// Each provider maps to the env var the `agents` interpreter actually reads. A drift here
    /// would make [`key_source`] report "no key" for a key that is plainly working.
    #[test]
    fn env_vars_match_the_interpreter() {
        assert_eq!(Provider::Anthropic.env_var(), "ANTHROPIC_API_KEY");
        assert_eq!(Provider::OpenRouter.env_var(), "OPENROUTER_API_KEY");
    }

    /// Every state must offer the seeker something true — including the one that says a key exists
    /// but isn't the one being used. `EnvOverridesVault` was added because collapsing it into
    /// `Vault` made the surface render replace/remove buttons for an inert key, and collapsing it
    /// into `Env` hid the only key we could actually manage.
    #[test]
    fn the_override_state_reads_as_neither_ours_nor_a_strangers() {
        let line = KeySource::EnvOverridesVault.line();
        assert!(KeySource::EnvOverridesVault.present());
        // It must name the environment as the winner — that's the whole point of the state.
        assert!(line.contains("Environment"), "{line}");
        // ...and must not claim the saved key is in use, which is what `Vault`'s line says.
        assert_ne!(line, KeySource::Vault.line());
        assert_ne!(line, KeySource::Env.line());
        assert!(!line.contains("sk-"), "{line}");
    }
}
