# Feature PRD — The Layered Grounding Pipeline

> Ziqpu L2 · Cycle 3 · **Week 6 agent addition.** This is the committed in-repo snapshot of
> the feature PRD — the version a reviewer can check the build against. The living product PRD is
> maintained separately; this file mirrors it for the Week 6 capability. Shipped in
> `v1.3.0-nightfall.1`. Evals: [eval-card.md](eval-card.md).

**Feature:** the layered grounding pipeline — *“local drafts, frontier synthesizes,”* with an
honesty ladder.

## Define — what we are adding

During the checkpoint pause, the **local** model drafts an “agent-effect” framing brief for the
grounded reading (from the measures only). On approval, the real signals are pulled and the
**frontier** model writes the comprehensive grounded read, guided by that brief. If the frontier is
unavailable, the read degrades down an **honesty ladder** — `Frontier → LocalGrounded →
LocalUnsourced → Template` — and is badged truthfully for how much reality backs it.

## Why — the real gap

- **Dead pause** — the checkpoint was idle time while the human decided.
- **Cold, costly frontier call** — a bare frontier call is more expensive and less tailored.
- **Dishonest degrade** — an offline / frontier-down read could silently degrade or, worse,
  hallucinate a fake source and price.

The layered pipeline makes the pause productive (local frames for free), makes the frontier call
cheaper and better-targeted, and makes every degraded read **honest about how much reality backs
it.**

## Tools

- **Drafter** — the local model (LM Studio / llama.cpp, keyless `:1234`).
- **Synthesizer** — the frontier (OpenRouter / Anthropic).
- **Grounded source** — SEC EDGAR + Wikipedia (keyless, read-only).
- **Code** — the `Interpreter` trait + `grounded_layered()` + the `GroundedRung` enum in
  `crates/agents`.

## Architecture

`observe → decide → checkpoint (local drafts the brief off-thread) → act (approve → pull signals →
frontier synthesizes with the brief → rung-badged read)`.

The draft runs `Send`-safe off the event loop and sees the **measures only**, so it structurally
cannot touch the gated pull early. One `GroundedRung` enum is the single source of truth for both the
badge and the disclaimer, so they can never disagree.

## Blast radius

- **The draft touches no external data** (measures only) → it cannot leak spend past the gate.
- **The frontier call is the one costed action** — still behind the human approval gate.
- **The unsourced strip** removes any fabricated source / price, so a degraded read can never emit a
  market claim.
- **Guardrails ride every rung** — the no-advice refusal + the standing REMINDER are on every output.

Net new blast radius versus the prior grounded read: **near-zero** — the same gated, costed frontier
call, plus a keyless local pre-draft.

## Success criteria / evals

Covered by the Ziqpu [Eval Card](eval-card.md) (golden: ranked fits + quiet-sky edge; adversarial:
refusal + injection + the hallucinated-grounded strip) and 52 green `crates/agents` tests.

## Entitlement

The editable brief and the frontier-drafter path are premium (dev-build toggle / `PremiumLock`); the
free tier gets the local-drafted brief read-only plus the full honesty ladder.
