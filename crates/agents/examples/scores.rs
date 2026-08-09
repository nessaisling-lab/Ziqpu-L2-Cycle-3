//! Print the measured score, band and node contacts for every demo choice — **deterministic, no
//! model, no network**.
//!
//! ```text
//! cargo run -p agents --example scores
//! ```
//!
//! # Why this exists
//!
//! It is the instrument for verifying a scoring change, and it has to be run **before** the change,
//! not after. A score delta cannot be measured retroactively: implement first and you are left
//! arguing that the new numbers look plausible, which is not evidence.
//!
//! It prints the node contacts separately because that is the thing under test. Removing the
//! duplicated lunar node should move a score **only** where a node aspect is in orb — a score that
//! moves without one means something else changed, and this is what makes that visible rather than
//! inferred.

use agents::{ChartSource, EngineChartSource, Fit};

fn main() {
    let seeker = agents::demo_seeker();
    let chart = EngineChartSource::default();
    let a = chart.chart(&seeker);

    // Which engine produced these numbers. Without this line a printout is unattributable, and two
    // runs on different machines look like a scoring change when they are an ephemeris difference:
    // swapping the analytic floor for DE440 moved Apple 33 → 28 and added a Pluto contact to four
    // of the five choices. A baseline you cannot attribute is not a baseline.
    println!("engine: {}", chart.engine().badge());
    if let Some(caveat) = chart.engine().caveat() {
        println!("        {caveat}");
    }
    println!();

    println!(
        "{:<20} {:>5}  {:<17} NODE CONTACTS",
        "CHOICE", "SCORE", "BAND"
    );
    println!("{}", "─".repeat(96));

    for choice in agents::demo_choices() {
        let b = chart.chart(&choice.birth);
        let aspects = chart.synastry(&a, &b);
        let score = agents::synastry_score(&aspects);
        let band = Fit::from_score(score);

        // Every contact touching a lunar node, with its orb and polarity — the quantity under test.
        let nodes: Vec<String> = aspects
            .iter()
            .filter(|x| x.body_a.contains("Node") || x.body_b.contains("Node"))
            .map(|x| {
                format!(
                    "{} {} {} ({:.2}°, {})",
                    x.body_a,
                    x.aspect.to_lowercase(),
                    x.body_b,
                    x.orb,
                    if x.harmonious { "flow" } else { "friction" }
                )
            })
            .collect();

        println!(
            "{:<20} {score:>5}  {:<17} {}",
            choice.name,
            band.label(),
            if nodes.is_empty() {
                "(none — this score must not move)".to_string()
            } else {
                format!("{} contact(s)", nodes.len())
            }
        );
        for n in &nodes {
            println!("{:<20} {:>5}  {:<17} · {n}", "", "", "");
        }
    }

    // The count itself is part of the claim: two node bodies is the bug, one is the fix.
    let bodies: Vec<&str> = a
        .bodies
        .iter()
        .map(|b| b.body.name())
        .filter(|n| n.contains("Node"))
        .collect();
    println!("\nnode bodies charted: {} {:?}", bodies.len(), bodies);
}
