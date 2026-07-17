# `ui` — the Ziqpu desktop app

Dioxus 0.6 desktop (binary `ziqpu-ui`) — the seeker-facing surface over the two-vizier loop:
**Setup → Ranked fits → Checkpoint → Grounded briefing**, plus onboarding and self-readings.

## Run

```bash
cargo run -p ui
```

The stylesheet (`assets/ziqpu.css`, with base64-embedded OFL fonts) is inlined into the binary, so a
plain `cargo build` produces a fully-branded app with no external assets.

## What it does

- **The loop, visibly** — rank choices by fit → pick one → the human-in-the-loop **checkpoint** (the
  gate before the costed grounded pull) → the grounded briefing. A **Backstage** strip shows
  Hamun-ana's raw JSON beneath Ungasaga's prose.
- **Reading-source toggle** — **Raw** (deterministic template) · **Local** (a model on your own GPU) ·
  **Live** (OpenRouter → Anthropic). Each mode's prose is cached, so switching is instant.
- **In-app model panel** (Settings + onboarding) — benchmark the machine (tier · GPU · RAM), search
  Hugging Face (with an uncensored-model flag), and **download & serve** a local model with no CLI:
  CUDA-first GPU pinning, quant-aware fit, single-active serve, reconnect-not-reload. The **wheat
  loader** shows progress (download = stalks grow; VRAM load = sway, red→green→gold). See the
  [`model`](../model/README.md) crate.
- **Layered grounding** — at the checkpoint the local model drafts the frontier's brief; the grounded
  read is badged by rung (`GROUNDED · LIVE` / `GROUNDED · LOCAL` / `LOCAL · UNSOURCED` / `GROUNDED`).
  See [docs/PRD-layered-grounding-pipeline.md](../../docs/PRD-layered-grounding-pipeline.md).
- **Onboarding** — a first-run gate (welcome → birth chart → anonymous two-word handle). Force it for
  a demo/QA pass with a truthy `ZIQPU_ONBOARD`.
- **Your sky** — daily + weekly self-readings from your own chart's transits (deterministic).
- **Developer-build toggle** — `⚙ dev build ↔ 👤 customer view`; paywalled features (editing the
  frontier brief, the frontier-drafter path) lock with a 🔒 in customer view. Persisted in
  `settings.json`.

## Config

In-app **Settings** (a page — `⚙ settings` in the header) holds the provider choice, the model, and
an optional local endpoint. No env vars needed; env still wins where set — see the repo
`.env.example` (`ZIQPU_LLM_URL`, `ZIQPU_LOCAL_MODEL`, `ZIQPU_MOCK`, `ZIQPU_ONBOARD`).

**Two stores, split by sensitivity:**

- **API keys → the OS credential vault** (Windows Credential Manager · macOS Keychain · Linux Secret
  Service), via `src/vault.rs`. Never written to disk in the clear, never placed on a command line,
  and **never displayed** — the UI asks `vault::key_source()` for presence and origin, never for a
  value, and a test fails the build if any component calls `get_key`. A key from a pre-vault install
  is migrated out of `settings.json` on startup.
- **Non-secrets → `<data_dir>/settings.json`**; birth data → `<data_dir>/profile.json`. Both are
  `0o600` on Unix (the profile holds birth PII).
