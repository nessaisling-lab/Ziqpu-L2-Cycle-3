# Architecture pattern — orchestrator-workers

**Team:** Ziqpu (Pursuit L2, Cycle 3) · **Week 6 deliverable:** pattern summary + team decision

> **Feature PRD:** [Ziqpu_Feature_PRD_EntityKind.docx](Ziqpu_Feature_PRD_EntityKind.docx)
> ([PDF](Ziqpu_Feature_PRD_EntityKind.pdf)) — problem, users, requirements, success metrics, and the
> agent sections (tools, blast radius, system prompt, evals) for the change described in §5.
>
> **Visual teachback deck:** <https://claude.ai/code/artifact/3cc0a6d8-3381-46a9-9781-85cb989433d3>
> — the same argument laid out for the table, with the latency measurement drawn as a range plot.
> *(Private to the author's Claude workspace unless explicitly shared; this document is the canonical
> copy and stands alone.)*

---

## 1. The pattern we are

**Orchestrator-workers.** A lead component decomposes a request into subtasks, dispatches
specialized workers, and merges their results — where *the number and kind of subtasks are decided at
runtime*, not written into the code path in advance.

We didn't choose this off a menu. It's the shape the product already had, and we can point at the
code for every claim:

| Layer | Where | Role |
|---|---|---|
| Orchestrator | `crates/agents/src/orchestrator.rs` — `Session` | Sequences the loop, owns the approval gate |
| Worker (agent) | Hamun-ana — measurer, temp 0, JSON only | Computes positions/aspects. Never interprets |
| Worker (agent) | Ungasaga — interpreter, temp 0.7 | Turns measures into a reading. Never measures |
| Workers (tools) | `EdgarSource`, `SecFactsSource`, `WikidataSource`, `WebNewsTool` | Each fetches one real external record |
| Orchestrator (model-driven) | `crates/agents/src/research.rs` — `run_tool_loop` | The *model* decides which source workers to call |

The loop is `observe → decide → CHECKPOINT → act`, and the costed step (`pull_grounded`) is
unreachable without an `ApprovalToken` minted by a human.

### The nuance that makes this interesting: we have two orchestrators, and only one is a model

- **The Session is a *code* orchestrator.** Deliberately. The measure sequence is fixed and
  CI-asserted (`expected_sequence` → `get_chart(you)` → `get_chart(choice)` → `get_synastry`), and
  the human checkpoint sits between decide and act. Putting an LLM in that seat would buy nothing
  and cost everything: nondeterministic tool order, an unauditable plan, and a model sitting between
  the user and the safety gate.
- **The research loop is an *LLM* orchestrator.** Here the plan genuinely varies with the input —
  which public records exist depends on what the entity *is* — so a model choosing the workers earns
  its complexity.

**Takeaway we'd defend:** "orchestrator-workers" is not a synonym for "an LLM plans it." The
orchestrator should be a model *only when the plan can't be known in advance*.

---

## 2. Concrete use case — outside Ziqpu

**A dependency-CVE response agent.**

A CVE lands for a widely used library. Someone has to answer, per service: are we actually
vulnerable, and what's the fix?

The orchestrator reads the advisory, searches the monorepo for the vulnerable import path, and finds
that of ~40 services, some subset actually pulls it in. For **each affected service** it spawns a
worker to: confirm the vulnerable call path is genuinely reachable (not just a transitive dep that's
never invoked), draft the version bump, repair whatever the new major version breaks, and run *that
service's* test command.

Why this needs an orchestrator, specifically:

- **The subtask list is a function of the input.** You don't know whether it's 2 services or 19 until
  the orchestrator looks. No fixed pipeline can express "one worker per thing I'm about to discover."
- **The subtasks aren't uniform.** A Go service and a Node service need different bump mechanics,
  different breakage repair, different test invocations. That's not one prompt applied N times.
- **The work is genuinely separable.** Each service is its own repo path, its own test suite. Workers
  don't need to see each other.

A second, non-engineering one with the same shape: **contract review.** The orchestrator reads the
agreement, identifies which clause families are actually present (indemnity, IP assignment,
non-compete, data processing…), and spawns a specialist per clause type *found*. A contract with no
data-processing clause shouldn't run the DPA reviewer at all.

---

## 3. When we would NOT use it

Five cases, roughly in order of how often people get them wrong:

1. **The subtask list is fixed.** Invoice processing: extract → validate → post to ledger. Always
   three, always that order. An orchestrator adds a planning round-trip, token cost, and run-to-run
   variance to decide something you already know. → **Prompt chain.** This is exactly why Ziqpu's
   measure step is *not* orchestrated.

2. **The work is decomposable but the decomposition is static.** "Summarize these 12 documents" —
   you know it's 12. You don't need a model to plan it; you need to fan out. → **Parallelization
   (sectioning).** We got this wrong-ish ourselves and fixed it — see §4.

3. **The workers need to see each other's work.** Orchestrator-workers assumes subtasks are largely
   independent. If worker B's correctness depends on what worker A discovered mid-flight, you get
   either duplicated effort or direct conflicts — the classic version being parallel agents editing
   the same file.

4. **The plan is safety-critical and must be auditable.** A runtime-generated plan varies between
   runs. Anywhere the sequence is the safety property — moving money, a medical pathway, our own
   approval gate — you want it fixed in code, testable, and reviewable by a human who isn't there at
   runtime.

5. **Latency or cost is the binding constraint.** The orchestrator burns at least one round-trip
   before any real work starts. For a sub-second interactive feature, the planning overhead can
   exceed the work being planned.

---

## 4. Team decision — what we added in Week 6, and what we deliberately didn't

### We did not add a second agent

We already have two (Hamun-ana + Ungasaga), and the separation *is* the product's integrity
guarantee: measurement and meaning are distinct by architecture, not by prompt discipline. Adding a
third agent that re-did either job would be complexity for its own sake.

### We made the existing workers actually concurrent

The grounded pull asks three independent sources for the same choice. It had the *shape* of
parallelization while doing none of it — a plain `for` loop, three blocking network calls in series,
with the user waiting at the checkpoint.

`CompositeSource::fetch` now runs its workers on scoped threads.

**Measured against the real sources (Manhattan Associates, 4 runs, byte-identical readings):**

| Run | Sequential | Parallel | Speedup |
|---|---:|---:|---:|
| 1 | 1976 ms | 617 ms | 3.20× |
| 2 | 2326 ms | 667 ms | 3.49× |
| 3 | 1255 ms | 627 ms | 2.00× |
| 4 | 1302 ms | 635 ms | 2.05× |
| **Range** | **1255–2326 ms** | **617–667 ms** | **2.0–3.5×** |

```bash
cargo test -p agents grounded -- --ignored --nocapture bench_parallel
```

**The headline isn't the average — it's the spread.** Parallel varies by ~50 ms; sequential by
~1000 ms. Concurrent time is bounded by the *slowest single source*; sequential time is the *sum*, so
it accumulates every source's variance. The change made the wait shorter **and much more
predictable**, and predictability is what the person staring at the checkpoint actually experiences.

### Two design decisions inside that change

- **Both modes share one merge, in declared source order.** Going concurrent changes only *when* the
  calls happen, never *what* the reading says. Merging by completion order would let a fast Wikidata
  jump ahead of a slow EDGAR and quietly reshuffle a user's grounded lines between two runs of the
  same choice. For a product whose claim is *measured, and you can check it*, a reading that
  reorders itself according to network weather is a defect, not a detail.
- **A panicking worker degrades; it doesn't take down the reading.** A worker that unwinds is treated
  exactly like one that found nothing: contributes no items, isn't named as a source — the same
  honest degrade a failed HTTP call already gets.

It's user-controllable: Settings → *"Fetch grounding sources at the same time"* (on by default). The
off position exists so the difference can be demonstrated, not because anyone would want it.

### What this cost us in complexity

Real but bounded: the source trait objects needed `Send + Sync`, and the merge had to be lifted out
so both execution modes provably share it. Four tests hold the line — that the calls actually
overlap, that both modes produce identical output, that declared order survives concurrency
(proven with deliberately inverted delays), and that a panicking worker degrades gracefully.

---

## 5. Week 6 plan — what we're adding next

Concurrent workers (§4) is **shipped and measured**. That's the baseline, not the plan. The honest
gap is that **our orchestrator doesn't yet do the one thing that defines the pattern: choose its
worker set from the input.**

Today `research.rs` advertises the *same three stock-shaped tools* to the model for every entity —
SEC filings, SEC financials, Wikidata — regardless of what the entity actually is. A model picking
among a fixed roster is closer to routing than orchestration. And `DecodeVinTool` (built, tested,
exported) is wired into *nothing*: a worker with no orchestrator to dispatch it.

**What we will add, specifically:**

1. **An entity-kind resolution step.** Before dispatch, the orchestrator establishes what the thing
   *is* — public company / vehicle / drug or device / product — instead of assuming "US-listed
   ticker".
2. **A worker roster selected from that kind.** A vehicle gets `DecodeVinTool` + Wikidata; a drug
   gets openFDA; a public company gets the SEC pair. Impossible lookups are never dispatched, and
   possible ones stop being invisible.
3. **New workers to make the roster meaningful** — openFDA (day-precise, public domain) and
   Wikidata `P577` product launches, per the vetted source list.
4. **A live proof.** The tool loop has only ever run against mocks and unit tests. We will serve a
   real `--jinja` tool-calling model and verify the orchestrator dispatches *different* worker sets
   for different entity kinds.

**Success criterion (falsifiable):** ground an entity that today's fixed fan **cannot ground at
all** — a car — with real sourced signals and no fabricated date. Today that request returns
nothing, because every worker in the roster speaks SEC.

**Why this is the right add rather than more speed:** parallelism made an existing capability
faster. This makes the pattern *true* — the subtask list finally becomes a function of the input,
which is the entire justification for choosing orchestrator-workers in §3. It is also the direct
prerequisite for the product's next milestone ("chart anything"), so the architecture exercise and
the roadmap are the same work.

**Known risk + the mitigation already in place:** a model can select a wrong or empty roster. The
deterministic multi-source composite remains the fallback — if the loop returns nothing, grounding
degrades to the fixed fan rather than to an invented answer.

---

## 6. Summary

| Question | Our answer |
|---|---|
| Pattern | Orchestrator-workers |
| Already in the build? | Yes — the Session (code orchestrator) and the research loop (LLM orchestrator) |
| Shipped so far | Made the source workers concurrent |
| Evidence | 1255–2326 ms → 617–667 ms; identical readings; 4 tests |
| Week 6 addition | Entity-kind worker selection — the orchestrator picks *which* workers exist per entity |
| Proof it worked | Ground a car (today: impossible — every worker speaks SEC) with real sourced signals |
| Second agent? | No — we have two, and a third would duplicate an existing role |
| Biggest lesson | An orchestrator should be a model only when the plan can't be known in advance. When it can, use a chain — and when it's merely wide, use parallelism |
