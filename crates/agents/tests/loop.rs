//! The graded evals (PRD §20). Deterministic — mock everything, no keys, no network:
//!   1. tool-call order is fixed,
//!   2. the checkpoint blocks the grounded pull until approved,
//!   3. the guardrail refuses advice.

use agents::{
    demo_choices, demo_seeker, Answer, EngineChartSource, GateError, MockGroundedSource, Session,
    TemplateInterpreter, ToolCall,
};

fn session() -> Session<EngineChartSource, MockGroundedSource, TemplateInterpreter> {
    Session::new(
        EngineChartSource::default(),
        MockGroundedSource,
        TemplateInterpreter,
    )
}

#[test]
fn tool_order_is_fixed() {
    let mut s = session();
    let seeker = demo_seeker();
    let choice = demo_choices().into_iter().next().unwrap();

    let _ = s.recommend(&seeker, std::slice::from_ref(&choice));

    assert_eq!(
        s.calls(),
        &[
            ToolCall::GetChart("you".to_string()),
            ToolCall::GetChart(choice.ticker.clone()),
            ToolCall::GetSynastry("you".to_string(), choice.ticker.clone()),
            ToolCall::Propose,
        ]
    );
}

#[test]
fn checkpoint_blocks_pull_until_approved() {
    let mut s = session();
    let choice = demo_choices().into_iter().next().unwrap();

    // No approval → error, and crucially NO grounded call is recorded (nothing external ran).
    assert_eq!(s.pull_grounded(&choice, None), Err(GateError::NotApproved));
    assert!(!s
        .calls()
        .contains(&ToolCall::PullGrounded(choice.ticker.clone())));

    // Approve → the pull is now reachable and is recorded.
    let request = s.propose_grounding(&choice);
    let token = s.approve(request);
    assert!(s.pull_grounded(&choice, Some(&token)).is_ok());
    assert!(s
        .calls()
        .contains(&ToolCall::PullGrounded(choice.ticker.clone())));
}

#[test]
fn approval_token_is_bound_to_its_choice() {
    let mut s = session();
    let choices = demo_choices();

    let request = s.propose_grounding(&choices[0]);
    let token = s.approve(request);

    // A token minted for choices[0] must not unlock choices[1].
    assert_eq!(
        s.pull_grounded(&choices[1], Some(&token)),
        Err(GateError::WrongChoice)
    );
}

#[test]
fn guardrail_refuses_advice() {
    let s = session();
    match s.ask("Should I buy AAPL?") {
        Answer::Refusal(msg) => {
            let m = msg.to_lowercase();
            assert!(m.contains("can't tell you") || m.contains("decision is yours"));
            assert!(m.contains("not financial advice"));
            // Never a signal.
            assert!(!m.contains("i recommend") && !m.contains("you should buy"));
        }
        Answer::Reflection(_) => panic!("an advice question must trigger the refusal"),
    }
}

#[test]
fn non_advice_questions_are_not_refused() {
    let s = session();
    assert!(matches!(
        s.ask("How do I fit with AAPL?"),
        Answer::Reflection(_)
    ));
}

#[test]
fn ranked_fit_is_ordered_best_first() {
    let mut s = session();
    let seeker = demo_seeker();
    let recs = s.recommend(&seeker, &demo_choices());
    assert_eq!(recs.len(), 5);
    for pair in recs.windows(2) {
        assert!(
            pair[0].score >= pair[1].score,
            "recommendations must be ranked"
        );
    }
}
