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

## Install

**[Download the latest release →](https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3/releases)**
(Windows `.zip` · macOS `.dmg` · Linux `.tar.gz`)

One application. No account, no database, no Docker. It runs **offline out of the box** — the chart
math and the company data are compiled into the binary — and your chart and keys never leave your
machine. First-launch notes (WebView2 on Windows, Gatekeeper on macOS, WebKitGTK on Linux) are in
each release's notes and in the `README.txt` inside the download.

To get readings written by a model rather than by templates, paste your own API key (Anthropic or
OpenRouter — kept in your OS keychain, never shown back to you) or run a model locally.

Everything below is for **building from source**.

## Status

| Layer | State |
|---|---|
| **Data** — 5,271 US-market tickers, compiled into the binary | ✅ built · **4,507 chartable** (Polygon + SEC 8-A), 764 honestly date-unknown |
| **Ephemeris** — pluggable trait, 13 bodies | ✅ built · analytic (default, no data files) + ANISE backends, Chiron table, all JPL-validated |
| **Engine** — chart assembly + aspects | ✅ built · `compute_chart`, direction-agnostic `find_aspect` |
| **Agents** — Hamun-ana + Ungasaga loop + checkpoint | ✅ observe→decide→act, approval gate, grounded tool, evals; interpreter = template / local / live (OpenRouter → Anthropic) |
| **MCP + profile** — run the loop from any MCP host | ✅ `make_profile` · `chart` · `recommend` · `pull_grounded_signals` (checkpoint) |
| **UI** — Dioxus 0.6 desktop app | ✅ shipped · onboarding, weekly readings, checkpoint, Raw/Local/Live, layered grounding, in-app model panel |
| **Sidecar** — read-only HTTP API over Postgres | 🧪 dev/CI only · **the app does not use it** (see [Architecture](#architecture)) |

Stable **v1.2.0** on `main`; build-ahead **v1.4.0** on `nightfall`. Full history in
[CHANGELOG.md](CHANGELOG.md); release + two-track governance in [RELEASING.md](RELEASING.md).

## The two-agent design (the graded artifact)

Every reading is produced by **two visible agents**:

- **Hamun-ana** — the *measurer*. Computes exact positions/aspects; returns structured JSON only. Never interprets.
- **Ungasaga** — the *interpreter*. Turns those measures into a reading in three beats —
  **measured → meaning → reminder** — and refuses to give advice.

The separation *is* the product's integrity guarantee: measurement and meaning are distinct by architecture.

## Architecture

**Everything runs in one process.** No server, no database, no network required.

```
Dioxus desktop UI  (ziqpu-ui)                    ← the whole product; one binary
  └── agents        observe → decide → CHECKPOINT → act
        ├── Hamun-ana  measures   (temp 0, JSON only)   → engine ── over ── ephemeris
        │                                                   aspects,          VSOP87 + Meeus
        │                                                   dignities,        (no data files)
        │                                                   synastry
        ├── Ungasaga   interprets (template · local model · live model)
        ├── tickers    5,271 dated companies — compiled in via `include_str!`
        └── grounded   SEC EDGAR + Wikipedia — only after you approve at the checkpoint
```

The desktop app depends on `agents`, `model`, `geo`, `tickers` — and on no database and no HTTP
service of ours. A chart is arithmetic over data already inside the executable.

> **About `crates/sidecar`.** It is a Phase-0 artifact: an axum read-only HTTP API that serves chart
> math from a Postgres copy of the ticker table. **Nothing depends on it** — it has no `agents`
> dependency, so it cannot produce a Ziqpu reading, and the desktop app never contacts it. Its
> single query reads data that `crates/tickers` already compiles into the binary. It is kept as the
> seed of the future hosted web app and is exercised only by one CI job. **`docker compose` exists
> for that job, not for the app.**

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
| `crates/astro` | astrotopography — relocation charts (additive; soaking on `nightfall`) | 🔄 nightfall |
| `crates/sidecar` | axum read-only API (`/chart/:t`, `/synastry/:a/:b`, `/transits/:date`) | ✅ |
| `crates/geo` | offline geocoder over a committed GeoNames gazetteer | ✅ |
| `crates/tickers` | choice universes — Stocks · Airlines · Insurance | ✅ |
| `crates/agents` | observe→decide→act loop + checkpoint + grounded tool + template/local/live interpreters + layered grounding + portable profile + tool-calling loop + free-tier health + VIN resolver (N3) | ✅ |
| `crates/model` | local-model tier benchmark + `get`/`serve` (llama.cpp) + CUDA-first runtime resolution + quant-aware fit | ✅ |
| `crates/mcp` | MCP server: drive the loop from any host (Claude Desktop, IDEs) | ✅ |
| `crates/ui` | Dioxus 0.6 desktop app (`ziqpu-ui`) | ✅ |

## Quickstart

```bash
# The app. That's it — no database, no services, works offline.
cargo run -p ui
```

```bash
# Optional — which local model fits this machine (then `serve` it on :1234)
cargo run -p model -- benchmark
```

<details>
<summary>Optional: the sidecar + Postgres (dev/CI only — the app does not use them)</summary>

The sidecar is a standalone read-only HTTP API over a Postgres copy of the ticker table. You do
**not** need it to run, develop, or test Ziqpu — see [Architecture](#architecture). It is here for
one CI job and as the seed of the future hosted web app.

```bash
docker compose up -d --wait db           # contained Postgres, seeded with the ticker dataset
cargo run -p sidecar                     # analytic backend, no data files
curl localhost:8787/chart/AAPL           # 12–13 body natal chart
curl localhost:8787/synastry/AAPL/MSFT   # cross-aspects between two charts

# …or with the high-accuracy ANISE backend (adds Pluto):
bash scripts/fetch-ephemeris.sh          # downloads the DE440 kernel (~32 MB, gitignored)
cargo run -p sidecar --features anise
```

</details>

## Develop

```bash
cargo test --workspace --all-features                     # unit + smoke tests
cargo fmt --all -- --check                                # formatting gate
cargo clippy --workspace --all-features -- -D warnings
cargo deny check                                          # advisories + licenses + bans + sources
```

Copy `.env.example` to `.env` for local runs (never commit it).

## Data

- **Stocks** — `db/` (schema + generated seed). Provenance and enrichment in [db/README.md](db/README.md).
- **New domains** — partners collect "birth moment" datasets under [datasets/](datasets/) using a
  shared schema, so any dated entity plugs into the same synastry engine.

## Building in phases

Every change must be **all-green on GitHub Actions** —
`test`, `stability`, `smoke`, `security`, `desktop`, `integration`, `anise cross-check` (macOS/Windows/Linux) plus `DCO`.
`main` is the protected, all-green **stable** line; day-to-day work builds ahead on **`nightfall`** and is
promoted to `main` (via a merge commit) when green. Contributions PR into `nightfall`, owner-approved —
see [CONTRIBUTING.md](CONTRIBUTING.md) and [RELEASING.md](RELEASING.md).

## Team

Pursuit NYC Fellowship, Cycle 3 — Aisling Leiva-Davila (lead), Ahsan Abbasi
([@1abbasia](https://github.com/1abbasia)), Nathan Hutton
([@nathanhutton-design](https://github.com/nathanhutton-design)).

## License

Original source: **PolyForm Noncommercial License 1.0.0** (see [LICENSE](LICENSE) and [NOTICE](NOTICE)).
Noncommercial use is free; commercial rights are reserved by the author. Contributions require a DCO
sign-off — see [CONTRIBUTING.md](CONTRIBUTING.md).
