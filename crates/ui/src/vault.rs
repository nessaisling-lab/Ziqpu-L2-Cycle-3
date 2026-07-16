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

/// Fetch `provider`'s stored key, or `None` when absent/empty/unavailable. Never logs the value.
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
