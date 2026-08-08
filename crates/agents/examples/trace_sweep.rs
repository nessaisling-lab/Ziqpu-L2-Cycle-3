//! Trace the grounding layer across **every entity kind**, not just the five demo tickers.
//!
//! ```text
//! cargo run -p agents --example trace_sweep              # real sources, no model calls, free
//! ZIQPU_TRACE=full cargo run -p agents --example trace_sweep
//! ```
//!
//! # Why this exists
//!
//! Reading one trace found two defects the output never showed: a tool returning `recent filing: 4`
//! that a model rendered as "four filings", and the lunar node arriving twice so four measures were
//! really two facts. Both were **tool return-format** problems, and both were found on the *stock*
//! path — the only path anything had ever been traced on.
//!
//! There are four entity kinds and the roster differs for each. A vehicle has no filings; a mutual
//! insurer has no CIK; a scanned barcode's whole record is on the object. Each dispatches a
//! different set of workers, each worker has its own return format, and none of the others had ever
//! been read.
//!
//! # Why it makes no model calls by default
//!
//! What this sweep examines is which workers get chosen and what shape they answer in — the
//! grounding layer, which is keyless and free. Adding a model would cost money to observe something
//! this is not asking about. `ZIQPU_LIVE=1` adds the reading if you want to see prose too.

use agents::{Choice, GroundedSource};

fn main() {
    // The harness reads whatever the app would. Without this every measurement silently went to
    // whichever provider happened to be exported in the shell, which is exactly how a week of
    // tracing got done against one provider without anyone noticing.
    agents::vault::fill_env_from_vault();

    let live = std::env::var("ZIQPU_LIVE").is_ok();
    println!(
        "grounding: LIVE public sources (keyless) · model: {}\n",
        if live { "LIVE (billed)" } else { "none" }
    );

    for (label, choice) in subjects() {
        let Some(choice) = choice else {
            println!("── {label}\n   SKIPPED — no such entity in the shipped data\n");
            continue;
        };
        sweep(label, &choice);
    }

    if agents::trace::on() {
        println!(
            "\n══════════════ trace ({:?}) ══════════════\n{}",
            agents::trace::level(),
            agents::trace::dump()
        );
    }
}

/// One subject per entity kind, drawn from real shipped data wherever possible.
///
/// Deliberately **not** the five demo tickers. Those are the only entities that have ever been
/// exercised, which is exactly why they are the least likely to reveal anything.
fn subjects() -> Vec<(&'static str, Option<Choice>)> {
    vec![
        // A public filer outside the demo set — has a CIK, so the full SEC roster applies.
        (
            "STOCK (non-demo) — Alcoa",
            tickers::choice_in(tickers::Universe::Stocks, "AA"),
        ),
        // An airline dated by its FOUNDING, not its IPO. Different universe, different date
        // provenance — and the row carries no CIK, so the roster should change.
        (
            "AIRLINE — American Airlines (founded 1926)",
            tickers::choice_in(tickers::Universe::Airlines, "AAL"),
        ),
        // A mutual insurer: not publicly traded at all, so there is no CIK and no filings to fetch.
        // The interesting question is whether it degrades honestly or reaches for SEC anyway.
        (
            "INSURER (mutual, no CIK) — State Farm",
            tickers::choice_in(tickers::Universe::Insurance, "25178"),
        ),
        // A vehicle. The identifier IS the ticker, and vPIC is the only worker that can speak.
        ("VEHICLE — a real VIN", Some(vehicle())),
        // A scanned item: the record is on the object, not the internet.
        ("SCANNED ITEM — GS1 with a production date", Some(scanned())),
        // A bare name with no identifier of any kind — the widest, least-determined roster.
        ("NAMED — a medicine", Some(named("Ibuprofen"))),
    ]
}

/// A structurally valid VIN (2003 Honda Accord) — the same one the vin module's tests use.
fn vehicle() -> Choice {
    Choice {
        ticker: "1HGCM82633A004352".into(),
        name: "2003 Honda Accord".into(),
        birth: agents::demo_seeker(), // placeholder; the origin resolver supplies the real moment
        cik: None,
        wiki: None,
    }
}

/// A GS1 element string: GTIN + a production date of 2024-03-15.
fn scanned() -> Choice {
    Choice {
        ticker: "(01)00012345678905(11)240315".into(),
        name: "a scanned product".into(),
        birth: agents::demo_seeker(),
        cik: None,
        wiki: None,
    }
}

fn named(name: &str) -> Choice {
    Choice {
        ticker: name.into(),
        name: name.into(),
        birth: agents::demo_seeker(),
        cik: None,
        wiki: None,
    }
}

/// Classify, ground, and print what each worker actually returned.
fn sweep(label: &str, choice: &Choice) {
    let kind = agents::classify_entity(choice);
    println!("── {label}");
    println!("   ticker  {}", choice.ticker);
    println!("   kind    {kind:?}   cik={:?}", choice.cik);
    println!("   consent {}", agents::grounding_consent(choice));

    agents::trace::note(&format!("sweep {label} kind={kind:?}"));

    let started = std::time::Instant::now();
    let signals = agents::CompositeSource::for_entity(choice).fetch(choice);
    let ms = started.elapsed().as_millis();

    println!("   source  {}", signals.source);
    println!("   {} signal(s) in {ms}ms:", signals.items.len());
    for item in &signals.items {
        println!("     · {item}");
    }

    // The two defects found so far were both return-FORMAT problems, so flag the shapes that
    // caused them rather than trusting a human to re-notice.
    for item in &signals.items {
        if let Some(rest) = item.strip_prefix("recent filing: ") {
            let form = rest.split(' ').next().unwrap_or("");
            if form.chars().all(|c| c.is_ascii_digit()) {
                println!("     !! numeric form type {form:?} — reads as a count, not a form");
            }
        }
        if item.len() > 220 {
            println!(
                "     !! {}B — long free text, the injection surface",
                item.len()
            );
        }
    }
    if signals.items.iter().any(|i| i.contains(agents::NO_SIGNALS)) {
        println!("     (no signals — a reading here must be marked unsourced)");
    }
    println!();
}
