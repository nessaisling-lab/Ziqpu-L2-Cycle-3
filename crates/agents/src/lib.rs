//! Ziqpu agents — the two-vizier **observe → decide → act** loop with a human-in-the-loop
//! checkpoint (design spec: PRD §16–§20).
//!
//! - **Hamun-ana** (measurer) calls the chart tools in a fixed order and returns structured
//!   [`Measures`] — never prose. See [`Session::measure`].
//! - **Ungasaga** (interpreter) turns measures into a reading in three beats
//!   (measured → meaning → reminder) and never advises. See [`Interpreter`].
//! - The **checkpoint** pauses before the costed grounded pull; [`Session::pull_grounded`] is
//!   unreachable without an [`ApprovalToken`] minted by [`Session::approve`].
//! - The **guardrail** ([`Session::ask`]) refuses buy/sell/hold questions in code.
//!
//! The default wiring is deterministic (no keys, no network) so the loop is CI-testable and the
//! live demo is reliable; real models and the live SEC EDGAR pull are drop-in via the traits.

pub mod grounded;
pub mod identity;
pub mod interpret;
pub mod interpret_llm;
pub mod llm_http;
pub mod measure;
pub mod measure_llm;
pub mod models;
pub mod orchestrator;
pub mod profile;
pub mod score;
pub mod tier;
pub mod tools;
pub mod traction;
pub mod types;
pub mod vin;

/// Spawn a subprocess without flashing a console window on Windows (CREATE_NO_WINDOW). No-op
/// elsewhere. Wrap every `Command::new(...)` spawned from a windowless build (the GUI links this
/// crate in-process) so no console flashes. The remaining subprocesses here are `curl` health/probe
/// calls; the live LLM HTTPS now rides in-process via `ureq` (see `llm_http`). Two cfg'd defs keep it
/// warning-clean on non-Windows.
#[cfg(windows)]
pub(crate) fn no_window(mut cmd: std::process::Command) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    cmd
}
#[cfg(not(windows))]
pub(crate) fn no_window(cmd: std::process::Command) -> std::process::Command {
    cmd
}

pub use grounded::{
    CompositeSource, EdgarSource, GroundedSource, MockGroundedSource, SecFactsSource,
    WikidataSource,
};
pub use identity::{anon_handle, anon_handle_for, anon_handle_reroll, handle_seed};
pub use interpret::{Interpreter, TemplateInterpreter};
pub use interpret_llm::{
    active_source_label, build_interpreter, draft_grounding_prompt, grounded_brief_for,
    grounded_layered, reading_for, reading_for_mode, wait_for_local, AnthropicInterpreter,
    GroundedRung, LayeredBrief, OpenAiCompatInterpreter, ReadMode,
};
pub use measure::{
    expected_sequence, ChartSource, DeterministicMeasurer, EngineChartSource, Measurer,
    SYNASTRY_ORB,
};
pub use measure_llm::LocalMeasurer;
pub use orchestrator::{
    is_advice_seeking, merge_placed, Answer, ApprovalRequest, ApprovalToken, Session,
};
pub use profile::{export_profile, import_profile, make_profile, ProfileError};
pub use score::{assess_confidence, dominant_theme, synastry_score};
pub use tools::{run_tool_loop, Tool, DEFAULT_MAX_STEPS};
pub use types::{
    AspectHit, BirthMoment, Briefing, Choice, Confidence, DailyReading, DayBeat, Fit, GateError,
    GroundedSignals, Measures, Recommendation, SynastryReport, Theme, Tone, ToolCall, TransitBeat,
    Verdict, WeeklyReading,
};
pub use vin::{is_valid_vin, parse_vpic, resolve_vin, DecodeVinTool, VehicleId};

// The engine's synastry + pattern surface, re-exported so callers above the agents layer can
// build placements and score contacts without depending on `engine` directly.
pub use engine::{
    body_weight, detect_patterns, dignity_modifier, planet_nature, score_synastry_aspect, Member,
    Pattern, PatternOrbs, Placed, Who,
};

use chrono::NaiveDate;

/// A human-readable natal-chart summary (one line per body) — for surfaces that don't depend on
/// the engine directly (e.g. the MCP server).
pub fn chart_summary(birth: &BirthMoment) -> Vec<String> {
    use crate::measure::{ChartSource, EngineChartSource};
    EngineChartSource::default()
        .chart(birth)
        .bodies
        .iter()
        .map(|b| format!("{:<9} {:>5.1}° {}", b.body.name(), b.degree, b.sign))
        .collect()
}

fn ny(year: i32, month: u32, day: u32, hour: u32, min: u32) -> BirthMoment {
    BirthMoment {
        date: NaiveDate::from_ymd_opt(year, month, day).expect("valid date"),
        time: Some(chrono::NaiveTime::from_hms_opt(hour, min, 0).expect("valid time")),
        tz: chrono_tz::America::New_York,
        lat: 40.7128,
        lon: -74.0060,
    }
}

fn ny_dateonly(year: i32, month: u32, day: u32) -> BirthMoment {
    BirthMoment {
        date: NaiveDate::from_ymd_opt(year, month, day).expect("valid date"),
        time: None,
        tz: chrono_tz::America::New_York,
        lat: 40.7128,
        lon: -74.0060,
    }
}

/// A fixed demo seeker (deterministic birth moment), for the live demo and tests.
pub fn demo_seeker() -> BirthMoment {
    ny(1990, 5, 15, 14, 30)
}

/// The five seeded choices for the graded demo — each a **real US IPO (listing) date**, verified by
/// hand, with its CIK for the live SEC EDGAR grounded pull.
///
/// | Ticker | Company            | Listing (IPO)  | Exchange | Time      | Source / confidence                         |
/// |--------|--------------------|----------------|----------|-----------|---------------------------------------------|
/// | AAPL   | Apple              | 1980-12-12     | NASDAQ   | 09:30 ET  | Well-documented IPO ($22/sh). High.         |
/// | MSFT   | Microsoft          | 1986-03-13     | NASDAQ   | 09:30 ET  | Well-documented IPO ($21/sh). High.         |
/// | TSLA   | Tesla              | 2010-06-29     | NASDAQ   | 09:30 ET  | **Cross-confirmed by our own SEC 424B4 re-derivation** (see `tickers`). Highest. |
/// | KO     | Coca-Cola          | 1919-09-05     | NYSE     | *unknown* | NYSE listing ($40/sh). No reliable 1919 intraday time → `time = None`. |
/// | JNJ    | Johnson & Johnson  | 1944-09-24     | NYSE     | *unknown* | NYSE listing ($37.50/sh). No reliable 1944 intraday time → `time = None`. |
///
/// Two conventions, both honest:
/// - **Time.** The three modern IPOs carry the real opening bell (09:30 ET). The two pre-war
///   listings have no trustworthy intraday time, so `time = None` and they chart **without angles** —
///   the same honest-partial behavior the industry universes use, never an invented bell.
/// - **These are LISTINGS, deliberately.** All five chart the day the company went *public*, not its
///   founding — the v1 "born onto the market" framing. That is why this hardcoded set differs from
///   what the search-flow CSV resolves: the CSV can't date a pre-1994 listing (EDGAR starts ~1994),
///   so it falls back to a founding for AAPL/MSFT and marks KO unchartable. The demo is a
///   hand-verified tier that is MORE precise than the automated pipeline can be for old companies —
///   which is exactly why it's hardcoded and never reads the CSV. Editing a date here changes the
///   graded demo and nothing else.
///
/// Location is a neutral lower-Manhattan point for all five (the demo predates per-exchange floor
/// coordinates; it is a fixed, reproducible input, not a precision claim).
pub fn demo_choices() -> Vec<Choice> {
    vec![
        Choice {
            ticker: "AAPL".into(),
            name: "Apple".into(),
            birth: ny(1980, 12, 12, 9, 30),
            cik: Some(320193),
            wiki: Some("Apple_Inc.".into()),
        },
        Choice {
            ticker: "MSFT".into(),
            name: "Microsoft".into(),
            birth: ny(1986, 3, 13, 9, 30),
            cik: Some(789019),
            wiki: Some("Microsoft".into()),
        },
        Choice {
            ticker: "TSLA".into(),
            name: "Tesla".into(),
            birth: ny(2010, 6, 29, 9, 30),
            cik: Some(1318605),
            wiki: Some("Tesla,_Inc.".into()),
        },
        Choice {
            ticker: "KO".into(),
            name: "Coca-Cola".into(),
            birth: ny_dateonly(1919, 9, 5),
            cik: Some(21344),
            wiki: Some("The_Coca-Cola_Company".into()),
        },
        Choice {
            ticker: "JNJ".into(),
            name: "Johnson & Johnson".into(),
            birth: ny_dateonly(1944, 9, 24),
            cik: Some(200406),
            wiki: Some("Johnson_&_Johnson".into()),
        },
    ]
}
