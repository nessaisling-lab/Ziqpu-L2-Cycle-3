//! Ungasaga — turns measures into a reading in three beats: measured → meaning → reminder.
//! The default interpreter is deterministic (CI-safe, demo-safe). The trait is the seam a
//! real-model interpreter (Claude via the Anthropic API) implements later, unchanged above it.

use crate::types::{Confidence, Fit, GroundedSignals, Measures, TransitBeat};
use chrono::NaiveDate;

const REMINDER: &str = "REMINDER: measured, not fate — not financial advice.";

/// The interpreter seam. Never computes; never advises.
pub trait Interpreter {
    /// The primary, always-on fit read (no external data).
    fn fit_read(&self, measures: &Measures, fit: Fit, name: &str) -> String;
    /// The grounded briefing, after the checkpoint — sits real signals beside the chart read.
    fn grounded_brief(
        &self,
        measures: &Measures,
        fit: Fit,
        name: &str,
        grounded: &GroundedSignals,
    ) -> String;

    /// The flagship long-form **Report** reading — measured → meaning → confidence → reminder.
    /// Defaulted so real-model interpreters (e.g. [`crate::AnthropicInterpreter`]) compile
    /// unchanged; the deterministic template overrides it.
    fn report_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "REPORT: {} ({} / 100) — {name}\n  measured: {}\n  meaning: tradition reads {}.\n  {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            measured_clause(measures),
            meaning_clause(fit),
            confidence_beat(measures.confidence),
        )
    }

    /// The one-line **Verdict** — band, score, a confidence beat, one meaning clause, and the
    /// guardrail (ends in `Not financial advice.`). Defaulted like [`Interpreter::report_read`].
    fn verdict_line(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "VERDICT: {} ({}/100) — {name}. {}. Tradition reads {}. Not financial advice.",
            fit.label(),
            measures.score,
            confidence_beat(measures.confidence),
            meaning_clause(fit),
        )
    }

    /// The daily one-beat read: a single transit line ending in the guardrail. Defaulted so
    /// real-model interpreters compile unchanged; the deterministic template overrides it.
    fn daily_beat(&self, beat: Option<&TransitBeat>, date: NaiveDate) -> String {
        daily_beat_template(beat, date)
    }
}

/// Lets a boxed interpreter be used wherever an `Interpreter` is expected (runtime selection).
/// Every method is forwarded explicitly — including the defaulted [`Interpreter::report_read`] and
/// [`Interpreter::verdict_line`], or a boxed override would be silently shadowed by the default.
impl Interpreter for Box<dyn Interpreter> {
    fn fit_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        (**self).fit_read(measures, fit, name)
    }
    fn grounded_brief(
        &self,
        measures: &Measures,
        fit: Fit,
        name: &str,
        grounded: &GroundedSignals,
    ) -> String {
        (**self).grounded_brief(measures, fit, name, grounded)
    }
    fn report_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        (**self).report_read(measures, fit, name)
    }
    fn verdict_line(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        (**self).verdict_line(measures, fit, name)
    }
    fn daily_beat(&self, beat: Option<&TransitBeat>, date: NaiveDate) -> String {
        // Explicit forward — or a boxed override is silently shadowed by the trait default.
        (**self).daily_beat(beat, date)
    }
}

/// The tightest three contacts, one plain clause each — shared by the default trait bodies.
fn measured_clause(measures: &Measures) -> String {
    if measures.top.is_empty() {
        return "no close contacts between the two charts".to_string();
    }
    measures
        .top
        .iter()
        .take(3)
        .map(|a| {
            format!(
                "{} {} {} (orb {:.1}°, {})",
                a.body_a,
                a.aspect.to_lowercase(),
                a.body_b,
                a.orb,
                if a.harmonious { "flowing" } else { "friction" }
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// The tradition-side meaning of a fit band — shared by the default trait bodies.
fn meaning_clause(fit: Fit) -> &'static str {
    match fit {
        Fit::StronglyAligned => "an easy, recognizing resonance",
        Fit::Aligned => "more ease than friction",
        Fit::Mixed => "a genuine mix of ease and tension",
        Fit::Misaligned => "more friction than flow",
    }
}

/// A short confidence beat naming the level — always carries a confidence word.
fn confidence_beat(confidence: Confidence) -> String {
    format!("confidence {}", confidence.label())
}

/// Deterministic templated interpreter — the default.
#[derive(Default)]
pub struct TemplateInterpreter;

impl TemplateInterpreter {
    fn measured(&self, measures: &Measures) -> String {
        if measures.top.is_empty() {
            return "no close contacts between the two charts".to_string();
        }
        measures
            .top
            .iter()
            .take(3)
            .map(|a| {
                format!(
                    "{} {} {} (orb {:.1}°, {})",
                    a.body_a,
                    a.aspect.to_lowercase(),
                    a.body_b,
                    a.orb,
                    if a.harmonious { "flowing" } else { "friction" }
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn meaning(&self, fit: Fit) -> &'static str {
        match fit {
            Fit::StronglyAligned => "an easy, recognizing resonance",
            Fit::Aligned => "more ease than friction",
            Fit::Mixed => "a genuine mix of ease and tension",
            Fit::Misaligned => "more friction than flow",
        }
    }
}

impl Interpreter for TemplateInterpreter {
    fn fit_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "FIT: {} ({} / 100) — {name}\n  measured: {}\n  meaning: tradition reads {}.\n  {REMINDER}",
            fit.label(),
            measures.score,
            self.measured(measures),
            self.meaning(fit),
        )
    }

    fn grounded_brief(
        &self,
        measures: &Measures,
        fit: Fit,
        name: &str,
        grounded: &GroundedSignals,
    ) -> String {
        let signals = if grounded.items.is_empty() {
            format!("no recent signals from {}", grounded.source)
        } else {
            grounded.items.join("; ")
        };
        format!(
            "FIT: {} ({} / 100) — {name}\n  measured: {}\n  meaning: tradition reads {}.\n  GROUNDED ({}): {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            self.measured(measures),
            self.meaning(fit),
            grounded.source,
            signals,
        )
    }

    fn report_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "REPORT: {} ({} / 100) — {name}\n  measured: {}\n  meaning: tradition reads {}.\n  confidence {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            self.measured(measures),
            self.meaning(fit),
            measures.confidence.label(),
        )
    }

    fn verdict_line(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "VERDICT: {} ({}/100) — {name}. confidence {}. Tradition reads {}. Not financial advice.",
            fit.label(),
            measures.score,
            measures.confidence.label(),
            self.meaning(fit),
        )
    }

    fn daily_beat(&self, beat: Option<&TransitBeat>, date: NaiveDate) -> String {
        daily_beat_template(beat, date)
    }
}

/// One beat: the measured transit + a short traditional tone + the guardrail. Never advises. A
/// single line (not the 3-line FIT/REPORT block); `None` renders the quiet-sky beat.
fn daily_beat_template(beat: Option<&TransitBeat>, date: NaiveDate) -> String {
    match beat {
        Some(b) => format!(
            "TODAY ({date}): transiting {} {} your natal {} (orb {:.1}°, {}) — {}. {REMINDER}",
            b.transiting,
            b.aspect.to_lowercase(),
            b.natal,
            b.orb,
            if b.harmonious { "flowing" } else { "friction" },
            if b.harmonious {
                "a day that leans toward ease"
            } else {
                "a day with friction to work with, not against"
            },
        ),
        None => format!(
            "TODAY ({date}): no close transit to your natal chart — a quiet sky to steer yourself. {REMINDER}"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AspectHit, Confidence};

    fn measures() -> Measures {
        let hit = AspectHit {
            body_a: "Sun".into(),
            body_b: "Moon".into(),
            aspect: "Trine".into(),
            orb: 1.2,
            harmonious: true,
            weight: 7.0,
        };
        Measures {
            choice: "AAPL".into(),
            aspects: vec![hit.clone()],
            score: 72,
            top: vec![hit],
            theme: None,
            patterns: vec![],
            confidence: Confidence::High,
        }
    }

    #[test]
    fn report_and_verdict_carry_guardrail_and_confidence() {
        let interp = TemplateInterpreter;
        let m = measures();
        let report = interp.report_read(&m, Fit::Aligned, "Apple");
        let verdict = interp.verdict_line(&m, Fit::Aligned, "Apple");

        for text in [&report, &verdict] {
            assert!(
                text.to_lowercase().contains("not financial advice"),
                "missing guardrail: {text}"
            );
            assert!(
                text.contains("confidence"),
                "missing confidence beat: {text}"
            );
            assert!(text.contains("High"), "missing confidence word: {text}");
        }
        // The verdict must end on the guardrail.
        assert!(verdict.trim_end().ends_with("Not financial advice."));
    }

    /// A distinctive interpreter whose overrides return sentinels absent from the default bodies.
    struct Marker;
    impl Interpreter for Marker {
        fn fit_read(&self, _m: &Measures, _f: Fit, _n: &str) -> String {
            "MARKER_FIT".into()
        }
        fn grounded_brief(&self, _m: &Measures, _f: Fit, _n: &str, _g: &GroundedSignals) -> String {
            "MARKER_GROUNDED".into()
        }
        fn report_read(&self, _m: &Measures, _f: Fit, _n: &str) -> String {
            "MARKER_REPORT".into()
        }
        fn verdict_line(&self, _m: &Measures, _f: Fit, _n: &str) -> String {
            "MARKER_VERDICT".into()
        }
    }

    #[test]
    fn boxed_interpreter_forwards_overrides_not_defaults() {
        // Trap #2: without explicit forwarding in `impl Interpreter for Box<dyn Interpreter>`,
        // these would fall through to the trait defaults and lose the override.
        let boxed: Box<dyn Interpreter> = Box::new(Marker);
        let m = measures();
        assert_eq!(
            boxed.report_read(&m, Fit::Aligned, "Apple"),
            "MARKER_REPORT"
        );
        assert_eq!(
            boxed.verdict_line(&m, Fit::Aligned, "Apple"),
            "MARKER_VERDICT"
        );
    }
}
