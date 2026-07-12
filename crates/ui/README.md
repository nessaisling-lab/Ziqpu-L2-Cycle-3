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
- **Reading-source toggle** — **Raw** (deterministic template) · **Local** (your LM Studio / llama.cpp
  on `:1234`) · **Live** (OpenRouter → Anthropic). Each mode's prose is cached, so switching is instant.
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

In-app **Settings** stores an OpenRouter key + model + local URL on this machine — no env vars needed.
Env still wins where set; see the repo `.env.example` (`ZIQPU_LLM_URL`, `ZIQPU_LOCAL_MODEL`,
`ZIQPU_MOCK`, `ZIQPU_ONBOARD`). Birth data and the API key live in the OS data dir (`0o600` on Unix).
