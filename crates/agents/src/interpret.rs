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

/// The tradition-side meaning of a fit band — shared by the default trait bodies. Committed, not
/// hedged: each band names what the contacts actually do.
fn meaning_clause(fit: Fit) -> &'static str {
    match fit {
        Fit::StronglyAligned => {
            "a deep, unmistakable resonance — the two charts recognize each other on sight"
        }
        Fit::Aligned => "clearly more ease than friction; the flow carries the day",
        Fit::Mixed => "a real, unresolved split — genuine ease braided through genuine tension",
        Fit::Misaligned => "friction that outweighs the flow, plainly",
    }
}

/// A short confidence beat naming the level — always carries a confidence word.
fn confidence_beat(confidence: Confidence) -> String {
    format!("confidence {}", confidence.label())
}

/// One warm, plain-English sentence per fit band, carrying a **staked** verdict on the fit — the
/// conviction of the old `verdict:` line, in language a normal person gets, with no jargon. This is
/// the body of the reading a seeker actually reads; the raw aspects live in the Backstage panel.
fn warm_prose(fit: Fit) -> &'static str {
    match fit {
        Fit::StronglyAligned => {
            "This one sings — an easy, unmistakable pull, the kind of fit you feel before you can \
             explain it. I'd stake the read right here: you two move together."
        }
        Fit::Aligned => {
            "This one flows — more ease than friction, a fit that carries its own weight. I'd \
             stake the read on it: the current runs with you, not against you."
        }
        Fit::Mixed => {
            "This one is genuinely split — real ease braided through real strain, and neither side \
             wins outright. I'd stake the read on exactly that: it pulls both ways at once."
        }
        Fit::Misaligned => {
            "There's a real grind here — more friction than flow, effort without much ease. I'd \
             stake the read on it: this one asks more of you than it gives back."
        }
    }
}

/// Translate one raw contact into a plain-English *dynamic* — human words for the two bodies, and
/// whether they meet with ease or friction. No aspect names, no orbs, no degrees ever leave this
/// helper; that technical detail belongs only in the Backstage panel.
fn plain_dynamic(body_a: &str, body_b: &str, harmonious: bool) -> String {
    let a = body_word(body_a);
    let b = body_word(body_b);
    if harmonious {
        format!("an easy flow between {a} and {b}")
    } else {
        format!("a tension between {a} and {b}")
    }
}

/// Map a chart body to the human thing it stands for — plain words a normal person gets. Unknown
/// bodies fall back to their own name, so nothing is ever invented.
fn body_word(body: &str) -> String {
    match body {
        "Sun" => "who you are",
        "Moon" => "how you feel",
        "Mars" => "your drive",
        "Venus" => "what you value",
        "Saturn" => "caution and limits",
        "Jupiter" => "an appetite for more",
        "Mercury" => "how you think",
        "Pluto" => "pressure and intensity",
        other => return other.to_string(),
    }
    .to_string()
}

/// The one plain "why:" line — the single tightest contact (`top[0]`) translated into a human
/// dynamic. Names the dominant thread without a single aspect name, orb, or degree.
fn why_line(measures: &Measures) -> String {
    match measures.top.first() {
        Some(a) => format!(
            "why: the strongest thread is {}, {}",
            if a.harmonious {
                "an easy one"
            } else {
                "a tense one"
            },
            plain_dynamic(&a.body_a, &a.body_b, a.harmonious),
        ),
        None => "why: the two charts barely touch — no single thread stands out".to_string(),
    }
}

/// One neutral, hedged "this is what reality says:" sentence for the grounded beat. The deterministic
/// template cannot read the *content* of the filings, so it must never characterize them (no "lean
/// steady", no direction, no price) — that would be an invented claim that could contradict the real
/// `GROUNDED (...)` items printed right above it. Instead it points at the actual record as the thing
/// to weigh. (The live model, which does read the signal text, writes a substantive reality sentence.)
fn reality_sentence(grounded: &GroundedSignals) -> &'static str {
    if grounded.items.is_empty() {
        "this is what reality says: the record is quiet right now — nothing on file to set beside the chart"
    } else {
        "this is what reality says: the filings above are the actual record — weigh those, not the sky, for the numbers"
    }
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
            Fit::StronglyAligned => {
                "a deep, unmistakable resonance — the two charts recognize each other on sight"
            }
            Fit::Aligned => "clearly more ease than friction; the flow carries the day",
            Fit::Mixed => "a real, unresolved split — genuine ease braided through genuine tension",
            Fit::Misaligned => "friction that outweighs the flow, plainly",
        }
    }
}

impl Interpreter for TemplateInterpreter {
    fn fit_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        // Warm, plain prose a normal person gets — a staked verdict on the fit, plus one plain
        // "why:" line. The raw aspects/orbs/degrees stay out of the reading (they live in Backstage).
        format!(
            "FIT: {} ({} / 100) — {name}\n{}\n  {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            warm_prose(fit),
            why_line(measures),
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
            "FIT: {} ({} / 100) — {name}\n{}\n  {}\n  GROUNDED ({}): {}\n  {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            warm_prose(fit),
            why_line(measures),
            grounded.source,
            signals,
            reality_sentence(grounded),
        )
    }

    fn report_read(&self, measures: &Measures, fit: Fit, name: &str) -> String {
        format!(
            "REPORT: {} ({} / 100) — {name}\n  measured: {}\n  meaning: tradition reads {}.\n  verdict: the ledger stakes this at {}.\n  confidence {}\n  {REMINDER}",
            fit.label(),
            measures.score,
            self.measured(measures),
            self.meaning(fit),
            fit.label(),
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

    #[test]
    fn grounded_brief_states_reality_beside_the_read() {
        let interp = TemplateInterpreter;
        let m = measures();

        // With signals present → a neutral reality sentence that points at the real record WITHOUT
        // characterizing the filings (the template can't read them, so it must not invent "steady"
        // etc. — that could contradict the GROUNDED items above). Guardrail + GROUNDED intact.
        let with_signals = interp.grounded_brief(
            &m,
            Fit::Aligned,
            "Apple",
            &GroundedSignals {
                choice: "AAPL".into(),
                source: "SEC EDGAR".into(),
                items: vec!["10-Q filed 2024-05-02".into()],
            },
        );
        assert!(
            with_signals.contains("this is what reality says:"),
            "missing reality framing: {with_signals}"
        );
        assert!(
            with_signals.contains("the actual record"),
            "reality sentence should point at the record, not characterize it: {with_signals}"
        );
        // It must NOT invent an un-grounded characterization of the numbers.
        assert!(
            !with_signals.contains("lean steady") && !with_signals.contains("nothing dramatic"),
            "template must not invent a filings characterization: {with_signals}"
        );
        assert!(
            with_signals.contains("GROUNDED (SEC EDGAR)"),
            "{with_signals}"
        );
        assert!(
            with_signals.to_lowercase().contains("not financial advice"),
            "missing guardrail: {with_signals}"
        );

        // With no signals → the quiet-record reality sentence.
        let empty = interp.grounded_brief(
            &m,
            Fit::Aligned,
            "Apple",
            &GroundedSignals {
                choice: "AAPL".into(),
                source: "SEC EDGAR".into(),
                items: vec![],
            },
        );
        assert!(
            empty.contains("this is what reality says: the record is quiet right now"),
            "missing quiet-record reality sentence: {empty}"
        );
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
