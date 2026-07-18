# Changelog

All notable changes to Ziqpu are recorded here. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project versions as
**major.feature-phase.fix** (see [RELEASING.md](RELEASING.md)). Two tracks: `main` = stable
(GitHub "Latest"), `nightfall` = build-ahead pre-releases (never "Latest").

## [1.5.0] — 2026-07-18 · nightfall pre-release

**Honest local models + the first step of "chart anything."** Five features on top of the v1.4.1
distribution build — three that make the local-model and free-tier experience honest and durable,
and two that lay the foundation for the origin-resolver (N3). Every user-facing claim is backed by
code; nothing here overstates.

### The built-in free tier now tells you when it's spent
The v1.4.1 changelog promised the app "says so honestly" when the free tier is over budget — now the
behavior exists. The proxy returns `429 monthly_budget_exhausted` / `rate_limited` (and `503
service_disabled` for the kill switch); the HTTP layer used to collapse every non-2xx to "nothing
came back", so a seeker silently got the offline template. New `agents::tier` classifies that
response (keyed on the proxy's error-string contract — a generic 500 or an offline blip never
masquerades as "over budget") and drives a one-line notice: *"the built-in free readings are over
their budget for now… add your own API key or run a local model."* Only the built-in proxy path
moves the latch — your own key or OpenRouter never does — and it resets each ranking, so the notice
vanishes the moment you switch to your own key.

### Your local model is remembered across restarts
The served local model was forgotten every launch (the app reconnected only while the server stayed
alive, and never recorded *which* model). Now the single active model persists beside the PID as the
whole resolved plan — repo, quant, size, and crucially `fits_gpu`, so the anti-OOM CPU-side decision
survives a restart. On launch: server still up → a "Local model ready — model on :port" chip with a
**Stop** (frees the VRAM); server gone → a one-click **Re-serve** of the exact same model, no
re-benchmark. Single-active is now an explicit record (one file, one model, or none). The ~150-line
serve machinery is one shared `serve_target`, so the benchmark-serve and the re-serve behave
identically.

### Local tool-calling — the capability behind "chart anything"
New `agents::tools`: an OpenAI-compatible agentic tool-call loop (request → the model asks to call
tools → execute → feed results back → repeat, with a step cap so it can never hang). A `Tool` trait
lets future resolvers drop in. The serve now passes `--jinja` so a local `llama-server` uses the
model's chat template + tool-call grammar (additive for plain readings; every recommended model
ships a template). This is the engine the origin-resolver runs on — works against a local model or a
hosted one.

### Capability badges — pick a model that can do the job
`model::ModelPick` gains `Caps { tools, vision, reasoning, ctx }`, surfaced as chips under the
selected local pick so you can tell, before serving, whether it can drive the tool-calling flow.
Honest values: every recommended tier pick is tool-capable (the ledger only recommends models that
can); reasoning distinguishes the Qwen3 family + GPT-OSS; vision is uniformly off (the local picks
are text models); the forced sub-floor 3B is marked *not* a reliable tool-caller.

### N3 "chart anything" — cars, step one: the VIN resolver
New `agents::vin`: decode a car from its VIN via NHTSA vPIC (free, keyless, public domain) →
make/model/year + assembly plant. Deliberately honest about what a VIN *is*: the resolver, **not a
date source**. A VIN carries only the model year, never a chartable build date — so this hands back
identity + the plant (the origin place); the chartable moment is deferred rather than fabricated (no
year-only Jan-1). A `DecodeVinTool` lets the tool-loop decode a VIN mid-conversation. Live-verified:
`1HGCM82633A004352 → "2003 Honda Accord EX-V6 — Marysville, Ohio, United States"`.

### Under the hood
A Windows-only CI test flake (a mock socket that RST'd on close before draining the request) was made
robust. No new dependencies. All local picks stay off-by-default in the graded path; the demo remains
self-contained.

## [1.4.1] — 2026-07-17 · nightfall pre-release

**The distribution release.** Outside evaluators download this from the repo and run it on their
own machines; every change here exists so that "runs on their machine as though it's mine" is true
rather than hoped.

### Windows — WebView2 installs itself
The panic-dialog floor (1.4.0) told a stranger *why* the window never opened; now the common cause
is fixed before it happens. `preflight::ensure_webview2` reads the registry the way Microsoft
documents for deployment detection (the `pv` value under the Runtime client GUID — HKLM WOW6432Node,
HKLM plain, and HKCU for per-user installs; missing/empty/`0.0.0.0` = absent) and, if absent, offers
to download the official Evergreen Bootstrapper (~2 MB, in-process HTTPS) and run it
`/silent /install` — a **per-user** install, no UAC, so a non-admin evaluator on a locked-down
machine still reaches the window. Every prompt is a native MessageBox, because a progress *window*
would itself need the webview being installed.

### macOS — one universal app
`Ziqpu-<tag>-macos-universal.dmg` now carries both architectures (`lipo` arm64 + x86_64) and runs
natively on Apple Silicon **and** Intel Macs, macOS 11+. The "Apple Silicon only" warning is gone
because it stopped being true.

### Local models — the app installs its own runtime, and the choice is curated
- **`ensure_runtime`** (model crate + `ziqpu-model runtime`): fetches the backend-correct
  llama.cpp build for this OS/arch/GPU from the official releases — NVIDIA → CUDA (12 vs 13 by card
  generation, Blackwell needs 13, Pascal needs 12) plus the version-matched cudart DLLs laid beside
  the exe; AMD/Intel → Vulkan (deliberately not HIP/SYCL: baked gfx lists and oneAPI system deps
  are remote hard-fails); Apple → Metal; no GPU → CPU, never Vulkan. One-time, per-user, no admin.
  `serve` (CLI and UI) self-installs instead of dead-ending on "install llama.cpp first".
- **The curated flow** (owner spec #25): benchmark before anything; below the floor = **wild
  wheat** (bent, muted, unharvestable) and *nothing installs*; above it the machine wears its tier
  as a card-sized wheat emblem (seed → red → green → gold). The model dropdown offers exactly
  **#1 Stable** (tier-correct) and **#2 Max** (the biggest model the machine can hold — bigger
  answers, slower tokens), and never a model the machine can't run: a Max pick too big for the
  card's memory serves **CPU-side, RAM-fitted** (labelled so) instead of being forced onto the GPU.

### Pre-tag adversarial audit (4 lenses + refute — the v1.4.0 discipline, kept)
11 raw findings, 6 refuted under adversarial verification, 5 confirmed and fixed before this tag:
a failed runtime install could leave a half-laid-down llama.cpp that later launches trusted forever
(now: self-check on the fast path + partial trees removed on any failure); a "Max" model bigger
than the GPU's pool was forced onto the card with full offload (now the CPU-side fallback above);
and three user-facing lines that claimed more than the code delivers (the free-tier budget wording,
the below-floor guidance that ignored the built-in tier, a button asserting a download that might
not happen) were rewritten to what is actually true.

### Built-in free Live tier — **now configured and shipping**
The 1.4.0 changelog recorded, honestly, that the proxy tier existed in code and in no artifact.
Both halves are now real: the Worker is deployed (KV namespace id committed in
`proxy/wrangler.toml`), and `release.yml` passes `ZIQPU_PROXY_URL`/`ZIQPU_PROXY_TOKEN` from repo
secrets into the build — with a loud per-build notice stating whether the tier is present, so its
absence can never again be silent. The binary carries only the proxy URL and a revocable,
rate-limited token; the real API key lives solely on the Worker.

### Docs
`INSTALL_NOTES.md` (published as every release body) rewritten to match: WebView2 auto-install,
the universal .dmg, the three no-setup roads (built-in free tier / own key / self-installing local
model), and a Known-limitations list that now contains only things that are actually still true.

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

[Unreleased]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/compare/v1.4.0...nightfall
[1.4.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.4.0
[1.3.2]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.3.2
[1.3.0-nightfall.1]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.3.0-nightfall.1
[1.2.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.2.0
[1.1.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.1.0
[1.0.0]: https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases/tag/v1.0.0
