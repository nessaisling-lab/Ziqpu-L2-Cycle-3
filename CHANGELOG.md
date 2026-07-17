# Changelog

All notable changes to Ziqpu are recorded here. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project versions as
**major.feature-phase.fix** (see [RELEASING.md](RELEASING.md)). Two tracks: `main` = stable
(GitHub "Latest"), `nightfall` = build-ahead pre-releases (never "Latest").

## [Unreleased] — built but not shipping (nightfall; not yet wired into a release)

### Built-in free Live tier — code complete, **not configured in any published build**
`proxy/` is a Cloudflare Worker that holds the real API key as a platform secret and forwards
`/v1/messages`; the app authenticates with a low-value revocable token. The app side is done and
tested (`AnthropicInterpreter::proxy_from_env`, the onboarding option, `built_in_available()`).

It does **not** exist in any installer, and this entry is here rather than under a released version
because of exactly that. `main.rs` bakes the URL + token via `option_env!` at **build** time;
`release.yml` never passes them, so they resolve to `None`, `built_in_available()` is false, and the
"free ✦" option never renders. `proxy/wrangler.toml` also still carries a placeholder KV namespace
id — the Worker has never been deployed, so there is no endpoint to bake in even if the workflow
passed the variables.

It ships when the Worker is deployed and `release.yml` is given the two secrets — no sooner. Until
then a seeker with no key of their own gets the offline template (Raw), or a local model if they
install llama.cpp themselves.

## [Unreleased] — design + research (nightfall; not yet wired into the app)

Product/design thinking for this cycle lives in the **internal living PRD** and the
`Ziqpu-Design/` working folder, not in this public tree. Summarized here for traceability.

### UI — shipped to `nightfall`
- **Persistent disclaimer footer** — the per-reading "not financial advice" reminder now shows
  once, pinned to the viewport, instead of closing every card. Disclaimer stays in the reading
  data + the no-advice guardrail; only the display is de-duplicated.
- **Selected-choice "spotlight"** — the picked ranked card stays full and rises; the rest recede.
  Ranked cards are now real keyboard controls (role/aria-current/tabindex, Enter+Space select,
  visible focus ring) — **WCAG 2.1 Level A** (2.1.1 / 4.1.2).

### Design direction (explored; internal docs)
- **Art direction crystallized** — flat mosaic-inlay Sumerian language: cuneiform-wedge atom,
  ziggurat setbacks, the 8-point star of Inanna (= live/summit only), guilloche, cylinder-seal
  frieze; **clay → shell → gold = raw → local → live**; ink-always-on-solid-substrate WCAG spine.
- **Source-selector explorations** — six layouts for the raw→local→live source ladder + model
  selector, all WCAG-AA; owner finalists **D (Rosette Dial) › B (Dawn-Horizon) › E (Register Strip)**.
- **Rosette Dial review** — model list → compact dropdown; "raw" split into **Hamun-ana** (literal
  measure) vs **Ungasaga** (interpreted template); a "prompt-ready" indicator for the local→live
  handoff draft; split-glyph icon language (identity ○◐✦ / capability / state); **DINGIR connect
  micro-spinner** (as inline SVG, codepoint **U+1202D**) while the wheat loader stays for long loads.

### Research (internal)
- **Token-efficiency + "Compact handoff encoding"** — established (web-sourced + verified + local
  token tests) that no natural language beats English on the billed tokenizer, JSON/YAML are a token
  *trap*, and the real lever is **structured-English + LLMLingua-2 compression** on the local→live
  handoff. Reframed the planned "Chinese" toggle → **Compact handoff encoding** (OFF / STRUCTURED /
  COMPRESSED). Noted its **security side-benefits** (channel obfuscation + reduced prompt-injection
  surface — defense-in-depth, *not* encryption; see [SECURITY.md](SECURITY.md)).

## [1.4.0] — 2026-07-16 · nightfall pre-release

Bring your own model, and stop being able to see your own key.

### Added
- **Live model catalogs — no hardcoded lists.** A dropdown in onboarding *and* Settings, filled from
  each provider's real API: Anthropic `GET /v1/models`, OpenRouter `GET /api/v1/models` (keyless),
  built-in via the proxy's own `/v1/models` (which asks Anthropic and filters to its allowlist, so
  the advertised list can't drift from what it accepts).
- **Badges earned from live fields, never asserted** — ✦ best for readings (fit rules: no
  `max_tokens` → unfit; mandatory reasoning → caution, since our "don't think" flag can't disable
  it; survivors ranked on the published intelligence index), ✧ best free, ★ top quality (kept
  deliberately separate: the catalog's top scorer is mandatory-reasoning, so it is *not* a pick).
  **There is no "most popular" badge** — OpenRouter publishes no usage or rank data, and we won't
  invent one.
- **Traction filter** — drop a model only if it is unused *and* unloved *and* untouched (Hugging
  Face downloads + likes + last-modified). Downloads alone is the wrong signal: a 2.1M-download
  Llama was expiring in three days while an 11.8K-download repo had shipped that morning.
  Closed-weight vendors are exempt. Cached weekly; the cache records *attempts* so an unanswerable
  repo can't trigger an endless re-sweep (which is how we rate-limited ourselves at 500 req/5 min).
- **OS credential vault** for provider keys (Windows Credential Manager / macOS Keychain / Linux
  Secret Service), with a one-time migration out of the old plaintext `settings.json`.
- **Provider-first onboarding** — pick Anthropic or OpenRouter (or skip), and it now **detects a key
  already on the machine** rather than asking for one you already configured.

### Security
- **An API key can no longer be shown.** Not "masked" — *unreachable*. Settings used to seed its
  field from the vault, so the plaintext sat in a signal and in the DOM even behind a password
  input; deleting the reveal button would have changed nothing. Surfaces now ask
  `vault::key_source()` for presence + origin and never for a value. Enforced by a test that fails
  if any UI surface calls `get_key`.
- **The SEC contact is the project's own mailbox**, not a maintainer's institutional address, and is
  overridable via `ZIQPU_EDGAR_UA`. The old one shipped in the binary and rode every user's request.
- **Ziqpu only stops the llama-server it started** (tracked by PID, guarded against PID reuse).
  It used to kill every llama-server on the machine, including yours.
- API keys travel in-process via `ureq` (never on a command line); Windows spawns are windowless.

### Fixed
- **Settings is reachable.** It was a modal; it fought back three times (header scrolled out of
  reach, then a native `<select>` popup escaped the frame, then the whole thing collapsed to a strip
  in the header). It is a page now — normal flow, nothing to trap, nothing to clip.
- **The provider you pick is the provider you get** — an explicit choice now reorders the Live
  attempts instead of losing to whatever key happened to be exported.
- **Model ids are provider-scoped**, so an OpenRouter id can't reach Anthropic and silently degrade
  the reading to the template.
- The checkpoint now names **every** source it spends (SEC EDGAR filings *and* Wikipedia). The
  reading was already disclosing Wikipedia; the consent that authorized it wasn't.
- Save can no longer delete a working key (an unreachable keystore blanked the field, and a blank
  field meant "clear this provider").

## [1.3.2] — 2026-07-14 · nightfall pre-release

Local-model runtime made reliable on real hardware.

### Added
- **CUDA-first runtime resolution** with per-vendor device pinning (`-ngl 99 -dev <token>`), so a
  hybrid laptop serves on the discrete NVIDIA GPU, not the iGPU.
- **Quant-aware model selection** — enumerate a repo's real GGUFs and pick the largest quant that
  fits the VRAM budget; `plan` diagnostic dry-runs the serve pick with no download.
- **Reconnect-to-loaded-model** + cached-aware serving (detect an already-downloaded/running model
  instead of re-downloading/re-loading).
- **Wheat loader** — the owner-chosen loading indicator (download = stalks grow; VRAM load = sway +
  red→green→gold), wired across download/serve/read.
- **RuntimeHealth** line (discrete-GPU green / integrated-or-CPU carnelian).

### Fixed
- **Real OOM cause**: uncapped serve context (full 32k KV) → cap to 8192; VRAM budget 0.80.
- Discrete-GPU pick within a backend (Vulkan iGPU-trap) via name-based discreteness ranking.
- **Single-active serve** — stop stacked llama-servers that hung the machine.
- Strip reasoning-model chain-of-thought from local readings; warm-gate local reads.
- Inline SVG favicon (kill the webview `/favicon.ico` 404).

### Changed
- Version scheme fixed to **plain semver** on nightfall tags (`v1.3.2`, not `-nightfall.N`);
  `release.yml` keys pre-release classification off the branch, not the tag.

## [1.3.0-nightfall.1] — 2026-07-12 · nightfall pre-release

- **Layered grounding pipeline** — the local model drafts the interpreter's framing brief during the
  checkpoint pause (measures only, never external data); on approval the frontier writes the grounded
  read, degrading down an honesty ladder (`GroundedRung`: Frontier / LocalGrounded / LocalUnsourced /
  Template). Unsourced reads strip any hallucinated GROUNDED beat.
- **Dev-build entitlement** — one header toggle simulates the free-customer view; paywalled
  affordances lock.

## [1.2.0] — 2026-07-12 · stable

Promoted the N2 desktop-agent work to `main` and ran a full 8-phase docs/compliance review.

### Added
- **Onboarding** (first-run birth → reveal → enter) + **anonymous chart-derived identity** handle
  (no email/login).
- **Weekly self-readings** ("Your sky") — deterministic 7-day transit summary.
- **Local-model `get`/`serve`** (llama.cpp, OpenAI-compatible on :1234) + a two-layer benchmark
  (device→tier, online best-GGUF), with a "no local model" capability floor.
- **In-app model panel** (benchmark, HF search with an uncensored-model flag, download & serve).
- Warmer Ungasaga voice (narrative measured→meaning→reminder).

### Security / legal
- 3-reviewer audit; fixed subprocess-path planting (CWE-427), `profile.json` perms (birth PII),
  GPU-probe timeouts, curl `--max-filesize`. NOTICE/`deny` reconciled with the real tree; **OFL 1.1
  license text added**.

## [1.2.0-nightfall.1] — 2026-07-12 · nightfall pre-release
- Capability-envelope reframe (`recommend_for` → Local / NoLocal); GPU/VRAM detection (no-admin);
  `get`/`serve` validated live on the dev machine.

## [1.1.0] — 2026-07-11 · stable
- Industry universes (`tickers` → Stocks · Airlines · Insurance); mixed baskets.

## [1.0.0] — 2026-07-10 · stable
- First graded artifact: two visible agents (Hamun-ana measurer + Ungasaga interpreter), the
  observe→decide→act loop with a human-in-the-loop **approval checkpoint** before the costed
  grounded pull, the no-advice guardrail, the Dioxus desktop UI, and a green 3-OS CI gauntlet.

[Unreleased]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/compare/v1.3.2...nightfall
[1.3.2]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.3.2
[1.3.0-nightfall.1]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.3.0-nightfall.1
[1.2.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.2.0
[1.1.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.1.0
[1.0.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.0.0
