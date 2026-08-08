//! Resolve real names to their Wikidata origin lifecycle and print what actually came back.
//!
//! ```text
//! cargo run -p agents --example origin_probe                      # the standing probe set
//! cargo run -p agents --example origin_probe -- "Nintendo Switch" "Eiffel Tower"
//! ```
//!
//! # Why this exists rather than more unit tests
//!
//! `origin.rs` is tested against JSON fixtures I wrote, which proves the parsing does what I think
//! Wikidata sends — not that Wikidata sends it. Every real defect this project has found lived in
//! that gap: a fabricated citation, a rejected key, a provider substitution, an ephemeris backend
//! with no caller. All were invisible to a green suite and obvious on the first live run.
//!
//! The probe set is chosen to hit each branch on purpose, not to look good:
//!
//! - a console with **several regional releases** (the earliest-wins rule),
//! - a landmark with a **P1619 opening** (a place, not a product),
//! - an old company whose inception is **year-precision only** (must refuse to chart),
//! - a name that should **fail to resolve** (the honest-empty path).
//!
//! It is keyless and CC0, so it is safe to run anywhere and costs nothing.

use agents::origin::{resolve_origin_default, OriginGap};

/// Names picked to exercise one branch each — see the module docs.
const PROBES: [&str; 6] = [
    "PlayStation 5",           // several day-precise regional releases
    "Nintendo Switch",         // product, single release
    "Eiffel Tower",            // P1619 — a place
    "Coca-Cola",               // old company; inception likely year-precision
    "Ford Motor Company",      // old company, N3 vehicle-adjacent
    "Zzzqqq Not A Real Thing", // must fail to resolve, cleanly
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let names: Vec<&str> = if args.is_empty() {
        PROBES.to_vec()
    } else {
        args.iter().map(|s| s.as_str()).collect()
    };

    println!("Wikidata origin resolver — keyless, CC0, no key required.\n");

    let (mut chartable, mut known_but_coarse, mut empty) = (0, 0, 0);

    for name in &names {
        println!("══════════════════════════════════════════════════════════════");
        println!("{name}");
        match resolve_origin_default(name) {
            Ok(life) => {
                chartable += 1;
                println!("  resolved   {} ({})", life.label, life.qid);
                println!("  source     {}", life.source_url());
                let best = life
                    .default_moment()
                    .expect("Ok implies a chartable moment");
                println!("  → CHART    {}", best.describe());
                println!("             {}", best.property.means);
                for m in life.moments.iter().filter(|m| *m != best) {
                    println!("    also     {}", m.describe());
                }
            }
            Err(gap) => {
                match &gap {
                    OriginGap::NothingChartable(_) => known_but_coarse += 1,
                    _ => empty += 1,
                }
                println!("  no chart   {}", gap.explain());
            }
        }
        println!();
    }

    println!("══════════════════════════════════════════════════════════════");
    println!(
        "{chartable} chartable · {known_but_coarse} known but too coarse · {empty} nothing found"
    );
    println!(
        "\n\"known but too coarse\" is a SUCCESS of this design, not a miss: the entity is real, the\n\
         date is real, and it is a year rather than a day. Casting a chart from it would mean\n\
         inventing a January 1st — the exact fabrication this project deleted 904 charts to remove."
    );
}
