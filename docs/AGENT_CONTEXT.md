# Agent context — read me first

This file is the working memory for whoever (human or AI) picks up Ziqpu next, especially from a
fresh cloud/phone session (`claude.ai/code`) that doesn't have the local `.claude` memory. It captures
current state, decisions, gotchas, and the backlog. Keep it updated as things change.

> **Ethos:** *measured, not fate — not financial advice.* Reflection, never advice. The no-advice
> guardrail is enforced in code, not just prompts.

## What Ziqpu is

A consumer **astrology decision-compatibility** desktop agent. Your birth chart × a choice's "birth
moment" (a company's IPO/founding date, etc.) → a synastry reading + a ranked list. Two visible
agents: **Hamun-ana** (measurer, temp 0, JSON) and **Ungasaga** (interpreter, warm prose). A
human-in-the-loop **checkpoint** gates the costed "grounded" pull (real external signals). Public repo:
`nessaisling-lab/Ziqpu-L2-Cycle-3`. Rust workspace + **Dioxus 0.6 desktop** UI (`ziqpu-ui`).

**Graded fellowship LIVE DEMO: Sat 2026-07-18.**

## Current state (as of v1.3.2, 2026-07-15)

Shipped **"Ziqpu Nightfall v1.3.2"** (pre-release; stable v1.2.0 stays Latest). The whole local-model
reliability run is in it and **works**: GPU-pinned serve, single-active, reconnect/no-reload, the wheat
loader, reasoning-CoT stripping, progress indicators. Local readings run on the user's GPU
(Qwen3-14B); Live readings via OpenRouter; Raw = offline template.

## Architecture (10 crates)

`ephemeris` · `engine` (interpretation IP, PolyForm licensed) · `astro` · `sidecar` (axum read-only) ·
`geo` · `tickers` (universes: stocks/airlines/insurers) · **`agents`** (the loop, interpreters,
grounded tool) · **`model`** (local-LLM benchmark + runtime + serve) · `mcp` · **`ui`** (Dioxus).

- **`ReadMode { Raw, Local, Live }`** — the reading source. `reading_for_mode()` /
  `grounded_layered()` in `crates/agents/src/interpret_llm.rs`.
- **Interpreters:** `TemplateInterpreter` (Raw/fallback, `interpret.rs`), `OpenAiCompatInterpreter`
  (Local + OpenRouter, `interpret_llm.rs` + `llm_http.rs`), `AnthropicInterpreter` (Live via Claude).
- **`model` crate** is the local-LLM brain: `recommend_for` (device→tier→ModelPick), `plan_serve`
  (fit the best quant to VRAM), `resolve_llama_server` / `running_server_port` / `model_cached`,
  `Backend`/`select_device`/`gpu_serve_args` (GPU backend + device pin), `SERVE_CTX_SIZE`.

## Key decisions & gotchas (don't re-learn these the hard way)

- **GPU serve backend.** The winget llama.cpp is a **Vulkan** build; on a hybrid laptop it can grab the
  *integrated* GPU and OOM. `select_device` prefers a **discrete** CUDA/Metal/ROCm device (and a
  discrete GPU even within Vulkan, by name). Serve pins `-ngl 99 -dev <CUDA0/Vulkan1/…>`. CUDA is
  faster than Vulkan-on-NVIDIA — the shipping upgrade is **`ensure_runtime`** (app auto-downloads the
  right per-vendor llama.cpp build; NOT built yet — a managed CUDA build laid down by hand is invisible
  to the desktop-app process, ERROR_PATH_NOT_FOUND, a per-process known-folder view thing, so the app
  must create the dir itself).
- **Single-active serve.** Every "serve" click spawns a detached llama-server; `stop_prior_servers()`
  kills the previous one first, or they STACK (4 copies of a 14B maxed the commit charge and hung the
  machine). Do not remove this.
- **Reconnect, don't reload.** `running_server_port()` finds a healthy server (`/health`=ok; LM Studio
  doesn't answer that way) → the serve path and startup reconnect instead of reloading.
- **Wheat loader = `.wload*` namespace.** The FitCard has its own living `.wheat` health plot; the
  loader MUST stay in the separate `.wload*` namespace (`components/wheat_loader.rs` + `.wload` CSS) or
  it clobbers the card's wheat. Loading beat = sway + **red→green→gold** wash; download = grow-in.
- **Reasoning models leak CoT.** `strip_reasoning()` in `llm_http.rs` drops `<think>` and keeps only
  the final `FIT:` block; OpenRouter gets `reasoning:{exclude:true}`. Needed for nemotron-super etc.
- **Versioning:** title **"Ziqpu Nightfall vX.Y.Z"**, tag plain semver (`v1.3.2`). Numbers =
  `major.feature-phase.fix`. `release.yml` classifies **off-main tags = nightfall pre-release** (never
  Latest), on-main = stable Latest — so a plain nightfall tag just works.
- **Merge policy:** nightfall↔main via **merge commits** (squash caused divergence). CI (`ci.yml`) runs
  on push-to-main or PR; for nightfall, `gh workflow run ci.yml --ref nightfall`.
- **Local model = Qwen3-14B** (Medium/16 GB tier; Q6_K fits with headroom). Gemma-3-4B is the fallback.
- **Owner preference:** for any design/style/voice decision, present **~5 distinct options** (ideally a
  visual artifact), never a single take. Owner records **voice reviews** as screen recordings —
  transcribe them with the **`/transcribe`** skill (local faster-whisper, `large-v3-turbo`) before
  acting; typed summaries are incomplete.

## Backlog — from the owner's 2026-07-14 review (priority order)

1. **Persistent disclaimer.** Drop the per-reading `REMINDER: … not financial advice`; replace with ONE
   always-visible banner. (Guardrail stays code-enforced — display only.)
2. **Selected-choice indicator.** The card selected for grounding needs an obvious selected state.
3. **RAW → LOCAL → LIVE as a clear escalation** in the mode toggle (template → local horsepower +
   tool-calling → full APIs).
4. **Model selector (offline)** — download tiered models: stable/light, medium, powerful web-scraping
   tool-caller. (The "model ledger.")
5. **Settings as a card system**, no scrolling.
6. **Onboarding + hover tooltips** — explain color codes / badges on first run and on hover-`?`.
7. **Accessibility: build the UI to WCAG 2.0/2.1 Level A ("2A").**
8. Minor: investigate a choice that produced no reading ("didn't read Home Depot").
9. **`ensure_runtime`** (cross-machine distribution: partner's Mac = Metal, office laptop = Vulkan/CPU)
   — the big one for the Saturday demo; partners have no llama.cpp installed.

## Build / test / release

```bash
cargo build --release -p ui -p model        # build the app + model CLI
cargo test -p agents -p model               # unit tests
cargo clippy -p ui -p model -p agents --all-targets -- -D warnings
cargo run -p ui                             # run the desktop app (needs a display — not on headless cloud)
```

- **Cloud/phone caveat:** a headless cloud box compiles + tests + cuts installers fine, but has **no
  screen** (can't watch the desktop GUI) and **no GPU** (can't test the llama-server local-model path).
  Code, Live/OpenRouter readings, and releases work anywhere.
- **Release:** commit to `nightfall` → `git tag -a vX.Y.Z -m "Ziqpu Nightfall vX.Y.Z"` →
  `git push origin vX.Y.Z`. GitHub Actions (`release.yml`) runs guard → gauntlet → bundle → publishes
  the Win/mac/Linux installers automatically. You can trigger this from the GitHub mobile app.

## Working from a phone

Open **`claude.ai/code`**, connect this repo, and direct the agent as usual. It reads this file for
state. Visual UI checks + GPU/local-model testing stay on a machine with a screen + GPU.
