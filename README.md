# Ziqpu — L2 Cycle 3

> You have a birth chart. So does everything you might choose. **Ziqpu** shows you how
> they aspect — measured from real ephemeris data, read back to you in plain language,
> and never dressed up as fate.

Ziqpu reframes **synastry** (chart-to-chart comparison) as a decision lens: it scores the
relationship between *your* natal chart and the *natal chart of a choice you're weighing*.
The v1 domain is **stocks**, dated by their IPO moment — but the engine is domain-agnostic:
anything with a **date of origin** (a founding, a launch, a policy's effective date) can be charted.

**Ziqpu is for reflection and entertainment.** Astrological interpretations are traditional and
symbolic — not statements of fact, not predictions, not guarantees. Nothing here is financial,
medical, legal, or psychological advice.

> 📄 Product spec: **[docs/PRD.md](docs/PRD.md)** (v2.0). Build plan is phase-gated (see below).

## Status

| Layer | State |
|---|---|
| **Data** — contained Postgres, 5,271 US-market tickers | ✅ built · **4,507 chartable** (Polygon + SEC 8-A), 764 honestly date-unknown |
| **Ephemeris** — pluggable trait, 13 bodies | ✅ built · analytic + ANISE backends, Chiron table, all JPL-validated |
| **Engine** — chart assembly + aspects | ✅ built · `compute_chart`, direction-agnostic `find_aspect` |
| **Sidecar** — read-only API | ✅ built · `/chart` `/synastry` `/transits` over real data |
| **Agents** — Hamun-ana + Ungasaga (two-agent flow) | ⏳ Phase 1–2 (next) |
| **UI** — Dioxus + Tauri | ⏳ Phase 4 |

## The two-agent design (the graded artifact — Phase 1–2)

Every reading is produced by **two visible agents**:

- **Hamun-ana** — the *measurer*. Computes exact positions/aspects; returns structured JSON only. Never interprets.
- **Ungasaga** — the *interpreter*. Turns those measures into a reading in three beats —
  **measured → meaning → reminder** — and refuses to give advice.

The separation *is* the product's integrity guarantee: measurement and meaning are distinct by architecture.

## Architecture

```
Postgres (company_metadata) ─▶ axum sidecar (read-only) ─▶ rig-core: Hamun-ana + Ungasaga ─▶ Dioxus/Tauri UI
                                        │                              (Phase 1–2)            (Phase 4)
                              engine (interpretation)  ── over ──  ephemeris (pluggable trait)
```

The **`ephemeris` trait** is the seam that keeps the public tree free of copyleft:

- **`analytic`** (default) — pure-Rust VSOP87 planets + Meeus Moon, **no data files**.
- **`anise`** (opt-in `--features anise`) — ANISE + JPL DE440 kernel; adds **Pluto**, higher accuracy.
- **Chiron** — a committed table of JPL Horizons longitudes, interpolated (works on both backends).
- **`swisseph`** — a private/commercial backend stub; never shipped in this repository.

Analytic and ANISE agree to **<1°** (a CI cross-check enforces it).

## Workspace

| Crate | Purpose | State |
|---|---|---|
| `crates/ephemeris` | `Ephemeris` trait, analytic + ANISE backends, Chiron table, Asc/MC | ✅ |
| `crates/engine` | chart assembly (`compute_chart`) + `find_aspect` keystone | ✅ |
| `crates/sidecar` | axum read-only API (`/chart/:t`, `/synastry/:a/:b`, `/transits/:date`) | ✅ |
| `crates/agents` | rig-core two-agent orchestration | ⏳ Phase 1–2 |
| `crates/ui` | Dioxus 0.6 + Tauri 2 | ⏳ Phase 4 |

## Quickstart

```bash
# 1. contained Postgres, seeded with the 5,271-ticker dataset
docker compose up -d --wait db

# 2. run the read-only sidecar (analytic backend, no data files)
cargo run -p sidecar
#    …or with the high-accuracy ANISE backend (adds Pluto):
bash scripts/fetch-ephemeris.sh          # downloads the DE440 kernel (~32 MB, gitignored)
cargo run -p sidecar --features anise

# 3. ask it things
curl localhost:8787/chart/AAPL           # 12–13 body natal chart
curl localhost:8787/synastry/AAPL/MSFT   # cross-aspects between two charts
```

## Develop

```bash
cargo test --workspace --all-features                     # unit + smoke tests
cargo fmt --all -- --check                                # formatting gate
cargo clippy --workspace --all-features -- -D warnings
cargo deny check licenses bans sources                    # keeps the tree permissive-only
```

Copy `.env.example` to `.env` for local runs (never commit it).

## Data

- **Stocks** — `db/` (schema + generated seed). Provenance and enrichment in [db/README.md](db/README.md).
- **New domains** — partners collect "birth moment" datasets under [datasets/](datasets/) using a
  shared schema, so any dated entity plugs into the same synastry engine.

## Building in phases

Development is a **phase-gated tasklist**. Each phase must be **all-green on GitHub Actions** —
`test`, `stability`, `smoke`, `security`, `integration`, `anise cross-check` across macOS/Windows/Linux —
before the next begins. `main` is protected: contributions land via pull request, owner-approved.

## Team

Pursuit NYC Fellowship, Cycle 3 — Aisling Leiva-Davila (lead), Ahsan Abbasi
([@1abbasia](https://github.com/1abbasia)), Nathan Hutton
([@nathanhutton-design](https://github.com/nathanhutton-design)).

## License

Original source: **PolyForm Noncommercial License 1.0.0** (see [LICENSE](LICENSE) and [NOTICE](NOTICE)).
Noncommercial use is free; commercial rights are reserved by the author. Contributions require a DCO
sign-off — see [CONTRIBUTING.md](CONTRIBUTING.md).
