# Releasing — `stable` & `nightfall`

Ziqpu ships on **two tracks**.

## `main` — **stable**
The audited, all-green release line. Every commit is 3-OS CI-green across every gate
(test · fmt · clippy · smoke · security [deny / audit / gitleaks] · integration · DCO) and
`cargo deny` licenses/bans/sources are clean (protects the PolyForm commercial path).
Tagged `vX.Y.Z`. Only code that has soaked on `nightfall` — or a small, low-risk fix —
lands here. **`v1.0.0` = the Phase 0–4 build** (engine · agent loop · MCP · desktop UI ·
weighted synastry · birth-input + geocoding + daily reading).

## `nightfall` — **nightly / experimental**
The integration branch for in-progress and experimental work — astrocartography
([#21](https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/issues/21)), mobile
([#19](https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/issues/19)), full-coverage
geocoding, partner-domain datasets, … Same CI gates as stable. Features land here first,
soak, then get **promoted** to `main` when green + audited.

## Flow
1. Branch a feature off `nightfall`: `feat/<name>`.
2. PR into **`nightfall`**. CI must be green (same gates as stable).
3. **Promote** `nightfall` → `main` when the accumulated changes are audited green
   (open a `nightfall` → `main` PR).
4. Keep `nightfall` current with `main` — merge `main` into `nightfall` after each stable fix.
5. Tag stable releases on `main`.

## Non-negotiables on BOTH tracks
- The graded loop evals (`crates/agents/tests/loop.rs`) stay green: fixed tool order ·
  checkpoint blocks the grounded pull · advice refusal.
- Tested paths are **deterministic, keyless, networkless** — no clock, no `tanh`/transcendentals
  in scored math, committed data blobs over live downloads.
- `cargo deny` licenses/bans/sources clean.

## Experimental-feature isolation rule
Anything that could destabilize stable (new heavy deps, a map UI, network calls) ships
**additively and behind a boundary** — a new module/crate, a Cargo feature flag off by default,
and/or gated on `time_known` — so promoting to `main` is a config flip, never a rewrite.
