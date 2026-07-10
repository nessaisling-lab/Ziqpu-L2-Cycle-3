//! Shared app state — the session handle, the phase machine, and the reactive signals every
//! component reads and writes through the Dioxus context.
//!
//! `Session` is generic, `!Send`, and `!Clone`, so it lives in an `Rc<RefCell<_>>` (never a Signal
//! or an `Arc`). The signals hold only the plain, cloneable *outputs* of the loop.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{
    ApprovalRequest, BirthMoment, Briefing, Choice, EdgarSource, EngineChartSource, Fit,
    GroundedSignals, GroundedSource, Interpreter, LocalMeasurer, Measures, MockGroundedSource,
    Recommendation, Session, TemplateInterpreter, ToolCall,
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
/// one of these: a **Refusal** (buy/sell/hold — the bold no-advice palette), a plain **Reflection**,
/// a **Reading** that measures a named company (which fills in off-thread — `pending` shows the
/// "Consulting the viziers…" state until the prose lands), or a **Nudge** when no company was named.
#[derive(Debug, Clone, PartialEq)]
pub enum AnswerView {
    /// The enforced-in-code refusal for an advice-seeking question.
    Refusal(String),
    /// A plain reflection (non-advice, no company named-and-measured).
    Reflection(String),
    /// A measured reading for a named company. `text` is empty while `pending`, then filled by the
    /// off-thread reader; `ticker` lets the reader fill only the matching in-flight request.
    Reading {
        name: String,
        ticker: String,
        label: String,
        pending: bool,
        text: String,
    },
    /// A helpful nudge — the question named no measurable choice.
    Nudge(String),
}

/// Build a session. The grounded briefing shows **real reality by default**: [`EdgarSource`] does a
/// live keyless SEC EDGAR pull when online and transparently falls back to the bundled recorded
/// filing fixture when the network is blocked (never a bare mock) — both non-panicking. Set
/// `ZIQPU_MOCK=1` to force the offline [`MockGroundedSource`] (used by deterministic CI/tests).
pub fn build_session() -> SessionT {
    let grounded: Box<dyn GroundedSource> = if std::env::var("ZIQPU_MOCK").is_ok() {
        Box::new(MockGroundedSource)
    } else {
        Box::new(EdgarSource::default())
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
    /// `true` = **Live** (show the streamed model prose in `recs`, with the wheat loader while
    /// pending); `false` = **Raw** (show the deterministic local template reading from
    /// [`Self::raw_readings`], instantly). A pure *display* switch — toggling never re-runs the loop
    /// or re-fetches, because both readings are already computed.
    pub live_mode: Signal<bool>,
    /// The deterministic local template reading for every ranked ticker, filled synchronously in
    /// [`run_recommend`] (no network). Always present the instant the ranking paints, so the Raw view
    /// renders immediately when the toggle flips.
    pub raw_readings: Signal<HashMap<String, String>>,
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
    /// The event-loop-thread coroutine that receives `(ticker, prose)` from the Ask box's worker
    /// thread and fills the in-flight [`AnswerView::Reading`] with the returned prose — so a live
    /// interpreter call from the guardrail never blocks (freezes) the window. Same `!Send` discipline
    /// as [`Self::reader`]: only this channel crosses the thread boundary, never a `Signal`.
    pub ask_reader: Coroutine<(String, String)>,
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
    ctx.phase.set(Phase::Ranked);

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

/// Compute per-choice [`Measures`] on a **throwaway** session, so the graded main session's
/// `calls()` log records only the real run — not five extra measurement passes for the cards.
pub fn measures_for(seeker: &BirthMoment, choices: &[Choice]) -> HashMap<String, Measures> {
    let mut throwaway = build_session();
    choices
        .iter()
        .map(|c| (c.ticker.clone(), throwaway.measure(seeker, c)))
        .collect()
}
