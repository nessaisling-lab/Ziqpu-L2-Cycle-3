//! Persist the downloader's **own** model credentials so they never have to touch an env var or a
//! `.env` file. A single, best-effort JSON file — `<data_dir>/settings.json` (see
//! [`crate::profile::data_dir`]) — holds an OpenRouter API key, an optional model id, and an
//! optional local-model URL. All three feed the same env vars the `agents` crate already reads
//! ([`build_interpreter`](agents::build_interpreter) / [`reading_for`](agents::reading_for)), so
//! nothing downstream changes: the file just fills the environment.
//!
//! ## Security
//! - The file lives in the user's OS data dir, **outside the repo** — never committed.
//! - On Unix it is chmod'd to `0o600` (owner read/write only) right after each write.
//! - The key is **never logged** and never printed; the UI masks it (a password field).
//!
//! ## Precedence — env wins
//! [`apply_settings_to_env`] only sets a var that is **not already present**, so a power user or CI
//! that exports `OPENROUTER_API_KEY` (etc.) still overrides the file. The in-app Save path
//! ([`apply_settings_live`]) is the one exception: the user just chose those values explicitly, so
//! it overwrites the live environment (and clears vars for emptied fields) to take effect without a
//! restart.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The on-disk settings schema. Every field is optional so a partially-filled file (or one written
/// by an older build) still loads. `openrouter_key` → `OPENROUTER_API_KEY`, `model` → `ZIQPU_MODEL`,
/// `local_url` → `ZIQPU_LLM_URL`.
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SettingsFile {
    #[serde(default)]
    pub openrouter_key: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub local_url: Option<String>,
    /// The **developer-build** master switch (the entitlement gate). `Some(true)` = the developer
    /// build with every paywalled feature unlocked; `Some(false)` = "preview as customer", where
    /// premium features lock (each shows a 🔒). Absent → the default from [`dev_build_default`]
    /// (on, since we're still building). Persisted so the chosen view survives a restart.
    #[serde(default)]
    pub dev_build: Option<bool>,
}

/// Whether the developer build is on — premium features unlocked. Defaults to **on** while we're
/// building (so the developer sees everything); flip it off to preview the free-customer experience.
pub fn dev_build_default() -> bool {
    load_settings().dev_build.unwrap_or(true)
}

/// Persist just the developer-build switch, leaving the credential fields untouched. Best-effort
/// (a failed read/write is swallowed — never panics).
pub fn save_dev_build(on: bool) {
    let mut settings = load_settings();
    settings.dev_build = Some(on);
    save_settings(&settings);
}

/// A **redacting** `Debug` — deliberately hand-written (not derived) so a stray `dbg!(settings)` or a
/// log line can never spill the API key. The key is shown only as present/absent + its length.
impl std::fmt::Debug for SettingsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match &self.openrouter_key {
            Some(k) if !k.is_empty() => format!("Some(<redacted {} chars>)", k.len()),
            _ => "None".to_string(),
        };
        f.debug_struct("SettingsFile")
            .field("openrouter_key", &format_args!("{key}"))
            .field("model", &self.model)
            .field("local_url", &self.local_url)
            .finish()
    }
}

/// `<data_dir>/settings.json`, or `None` when no user-data dir can be resolved.
fn settings_path() -> Option<PathBuf> {
    Some(crate::profile::data_dir()?.join("settings.json"))
}

/// Read + deserialize the settings. Returns an empty [`SettingsFile`] on any missing/unreadable/
/// corrupt file — never panics — so startup and the Settings panel always have something to seed
/// from.
pub fn load_settings() -> SettingsFile {
    let Some(path) = settings_path() else {
        return SettingsFile::default();
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return SettingsFile::default();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

/// Serialize + write the settings, best-effort (any error is swallowed — never panics). On Unix the
/// file is then chmod'd to `0o600` so only the owner can read the API key.
pub fn save_settings(settings: &SettingsFile) {
    let Some(path) = settings_path() else {
        return;
    };
    let Ok(text) = serde_json::to_string_pretty(settings) else {
        return;
    };
    if std::fs::write(&path, text).is_ok() {
        // Owner-only perms on Unix so the key at rest isn't world/group readable. Best-effort.
        set_owner_only(&path);
    }
}

/// Restrict `path` to `0o600` (owner read/write only) on Unix. A no-op on Windows, where NTFS ACLs
/// already scope a user's `%APPDATA%` to that user.
#[cfg(unix)]
pub(crate) fn set_owner_only(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

/// See the Unix variant — a no-op on non-Unix targets.
#[cfg(not(unix))]
pub(crate) fn set_owner_only(_path: &std::path::Path) {}

/// Load the persisted settings into the process environment **without clobbering** any var that is
/// already set — so an exported `OPENROUTER_API_KEY` / `ZIQPU_MODEL` / `ZIQPU_LLM_URL` (power users,
/// CI) still wins. Call once at startup, before [`build_session`](crate::state::build_session) /
/// [`build_interpreter`](agents::build_interpreter) read the environment.
pub fn apply_settings_to_env(settings: &SettingsFile) {
    fn set_if_absent(key: &str, val: &Option<String>) {
        if let Some(v) = val {
            if !v.is_empty() && std::env::var_os(key).is_none() {
                std::env::set_var(key, v);
            }
        }
    }
    set_if_absent("OPENROUTER_API_KEY", &settings.openrouter_key);
    set_if_absent("ZIQPU_MODEL", &settings.model);
    set_if_absent("ZIQPU_LLM_URL", &settings.local_url);
}

/// Apply the settings to the live environment on Save so a new value takes effect on the next
/// reading (`agents::reading_for` reads the env fresh per call) without a restart. A **non-empty**
/// field overrides the live var; an **empty** field is **left alone** — it does NOT remove the var.
///
/// This is deliberate: the key field is only ever pre-seeded from `settings.json`, so a key provided
/// via the shell/env (or not yet saved to the file) leaves the field blank — and a blank-field Save
/// used to `remove_var` it, silently dropping Live mid-session (observed in testing). Saving can only
/// ever *set* a value now; to remove a saved key, clear it from `settings.json` (or the data folder).
pub fn apply_settings_live(settings: &SettingsFile) {
    fn set_if_present(key: &str, val: &Option<String>) {
        if let Some(v) = val {
            if !v.is_empty() {
                std::env::set_var(key, v);
            }
        }
        // Empty/None → leave the existing env var untouched (never wipe a live key on an empty field).
    }
    set_if_present("OPENROUTER_API_KEY", &settings.openrouter_key);
    set_if_present("ZIQPU_MODEL", &settings.model);
    set_if_present("ZIQPU_LLM_URL", &settings.local_url);
}

/// A short, human-readable label for which interpreter the **current environment** selects — shown
/// in the Settings panel so Save gives immediate, truthful feedback about what a reading will use.
/// Mirrors [`agents::build_interpreter`]'s precedence (OpenAI-compat/OpenRouter → Anthropic →
/// template). Reads only presence, never the key value.
pub fn active_mode_label() -> &'static str {
    if std::env::var_os("OPENROUTER_API_KEY").is_some()
        || std::env::var_os("OPENAI_API_KEY").is_some()
    {
        "Live · OpenRouter / OpenAI-compatible"
    } else if std::env::var_os("ANTHROPIC_API_KEY").is_some() {
        "Live · Anthropic (Claude)"
    } else {
        "Offline template · no key set"
    }
}
