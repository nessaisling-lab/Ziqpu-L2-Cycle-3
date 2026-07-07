# Ziqpu — L2 Cycle 3

> You have a birth chart. So does everything you might choose. **Ziqpu** shows you how
> they aspect — measured from real ephemeris data, read back to you in plain language,
> and never dressed up as fate.

Ziqpu reframes **synastry** (chart-to-chart comparison) as a decision lens: it scores the
relationship between *your* natal chart and the *natal chart of a choice you're weighing*.
v1 covers one domain — **stocks**, dated by their IPO moment.

Every reading is produced by **two visible agents**:

- **Hamun-ana** — the *measurer*. Computes exact positions and aspects; returns structured
  JSON only. Never interprets.
- **Ungasaga** — the *interpreter*. Turns those measures into a reading in three beats —
  **measured → meaning → reminder** — and refuses to give advice.

**Ziqpu is for reflection and entertainment.** Astrological interpretations are traditional
and symbolic — not statements of fact, not predictions, not guarantees. Nothing here is
financial, medical, legal, or psychological advice.

## Architecture

```
private Nisaba Postgres ─▶ axum sidecar (read-only) ─▶ rig-core: Hamun-ana + Ungasaga ─▶ Dioxus/Tauri UI
                                    │
                          engine (interpretation)  ── over ──  ephemeris (pluggable trait)
```

The **`ephemeris` trait** is the seam: the public/default backend is pure-Rust **ANISE +
JPL DE440** (permissive); the high-accuracy **Swiss Ephemeris** backend lives behind a
private `swisseph` feature and is not part of this repository.

## Workspace

| Crate | Purpose |
|-------|---------|
| `crates/ephemeris` | `Ephemeris` trait + backends (ANISE default) |
| `crates/engine` | interpretation IP: aspects, dignities, patterns, synastry scoring |
| `crates/sidecar` | axum read-only API (`/chart`, `/synastry`, `/transits`) — *Phase 0* |
| `crates/agents` | rig-core two-agent orchestration — *Phases 1–2* |
| `crates/ui` | Dioxus 0.6 + Tauri 2 — *Phase 4* |

## Develop

```bash
cargo test --workspace --all-features     # unit + smoke tests
cargo fmt --all -- --check                # formatting gate
cargo clippy --workspace --all-features -- -D warnings
cargo deny check licenses bans sources    # keeps the tree permissive-only
```

Copy `.env.example` to `.env` for local runs (never commit it).

## Building in phases

Development is a **phase-gated tasklist**. Each phase must be **all-green on GitHub Actions**
— `test`, `stability`, `smoke`, `security` across macOS/Windows/Linux — before the next
begins. See the build plan for the phase breakdown and exit gates.

## Team

Pursuit NYC Fellowship, Cycle 3 — Aisling Leiva-Davila (lead), Ahsan Abbasi
([@1abbasia](https://github.com/1abbasia)), Nathan Hutton
([@nathanhutton-design](https://github.com/nathanhutton-design)).

## License

Original source: **PolyForm Noncommercial License 1.0.0** (see [LICENSE](LICENSE) and
[NOTICE](NOTICE)). Noncommercial use is free; commercial rights are reserved by the author.
Contributions require a DCO sign-off — see [CONTRIBUTING.md](CONTRIBUTING.md).
