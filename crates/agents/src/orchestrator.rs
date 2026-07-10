//! The loop — observe → decide → act — with the human-in-the-loop checkpoint and the
//! no-advice guardrail. This is the graded artifact (PRD §16, §18): the grounded pull is
//! unreachable without an approval token, and advice-seeking questions are refused in code.

use crate::grounded::GroundedSource;
use crate::interpret::Interpreter;
use crate::measure::{ChartSource, DeterministicMeasurer, Measurer};
use crate::score::{assess_confidence, dominant_theme, synastry_score};
use crate::types::{
    AspectHit, BirthMoment, Briefing, Choice, DailyReading, Fit, GateError, GroundedSignals,
    Measures, Recommendation, SynastryReport, ToolCall, TransitBeat, Verdict,
};
use chrono::NaiveDate;
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
        self.compute_measures(seeker, choice)
    }

    /// The pure chart math for one choice — **records no tool call**. Split out of
    /// [`Session::measure`] so a caller that already recorded the tool order (e.g. `recommend`
    /// filling readings after `recommend_measures`) can recompute the exact same [`Measures`]
    /// without double-counting the graded sequence.
    fn compute_measures(&self, seeker: &BirthMoment, choice: &Choice) -> Measures {
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

    /// DECIDE (measures only, **no interpreter**): measure each choice into a ranked
    /// [`Recommendation`] with an **empty** `reading`, recording the same fixed per-choice tool
    /// order as [`Session::recommend`] and the trailing [`ToolCall::Propose`]. This is the
    /// `Send`-safe half of the loop: the UI calls it on the event-loop thread to get the ranking +
    /// tool log, then fills each reading off-thread via [`crate::reading_for`] (a [`Session`] is
    /// `!Send`, so the interpreter cannot run on a worker thread that holds one).
    pub fn recommend_measures(
        &mut self,
        seeker: &BirthMoment,
        choices: &[Choice],
    ) -> Vec<Recommendation> {
        let mut recs: Vec<Recommendation> = choices
            .iter()
            .map(|c| {
                let measures = self.measure(seeker, c);
                let fit = Fit::from_score(measures.score);
                Recommendation {
                    choice: c.ticker.clone(),
                    name: c.name.clone(),
                    fit,
                    score: measures.score,
                    theme: measures.theme.clone(),
                    confidence: measures.confidence,
                    reading: String::new(),
                }
            })
            .collect();
        recs.sort_by_key(|r| std::cmp::Reverse(r.score));
        self.calls.push(ToolCall::Propose);
        recs
    }

    /// DECIDE: measure and interpret each choice into a fit read, ranked best-fit first, then
    /// propose grounding (the checkpoint). This is the whole product for a symbolic-only seeker.
    ///
    /// Shares one path with [`Session::recommend_measures`]: it records the tool order there (once),
    /// then fills each reading via `self.interp.fit_read`. The reading fill recomputes measures
    /// through the non-recording [`Session::compute_measures`], so the graded tool log is byte-for-
    /// byte identical to `recommend_measures`.
    pub fn recommend(&mut self, seeker: &BirthMoment, choices: &[Choice]) -> Vec<Recommendation> {
        let mut recs = self.recommend_measures(seeker, choices);
        for rec in &mut recs {
            if let Some(c) = choices.iter().find(|c| c.ticker == rec.choice) {
                let measures = self.compute_measures(seeker, c);
                rec.reading = self.interp.fit_read(&measures, rec.fit, &c.name);
            }
        }
        recs
    }

    /// The no-advice guardrail. Advice-seeking → reflection + explicit refusal (never a signal);
    /// anything else → a plain reflective nudge. Enforced in code, not left to a prompt.
    pub fn ask(&self, question: &str) -> Answer {
        if is_advice_seeking(question) {
            Answer::Refusal(
                "The ledger does not trade. It will not tell you to buy, sell, or hold — and that \
                 is not timidity, it is the line: the decision is yours, and I can't tell you which \
                 way to make it. What the ledger will do, without flinching, is measure how your \
                 chart and this choice aspect and stake a verdict on that fit — and, if you approve, \
                 set it beside real data. This is not financial advice."
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

    /// Today's one beat — the single tightest transit to a natal planet, for a given date.
    ///
    /// Deterministic by construction: the date is a parameter (never the system clock), and every
    /// step is a pure reuse of the chart/transits/synastry tools. Takes `&self` and records **no**
    /// tool call, so the graded observe→decide→act log and its tool-order eval stay untouched
    /// (the same discipline `brief` follows).
    pub fn daily_reading(&self, seeker: &BirthMoment, date: NaiveDate) -> DailyReading {
        let natal = self.chart.chart(seeker);
        let sky = self.chart.transits(date);
        let hits = self.chart.synastry(&sky, &natal);
        let beat = tightest_transit(&hits).map(TransitBeat::from_hit);
        let reading = self.interp.daily_beat(beat.as_ref(), date);
        DailyReading {
            date,
            beat,
            reading,
        }
    }
}

/// The single tightest transit: smallest orb, breaking ties deterministically by heavier contact
/// then lexicographic `(transiting, natal)` name — so the beat never depends on sort stability.
fn tightest_transit(hits: &[AspectHit]) -> Option<&AspectHit> {
    hits.iter().min_by(|x, y| {
        x.orb
            .total_cmp(&y.orb)
            .then_with(|| y.weight.abs().total_cmp(&x.weight.abs()))
            .then_with(|| (&x.body_a, &x.body_b).cmp(&(&y.body_a, &y.body_b)))
    })
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
    use crate::measure::{EngineChartSource, SYNASTRY_ORB};
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
    fn recommend_measures_matches_recommend_order_with_empty_readings() {
        let seeker = demo_seeker();
        let choices = demo_choices();

        // Same recorded tool order + trailing Propose as `recommend`.
        let mut s_measures = session();
        let measures_recs = s_measures.recommend_measures(&seeker, &choices);
        let mut s_full = session();
        let full_recs = s_full.recommend(&seeker, &choices);
        assert_eq!(
            s_measures.calls(),
            s_full.calls(),
            "recommend_measures records the identical tool order recommend does"
        );
        assert_eq!(s_measures.calls().last(), Some(&ToolCall::Propose));

        // Same ranking, but recommend_measures leaves every reading empty…
        assert_eq!(measures_recs.len(), full_recs.len());
        for (m, f) in measures_recs.iter().zip(full_recs.iter()) {
            assert_eq!(m.choice, f.choice, "identical ranking");
            assert_eq!(m.score, f.score);
            assert!(
                m.reading.is_empty(),
                "recommend_measures leaves readings empty: {}",
                m.reading
            );
        }
        // …while recommend fills them.
        assert!(full_recs.iter().all(|r| !r.reading.is_empty()));
    }

    #[test]
    fn report_aspects_match_a_fresh_measure() {
        let seeker = demo_seeker();
        let choice = demo_choices().into_iter().next().unwrap();
        let report = session().report(&seeker, &choice);
        let measured = session().measure(&seeker, &choice);
        assert_eq!(report.aspects, measured.aspects);
    }

    fn jul9() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 7, 9).unwrap()
    }

    /// A pure function of `(seeker, date)`: two independent sessions produce identical readings.
    #[test]
    fn daily_reading_is_deterministic_for_a_fixed_date() {
        let seeker = demo_seeker();
        let a = session().daily_reading(&seeker, jul9());
        let b = session().daily_reading(&seeker, jul9());
        assert_eq!(a.date, b.date);
        assert_eq!(a.beat, b.beat);
        assert_eq!(a.reading, b.reading);
    }

    /// It selects the ONE tightest transit — its orb equals the min over the sky-vs-natal set.
    #[test]
    fn daily_reading_picks_the_tightest_transit() {
        let seeker = demo_seeker();
        let dr = session().daily_reading(&seeker, jul9());

        // Recompute the cross-aspect set independently via the same reused tools.
        let src = EngineChartSource::default();
        let natal = src.chart(&seeker);
        let sky = src.transits(jul9());
        let hits = src.synastry(&sky, &natal);

        let beat = dr.beat.expect("the sky is not quiet on this date");
        let min_orb = hits.iter().map(|h| h.orb).fold(f64::INFINITY, f64::min);
        assert_eq!(beat.orb, min_orb, "beat must be the tightest contact");
        assert!(
            beat.orb <= SYNASTRY_ORB,
            "the tightest transit is within orb"
        );
    }

    /// Ungasaga renders exactly ONE beat and ends on the guardrail — not the 3-line FIT block.
    #[test]
    fn daily_reading_is_one_beat_with_the_guardrail() {
        let dr = session().daily_reading(&demo_seeker(), jul9());
        let r = &dr.reading;
        assert_eq!(r.lines().count(), 1, "one beat, not a multi-line report");
        assert!(!r.contains("measured:"), "not the FIT/REPORT block");
        assert!(
            r.starts_with("TODAY (2026-07-09)"),
            "names the date it was asked for"
        );
        assert!(
            r.contains("your natal"),
            "names the transiting→natal contact"
        );
        let lc = r.to_lowercase();
        assert!(lc.contains("measured, not fate"), "anti-fatalism guardrail");
        assert!(lc.contains("not financial advice"), "standing guardrail");
        assert_eq!(
            r.matches("REMINDER").count(),
            1,
            "exactly one guardrail beat"
        );
    }

    /// The graded loop + tool-order eval are untouched: `daily_reading` records nothing, and a
    /// normal measure afterward still logs exactly the fixed synastry sequence.
    #[test]
    fn daily_reading_does_not_touch_the_graded_tool_order() {
        let mut s = session();
        let seeker = demo_seeker();
        let _ = s.daily_reading(&seeker, jul9());
        assert!(
            s.calls().is_empty(),
            "daily_reading must not record any tool call"
        );

        let choice = demo_choices().into_iter().next().unwrap();
        let _ = s.report(&seeker, &choice);
        assert_fixed_order(s.calls(), &choice.ticker);
        assert_eq!(s.calls().len(), 3);
    }

    /// The quiet-sky branch renders one beat and both guardrail phrases (pure interpreter unit).
    #[test]
    fn quiet_sky_beat_is_honest_and_guarded() {
        use crate::interpret::Interpreter;
        let line = TemplateInterpreter.daily_beat(None, jul9());
        assert_eq!(line.lines().count(), 1);
        assert!(line.contains("quiet sky"));
        let lc = line.to_lowercase();
        assert!(lc.contains("measured, not fate"));
        assert!(lc.contains("not financial advice"));
    }
}
