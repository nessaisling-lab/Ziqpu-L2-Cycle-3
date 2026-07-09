//! Shared app state — the session handle, the phase machine, and the reactive signals every
//! component reads and writes through the Dioxus context.
//!
//! `Session` is generic, `!Send`, and `!Clone`, so it lives in an `Rc<RefCell<_>>` (never a Signal
//! or an `Arc`). The signals hold only the plain, cloneable *outputs* of the loop.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use agents::{
    AnthropicInterpreter, ApprovalRequest, BirthMoment, Briefing, Choice, EdgarSource,
    EngineChartSource, Fit, GroundedSignals, GroundedSource, Interpreter, LocalMeasurer, Measures,
    MockGroundedSource, Recommendation, Session, TemplateInterpreter,
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

/// Build a session exactly as the terminal demo does (`agents/src/main.rs`): deterministic and
/// offline unless the operator opts in via env (`ZIQPU_LIVE`, `ANTHROPIC_API_KEY`, `ZIQPU_LOCAL_LLM`).
pub fn build_session() -> SessionT {
    let grounded: Box<dyn GroundedSource> = if std::env::var("ZIQPU_LIVE").is_ok() {
        Box::new(EdgarSource::default())
    } else {
        Box::new(MockGroundedSource)
    };

    let interp: Box<dyn Interpreter> = match AnthropicInterpreter::from_env() {
        Some(a) => Box::new(a),
        None => Box::new(TemplateInterpreter),
    };

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
    /// `(is_refusal, message)` from the no-advice guardrail.
    pub answer: Signal<Option<(bool, String)>>,
    /// The graded session's tool-call order, rendered for the Backstage panel.
    pub calls: Signal<Vec<String>>,
}

/// Map a fit band to its CSS modifier class.
pub fn fit_class(fit: Fit) -> &'static str {
    match fit {
        Fit::StronglyAligned => "band-strong",
        Fit::Aligned => "band-aligned",
        Fit::Mixed => "band-mixed",
        Fit::Misaligned => "band-misaligned",
    }
}

/// Run OBSERVE → DECIDE on the graded session and advance to the Ranked phase. Factored out of the
/// seeded button so both Setup modes (seeded demo and the custom birth form) converge on one submit
/// path — identical graded behavior, identical tool-order log. The caller sets `ctx.seeker` first.
pub fn run_recommend(mut ctx: AppCtx) {
    let seeker = ctx.seeker.read().clone();
    let choices = ctx.choices.read().clone();
    let recs = {
        let mut session = ctx.session.borrow_mut();
        session.recommend(&seeker, &choices)
    };
    let calls: Vec<String> = {
        let session = ctx.session.borrow();
        session.calls().iter().map(|c| format!("{c:?}")).collect()
    };
    ctx.measures.set(measures_for(&seeker, &choices));
    ctx.recs.set(recs);
    ctx.calls.set(calls);
    ctx.selected.set(0);
    ctx.phase.set(Phase::Ranked);
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
