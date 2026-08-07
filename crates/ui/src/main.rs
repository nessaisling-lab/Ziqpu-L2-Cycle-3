//! `ziqpu-ui` — a Dioxus desktop front-end for the two-vizier loop.
//!
//! It reuses the `agents` crate **in-process** (no server, no MCP): the same `Session`, the same
//! graded checkpoint, and the same no-advice guardrail, driven from native UI. Deterministic and
//! offline by default (mock grounded source + template interpreter), so it needs no keys or network.

// Release builds link as a GUI app (no console window on launch). Debug keeps the console so the
// ZIQPU_DEBUG traces + panics stay visible while developing. (Subprocess console flashes are
// suppressed separately via CREATE_NO_WINDOW — see `child_cmd` in the model/agents/ui spawn sites.)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod preflight;
mod profile;
mod settings;
mod state;
mod vault;

/// Prepare a subprocess the GUI is about to spawn: **strip inherited credentials**, and don't flash
/// a console window on Windows. Wrap every `Command::new(...)` the GUI reaches.
pub(crate) fn child_cmd(cmd: std::process::Command) -> std::process::Command {
    no_console_window(strip_credentials(cmd))
}

/// Remove every environment variable that looks like a credential before a child inherits it.
///
/// This crate is where the leak began: `apply_settings_to_env` copies the vaulted provider keys into
/// this process at startup (that is what makes a vaulted key "always available"), and every child
/// then inherits them — most importantly `llama-server`, a third-party binary the app downloads at
/// runtime and executes. Stripping by **shape rather than by a list of names** is deliberate: a list
/// has to be edited whenever a provider is added, and the forgotten edit is the one that leaks.
/// Kept in step with the identical rule in `agents` and `model`.
fn strip_credentials(mut cmd: std::process::Command) -> std::process::Command {
    for (name, _) in std::env::vars_os() {
        if name.to_str().is_some_and(looks_like_credential) {
            cmd.env_remove(&name);
        }
    }
    cmd
}

/// Whether an environment variable name looks like it carries a secret.
fn looks_like_credential(name: &str) -> bool {
    let n = name.to_ascii_uppercase();
    n.ends_with("_KEY") || n.ends_with("_TOKEN") || n.ends_with("_SECRET")
}

/// Don't flash a console window on Windows (CREATE_NO_WINDOW). No-op elsewhere; two cfg'd defs keep
/// it warning-clean on non-Windows.
#[cfg(windows)]
fn no_console_window(mut cmd: std::process::Command) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    cmd
}
#[cfg(not(windows))]
fn no_console_window(cmd: std::process::Command) -> std::process::Command {
    cmd
}

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::LaunchBuilder;

fn main() {
    // FIRST, before anything that can fail. A release build has no console (see the attribute
    // above), so without this a panic on the way to the window — the common one being a missing
    // WebView2 runtime — exits silently: no window, no error, nothing. The user concludes the
    // download is broken. This turns that into a dialog that names the cause.
    preflight::install_startup_dialog();

    // Then fix the common cause instead of just narrating it: on Windows, detect the WebView2
    // Runtime in the registry and — if it's missing — offer to download Microsoft's bootstrapper
    // and install it per-user (no admin rights), so a fresh machine reaches the window on the
    // first double-click. No-op on macOS/Linux and, in the overwhelmingly common already-installed
    // case, a single registry read.
    preflight::ensure_webview2();

    // Bake the built-in free-tier proxy config into the runtime env. `ZIQPU_PROXY_URL`/`_TOKEN` are
    // set at RELEASE BUILD time (via the release workflow's env) and captured by `option_env!` here —
    // so the shipped binary carries the proxy URL + the (revocable, rate-limited) app token, but
    // NEVER the Anthropic key (that lives only on the proxy). A dev build with neither set simply
    // omits the built-in tier. An explicit runtime env var still wins.
    if let Some(url) = option_env!("ZIQPU_PROXY_URL") {
        if !url.is_empty() && std::env::var_os("ZIQPU_PROXY_URL").is_none() {
            std::env::set_var("ZIQPU_PROXY_URL", url);
        }
    }
    if let Some(token) = option_env!("ZIQPU_PROXY_TOKEN") {
        if !token.is_empty() && std::env::var_os("ZIQPU_PROXY_TOKEN").is_none() {
            std::env::set_var("ZIQPU_PROXY_TOKEN", token);
        }
    }

    // Upgrade any pre-vault install first: move a plaintext OpenRouter key out of settings.json and
    // into the OS credential vault BEFORE we read settings into the environment.
    settings::migrate_plaintext_keys_to_vault();

    // Load the downloader's saved prefs + vaulted provider keys into the environment BEFORE anything
    // reads it (`build_session` / `build_interpreter` run inside `App`). This only fills vars that
    // are not already set, so an exported env var still overrides for power users and CI.
    settings::apply_settings_to_env(&settings::load_settings());

    // If a local model server is already running — this machine's serve left alive from a prior
    // session — reconnect to it so Local mode works immediately, with no reload. An explicit
    // `ZIQPU_LLM_URL` (power users / CI) still wins.
    if std::env::var("ZIQPU_LLM_URL").is_err() {
        if let Some(port) = model::running_server_port() {
            std::env::set_var("ZIQPU_LLM_URL", format!("http://127.0.0.1:{port}/v1"));
        }
    }

    LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Ziqpu")
                    .with_inner_size(LogicalSize::new(1100.0, 820.0)),
            ),
        )
        .launch(app::App);
}
