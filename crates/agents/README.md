# `agents` — Ziqpu's two-vizier loop

The graded artifact: an **observe → decide → act** loop with a human-in-the-loop checkpoint,
built on the read-only engine below it. Design spec: PRD §16–§20.

```
observe (seeker + choices) ─▶ decide (Hamun-ana measures ─▶ Ungasaga fit read)
                                        │
                                        ▼
                            ▮ CHECKPOINT ▮  pause for a human
                                        │ approve
                                        ▼
                              act (grounded pull ─▶ briefing)      decline ─▶ stop at the fit read
```

- **Hamun-ana** ([`measure`](src/measure.rs), [`Session::measure`](src/orchestrator.rs)) — calls the
  chart tools in the fixed order `get_chart(you) → get_chart(choice) → get_synastry` and returns
  structured [`Measures`](src/types.rs). Never interprets.
- **Ungasaga** ([`interpret`](src/interpret.rs)) — turns measures into a reading in three beats,
  *measured → meaning → reminder*, on the four-band fit scale (Strongly Aligned / Aligned / Mixed /
  Misaligned). Never advises.
- **Checkpoint** — [`Session::pull_grounded`](src/orchestrator.rs) requires an
  [`ApprovalToken`](src/orchestrator.rs) that only [`Session::approve`] can mint.
- **Grounded tool** ([`grounded`](src/grounded.rs)) — the mock (default, CI) or the real, keyless
  SEC EDGAR pull (`ZIQPU_LIVE=1`).

## Run it

```bash
cargo test -p agents                 # the three graded evals (deterministic, no keys/network)
cargo run -p agents                  # the demo, mock grounded source
ZIQPU_LIVE=1 cargo run -p agents     # the demo, live SEC EDGAR pull (needs curl + network)
```

## Responsibility (blast radius · prompt injection · output accountability)

**Blast radius.** The loop has exactly one action that touches the outside world:
`pull_grounded_signals` (SEC EDGAR). It is external, rate-limited, and costs quota — it cannot be
un-spent. So it sits behind the checkpoint: `pull_grounded` returns `GateError::NotApproved` before
any external call unless it is handed an `ApprovalToken` minted for that same choice. The gate is
enforced by the type system (private token field, no public constructor), not by a prompt. Every
other tool is local, free, deterministic engine math.

**Prompt injection — data is not instructions.** A choice's name/notes and any fetched filing text
are treated as **data**, never as commands. Hamun-ana returns structured measures only; the
no-advice guardrail ([`is_advice_seeking`](src/orchestrator.rs)) is code, so a filing that "says" to
ignore it changes nothing. Advice-seeking questions ("should I buy?") are refused regardless of input.

**Output accountability.** Every reading is stamped *"measured, not fate — not financial advice,"*
shows the measures beneath the meaning, and traces each claim to a computed position. The builder —
not the model, not the seeker — owns the output; the guardrail and the checkpoint are how that is enforced.

## Status

Phase 1a + 1b polish. Both agents default to deterministic (CI-safe) with real models drop-in via
traits, opt-in by env:

- **Hamun-ana** — [`Measurer`](src/measure.rs) seam: `DeterministicMeasurer` (default) or
  [`LocalMeasurer`](src/measure_llm.rs) = a local model (`ZIQPU_LOCAL_LLM=1`). Speaks both
  **OpenAI-compatible** (LM Studio / Jan / llama.cpp — default, `gemma-4-e4b-it`) and **Ollama**
  (`ZIQPU_LLM_PROVIDER=ollama`, `qwen2.5:3b-instruct`); override with `ZIQPU_LLM_MODEL` /
  `ZIQPU_LLM_URL`. The model only *sequences* the tools — accepted only if it names the exact order
  `get_chart → get_chart → get_synastry`, else deterministic fallback — so it can never corrupt a
  number. Verified live end-to-end against LM Studio (`gemma-4-e4b-it`).
- **Ungasaga** — [`Interpreter`](src/interpret.rs) seam: `TemplateInterpreter` (default) or
  [`AnthropicInterpreter`](src/interpret_llm.rs) = **Claude** (`ANTHROPIC_API_KEY`).
- **Grounded tool** — `MockGroundedSource` (CI) or `EdgarSource` (`ZIQPU_LIVE=1`): real SEC EDGAR
  filings + industry **and** a keyless Wikipedia blurb, degrading cleanly if a source is blocked.
- **Portable profile** ([`profile`](src/profile.rs)) — export/import birth data so the agent travels.

Synastry scoring is a thin v1 scorer ([`score`](src/score.rs)); Phase 3 reuses the engine's
dignity/weight helpers (PRD §7.2).
