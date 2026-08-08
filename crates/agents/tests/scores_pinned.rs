//! The demo scores, pinned — so a change to the arithmetic cannot land unnoticed.
//!
//! # Why this exists
//!
//! Making DE440 the default ephemeris moved four of the five demo scores (Apple 33 → 28, Tesla
//! 87 → 85, Coca-Cola 51 → 56, Johnson & Johnson 52 → 53) and **the entire suite stayed green**,
//! because nothing anywhere pinned a synastry number. The Eval Card did not catch it either: it
//! grades structure and honesty — band matches score, citation is real, no advice — not arithmetic.
//! A card that checks "the band matches whatever the score is" passes just as happily when the score
//! is wrong.
//!
//! So a score is a **contract** now. If one of these numbers changes, that is either a deliberate
//! improvement — in which case update the constant in the same commit that caused it, and say why —
//! or a regression that would otherwise have shipped invisibly.
//!
//! # Why it pins the analytic floor rather than the shipped engine
//!
//! The numbers a seeker actually sees come from DE440, but the kernel is a 32 MB download that is
//! gitignored and absent on most CI runners. A test pinned to DE440 alone would be skipped exactly
//! where regressions get caught.
//!
//! So the always-on half pins **`AnalyticBackend` explicitly**, which needs no kernel, no network,
//! and no feature flag. That isolates the thing actually under test: the aspect-finding, orb, and
//! weighting logic, held still against a fixed ephemeris. The DE440 half then runs only when the
//! verified kernel is present, and pins what ships.
//!
//! Splitting them this way also means the two sets of constants below **document the cost of the
//! analytic floor** — the gap between the columns is what a user loses when the kernel is missing,
//! written down rather than described.

use agents::{demo_choices, demo_seeker, synastry_score, ChartSource, EngineChartSource, Fit};
use engine::compute_chart;
use ephemeris::AnalyticBackend;

/// `(ticker, score)` with the analytic floor — no Pluto, Meeus Moon.
const ANALYTIC: [(&str, u8); 5] = [
    ("AAPL", 33),
    ("MSFT", 54),
    ("TSLA", 87),
    ("KO", 51),
    ("JNJ", 52),
];

/// `(ticker, score)` with JPL DE440 — what a shipped build produces.
const DE440: [(&str, u8); 5] = [
    ("AAPL", 28),
    ("MSFT", 54),
    ("TSLA", 85),
    ("KO", 56),
    ("JNJ", 53),
];

/// Score every demo choice against the demo seeker using a named backend.
///
/// Charts are computed directly against `backend` rather than through
/// [`EngineChartSource::chart`], which resolves the process-wide engine — that resolution is the
/// variable this test is trying to hold still. The synastry and scoring path is the real one.
fn scores_with<E: ephemeris::Ephemeris + ?Sized>(backend: &E) -> Vec<(String, u8)> {
    let source = EngineChartSource::default();
    let seeker = demo_seeker();
    let (jd, time_known) = seeker.julian_day_ut();
    let a = compute_chart(backend, jd, seeker.lat, seeker.lon, time_known);

    demo_choices()
        .into_iter()
        .map(|choice| {
            let (jd, time_known) = choice.birth.julian_day_ut();
            let b = compute_chart(backend, jd, choice.birth.lat, choice.birth.lon, time_known);
            let score = synastry_score(&source.synastry(&a, &b));
            (choice.ticker, score)
        })
        .collect()
}

/// The scoring logic itself, held against a fixed ephemeris. Runs everywhere.
#[test]
fn analytic_scores_are_the_pinned_contract() {
    let got = scores_with(&AnalyticBackend);
    let expected: Vec<(String, u8)> = ANALYTIC.iter().map(|(t, s)| (t.to_string(), *s)).collect();

    assert_eq!(
        got, expected,
        "a demo score moved on the analytic backend.\n\
         If this was deliberate, update ANALYTIC in the same commit and say what changed.\n\
         If it was not, the aspect/orb/weight arithmetic regressed."
    );
}

/// What ships. Runs only where the verified DE440 kernel resolved.
///
/// Not `#[ignore]`: an ignored test never runs anywhere by accident and is indistinguishable from a
/// deleted one. This asserts unconditionally that the engine resolved to *something* coherent, and
/// pins the numbers when that something is DE440.
#[test]
fn de440_scores_are_the_pinned_contract_where_the_kernel_is_present() {
    let engine = ephemeris::shared();
    if !engine.source.is_authoritative() {
        // No kernel here. Say so loudly rather than passing in silence — a quiet skip is how a
        // claim goes unchecked for months.
        eprintln!(
            "SKIPPED the DE440 pin: {}",
            engine
                .source
                .caveat()
                .unwrap_or_else(|| "engine not authoritative".into())
        );
        // The analytic pin still has to hold on this machine, so the run is not vacuous.
        assert_eq!(
            scores_with(&AnalyticBackend).len(),
            5,
            "five demo choices, whatever the engine"
        );
        return;
    }

    let got = scores_with(engine.backend.as_ref());
    let expected: Vec<(String, u8)> = DE440.iter().map(|(t, s)| (t.to_string(), *s)).collect();
    assert_eq!(
        got, expected,
        "a demo score moved on JPL DE440 — this is what a user sees.\n\
         Update DE440 in the same commit as the cause, or find the regression."
    );
}

/// Bands are what the reading actually claims, and none of them may flip silently.
///
/// Separate from the score pins on purpose: a one-point drift is a curiosity, a band flip changes
/// the verdict a person reads. Swapping the whole ephemeris moved four scores and flipped **no**
/// band — that is the property worth guarding, and it is stated rather than inferred.
#[test]
fn no_band_differs_between_the_two_engines() {
    let analytic = scores_with(&AnalyticBackend);
    let de440: Vec<(String, u8)> = DE440.iter().map(|(t, s)| (t.to_string(), *s)).collect();

    for ((ticker, a), (_, d)) in analytic.iter().zip(de440.iter()) {
        assert_eq!(
            Fit::from_score(*a).label(),
            Fit::from_score(*d).label(),
            "{ticker}: the engines disagree on the BAND ({a} vs {d}), not just the score — \
             that changes the verdict a reader sees, and needs a deliberate decision"
        );
    }
}

/// The ranked order the demo opens on. Scores are arithmetic; the order must not wander.
#[test]
fn the_ranked_order_is_stable_across_runs() {
    let ranked = |mut v: Vec<(String, u8)>| {
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.into_iter().map(|(t, _)| t).collect::<Vec<_>>()
    };
    let first = ranked(scores_with(&AnalyticBackend));
    for _ in 0..3 {
        assert_eq!(
            ranked(scores_with(&AnalyticBackend)),
            first,
            "the ranking changed between identical runs — the Eval Card calls that a hard fail"
        );
    }
}
