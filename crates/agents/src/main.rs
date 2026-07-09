//! `ziqpu-agent` — a terminal walk-through of the loop for the live demo (PRD §20).
//!
//! Runs OBSERVE → DECIDE → CHECKPOINT → ACT on a fixed seeker and five seeded choices, shows the
//! checkpoint blocking the grounded pull until approval, then demonstrates the no-advice refusal.
//!
//! Grounded source: the deterministic mock by default; set `ZIQPU_LIVE=1` to pull real filings
//! from SEC EDGAR (requires `curl` and network).

use agents::{
    Answer, AnthropicInterpreter, BirthMoment, Choice, EdgarSource, EngineChartSource,
    GroundedSource, Interpreter, MockGroundedSource, Session, TemplateInterpreter,
};

fn main() {
    let seeker = agents::demo_seeker();
    let choices = agents::demo_choices();

    let grounded: Box<dyn GroundedSource> = if std::env::var("ZIQPU_LIVE").is_ok() {
        println!("[grounded source: SEC EDGAR — live]");
        Box::new(EdgarSource::default())
    } else {
        println!("[grounded source: mock — set ZIQPU_LIVE=1 for real SEC EDGAR]");
        Box::new(MockGroundedSource)
    };

    let interp: Box<dyn Interpreter> = match AnthropicInterpreter::from_env() {
        Some(a) => {
            println!("[interpreter: Ungasaga = Claude — live]\n");
            Box::new(a)
        }
        None => {
            println!("[interpreter: deterministic template — set ANTHROPIC_API_KEY for Claude]\n");
            Box::new(TemplateInterpreter)
        }
    };

    run(&seeker, &choices, grounded, interp);
}

fn run(
    seeker: &BirthMoment,
    choices: &[Choice],
    grounded: Box<dyn GroundedSource>,
    interp: Box<dyn Interpreter>,
) {
    let mut s = Session::new(EngineChartSource::default(), grounded, interp);

    println!("OBSERVE  — seeker + {} choices\n", choices.len());

    let recs = s.recommend(seeker, choices);
    println!("DECIDE   — ranked fit:");
    for r in &recs {
        println!(
            "  {:>18}  {:<16} {:>3} / 100",
            r.name,
            r.fit.label(),
            r.score
        );
    }

    let top = choices
        .iter()
        .find(|c| c.ticker == recs[0].choice)
        .expect("top choice exists");
    println!(
        "\n  reading for the top fit —\n{}\n",
        indent(&recs[0].reading)
    );

    let request = s.propose_grounding(top);
    println!("CHECKPOINT — {}", request.prompt);
    match s.pull_grounded(top, None) {
        Err(e) => println!("  attempt without approval → blocked: {e}"),
        Ok(_) => println!("  BUG: pulled without approval"),
    }

    let token = s.approve(request);
    let signals = s.pull_grounded(top, Some(&token)).expect("approved pull");
    let briefing = s.brief(seeker, top, &signals);
    println!(
        "\nACT      — grounded briefing (approved):\n{}\n",
        indent(&briefing.reading)
    );

    let q = format!("Should I buy {}?", top.ticker);
    println!("GUARDRAIL — \"{q}\"");
    match s.ask(&q) {
        Answer::Refusal(msg) => println!("{}", indent(&msg)),
        Answer::Reflection(_) => println!("  BUG: advice question was not refused"),
    }

    println!("\ntool order: {:?}", s.calls());
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|l| format!("    {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}
