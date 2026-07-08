# Ziqpu — Product Requirements Document

**Version 2.0** · 2026-07-07 · Pursuit NYC Fellowship, Cycle 3
Authors: Aisling Leiva-Davila (lead), Ahsan Abbasi, Nathan Hutton

| Version | Date | Notes |
|---|---|---|
| 1.0 | pre-build | Original spec (vision, agent architecture, roadmap). |
| **2.0** | 2026-07-07 | **Updated to reflect the implemented foundation** — pluggable ephemeris (13 bodies, dual backends, JPL-validated), contained Postgres seeded with 5,271 real tickers (4,507 chartable), the read-only sidecar (`/chart`, `/synastry`, `/transits`), phase-gated CI, and the domain-agnostic dataset model. Scope and guardrails unchanged from v1. |

---

## 1. Summary

> You have a birth chart. So does everything you might choose. Ziqpu shows you how they aspect —
> measured from real ephemeris data, read back to you in plain language, and never dressed up as fate.

Ziqpu reframes **synastry** (chart-to-chart comparison) as a **decision lens**. Instead of scoring the
compatibility between two *people*, it scores the compatibility between *you* (your natal chart) and a
*choice you're weighing* (that choice's natal chart). Every choosable thing has a **birth moment** — a
stock has an IPO date/time at an exchange; and more broadly, **anything with a date of origin** (a
company's founding, a school's charter, an insurance policy's effective date, a product's launch) can be
charted. v1 ships the **stocks** domain.

## 2. The problem

- **Horoscope apps** give personal readings but can't reason about a *specific decision* — they have no second chart.
- **Decision tools** (screeners, comparators) describe the *choice* but treat every user as interchangeable.
- **Ziqpu is the only product that holds both charts at once** and scores the relationship between them.

## 3. Users

- **Primary — "curious deciders":** people who take astrology seriously as a lens (not literal prophecy),
  are facing real choices, and want something richer than a generic daily horoscope.
- **Secondary — finance-astrology enthusiasts:** aware of the Gann/Adams/Morgan lineage; want a credible modern tool.
- **Not v1:** people seeking financial advice; skeptics wanting proof astrology predicts markets; professional astrologers wanting a full ephemeris workbench.

## 4. Product principles (load-bearing)

1. **Measured, not believed.** Ziqpu reports how a choice *aspects* you. It never predicts an outcome or gives advice.
2. **The vizier carries the tablet; it does not invent it.** Every interpretation traces to a real computed
   position. Measurement and meaning are distinct **by architecture** (the two-agent split), not just by disclaimer.
3. **One reading, legible.** A reading is a single clear compatibility story, not a wall of ephemeris data.
4. **Honest about the tradition.** Astro-finance heritage is cited as lineage, never as evidence of predictive power.
5. **One engine, many faces.** The synastry engine is domain-agnostic; each new domain is a dataset + a small adapter.

## 5. MVP scope (v1)

**In scope:** personal chart onboarding (birth date/time/place → natal chart); a daily personal reading;
the **synastry flagship on stocks** in all three output modes; the **two visible agents** (the graded
showcase); a **legible UI** (Dioxus + Tauri, desktop/web).

**Out of scope (v1):** other domains beyond stocks (roadmap); accounts, payments, app-store submission;
**any financial-advice surface** (buy/sell signals, price targets) — *permanently* out for guardrail integrity.

## 6. The three reading modes

All backed by the same synastry computation; they differ only in presentation.

| Mode | Question | Output |
|---|---|---|
| **Compatibility report** | "How do I fit with AAPL?" | 0–100 score + aspect breakdown + plain-language reading |
| **Ranked list** | "Which of these fit me best?" | choices sorted by score, one-line "why" each |
| **Verdict** | "Should I, yes or no-ish?" | Strongly Aligned / Aligned / Mixed / Misaligned + driving aspects + honesty reminder |

## 7. The two-agent contract (the integrity guarantee)

- **Hamun-ana (measurer):** computes exact chart data via the sidecar; returns structured measures only
  (positions, aspects, scores, patterns). No prose, no interpretation. Local model, temperature 0.
- **Ungasaga (interpreter):** takes those measures and writes the reading in three beats —
  **measured → meaning → reminder**. Never computes a chart itself; never invents a number; holds the
  no-advice guardrail. Hosted model (Claude), temperature ~0.7, prompt-cached.

Neither does the other's job. This separation is the product's integrity guarantee and the demo's centerpiece.

## 8. Guardrails (product requirements, not disclaimers)

1. **User-facing disclaimer** at onboarding and reachable from every reading (reflection/entertainment; not fact/prediction/advice).
2. **No financial advice, ever** — no buy/sell/hold signals or price expectations.
3. **Measurement vs meaning always visible** — the "backstage strip" shows Hamun-ana's raw measures beneath Ungasaga's reading.
4. **No fabricated data** — unknown birth time → flagged (houses approximate), never invented; missing entity data → "not enough data."
5. **Heritage is lineage, not proof.**

## 9. Architecture

Rust end-to-end for type-safe, checkable tool-calls:

```
Postgres (company_metadata) ─▶ axum sidecar (read-only) ─▶ rig-core: Hamun-ana + Ungasaga ─▶ Dioxus/Tauri UI
                                        │
                              engine (interpretation)  ── over ──  ephemeris (pluggable trait)
```

- **`ephemeris`** — a pluggable trait, the seam that keeps the public tree copyleft-free:
  - `analytic` (default): pure-Rust VSOP87 planets + Meeus Moon, **no data files**.
  - `anise` (opt-in): ANISE + JPL DE440 kernel; adds **Pluto**, higher accuracy.
  - **Chiron:** a committed table of JPL Horizons longitudes, interpolated (both backends).
  - `swisseph`: a private/commercial backend, never in this repo.
  - The two backends agree to **<1°** (enforced by a CI cross-check).
- **`engine`** — the interpretation IP (PolyForm-licensed): `compute_chart` (13 bodies + Asc/MC),
  the direction-agnostic `find_aspect` keystone, and the synastry scoring to come.
- **`sidecar`** — read-only axum API: `/chart/:ticker`, `/synastry/:a/:b`, `/transits/:date`. Never mutates the DB.
- **`agents`** (Phase 1–2) — rig-core orchestration of the two agents.
- **`ui`** (Phase 4) — Dioxus 0.6 + Tauri 2 (web + desktop from one Rust codebase).

## 10. Data

- **Stocks:** a contained, isolated Postgres seeded with **5,271 real US-market tickers** (NYSE/NASDAQ/
  NYSE American/CBOE). Provenance: Polygon `list_date` + **SEC EDGAR Form 8-A** listing dates (route-3
  enrichment). **4,507 are fully chartable**; 764 remain honestly date-unknown (flagged, never faked).
  A plausibility gate rejects impossible dates.
- **New domains:** partners curate "birth moment" datasets under `datasets/` (aviation, schools, insurance,
  or anything dated) using one shared schema (`id, name, birth_date, birth_time, location, tz, lat, lon,
  data_source, notes`), so any dated entity plugs into the same engine.

## 11. Current status (as of v2.0)

**Built and CI-gated (green on macOS/Windows/Linux):**
- Contained Postgres + 5,271-ticker seed; reproducible.
- Ephemeris: 13 bodies (Sun→Pluto, lunar nodes, Chiron) + Ascendant/MC, on two independent backends,
  each JPL-validated (planets <1°; Chiron <0.1° vs Horizons).
- Engine chart assembly + `find_aspect`.
- Read-only sidecar returning real charts, synastry cross-aspects, and transits over the live data.
- Security/quality gates: `cargo-deny` (permissive-only tree), `cargo-audit`, `gitleaks`, DCO, integration test.

**Next:** Phase 1 (Hamun-ana measurer) → Phase 2 (Ungasaga interpreter) → Phase 3 (three synastry modes) →
Phase 4 (UI + backstage strip) → Phase 5 (guardrail hardening, brand pass, demo).

## 12. Roadmap beyond v1

The synastry engine is domain-agnostic — each future domain is one dataset + one adapter.

| Domain | Birth moment | Owner |
|---|---|---|
| **Stocks** | IPO date/time + exchange | v1 (built) |
| **Aviation** | airline founding / first flight | Nathan |
| **Schools** | founding / charter date | Ahsan |
| **Insurance** | carrier incorporation / policy effective date | Ahsan |
| **Anything dated** | manufacturing date, launch date, charter… | community |

## 13. Success metrics (demo)

1. **The two-agent flow is legible** — a viewer sees Hamun-ana measure and Ungasaga interpret as distinct steps. (Primary.)
2. **A stock reading is produced end-to-end** in all three modes, from a real user chart vs a real IPO chart.
3. **The guardrail holds under pressure** — "Should I buy this?" yields reflection + refusal, not a signal. (Release blocker.)
4. **On-brand** — the "Oracle" visual direction and "measured, not believed" ethos.

## 14. License & commercial path

Original source under **PolyForm Noncommercial 1.0.0** (public/readable; commercial rights reserved to the
author). The dependency tree is kept **permissive-only** (`cargo-deny` enforced) so a future closed-source
commercial edition stays clean; the high-accuracy Swiss Ephemeris backend can be licensed and swapped in
behind the existing trait without touching product code.

---

*Ziqpu is a reflection tool. It does not predict outcomes and is not financial, medical, legal, or
psychological advice.*
