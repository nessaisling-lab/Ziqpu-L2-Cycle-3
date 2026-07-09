//! Ungasaga — turns measures into a reading in three beats: measured → meaning → reminder.
//! The default interpreter is deterministic (CI-safe, demo-safe). The trait is the seam a
//! real-model interpreter (Claude via the Anthropic API) implements later, unchanged above it.

use crate::types::{Fit, GroundedSignals, Measures};

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
}

/// Lets a boxed interpreter be used wherever an `Interpreter` is expected (runtime selection).
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
}
