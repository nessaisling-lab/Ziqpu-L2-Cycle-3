//! `ziqpu-ui` — a Dioxus desktop front-end for the two-vizier loop.
//!
//! It reuses the `agents` crate **in-process** (no server, no MCP): the same `Session`, the same
//! graded checkpoint, and the same no-advice guardrail, driven from native UI. Deterministic and
//! offline by default (mock grounded source + template interpreter), so it needs no keys or network.

// Release builds link as a GUI app (no console window on launch). Debug keeps the console so the
// ZIQPU_DEBUG traces + panics stay visible while developing. (Subprocess console flashes are
// suppressed separately via CREATE_NO_WINDOW — see `no_window` in the model/agents/ui spawn sites.)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod profile;
mod settings;
mod state;

/// Spawn a subprocess without flashing a console window on Windows (CREATE_NO_WINDOW). No-op
/// elsewhere. Wrap every `Command::new(...)` the GUI reaches so a windowless release build stays
/// windowless. Two cfg'd defs keep it warning-clean on non-Windows.
#[cfg(windows)]
pub(crate) fn no_window(mut cmd: std::process::Command) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    cmd
}
#[cfg(not(windows))]
pub(crate) fn no_window(cmd: std::process::Command) -> std::process::Command {
    cmd
}

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::LaunchBuilder;

fn main() {
    // Load the downloader's saved credentials into the environment BEFORE anything reads it
    // (`build_session` / `build_interpreter` run inside `App`). This only fills vars that are not
    // already set, so an exported env var still overrides the file for power users and CI.
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
