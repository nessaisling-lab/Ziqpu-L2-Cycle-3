//! Shared app state — the session handle, the phase machine, and the reactive signals every
//! component reads and writes through the Dioxus context.
//!
//! `Session` is generic, `!Send`, and `!Clone`, so it lives in an `Rc<RefCell<_>>` (never a Signal
//! or an `Arc`). The signals hold only the plain, cloneable *outputs* of the loop.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{
    draft_grounding_prompt, grounded_layered, ApprovalRequest, BirthMoment, Briefing, Choice,
    CompositeSource, EngineChartSource, Fit, GroundedRung, GroundedSignals, GroundedSource,
    Interpreter, LocalMeasurer, Measures, MockGroundedSource, ReadMode, Recommendation, Session,
    TemplateInterpreter, ToolCall,
};
use dioxus::prelude::*;

/// The concrete session the UI drives: the engine chart source, a boxed grounded source (mock or
/// live EDGAR), and a boxed interpreter (template or Claude) — selected at runtime by env.
pub type SessionT = Session<EngineChartSource, Box<dyn GroundedSource>, Box<dyn Interpreter>>;

/// Where the user is in the loop: observe → decide → checkpoint → act.
#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Setup,
    Ranked,
    Checkpoint,
    Briefing,
}

/// What the Ask box renders in its answer area. The no-advice guardrail routes a question to exactly
/// one of these: a **Refusal** (the bold no-advice redirect for an advice-seeking question that
/// named no measurable company), a **Reading** that measures a named company (which fills in
/// off-thread — `pending` shows the "Consulting the viziers…" state until the prose lands, and which
/// carries the redirect + checkpoint note above/below it when the question was advice-seeking), or a
/// **Nudge** when no company was named.
#[derive(Debug, Clone, PartialEq)]
pub enum AnswerView {
    /// The enforced-in-code refusal/redirect for an advice-seeking question with no measurable company.
    Refusal(String),
    /// A measured reading for a named company. `text` is empty while `pending`, then filled by the
    /// off-thread reader; `ticker` lets the reader fill only the matching in-flight request.
    ///
    /// `redirect` and `note` are the honest-answer framing for an **advice-seeking** question that
    /// *also* named a company: `redirect` is the enforced-in-code refusal ("the ledger doesn't call
    /// trades…"), rendered above the reading, and `note` ("approve grounding at the checkpoint…")
    /// sits below it. Both are `None` for a plain non-advice company reading.
    Reading {
        name: String,
        ticker: String,
        label: String,
        pending: bool,
        text: String,
        redirect: Option<String>,
        note: Option<String>,
        /// A monotonic id for this ask. The off-thread reply carries it back and fills only the
        /// reading with the *matching* id — so a slower earlier ask for the same ticker can't land
        /// in a newer ask's slot (the double-submit race). Set from a process-wide counter.
        req_id: u64,
    },
    /// A helpful nudge — the question named no measurable choice.
    Nudge(String),
}

/// Build a session. The grounded briefing shows **real reality by default** via a
/// [`CompositeSource`]: SEC EDGAR filings + Wikipedia (with the recorded-fixture fallback when the
/// network is blocked), plus the provenance-clean dimensions SEC financials (XBRL) and Wikidata
/// structured facts — every one keyless, public-domain/CC0, and non-panicking. Set `ZIQPU_MOCK=1` to
/// force the offline [`MockGroundedSource`] (used by deterministic CI/tests).
pub fn build_session() -> SessionT {
    let grounded: Box<dyn GroundedSource> = if std::env::var("ZIQPU_MOCK").is_ok() {
        Box::new(MockGroundedSource)
    } else {
        Box::new(CompositeSource::live_default())
    };

    // Interpreter precedence (OpenAI-compat / OpenRouter → Anthropic → deterministic template).
    let interp: Box<dyn Interpreter> = agents::build_interpreter();

    let mut session = Session::new(EngineChartSource::default(), grounded, interp);
    if let Some(local) = LocalMeasurer::from_env() {
        session = session.with_measurer(Box::new(local));
    }
    session
}

/// The reactive app context, provided once at the root and read by every component.
///
/// `Signal<T>` is `Copy` and the `Rc` is cheap to clone, so this whole struct is cheap to clone
/// into event handlers.
#[derive(Clone)]
pub struct AppCtx {
    /// The graded session. Interior-mutable, single-threaded — never a Signal or an `Arc`.
    pub session: Rc<RefCell<SessionT>>,
    pub phase: Signal<Phase>,
    pub seeker: Signal<BirthMoment>,
    pub choices: Signal<Vec<Choice>>,
    pub recs: Signal<Vec<Recommendation>>,
    /// Per-choice measures, computed on a throwaway session so the graded log stays clean.
    pub measures: Signal<HashMap<String, Measures>>,
    pub selected: Signal<usize>,
    pub request: Signal<Option<ApprovalRequest>>,
    /// The proof the gate blocks without approval (a rendered `GateError`).
    pub gate_proof: Signal<Option<String>>,
    pub signals: Signal<Option<GroundedSignals>>,
    pub briefing: Signal<Option<Briefing>>,
    /// `true` while the approved grounded pull + briefing run **off the event-loop thread** (the SEC
    /// EDGAR fetch and the grounded-brief prose). The checkpoint shows a "grounding…" loading view
    /// instead of the gate while this is set; the [`Self::grounder`] coroutine clears it and advances
    /// to [`Phase::Briefing`] when the worker thread lands. Keeps the window responsive at the
    /// checkpoint→briefing transition (the old inline pull froze it).
    pub grounding: Signal<bool>,
    /// The active display mode, cycled by the header toggle **Raw → Local → Live**:
    /// - [`ReadMode::Raw`] — the deterministic local template from [`Self::raw_readings`] (instant).
    /// - [`ReadMode::Local`] — the user's own machine (LM Studio), cached in [`Self::local_readings`];
    ///   fetched off-thread on first switch, wheat loader until it lands.
    /// - [`ReadMode::Live`] — the streamed hosted-model prose in `recs` (with the wheat loader while
    ///   pending). Live is fetched at ranking time; Raw is computed synchronously; Local is lazy.
    ///
    /// Switching modes never re-runs the loop — each mode's prose is cached in its own store.
    pub mode: Signal<ReadMode>,
    /// The deterministic local template reading for every ranked ticker, filled synchronously in
    /// [`run_recommend`] (no network). Always present the instant the ranking paints, so the Raw view
    /// renders immediately when the toggle flips.
    pub raw_readings: Signal<HashMap<String, String>>,
    /// The **Local**-mode reading (the user's own LM Studio) per ticker, filled lazily by
    /// [`ensure_local_readings`] the first time the toggle reaches Local. A card with no cached local
    /// reading shows the wheat loader until its off-thread fetch lands here.
    pub local_readings: Signal<HashMap<String, String>>,
    /// Per-ticker provenance of the Local reading: `Some("local · <model>")` when LM Studio produced
    /// the prose, `None` on template fallback. Drives the Local card's badge.
    pub local_sources: Signal<HashMap<String, Option<String>>>,
    /// Tickers whose **Local** reading is still being fetched off-thread — each shows the wheat loader
    /// until its reading arrives.
    pub local_pending: Signal<std::collections::HashSet<String>>,
    /// The event-loop-thread coroutine that receives `(ticker, prose, model)` from the Local fetch's
    /// worker thread and writes them into `local_readings`/`local_sources`/`local_pending`. Same
    /// `!Send` discipline as [`Self::reader`] — only this channel crosses the thread boundary.
    pub local_reader: Coroutine<(String, String, Option<String>)>,
    /// What the no-advice guardrail is currently showing (refusal, reflection, measured reading, or
    /// nudge). See [`AnswerView`].
    pub answer: Signal<Option<AnswerView>>,
    /// The graded session's tool-call order, rendered for the Backstage panel.
    pub calls: Signal<Vec<String>>,
    /// Tickers whose prose reading is still being fetched off-thread — each such card shows a
    /// shimmer placeholder until its reading arrives.
    pub pending: Signal<std::collections::HashSet<String>>,
    /// Per-ticker provenance of the finished reading: `Some(model)` when the OpenRouter/OpenAI-compat
    /// API produced the prose (the live model id), `None` when it fell back to the deterministic
    /// template. Drives the card's live/offline badge so it is unmistakable whether the API worked.
    pub sources: Signal<HashMap<String, Option<String>>>,
    /// The event-loop-thread coroutine that receives `(ticker, prose, live_model)` results from the
    /// worker thread and writes them back into `recs`/`sources`/`pending`. `Coroutine<T>` is `Copy`,
    /// and its `tx()` yields a `Send` sender the worker thread can hold (Signals are `!Send`, so they
    /// never cross the thread boundary — only this channel does).
    pub reader: Coroutine<(String, String, Option<String>)>,
    /// The event-loop-thread coroutine that receives `(req_id, prose)` from the Ask box's worker
    /// thread and fills the in-flight [`AnswerView::Reading`] whose `req_id` matches — so a live
    /// interpreter call from the guardrail never blocks (freezes) the window, and a stale reply for
    /// a superseded ask is dropped. Same `!Send` discipline as [`Self::reader`]: only this channel
    /// crosses the thread boundary, never a `Signal`.
    pub ask_reader: Coroutine<(u64, String)>,
    /// The event-loop-thread coroutine that receives `(GroundedSignals, Briefing, rung, source)`
    /// from the grounded pull's worker thread and commits them: sets `signals` + `briefing` + `rung`,
    /// clears `grounding`, and advances to [`Phase::Briefing`]. Same `!Send` discipline as
    /// [`Self::reader`] — only owned, `Send` values cross the channel; the real session and every
    /// `Signal` stay on the UI thread.
    pub grounder: Coroutine<(GroundedSignals, Briefing, GroundedRung, Option<String>)>,

    // ── The layered grounding pipeline (N2 feature 4) ───────────────────────────────────────────
    /// The **developer-build** entitlement switch (persisted in `settings.json`). `true` = the
    /// developer build with every paywalled feature unlocked (the default while building); `false` =
    /// "preview as customer", where premium affordances lock and show a 🔒. Read via [`Self::premium`].
    pub dev_build: Signal<bool>,
    /// The local model's **framing brief** for the grounded reading, drafted off-thread during the
    /// checkpoint pause (`None` until it lands, or when no local server is reachable). Shown read-only
    /// at the checkpoint (editable only in the developer build), and fed to the frontier on approval.
    pub draft: Signal<Option<String>>,
    /// `true` while the local draft is being written off-thread (the checkpoint shows a "preparing…"
    /// state for the brief). Cleared by [`Self::drafter`] when the worker lands.
    pub draft_pending: Signal<bool>,
    /// The event-loop-thread coroutine that receives the drafted brief (`Option<String>`: `None` when
    /// no local server answered) from [`run_draft`]'s worker thread, writes it into `draft`, and
    /// clears `draft_pending`. Same `!Send` discipline as [`Self::reader`].
    pub drafter: Coroutine<Option<String>>,
    /// Which rung of the honesty ladder produced the grounded briefing — drives the Briefing card's
    /// badge (`GROUNDED · LIVE` / `GROUNDED · LOCAL` / `LOCAL · UNSOURCED` / `GROUNDED`). Set by
    /// [`Self::grounder`] alongside `briefing`.
    pub rung: Signal<Option<GroundedRung>>,
}

impl AppCtx {
    /// Whether paywalled features are unlocked — true in the developer build, false while previewing
    /// as a customer. The single entitlement read every 🔒 gate consults.
    pub fn premium(&self) -> bool {
        *self.dev_build.read()
    }
}

/// Render a graded tool call in the Backstage's "tool order" voice — lowercase, call-shaped
/// (`get_chart(you)`, `get_synastry(you, TSLA)`, `propose`, `pull_grounded(TSLA)`). Matches the
/// approved mockup and lets the Backstage tag the `propose`/`pull_grounded` chips distinctly.
pub fn pretty_call(call: &ToolCall) -> String {
    match call {
        ToolCall::GetChart(who) => format!("get_chart({who})"),
        ToolCall::GetSynastry(a, b) => format!("get_synastry({a}, {b})"),
        ToolCall::Propose => "propose".to_string(),
        ToolCall::PullGrounded(ticker) => format!("pull_grounded({ticker})"),
    }
}

/// Map a fit band to the mockup's semantic band CSS variable name (drives the card's left stripe,
/// badge, and score meter via the `--band` custom property). Note the terse `--band-misalign` to
/// match the approved stylesheet's token spelling.
pub fn fit_band_var(fit: Fit) -> &'static str {
    match fit {
        Fit::StronglyAligned => "--band-strong",
        Fit::Aligned => "--band-aligned",
        Fit::Mixed => "--band-mixed",
        Fit::Misaligned => "--band-misalign",
    }
}

/// Run OBSERVE → DECIDE on the graded session and advance to the Ranked phase — **without blocking
/// the event loop on the readings**. Factored out of the seeded button so both Setup modes (seeded
/// demo and the custom birth form) converge on one submit path — identical graded behavior,
/// identical tool-order log. The caller sets `ctx.seeker` first.
///
/// The ranked list paints instantly: [`Session::recommend_measures`] records the graded tool order
/// synchronously on the real session and returns the ranking with **empty** readings, so no `curl`
/// runs here. Every card starts in `pending` (shimmer). One background thread then fills each
/// reading via [`agents::reading_for`] — where the blocking `curl` actually runs — and streams the
/// prose back through `ctx.reader` to the UI thread. The thread captures only owned, `Send` values;
/// it never touches `ctx.session` or any `Signal` (both `!Send`).
pub fn run_recommend(mut ctx: AppCtx) {
    let seeker = ctx.seeker.read().clone();
    let choices = ctx.choices.read().clone();

    // Per-choice measures (throwaway session, so the graded log stays clean). Also the worker
    // thread's source of truth for each reading's Measures + band.
    let measures = measures_for(&seeker, &choices);

    // Fast DECIDE on the REAL graded session: ranking + trailing `Propose`, empty readings, no curl.
    let recs = {
        let mut session = ctx.session.borrow_mut();
        session.recommend_measures(&seeker, &choices)
    };
    let calls: Vec<String> = {
        let session = ctx.session.borrow();
        session.calls().iter().map(pretty_call).collect()
    };

    // The Raw view's readings: the deterministic local template prose for every choice, computed
    // synchronously here (no network) so flipping to Raw is instant. The live prose still streams
    // into `recs` separately via the worker thread below.
    let raw_readings: HashMap<String, String> = recs
        .iter()
        .filter_map(|r| {
            let m = measures.get(&r.choice)?;
            let fit = Fit::from_score(m.score);
            Some((
                r.choice.clone(),
                TemplateInterpreter.fit_read(m, fit, &r.name),
            ))
        })
        .collect();

    // Every ranked ticker starts "still reading"; the list renders now, prose fills in later.
    let pending: std::collections::HashSet<String> =
        recs.iter().map(|r| r.choice.clone()).collect();
    // The (ticker, name) work list for the worker thread — owned and `Send`.
    let work: Vec<(String, String)> = recs
        .iter()
        .map(|r| (r.choice.clone(), r.name.clone()))
        .collect();

    ctx.measures.set(measures.clone());
    ctx.raw_readings.set(raw_readings);
    ctx.recs.set(recs);
    ctx.calls.set(calls);
    ctx.selected.set(0);
    ctx.pending.set(pending);
    // A fresh ranking invalidates any prior Local-mode cache; clear it. If the toggle is currently on
    // Local, re-fetch the new cards' local readings off-thread so they don't hang on the loader.
    ctx.local_readings.set(HashMap::new());
    ctx.local_sources.set(HashMap::new());
    ctx.local_pending.set(std::collections::HashSet::new());
    ctx.phase.set(Phase::Ranked);
    if *ctx.mode.read() == ReadMode::Local {
        ensure_local_readings(ctx.clone());
    }

    // Clear the built-in free-tier health latch so the "over budget / paused" banner reflects only
    // THIS ranking's built-in attempts — and stays hidden when the active source is the seeker's own
    // key (that path never records). The reader coroutine reads `agents::tier::notice()` as each
    // reading lands.
    agents::tier::reset();

    // Fill readings off the event-loop thread: the only place the blocking `curl` runs. The closure
    // moves only owned, `Send` values (the measures map, the work list, and the channel sender).
    let tx = ctx.reader.tx();
    std::thread::spawn(move || {
        for (ticker, name) in work {
            let Some(m) = measures.get(&ticker) else {
                continue;
            };
            let fit = Fit::from_score(m.score);
            // `reading_for` returns `(prose, live_model_id)` — `Some(model)` when the API produced
            // the prose, `None` on template fallback. Thread the source through so the card can badge it.
            let (prose, model) = agents::reading_for(m, fit, &name);
            // If the UI is gone (receiver dropped), this send just fails — nothing to do.
            let _ = tx.unbounded_send((ticker, prose, model));
        }
    });
}

/// Trim a reading down to what a card should DISPLAY: drop a leading redundant `FIT: … — name` line
/// (the card header already shows band/score/name), and drop the trailing `REMINDER: measured, not
/// fate — not financial advice` (now shown ONCE in the persistent footer + carried by the rung badge,
/// per the owner's "say it once" ask — no longer repeated on every reading). The disclaimer stays in
/// the reading DATA (the no-advice guardrail and the interpreter tests are untouched); this only
/// affects rendering.
pub fn strip_reading_chrome(text: &str) -> String {
    let body = match text.split_once('\n') {
        Some((first, rest)) if first.trim_start().starts_with("FIT:") => rest.trim_start(),
        _ => text,
    };
    match body.rfind("REMINDER") {
        Some(i) => body[..i].trim_end().to_string(),
        None => body.trim_end().to_string(),
    }
}

/// Advance the display mode one step in the header toggle's cycle: **Raw → Local → Live → Raw**.
pub fn next_mode(mode: ReadMode) -> ReadMode {
    match mode {
        ReadMode::Raw => ReadMode::Local,
        ReadMode::Local => ReadMode::Live,
        ReadMode::Live => ReadMode::Raw,
    }
}

/// Lazily fill the **Local** reading for every ranked card that isn't cached (or already in flight)
/// yet — the analog of the Live fill in [`run_recommend`], but on demand. Called when the header
/// toggle reaches Local. For each uncached ticker it marks `local_pending` (so the card shows the
/// wheat loader), then hands the blocking [`agents::reading_for_mode`] call for
/// [`ReadMode::Local`] to one background thread, which streams `(ticker, prose, model)` back through
/// `ctx.local_reader`. The closure captures only owned, `Send` values — never a `Signal` or the
/// `!Send` session.
pub fn ensure_local_readings(mut ctx: AppCtx) {
    let recs = ctx.recs.read().clone();
    let measures = ctx.measures.read().clone();

    // The cards to fetch — uncached, OR previously fell back to the TEMPLATE (source None), so a
    // warm-up-race template result gets retried on the next Local entry instead of sticking forever.
    // Not-in-flight either way.
    let to_fetch: Vec<(String, String, Measures, Fit)> = {
        let cached = ctx.local_readings.read();
        let sources = ctx.local_sources.read();
        let in_flight = ctx.local_pending.read();
        recs.iter()
            .filter(|r| {
                let done_by_model = cached.contains_key(&r.choice)
                    && sources.get(&r.choice).map(Option::is_some).unwrap_or(false);
                !done_by_model && !in_flight.contains(&r.choice)
            })
            .filter_map(|r| {
                let m = measures.get(&r.choice)?.clone();
                let fit = Fit::from_score(m.score);
                Some((r.choice.clone(), r.name.clone(), m, fit))
            })
            .collect()
    };
    if to_fetch.is_empty() {
        return;
    }

    {
        let mut in_flight = ctx.local_pending.write();
        for (ticker, ..) in &to_fetch {
            in_flight.insert(ticker.clone());
        }
    }

    let tx = ctx.local_reader.tx();
    std::thread::spawn(move || {
        // Wait for the local server to finish LOADING the model before firing any reads — otherwise
        // the first cards hit a not-yet-ready server, fall back to the template, and (being cached)
        // never recover. Up to 45 s (a big quant can take a while to load); `false` = still not up, in
        // which case the reads below just fall back to the template once (no wasted retries).
        let ready = agents::wait_for_local(std::time::Duration::from_secs(45));
        for (ticker, name, m, fit) in to_fetch {
            let (mut prose, mut model) = agents::reading_for_mode(&m, fit, &name, ReadMode::Local);
            // One retry on a transient fallback, but only when the server IS up (else don't hammer a
            // down server — a single template fallback is the right answer there).
            if ready && model.is_none() {
                std::thread::sleep(std::time::Duration::from_millis(400));
                let (p, mdl) = agents::reading_for_mode(&m, fit, &name, ReadMode::Local);
                prose = p;
                model = mdl;
            }
            let _ = tx.unbounded_send((ticker, prose, model));
        }
    });
}

/// Draft the interpreter's framing brief on the **local** model, off the event loop, the instant the
/// checkpoint pause begins — so the wait to approve is productive (the local model works while the
/// human decides). Sees only the measures (computed on a throwaway session), never external data, so
/// it is structurally safe to run before approval. Streams the draft (`Some(brief)`, or `None` when
/// no local server answered) back through `ctx.drafter`. The closure captures only owned, `Send`
/// values; it never touches `ctx.session` or a `Signal`.
pub fn run_draft(mut ctx: AppCtx, choice: Choice) {
    let seeker = ctx.seeker.read().clone();
    // A fresh draft supersedes any prior one; show the "preparing…" state.
    ctx.draft.set(None);
    ctx.draft_pending.set(true);

    let tx = ctx.drafter.tx();
    std::thread::spawn(move || {
        let measures = {
            let mut throwaway = build_session();
            throwaway.measure(&seeker, &choice)
        };
        let fit = Fit::from_score(measures.score);
        let draft = draft_grounding_prompt(&measures, fit, &choice.name);
        let _ = tx.unbounded_send(draft);
    });
}

/// Fetch the real grounded signals for a choice on a **throwaway**, `Send`-safe source — the same
/// mock/composite selection [`build_session`] makes. Called only from a worker thread (the source's
/// `curl` calls block), never on the event loop.
pub fn fetch_grounded(choice: &Choice) -> GroundedSignals {
    let source: Box<dyn GroundedSource> = if std::env::var("ZIQPU_MOCK").is_ok() {
        Box::new(MockGroundedSource)
    } else {
        Box::new(CompositeSource::live_default())
    };
    source.fetch(choice)
}

/// ACT (gated), **without freezing the window**. The analog of [`run_recommend`] for the grounded
/// pull: the two things that must touch the real (`!Send`) session — minting the approval token and
/// recording the graded [`ToolCall::PullGrounded`] — happen synchronously here, so the tool-order
/// log and the human-in-the-loop gate are preserved exactly. The blocking work (the SEC EDGAR fetch
/// **and** the grounded-brief prose) is then handed to one background thread; the UI shows a
/// "grounding…" loading state (`ctx.grounding`) until the thread streams `(signals, briefing)` back
/// through `ctx.grounder`, which commits them and advances to [`Phase::Briefing`].
///
/// The worker closure captures only owned, `Send` values (the choice, the seeker, the read mode, and
/// the channel sender). It never touches `ctx.session` or any `Signal` (both `!Send`): it builds a
/// throwaway session + grounded source of its own via [`fetch_grounded`]/[`build_session`].
pub fn run_grounding(mut ctx: AppCtx) {
    // Move the non-Copy request out of the signal (mirrors the old inline handler).
    let request = ctx.request.write().take();
    let Some(request) = request else { return };
    let ticker = request.choice.clone();
    let choice = ctx
        .choices
        .read()
        .iter()
        .find(|c| c.ticker == ticker)
        .cloned();
    let Some(choice) = choice else { return };

    // Approve mints the token; record the gated pull on the REAL session — synchronously, so the
    // graded tool-order log + the gate stay intact — but with NO blocking fetch here.
    let token = ctx.session.borrow().approve(request);
    {
        let mut session = ctx.session.borrow_mut();
        if session.pull_grounded_gate(&choice, Some(&token)).is_err() {
            return;
        }
    }
    let calls: Vec<String> = {
        let session = ctx.session.borrow();
        session.calls().iter().map(pretty_call).collect()
    };

    let seeker = ctx.seeker.read().clone();
    let mode = *ctx.mode.read();
    // The framing brief the local model drafted during the pause (or the developer's edit of it).
    // `None` when no local server answered or the human approved before it landed — the frontier then
    // simply gets its standard prompt. Read on the event loop here; the worker only holds the owned copy.
    let draft = ctx.draft.read().clone();

    // Enter the loading state: the tool log already shows the pull; the gate proof is cleared; the
    // checkpoint renders "grounding…" until the worker lands.
    ctx.calls.set(calls);
    ctx.gate_proof.set(None);
    ctx.grounding.set(true);

    // The only place the blocking SEC EDGAR curl + the grounded-brief model call run. Everything
    // captured is owned and `Send`; the throwaway session/source live entirely inside the thread.
    // The layered pipeline routes by mode: Live sends the frontier the (locally-drafted) brief and
    // degrades down the honesty ladder if it's down; Local/Raw stay on-device/template. The rung it
    // lands on rides back so the Briefing card badges the read truthfully.
    let tx = ctx.grounder.tx();
    std::thread::spawn(move || {
        let signals = fetch_grounded(&choice);
        let measures = {
            let mut throwaway = build_session();
            throwaway.measure(&seeker, &choice)
        };
        let fit = Fit::from_score(measures.score);
        let brief = grounded_layered(
            &measures,
            fit,
            &choice.name,
            &signals,
            draft.as_deref(),
            mode,
        );
        // If the UI is gone (receiver dropped), this send just fails — nothing to do.
        let _ = tx.unbounded_send((
            signals,
            Briefing {
                reading: brief.reading,
            },
            brief.rung,
            brief.source,
        ));
    });
}

/// One star in the fixed background field: `(left%, top%, size_px, delay_s, dur_s)`.
pub type Star = (f32, f32, f32, f32, f32);

/// The seeded, deterministic star field behind the whole app — **identical every launch** (a fixed
/// xorshift seed, no clock, no RNG crate). Returns ~84 stars scattered across the viewport, each with
/// its own size and twinkle timing so the field shimmers out of phase. Pure data; the CSS animates.
pub fn seeded_stars() -> Vec<Star> {
    let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
    // A tiny xorshift64 mapped into `[lo, hi)`. One closure owns the state, so there is no clock and
    // no external RNG — the same 84 stars come back on every run.
    let mut rnd = move |lo: f32, hi: f32| {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let unit = (s % 100_000) as f32 / 100_000.0;
        lo + unit * (hi - lo)
    };
    (0..84)
        .map(|_| {
            (
                rnd(0.5, 99.5), // left %
                rnd(1.0, 99.0), // top %
                rnd(1.0, 2.7),  // size px
                rnd(0.0, 4.5),  // twinkle delay s
                rnd(2.6, 6.0),  // twinkle duration s
            )
        })
        .collect()
}

/// Compute per-choice [`Measures`] on a **throwaway** session, so the graded main session's
/// `calls()` log records only the real run — not five extra measurement passes for the cards.
pub fn measures_for(seeker: &BirthMoment, choices: &[Choice]) -> HashMap<String, Measures> {
    let mut throwaway = build_session();
    choices
        .iter()
        .map(|c| (c.ticker.clone(), throwaway.measure(seeker, c)))
        .collect()
}
