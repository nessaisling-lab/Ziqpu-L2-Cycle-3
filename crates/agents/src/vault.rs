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

/// Fill each provider's env var from the vault, for **every** binary — not just the GUI.
///
/// # The gap this closes
///
/// The GUI called an equivalent of this at startup, and the vault lived in `ui`, so nothing else
/// could. A seeker who saved their key in Settings and then pointed an MCP host at `ziqpu-mcp` got
/// the deterministic template with no explanation — the key was sitting in the OS keystore and the
/// server structurally could not read it. The same was true of `ziqpu-agent` and of every eval and
/// trace harness, which is why every measurement taken while tracing this week went to whichever
/// provider happened to be exported in the shell.
///
/// **An exported variable always wins.** This only fills what is missing, so a power user, a CI job,
/// or a deliberate one-off comparison run keeps control by setting the env directly.
///
/// Best-effort and silent: no keystore (headless Linux, a locked login keyring) simply leaves the
/// environment as it was, and the caller degrades exactly as it did before. Nothing is logged,
/// because the only thing there is to log is the thing that must never be logged.
pub fn fill_env_from_vault() {
    for provider in [Provider::Anthropic, Provider::OpenRouter] {
        if std::env::var_os(provider.env_var()).is_some_and(|v| !v.is_empty()) {
            continue;
        }
        if let Some(key) = get_key(provider) {
            std::env::set_var(provider.env_var(), key);
        }
    }
}

/// The **shape** of a stored key — never its value.
///
/// A 401 is hard to diagnose on this project by design: the standing rule is that a key is never
/// shown, so "present" and "correct" look identical from every surface. That is exactly what let a
/// rejected key sit in the vault for over a month while the app reported "live" on every screen.
///
/// Shape is the honest middle ground. It separates a truncated paste, a trailing newline, or a value
/// from the wrong provider — all repairable — from a well-formed key the server still refuses, which
/// means revoked or rotated and needs replacing rather than fixing. None of it discloses the secret:
/// a length is not a key, and the prefix is printed on the provider's own dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyShape {
    /// Character count. Reported only against the expected length, never as a bare fingerprint.
    len: usize,
    /// Whether the value carries the provider's documented prefix.
    prefix_ok: bool,
    /// Whether leading or trailing whitespace survived the paste — the classic silent breaker,
    /// because a trailing newline is invisible in every UI that would show it.
    has_edge_whitespace: bool,
    /// Whether whitespace appears *inside* the value, which means a wrapped or partial paste.
    has_inner_whitespace: bool,
}

impl std::fmt::Display for KeyShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} chars · prefix {} · edge whitespace {} · inner whitespace {}",
            self.len,
            if self.prefix_ok { "ok" } else { "WRONG" },
            if self.has_edge_whitespace {
                "YES"
            } else {
                "none"
            },
            if self.has_inner_whitespace {
                "YES"
            } else {
                "none"
            },
        )
    }
}

impl KeyShape {
    /// Plain-language findings, most actionable first. Empty when the shape looks right — in which
    /// case a 401 is the provider's verdict on a well-formed key, not a formatting problem.
    pub fn diagnosis(&self, provider: Provider) -> Vec<String> {
        let mut out = Vec::new();
        if self.has_edge_whitespace {
            out.push(
                "leading/trailing whitespace — a trailing newline survives most paste fields and                  is invisible in all of them. Re-paste without it."
                    .to_string(),
            );
        }
        if self.has_inner_whitespace {
            out.push(
                "whitespace INSIDE the value — the paste wrapped or was partial. Copy the key as                  one unbroken string."
                    .to_string(),
            );
        }
        if !self.prefix_ok {
            out.push(format!(
                "does not start with {:?} — this looks like a key for a different provider, or the                  start of the value was lost.",
                provider.key_prefix()
            ));
        }
        // Length is only meaningful PER PROVIDER. The first version of this check used a single
        // `< 80` threshold calibrated on Anthropic's ~108 and applied it to both — and promptly told
        // the owner their working 73-character OpenRouter key was truncated. A diagnostic that cries
        // wolf about a healthy key is worse than none, because the next real warning is discounted.
        let expected = provider.key_len_hint();
        if self.prefix_ok && self.len + 8 < expected {
            out.push(format!(
                "{} characters, well short of the ~{expected} a {} key runs to — the paste was                  truncated.",
                self.len,
                provider.label()
            ));
        }
        out
    }
}

/// The documented prefix for a provider's keys — public information, printed on their dashboards.
impl Provider {
    pub fn key_prefix(self) -> &'static str {
        match self {
            Provider::Anthropic => "sk-ant-",
            Provider::OpenRouter => "sk-or-",
        }
    }

    /// Roughly how long a valid key from this provider runs — for spotting a truncated paste.
    ///
    /// Per provider, because the lengths genuinely differ: Anthropic's `sk-ant-api03-…` is about
    /// 108 characters, OpenRouter's `sk-or-v1-` plus 64 hex is 73. A single shared threshold
    /// reported a healthy OpenRouter key as truncated, which is the failure mode a diagnostic can
    /// least afford.
    pub fn key_len_hint(self) -> usize {
        match self {
            Provider::Anthropic => 108,
            Provider::OpenRouter => 73,
        }
    }
}

/// Describe the stored key's shape, or `None` when nothing is stored.
///
/// This is the **only** sanctioned caller of [`get_key`] besides [`key_source`] and the startup env
/// fill, and it discards the value immediately — nothing derived from it can reconstruct the key.
pub fn key_shape(provider: Provider) -> Option<KeyShape> {
    let key = std::env::var(provider.env_var())
        .ok()
        .filter(|k| !k.is_empty())
        .or_else(|| get_key(provider))?;
    let trimmed = key.trim();
    Some(KeyShape {
        len: trimmed.chars().count(),
        prefix_ok: trimmed.starts_with(provider.key_prefix()),
        has_edge_whitespace: trimmed.len() != key.len(),
        has_inner_whitespace: trimmed.chars().any(char::is_whitespace),
    })
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

    // The source-scanning guard that used to live here now lives in `ui`, beside the components
    // it scans. It broke the moment this module moved — a test that reads files by relative path
    // is coupled to its directory, not to the function it protects. See `ui::settings`.

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
