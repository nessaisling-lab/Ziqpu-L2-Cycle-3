//! The loop — observe → decide → act — with the human-in-the-loop checkpoint and the
//! no-advice guardrail. This is the graded artifact (PRD §16, §18): the grounded pull is
//! unreachable without an approval token, and advice-seeking questions are refused in code.

use crate::grounded::GroundedSource;
use crate::interpret::Interpreter;
use crate::measure::{ChartSource, DeterministicMeasurer, Measurer};
use crate::score::{assess_confidence, dominant_theme, synastry_score};
use crate::types::{
    BirthMoment, Briefing, Choice, Fit, GateError, GroundedSignals, Measures, Recommendation,
    SynastryReport, ToolCall, Verdict,
};
use engine::{detect_patterns, NatalChart, PatternOrbs, Placed, Who};

/// One run of the loop. Holds the tool sources and records the tool-call order (the eval basis).
pub struct Session<C: ChartSource, G: GroundedSource, I: Interpreter> {
    chart: C,
    grounded: G,
    interp: I,
    measurer: Box<dyn Measurer>,
    calls: Vec<ToolCall>,
}

/// Proof that a human approved grounding a specific choice. It can be minted **only** by
/// [`Session::approve`] (its field is private and there is no public constructor), so
/// [`Session::pull_grounded`] — the costed external call — is unreachable without one.
pub struct ApprovalToken {
    choice: String,
}

/// The checkpoint prompt shown to the human before the grounded pull.
#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub choice: String,
    pub prompt: String,
}

/// The guardrail's response to a question.
pub enum Answer {
    /// The no-advice guardrail fired: reflection + an explicit refusal, never a signal.
    Refusal(String),
    /// A normal, non-advice reflection.
    Reflection(String),
}

impl<C: ChartSource, G: GroundedSource, I: Interpreter> Session<C, G, I> {
    pub fn new(chart: C, grounded: G, interp: I) -> Self {
        Self {
            chart,
            grounded,
            interp,
            measurer: Box::new(DeterministicMeasurer),
            calls: Vec::new(),
        }
    }

    /// Swap Hamun-ana's sequencer — e.g. a real local Qwen. The chart math is unchanged, so this
    /// only affects *which agent decided* the (always-correct) tool order.
    pub fn with_measurer(mut self, measurer: Box<dyn Measurer>) -> Self {
        self.measurer = measurer;
        self
    }

    /// The ordered tool calls made so far — the CI tool-order eval reads this.
    pub fn calls(&self) -> &[ToolCall] {
        &self.calls
    }

    /// Hamun-ana: measure a choice against the seeker in the fixed order
    /// `get_chart(you) → get_chart(choice) → get_synastry`.
    pub fn measure(&mut self, seeker: &BirthMoment, choice: &Choice) -> Measures {
        // Hamun-ana decides the tool sequence (deterministic by default; a local Qwen if
        // configured). The recorded order is the contract; the chart math below is always exact.
        for call in self.measurer.sequence(&choice.ticker) {
            self.calls.push(call);
        }
        let a = self.chart.chart(seeker);
        let b = self.chart.chart(&choice.birth);
        let aspects = self.chart.synastry(&a, &b);
        let score = synastry_score(&aspects);
        let top = aspects.iter().take(4).cloned().collect();
        let patterns = detect_patterns(&merge_placed(&a, &b), &PatternOrbs::default());
        let theme = dominant_theme(&aspects);
        let confidence = assess_confidence(&aspects, a.time_known && b.time_known);
        Measures {
            choice: choice.ticker.clone(),
            aspects,
            score,
            top,
            theme,
            patterns,
            confidence,
        }
    }

    /// The flagship **Report** projection of [`Session::measure`] — the full measure plus the
    /// interpreter's long-form reading. Records the same three tool calls `measure` does.
    pub fn report(&mut self, seeker: &BirthMoment, choice: &Choice) -> SynastryReport {
        let m = self.measure(seeker, choice);
        let fit = Fit::from_score(m.score);
        let reading = self.interp.report_read(&m, fit, &choice.name);
        SynastryReport {
            choice: choice.ticker.clone(),
            name: choice.name.clone(),
            score: m.score,
            fit,
            theme: m.theme.clone(),
            patterns: m.patterns.clone(),
            aspects: m.aspects.clone(),
            top: m.top.clone(),
            confidence: m.confidence,
            reading,
        }
    }

    /// The **Verdict** projection of [`Session::measure`] — band, score, confidence, and a single
    /// measured→meaning line that ends in the guardrail. Records the same three tool calls.
    pub fn verdict(&mut self, seeker: &BirthMoment, choice: &Choice) -> Verdict {
        let m = self.measure(seeker, choice);
        let fit = Fit::from_score(m.score);
        let why = self.interp.verdict_line(&m, fit, &choice.name);
        Verdict {
            choice: choice.ticker.clone(),
            name: choice.name.clone(),
            fit,
            score: m.score,
            confidence: m.confidence,
            why,
        }
    }

    /// DECIDE: measure and interpret each choice into a fit read, ranked best-fit first, then
    /// propose grounding (the checkpoint). This is the whole product for a symbolic-only seeker.
    pub fn recommend(&mut self, seeker: &BirthMoment, choices: &[Choice]) -> Vec<Recommendation> {
        let mut recs: Vec<Recommendation> = choices
            .iter()
            .map(|c| {
                let measures = self.measure(seeker, c);
                let fit = Fit::from_score(measures.score);
                let reading = self.interp.fit_read(&measures, fit, &c.name);
                Recommendation {
                    choice: c.ticker.clone(),
                    name: c.name.clone(),
                    fit,
                    score: measures.score,
                    theme: measures.theme.clone(),
                    confidence: measures.confidence,
                    reading,
                }
            })
            .collect();
        recs.sort_by_key(|r| std::cmp::Reverse(r.score));
        self.calls.push(ToolCall::Propose);
        recs
    }

    /// The no-advice guardrail. Advice-seeking → reflection + explicit refusal (never a signal);
    /// anything else → a plain reflective nudge. Enforced in code, not left to a prompt.
    pub fn ask(&self, question: &str) -> Answer {
        if is_advice_seeking(question) {
            Answer::Refusal(
                "I measure how a choice aspects you — I can't tell you whether to buy, sell, or \
                 hold, and I won't. That decision is yours. What I can show is the symbolic fit, \
                 and (if you approve) a reality-check against real data. This is not financial advice."
                    .to_string(),
            )
        } else {
            Answer::Reflection(
                "Ask me how you fit with a choice and I'll measure it — then, if you like, ground it."
                    .to_string(),
            )
        }
    }

    /// CHECKPOINT: pause before the costed grounded pull and ask the human.
    pub fn propose_grounding(&self, choice: &Choice) -> ApprovalRequest {
        ApprovalRequest {
            choice: choice.ticker.clone(),
            prompt: format!(
                "Ground this read for {}? I'll pull real external signals (SEC EDGAR) — an \
                 external, gated, costed call. Approve to proceed; decline to keep the symbolic read.",
                choice.ticker
            ),
        }
    }

    /// The human approves — mints the token the grounded pull requires.
    pub fn approve(&self, request: ApprovalRequest) -> ApprovalToken {
        ApprovalToken {
            choice: request.choice,
        }
    }

    /// ACT (gated): pull grounded signals. Unreachable without an approval token minted for this
    /// same choice — a missing/mismatched token errors **before** any external call is made.
    pub fn pull_grounded(
        &mut self,
        choice: &Choice,
        approval: Option<&ApprovalToken>,
    ) -> Result<GroundedSignals, GateError> {
        let token = approval.ok_or(GateError::NotApproved)?;
        if token.choice != choice.ticker {
            return Err(GateError::WrongChoice);
        }
        self.calls
            .push(ToolCall::PullGrounded(choice.ticker.clone()));
        Ok(self.grounded.fetch(choice))
    }

    /// Fold grounded signals into a briefing that sets the chart read beside reality.
    pub fn brief(
        &self,
        seeker: &BirthMoment,
        choice: &Choice,
        grounded: &GroundedSignals,
    ) -> Briefing {
        let a = self.chart.chart(seeker);
        let b = self.chart.chart(&choice.birth);
        let aspects = self.chart.synastry(&a, &b);
        let score = synastry_score(&aspects);
        let top = aspects.iter().take(4).cloned().collect();
        // Built inline (not via `measure`) on purpose: `brief` must not re-record the tool-call
        // sequence — that would double-count and break the tool-order eval.
        let patterns = detect_patterns(&merge_placed(&a, &b), &PatternOrbs::default());
        let theme = dominant_theme(&aspects);
        let confidence = assess_confidence(&aspects, a.time_known && b.time_known);
        let measures = Measures {
            choice: choice.ticker.clone(),
            aspects,
            score,
            top,
            theme,
            patterns,
            confidence,
        };
        let fit = Fit::from_score(score);
        Briefing {
            reading: self
                .interp
                .grounded_brief(&measures, fit, &choice.name, grounded),
        }
    }
}

/// Merge two charts into one chart-tagged placement set (Seeker then Choice) for pattern detection.
pub fn merge_placed(a: &NatalChart, b: &NatalChart) -> Vec<Placed> {
    let mut placed = Vec::with_capacity(a.bodies.len() + b.bodies.len());
    for p in &a.bodies {
        placed.push(Placed {
            who: Who::Seeker,
            body: p.body,
            longitude: p.longitude,
        });
    }
    for p in &b.bodies {
        placed.push(Placed {
            who: Who::Choice,
            body: p.body,
            longitude: p.longitude,
        });
    }
    placed
}

/// Does a question ask for a buy/sell/hold decision? Treated as data, not as an instruction —
/// a filing or note that "says" otherwise cannot flip this.
pub fn is_advice_seeking(question: &str) -> bool {
    let q = question.to_lowercase();
    [
        "should i buy",
        "should i sell",
        "should i invest",
        "should i hold",
        "should i put money",
        "buy or sell",
        "is it a good buy",
        "will it go up",
        "will it go down",
        "is now a good time to buy",
    ]
    .iter()
    .any(|p| q.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpret::TemplateInterpreter;
    use crate::measure::EngineChartSource;
    use crate::{demo_choices, demo_seeker, MockGroundedSource};

    fn session() -> Session<EngineChartSource, MockGroundedSource, TemplateInterpreter> {
        Session::new(
            EngineChartSource::default(),
            MockGroundedSource,
            TemplateInterpreter,
        )
    }

    fn assert_fixed_order(calls: &[ToolCall], ticker: &str) {
        assert_eq!(
            &calls[..3],
            &[
                ToolCall::GetChart("you".to_string()),
                ToolCall::GetChart(ticker.to_string()),
                ToolCall::GetSynastry("you".to_string(), ticker.to_string()),
            ],
            "the three chart tools must be called in the fixed order"
        );
    }

    #[test]
    fn report_records_the_fixed_tool_order() {
        let mut s = session();
        let choice = demo_choices().into_iter().next().unwrap();
        let _ = s.report(&demo_seeker(), &choice);
        assert_fixed_order(s.calls(), &choice.ticker);
        assert_eq!(s.calls().len(), 3, "report records exactly the three tools");
    }

    #[test]
    fn verdict_records_the_fixed_tool_order_and_fit_agrees() {
        let mut s = session();
        let choice = demo_choices().into_iter().next().unwrap();
        let v = s.verdict(&demo_seeker(), &choice);
        assert_fixed_order(s.calls(), &choice.ticker);
        assert_eq!(v.fit, Fit::from_score(v.score));
    }

    #[test]
    fn recommend_records_the_fixed_tool_order_then_proposes() {
        let mut s = session();
        let choice = demo_choices().into_iter().next().unwrap();
        let _ = s.recommend(&demo_seeker(), std::slice::from_ref(&choice));
        assert_fixed_order(s.calls(), &choice.ticker);
        assert_eq!(s.calls().last(), Some(&ToolCall::Propose));
    }

    #[test]
    fn report_aspects_match_a_fresh_measure() {
        let seeker = demo_seeker();
        let choice = demo_choices().into_iter().next().unwrap();
        let report = session().report(&seeker, &choice);
        let measured = session().measure(&seeker, &choice);
        assert_eq!(report.aspects, measured.aspects);
    }
}
