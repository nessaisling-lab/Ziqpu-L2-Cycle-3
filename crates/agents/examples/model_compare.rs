//! Run **one** grounded reading through several writer models and print them side by side, with the
//! mechanical criteria checked for each.
//!
//! ```text
//! cargo run -p agents --example model_compare                       # dry: which models would run
//! ZIQPU_LIVE=1 cargo run -p agents --example model_compare
//! ZIQPU_LIVE=1 cargo run -p agents --example model_compare -- anthropic:claude-opus-4-8 openrouter:nvidia/nemotron-3-super-120b-a12b:free
//! ```
//!
//! # The design constraint that makes this a comparison
//!
//! The grounded signals are fetched **once** and replayed through every writer. Running the whole
//! pipeline per model would give each one its own SEC pull — different fetch time, possibly a
//! different set of filings — and then no difference in the output is attributable to the model. One
//! fetch, N writers, same measures: the model is the only thing that moves.
//!
//! It also means the network cost is one gated pull no matter how many models are compared, and the
//! model cost is exactly one reading each.
//!
//! # Why it reports the model that answered
//!
//! Failover is on by design: a provider that 429s or 5xxs falls through to the other. So "I asked
//! for Anthropic" and "Anthropic wrote it" are different claims, and only the second one is worth
//! putting in a comparison table. [`LayeredBrief::source`] carries the id that actually produced the
//! prose, and that is what prints in the `WROTE IT` column — with a `⚠` when it isn't what was
//! requested. A harness that printed the requested id would attribute one model's writing to
//! another, which is the same defect [`GroundedRung::Degraded`] exists to prevent one layer down.
//!
//! # What it grades, and what it leaves to a human
//!
//! Only checks that are mechanically decidable: the band the prose claims against the band the
//! measures computed, the reminder, the citation, advice-shaped language, and every money figure in
//! the prose against the figures actually in the signals. Quality of the writing is a human call —
//! this prints the readings in full so that call can be made against the same input.

use agents::{
    demo_choices, demo_seeker, grounded_layered, Choice, EngineChartSource, Fit, GroundedSignals,
    GroundedSource, Interpreter, Measures, ReadMode, Session,
};

/// One writer to compare: what to ask for, and how to ask for it.
struct Contender {
    /// `provider:model`, as typed on the command line.
    label: String,
    /// `ZIQPU_PROVIDER`.
    provider: String,
    /// The model id, in that provider's own dialect.
    model: String,
}

impl Contender {
    /// Parse `provider:model`. The model half may itself contain colons (`…-120b-a12b:free`), so
    /// this splits on the **first** colon only — splitting on the last silently mangled the free
    /// OpenRouter ids, which are exactly the ones worth comparing against the paid default.
    fn parse(spec: &str) -> Option<Self> {
        let (provider, model) = spec.split_once(':')?;
        (!provider.is_empty() && !model.is_empty()).then(|| Self {
            label: spec.to_string(),
            provider: provider.to_string(),
            model: model.to_string(),
        })
    }

    /// Point the process at this writer. Both provider-scoped variables are set every time — an id
    /// left over from the previous contender is worse than no id at all, because it looks like a
    /// deliberate choice.
    fn select(&self) {
        std::env::set_var("ZIQPU_PROVIDER", &self.provider);
        std::env::remove_var("ZIQPU_MODEL");
        match self.provider.as_str() {
            "anthropic" => {
                std::env::set_var("ZIQPU_ANTHROPIC_MODEL", &self.model);
                std::env::remove_var("ZIQPU_OPENROUTER_MODEL");
            }
            _ => {
                std::env::set_var("ZIQPU_OPENROUTER_MODEL", &self.model);
                std::env::remove_var("ZIQPU_ANTHROPIC_MODEL");
            }
        }
    }
}

/// The default field: the paid default, the free tier, and whatever is serving locally.
///
/// These three are the product question, not an arbitrary sample — the app offers all three, and
/// "does the free path hold up" is the one that decides whether the free path can be recommended.
const DEFAULT_FIELD: [&str; 2] = [
    "anthropic:claude-opus-4-8",
    "openrouter:nvidia/nemotron-3-super-120b-a12b:free",
];

fn main() {
    agents::prefs::load_saved_configuration();

    let specs: Vec<String> = std::env::args().skip(1).collect();
    let field: Vec<Contender> = if specs.is_empty() {
        DEFAULT_FIELD
            .iter()
            .filter_map(|s| Contender::parse(s))
            .collect()
    } else {
        specs.iter().filter_map(|s| Contender::parse(s)).collect()
    };
    if field.is_empty() {
        eprintln!(
            "nothing to compare — pass specs as provider:model, e.g. anthropic:claude-opus-4-8"
        );
        std::process::exit(2);
    }

    let live = std::env::var("ZIQPU_LIVE").is_ok();
    println!("model comparison — one reading each, same measures, same signals\n");
    for c in &field {
        println!("  · {}", c.label);
    }
    if !live {
        println!(
            "\nDRY RUN — set ZIQPU_LIVE=1 to fetch signals and bill {} model calls.",
            field.len()
        );
        return;
    }

    // ── one measure, one gated pull, reused by every contender ────────────────────────────────
    let choice = tesla();
    let seeker = demo_seeker();
    let mut session = Session::new(
        EngineChartSource::default(),
        Box::new(agents::CompositeSource::live_default()) as Box<dyn GroundedSource>,
        Box::new(agents::TemplateInterpreter) as Box<dyn Interpreter>,
    );
    let measures = session.measure(&seeker, &choice);
    let fit = Fit::from_score(measures.score);

    let request = session.propose_grounding(&choice);
    let token = session.approve(request);
    let signals = session
        .pull_grounded(&choice, Some(&token))
        .expect("approved pull");

    println!(
        "\nheld constant:\n  choice     {} ({})\n  measured   {} ({} / 100), confidence {:?}\n  signals    {} · {} item(s), fetched once\n",
        choice.name,
        choice.ticker,
        fit.label(),
        measures.score,
        measures.confidence,
        signals.source,
        signals.items.len()
    );

    let mut rows = Vec::new();
    for contender in &field {
        contender.select();
        let started = std::time::Instant::now();
        let brief = grounded_layered(&measures, fit, &choice.name, &signals, None, ReadMode::Live);
        let elapsed = started.elapsed();

        println!("══════════════════════════════════════════════════════════════");
        println!("ASKED FOR  {}", contender.label);
        println!(
            "WROTE IT   {}{}",
            brief.source.as_deref().unwrap_or("(no model — template)"),
            if wrote_what_was_asked(&brief.source, &contender.model) {
                ""
            } else {
                "   ⚠ NOT the model requested"
            }
        );
        println!("RUNG       {:?}  [{}]", brief.rung, brief.rung.badge());
        println!("TOOK       {:.1}s", elapsed.as_secs_f64());
        println!("──────────────────────────────────────────────────────────────");
        println!("{}", brief.reading);
        println!();

        let checks = grade(&brief.reading, fit, &measures, &signals);
        for (label, ok, detail) in &checks {
            println!("  {}  {label}{detail}", if *ok { "pass" } else { "FAIL" });
        }
        println!();

        rows.push((
            contender.label.clone(),
            brief.source.clone().unwrap_or_else(|| "template".into()),
            format!("{:?}", brief.rung),
            elapsed.as_secs_f64(),
            words(&brief.reading),
            checks.iter().filter(|(_, ok, _)| *ok).count(),
            checks.len(),
        ));
    }

    println!("══════════════════════════════════════════════════════════════");
    println!(
        "{:<44} {:<10} {:>6} {:>6} {:>7}",
        "WROTE IT", "RUNG", "SECS", "WORDS", "CHECKS"
    );
    for (asked, wrote, rung, secs, words, passed, total) in &rows {
        println!(
            "{:<44} {:<10} {secs:>6.1} {words:>6} {passed:>4}/{total}",
            wrote, rung
        );
        if !wrote.contains(asked.split_once(':').map(|(_, m)| m).unwrap_or(asked)) {
            println!("  (asked for {asked})");
        }
    }
    println!(
        "\nThe checks are the mechanical half. Read the four readings above against each other for\n\
         the half a machine cannot grade: whether the words are worth the money."
    );

    if agents::trace::on() {
        println!(
            "\n══════════════ trace ({:?}) ══════════════\n{}",
            agents::trace::level(),
            agents::trace::dump()
        );
    }
}

/// Did the model that answered match the one asked for? Substring rather than equality: OpenRouter
/// reports `anthropic/claude-opus-4.8` for what Anthropic calls `claude-opus-4-8`, and a strict
/// compare would flag every correct routing as a failover.
fn wrote_what_was_asked(source: &Option<String>, model: &str) -> bool {
    let Some(src) = source else { return false };
    let norm = |s: &str| s.replace(['.', '-', '_', '/'], "").to_lowercase();
    norm(src).contains(&norm(model)) || norm(model).contains(&norm(src))
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// The mechanically decidable criteria, each returning `(label, passed, detail)`.
///
/// Every one of these has a defect behind it that reached a real reading. They are checked per model
/// because "the guard holds" is a property of the guard *and* the writer it is guarding: the
/// truncation and template-echo defects both surfaced on one model and not another.
fn grade(
    reading: &str,
    fit: Fit,
    measures: &Measures,
    signals: &GroundedSignals,
) -> Vec<(&'static str, bool, String)> {
    let lower = reading.to_lowercase();

    // The band the prose claims must be the band the measures computed. A model that writes a
    // warmer read than the number supports is the single most damaging failure this app has.
    let claimed_other: Vec<&'static str> = bands_named_in(&lower)
        .into_iter()
        .filter(|l| *l != fit.label())
        .collect();

    // Money figures in the prose must appear in the signals. A plausible invented revenue number is
    // indistinguishable from a real one to a reader, which is what makes it worth checking.
    let haystack = signals.items.join(" ");
    let invented: Vec<String> = money_tokens(reading)
        .into_iter()
        .filter(|t| !haystack.contains(t.as_str()))
        .collect();

    // The shared definition, not a local list — see `agents::reads_like_advice` for why every
    // entry is a phrase. My first version of this check matched bare "sell" and "hold ", and flagged
    // the app's own Wikipedia-quoting citation line on the very first run.
    let advice = agents::reads_like_advice(reading);

    vec![
        (
            "band in prose matches the measured band",
            claimed_other.is_empty(),
            if claimed_other.is_empty() {
                format!("  ({} / {})", fit.label(), measures.score)
            } else {
                format!(
                    "  — prose also claims {claimed_other:?}, measured {}",
                    fit.label()
                )
            },
        ),
        (
            "REMINDER present",
            lower.contains("not financial advice"),
            String::new(),
        ),
        (
            "GROUNDED citation line present",
            reading.contains("GROUNDED"),
            String::new(),
        ),
        (
            "no advice-shaped language",
            advice.is_none(),
            match advice {
                None => String::new(),
                Some(phrase) => format!("  — found {phrase:?}"),
            },
        ),
        (
            "every money figure traces to a signal",
            invented.is_empty(),
            if invented.is_empty() {
                String::new()
            } else {
                format!("  — not in the signals: {invented:?}")
            },
        ),
        (
            "not truncated mid-sentence",
            reading.trim_end().ends_with(['.', '!', '?', '"', ')'])
                || reading.trim_end().ends_with("advice."),
            String::new(),
        ),
    ]
}

/// Which band names the prose actually claims.
///
/// The four band labels nest: `Aligned` is a substring of both `Strongly Aligned` and `Misaligned`.
/// A plain `contains` per label therefore reports *every* Strongly-Aligned reading as also claiming
/// Aligned — the check would fail on correct output, which is worse than not having it, because a
/// criterion that always fails gets ignored and then stops catching the real thing.
///
/// So it matches longest label first and blanks each hit before looking for the next, which is the
/// same longest-match-wins rule a tokenizer uses and for the same reason.
fn bands_named_in(lower: &str) -> Vec<&'static str> {
    let mut labels = [
        Fit::StronglyAligned.label(),
        Fit::Aligned.label(),
        Fit::Mixed.label(),
        Fit::Misaligned.label(),
    ];
    labels.sort_by_key(|l| std::cmp::Reverse(l.len()));

    let mut rest = lower.to_string();
    let mut found = Vec::new();
    for label in labels {
        let needle = label.to_lowercase();
        if rest.contains(&needle) {
            found.push(label);
            rest = rest.replace(&needle, " ");
        }
    }
    found
}

/// Every `$…` figure in the text, normalised to the token as written (`$28.24B`).
///
/// Deliberately literal: it compares the token as the model wrote it against the signals as fetched,
/// so a model that reformats `$28.24B` to `$28.2 billion` is flagged. That is a false positive worth
/// having — a reformatted figure is a figure the model re-derived, and re-derivation is exactly
/// where a digit goes missing.
fn money_tokens(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|t| t.starts_with('$'))
        .map(|t| t.trim_end_matches([',', '.', ';', ')']).to_string())
        .filter(|t| t.len() > 1)
        .collect()
}

fn tesla() -> Choice {
    demo_choices()
        .into_iter()
        .find(|c| c.ticker == "TSLA")
        .expect("TSLA is a demo choice")
}
