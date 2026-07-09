//! The loop — observe → decide → act — with the human-in-the-loop checkpoint and the
//! no-advice guardrail. This is the graded artifact (PRD §16, §18): the grounded pull is
//! unreachable without an approval token, and advice-seeking questions are refused in code.

use crate::grounded::GroundedSource;
use crate::interpret::Interpreter;
use crate::measure::ChartSource;
use crate::score::synastry_score;
use crate::types::{
    BirthMoment, Briefing, Choice, Fit, GateError, GroundedSignals, Measures, Recommendation,
    ToolCall,
};

/// One run of the loop. Holds the tool sources and records the tool-call order (the eval basis).
pub struct Session<C: ChartSource, G: GroundedSource, I: Interpreter> {
    chart: C,
    grounded: G,
    interp: I,
    calls: Vec<ToolCall>,
}

/// Proof that a human approved grounding a specific choice. It can be minted **only** by
/// [`Session::approve`] (its field is private and there is no public constructor), so
/// [`Session::pull_grounded`] — the costed external call — is unreachable without one.
pub struct ApprovalToken {
    choice: String,
}

/// The checkpoint prompt shown to the human before the grounded pull.
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
            calls: Vec::new(),
        }
    }

    /// The ordered tool calls made so far — the CI tool-order eval reads this.
    pub fn calls(&self) -> &[ToolCall] {
        &self.calls
    }

    /// Hamun-ana: measure a choice against the seeker in the fixed order
    /// `get_chart(you) → get_chart(choice) → get_synastry`.
    pub fn measure(&mut self, seeker: &BirthMoment, choice: &Choice) -> Measures {
        self.calls.push(ToolCall::GetChart("you".to_string()));
        let a = self.chart.chart(seeker);
        self.calls.push(ToolCall::GetChart(choice.ticker.clone()));
        let b = self.chart.chart(&choice.birth);
        self.calls.push(ToolCall::GetSynastry(
            "you".to_string(),
            choice.ticker.clone(),
        ));
        let aspects = self.chart.synastry(&a, &b);
        let score = synastry_score(&aspects);
        let top = aspects.iter().take(4).cloned().collect();
        Measures {
            choice: choice.ticker.clone(),
            aspects,
            score,
            top,
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
        let measures = Measures {
            choice: choice.ticker.clone(),
            aspects,
            score,
            top,
        };
        let fit = Fit::from_score(score);
        Briefing {
            reading: self
                .interp
                .grounded_brief(&measures, fit, &choice.name, grounded),
        }
    }
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
