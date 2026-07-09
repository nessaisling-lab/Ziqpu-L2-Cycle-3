//! `ziqpu-ui` — a Dioxus desktop front-end for the two-vizier loop.
//!
//! It reuses the `agents` crate **in-process** (no server, no MCP): the same `Session`, the same
//! graded checkpoint, and the same no-advice guardrail, driven from native UI. Deterministic and
//! offline by default (mock grounded source + template interpreter), so it needs no keys or network.

mod app;
mod components;
mod state;

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::LaunchBuilder;

fn main() {
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
