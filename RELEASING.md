# Releasing — `stable` & `nightfall`

Ziqpu ships on **two tracks**.

## `main` — **stable**
The audited, all-green release line. Every commit is 3-OS CI-green across every gate
(test · fmt · clippy · smoke · security [deny / audit / gitleaks] · desktop · integration · DCO) and
`cargo deny check` (advisories / licenses / bans / sources) is clean (protects the PolyForm commercial path).
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
   (open a `nightfall` → `main` PR, **merged with a merge commit — not squash**; see Merge policy).
4. Keep `nightfall` current with `main` — **merge** `main` into `nightfall` after each stable fix.
5. Tag stable releases on `main`.

## Merge policy
- **feature → `nightfall`** — squash is fine (a single clean feature commit).
- **`nightfall` ↔ `main`** — use **merge commits, not squash**. Squash-merging a promotion rewrites the
  SHAs, so `nightfall` diverges from `main` and the next `nightfall → main` PR shows up conflicted
  (GitHub can't compute a test-merge and required checks stall). Merge commits keep the two long-lived
  branches reconcilable.
- **Protection** — `main` is protected (PR + owner approval). `nightfall` is the owner's integration
  branch: the owner pushes build-ahead work directly; contributors open PRs into it (CI-gated).

## Version numbering & nightfall pre-releases
Versions are **major.feature-phase.fix** — `v1.3.2` = major `1`, feature-phase `3`, fix `2`.
- **Stable** tags on `main` publish as GitHub **"Latest"**.
- **Nightfall** pre-releases tag **plain semver** (`vX.Y.Z`, *not* `-nightfall.N`). `release.yml`
  classifies any tag whose commit is **not on `main`** as a pre-release
  (`--prerelease --latest=false`), so a nightfall build ships Win/mac/Linux installers **without ever
  overtaking the stable "Latest".**
- Tag + push: `git tag -a vX.Y.Z -m "Ziqpu Nightfall vX.Y.Z" && git push origin vX.Y.Z` → the release
  workflow runs guard → gauntlet → bundle → publish automatically (triggerable from the GitHub mobile app).
- The macOS `.dmg` is **unsigned** (Gatekeeper: right-click → Open).

## Non-negotiables on BOTH tracks
- The graded loop evals (`crates/agents/tests/loop.rs`) stay green: fixed tool order ·
  checkpoint blocks the grounded pull · advice refusal.
- Tested paths are **deterministic, keyless, networkless** — no clock, no `tanh`/transcendentals
  in scored math, committed data blobs over live downloads.
- `cargo deny check` clean (advisories · licenses · bans · sources).

## Experimental-feature isolation rule
Anything that could destabilize stable (new heavy deps, a map UI, network calls) ships
**additively and behind a boundary** — a new module/crate, a Cargo feature flag off by default,
and/or gated on `time_known` — so promoting to `main` is a config flip, never a rewrite.
