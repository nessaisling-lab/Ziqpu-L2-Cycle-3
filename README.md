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
| **Data** — 5,271 US-market tickers, compiled into the binary | ✅ built · **4,507 chartable** — conception from Wikidata `P571` (CC0), birth from SEC EDGAR 424B4/424B1 (public domain); 764 honestly date-unknown |
| **Ephemeris** — pluggable trait, 12 bodies | ✅ built · **JPL DE440 (default, bundled + digest-pinned)**, analytic VSOP87 fallback, Chiron table, all JPL-validated |
| **Engine** — chart assembly + aspects | ✅ built · `compute_chart`, direction-agnostic `find_aspect` |
| **Agents** — Hamun-ana + Ungasaga loop + checkpoint | ✅ observe→decide→act, approval gate, grounded tool, evals; interpreter = template / local / live (OpenRouter → Anthropic) |
| **MCP + profile** — run the loop from any MCP host | ✅ `make_profile` · `chart` · `recommend` · `pull_grounded_signals` (checkpoint) |
| **UI** — Dioxus 0.6 desktop app | ✅ shipped · onboarding, weekly readings, checkpoint, Raw/Local/Live, layered grounding, in-app model panel |

Stable **v1.4.1** on `main`; build-ahead **v1.5.0** on `nightfall`. Full history in
[CHANGELOG.md](CHANGELOG.md); release + two-track governance in [RELEASING.md](RELEASING.md).

## The two-agent design (the graded artifact)

Every reading is produced by **two visible agents**:

- **Hamun-ana** — the *measurer*. Computes exact positions/aspects; returns structured JSON only. Never interprets.
- **Ungasaga** — the *interpreter*. Turns those measures into a reading in three beats —
  **measured → meaning → reminder** — and refuses to give advice.

The separation *is* the product's integrity guarantee: measurement and meaning are distinct by architecture.

## What the agent can actually do — the tools

The loop is **orchestrator-and-workers**: code narrows the roster to the workers that could possibly
apply, then the model decides which of those to call. Two layers, and they are graded differently.

**Measurement — the fixed sequence, never model-chosen.** Hamun-ana records the order; the arithmetic
is exact and cannot be influenced by a model.

| Tool | Returns |
|---|---|
| `get_chart(you)` | your natal positions |
| `get_chart(choice)` | the choice's natal positions |
| `get_synastry(you, choice)` | the cross-aspects between them |

**Grounding — dispatched by what the entity IS** (`classify_entity`), so impossible lookups are never
made. A car has no filings; a company has no VIN.

| Entity kind | Tools offered |
|---|---|
| `PublicCompany` (has a CIK) | `sec_filings` · `sec_financials` (XBRL) · `company_facts` (Wikidata) |
| `Vehicle` (valid VIN) | `vehicle_record` — NHTSA vPIC: make, model, year, assembly plant. **A VIN carries no build date**, so it never claims one |
| `ScannedItem` (GS1 barcode) | `scanned_code` — offline; the object's own record, and the day *this unit* was made if the code carries it · `origin_moments` |
| `Named` (a bare name) | `company_facts` · `drug_approval` (openFDA) · `origin_moments` (Wikidata lifecycle) — genuinely ambiguous, so the model chooses |

`origin_moments` is the **N3 origin resolver** on the roster: it turns a name into the *lifecycle*
of dates Wikidata holds for it — released, officially opened, entered service, founded — and marks
which are day-precise enough to chart. A year-precision value is reported as a year and named
unchartable, never rendered as the January 1st that Wikidata stores it as.

Plus `web_news_search` behind the deep-research path.

**Every grounding call is gated.** `propose_grounding` returns a consent prompt naming the exact
sources; nothing external is fetched until a human approves. Fetched text is fenced as data, and
items shaped like instructions or trading advice are withheld before they reach the model *and*
before they reach the screen.

**From an MCP host** (Claude Desktop, IDEs): `make_profile` · `chart` · `recommend` ·
`pull_grounded_signals`.

## Architecture

**Everything runs in one process.** No server, no database, no network required.

```
Dioxus desktop UI  (ziqpu-ui)                    ← the whole product; one binary
  └── agents        observe → decide → CHECKPOINT → act
        ├── Hamun-ana  measures   (temp 0, JSON only)   → engine ── over ── ephemeris
        │                                                   aspects,          JPL DE440
        │                                                   dignities,        (bundled, pinned;
        │                                                   synastry          VSOP87 fallback)
        ├── Ungasaga   interprets (template · local model · live model)
        ├── tickers    5,271 dated companies — compiled in via `include_str!`
        └── grounded   SEC EDGAR + Wikipedia — only after you approve at the checkpoint
```

The desktop app depends on `agents`, `model`, `geo`, `tickers` — and on no database and no HTTP
service of ours. A chart is arithmetic over data already inside the executable.

> **There used to be a `crates/sidecar` here**, a Phase-0 axum API serving chart math from a
> Postgres copy of the ticker table, plus a `db/` schema and seed and a `docker compose`. It was
> removed, and not merely because nothing depended on it.
>
> It had become a **second source of truth for a birth moment**. Its seed still held the pre-purge
> dates — AAPL as its 1980-12-12 listing — while the compiled table charts the moment that survived
> re-derivation, AAPL's 1976-04-01 founding. Same ticker, different chart, and a CI job asserting
> the one the app had stopped producing. The assertions it made worth keeping now live in
> `crates/tickers/tests/chart_contract.rs`, which needs no database and no HTTP server to make them.

The **`ephemeris` trait** is the seam that keeps the public tree free of copyleft:

- **`anise-backend`** (default, **bundled with every release**) — ANISE over the JPL DE440 kernel
  (`de440s.bsp`, ~32 MB, verified against a pinned SHA-256). DE440 is a U.S. Government work and
  freely redistributable, which is exactly why it can ship where the AGPL Swiss Ephemeris cannot.
- **`analytic`** (the **floor**, not the choice) — pure-Rust VSOP87 planets + Meeus Moon, no data
  files. It runs when the kernel is absent or fails verification, and **cannot compute Pluto at
  all** — so the app says which engine drew a chart rather than quietly handing back one fewer
  planet. See `crates/ephemeris/src/resolve.rs`.
- **Chiron** — a committed table of JPL Horizons longitudes, interpolated (works on both backends).
- **`swisseph`** — a private/commercial backend stub; never shipped in this repository.

Analytic and ANISE agree to **<1°** on the bodies they share (a CI cross-check enforces it) — the
disagreement that matters is Pluto, which only DE440 has. Swapping the floor for DE440 moved the
demo scores by up to 5 points; `cargo run -p agents --example scores` prints which engine produced
the numbers it shows.

## Workspace

| Crate | Purpose | State |
|---|---|---|
| `crates/ephemeris` | `Ephemeris` trait, runtime engine resolver (DE440 default, analytic floor), Chiron table, Asc/MC | ✅ |
| `crates/engine` | chart assembly (`compute_chart`) + `find_aspect` keystone | ✅ |
| `crates/astro` | astrotopography — relocation charts (additive; soaking on `nightfall`) | 🔄 nightfall |
| `crates/geo` | offline geocoder over a committed GeoNames gazetteer | ✅ |
| `crates/tickers` | choice universes — Stocks · Airlines · Insurance | ✅ |
| `crates/agents` | observe→decide→act loop + checkpoint + grounded tool + template/local/live interpreters + layered grounding + portable profile + tool-calling loop + free-tier health + VIN resolver + Wikidata origin resolver (N3) | ✅ |
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

## Develop

```bash
cargo test --workspace --all-features                     # unit + smoke tests
cargo fmt --all -- --check                                # formatting gate
cargo clippy --workspace --all-features -- -D warnings
cargo deny check                                          # advisories + licenses + bans + sources
```

Copy `.env.example` to `.env` for local runs (never commit it).

**To evaluate the agent rather than the code** — these are the instruments, and they print for a
human to grade rather than asserting:

```bash
cargo run -p agents --example eval_card       # the Eval Card; add ZIQPU_LIVE=1 for real sources + model
cargo run -p agents --example scores          # measured scores + which ephemeris produced them
cargo run -p agents --example origin_probe    # N3: a name -> its origin lifecycle -> a chart
cargo run -p agents --example model_compare   # one reading, several writer models, input held constant
cargo run -p agents --example key_doctor      # diagnose a stored key by SHAPE — never prints its value
```

`ZIQPU_TRACE=1` records every model turn to an in-memory ring buffer (shapes only; `full` for
verbatim). It never touches disk. Reading it is how the provider-substitution bug was found.

## Data

- **Stocks** — compiled into the binary by `crates/tickers`. Dates are derived by
  [scripts/derive-dates.py](scripts/derive-dates.py): conception from Wikidata `P571` (CC0), birth
  from SEC EDGAR 424B4/424B1 (public domain). Rows whose moment could not be established stay
  honestly date-unknown rather than acquiring a plausible one.
- **New domains** — partners collect "birth moment" datasets under [datasets/](datasets/) using a
  shared schema, so any dated entity plugs into the same synastry engine.

## Building in phases

Every change must be **all-green on GitHub Actions** —
`test`, `stability`, `smoke`, `security`, `desktop`, `anise cross-check` (macOS/Windows/Linux) plus `DCO`.
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
