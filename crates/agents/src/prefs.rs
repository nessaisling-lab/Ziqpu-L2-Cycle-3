//! Where this app keeps its state, and the seeker's **non-secret** preferences — for every binary,
//! not just the GUI.
//!
//! # The gap this closes
//!
//! `settings.json` holds the model the seeker picked, the provider they chose, and their local-model
//! URL. Reading it lived in `crates/ui`, so the GUI honoured those choices and nothing else did.
//!
//! The symptom was measured rather than guessed: after the credential vault moved here and the CLI
//! could finally see a vaulted Anthropic key, a comparison run still used `claude-opus-4-8` while
//! the app's own banner said `claude-sonnet-5`. The key crossed the boundary; the preference did
//! not. Same shape one layer over — and worse on the MCP surface, where someone who picks a model in
//! Settings and points an MCP host at `ziqpu-mcp` gets the hardcoded default, silently, with the
//! right key.
//!
//! # Why this reads the JSON untyped
//!
//! `SettingsFile` stays in `ui`, which owns the file and every write to it. A second typed struct
//! here would be a second thing to keep in step, and the failure mode is nasty: a literal that omits
//! a field it doesn't know about silently *deletes* that field on the next save — a bug this project
//! has already had once, when a `SettingsFile { .. }` literal erased a plaintext key the migration
//! had deliberately left in place.
//!
//! So this reads the file as a `serde_json::Value` and pulls the keys it needs. **Read-only, by
//! construction**: it has no write path, so it cannot drop a field it never heard of.

use std::path::PathBuf;

/// The OS user-data directory for Ziqpu — created best-effort, shared by every persisted file
/// (`profile.json`, `settings.json`, the local server's PID).
///
/// - **Windows** — `%LOCALAPPDATA%\Ziqpu`
/// - **macOS** — `$HOME/Library/Application Support/Ziqpu`
/// - **Linux/other** — `$XDG_DATA_HOME/ziqpu` if set, else `$HOME/.local/share/ziqpu`
///
/// It lives here rather than in `ui` because the CLI surfaces need it too, and because a second
/// implementation of "where does my state live" is exactly the kind of duplicated decision that has
/// drifted three times in this codebase already.
pub fn data_dir() -> Option<PathBuf> {
    let dir = if cfg!(target_os = "windows") {
        // LOCALAPPDATA, not APPDATA. Roaming `%APPDATA%` replicates to a domain file server at
        // logoff, so on a managed machine every saved birth moment left the device by design.
        let local = PathBuf::from(std::env::var("LOCALAPPDATA").ok()?).join("Ziqpu");
        let _ = std::fs::create_dir_all(&local);
        migrate_out_of_roaming(&local);
        local
    } else if cfg!(target_os = "macos") {
        PathBuf::from(std::env::var("HOME").ok()?)
            .join("Library")
            .join("Application Support")
            .join("Ziqpu")
    } else {
        match std::env::var("XDG_DATA_HOME") {
            Ok(x) if !x.is_empty() => PathBuf::from(x).join("ziqpu"),
            _ => PathBuf::from(std::env::var("HOME").ok()?)
                .join(".local")
                .join("share")
                .join("ziqpu"),
        }
    };
    let _ = std::fs::create_dir_all(&dir);
    Some(dir)
}

/// Move an existing install's state out of roaming `%APPDATA%` into `%LOCALAPPDATA%`.
///
/// Best-effort and idempotent: a file moves only when absent at the destination, so a second call
/// does nothing and a partial move resumes. The roaming copy is removed after a successful move —
/// leaving it would defeat the point, since the stale birth data would go on replicating.
#[cfg(windows)]
fn migrate_out_of_roaming(local: &std::path::Path) {
    let Some(roaming) = std::env::var("APPDATA")
        .ok()
        .map(|a| PathBuf::from(a).join("Ziqpu"))
    else {
        return;
    };
    if !roaming.is_dir() {
        return;
    }
    for name in [
        "profile.json",
        "settings.json",
        "active_local.json",
        "llama-server.pid",
    ] {
        let (from, to) = (roaming.join(name), local.join(name));
        if from.is_file() && !to.exists() {
            // Copy-then-remove rather than rename: the two can sit on different volumes when the
            // profile is redirected, where rename fails outright.
            if std::fs::copy(&from, &to).is_ok() {
                set_owner_only(&to);
                let _ = std::fs::remove_file(&from);
            }
        }
    }
}

#[cfg(not(windows))]
fn migrate_out_of_roaming(_local: &std::path::Path) {}

/// Restrict `path` to `0o600` (owner read/write only) on Unix.
///
/// **On Windows this does nothing**, and callers must not assume otherwise: the default `%APPDATA%`
/// ACL is not a guarantee, and any process running as the user reads the file regardless. Windows is
/// this project's primary shipping platform, so the honest statement matters more than a reassuring
/// one. A real DACL needs `SetNamedSecurityInfo` and a new dependency; the mitigation that did land
/// is [`data_dir`] moving off roaming storage.
#[cfg(unix)]
pub fn set_owner_only(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

/// See the Unix variant — a no-op on non-Unix targets.
#[cfg(not(unix))]
pub fn set_owner_only(_path: &std::path::Path) {}

/// `<data_dir>/settings.json`.
pub fn settings_path() -> Option<PathBuf> {
    Some(data_dir()?.join("settings.json"))
}

/// Load the seeker's saved **non-secret** preferences into this process's environment.
///
/// **An exported variable always wins.** Only missing vars are filled, so a power user, CI, or a
/// deliberate one-off comparison run keeps control by setting the env directly — which is also what
/// makes a controlled A/B possible without touching the seeker's saved config.
///
/// Silent and best-effort: a missing or unreadable file leaves the environment untouched and the
/// caller falls back to defaults, exactly as it did before.
pub fn fill_env_from_settings() {
    let Some(path) = settings_path() else { return };
    let Ok(text) = std::fs::read_to_string(&path) else {
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        return;
    };

    // (settings.json field, environment variable) — the non-secret preferences only. Keys are never
    // read from this file; they live in the OS credential vault (see [`crate::vault`]).
    for (field, var) in [
        ("model", "ZIQPU_MODEL"),
        ("local_url", "ZIQPU_LLM_URL"),
        // Per-provider, so an id chosen for one provider can never be sent to the other.
        ("anthropic_model", "ZIQPU_ANTHROPIC_MODEL"),
        ("openrouter_model", "ZIQPU_OPENROUTER_MODEL"),
        // The explicit provider choice, which reorders the interpreter's Live attempts so the
        // seeker's pick beats a merely-present key.
        ("provider", "ZIQPU_PROVIDER"),
    ] {
        if let Some(value) = json.get(field).and_then(|v| v.as_str()) {
            if !value.is_empty() && std::env::var_os(var).is_none() {
                std::env::set_var(var, value);
            }
        }
    }

    if let Some(on) = json.get("parallel_grounding").and_then(|v| v.as_bool()) {
        if std::env::var_os("ZIQPU_PARALLEL_GROUNDING").is_none() {
            std::env::set_var("ZIQPU_PARALLEL_GROUNDING", if on { "1" } else { "0" });
        }
    }
}

/// Everything a binary needs before it reads a setting: the vaulted keys, then the saved
/// preferences.
///
/// One call, because the two were separated once and the separation is what let a CLI honour a key
/// while ignoring the model chosen next to it.
pub fn load_saved_configuration() {
    crate::vault::fill_env_from_vault();
    fill_env_from_settings();
}
