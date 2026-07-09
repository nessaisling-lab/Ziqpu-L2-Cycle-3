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
pub mod interpret;
pub mod interpret_llm;
pub mod measure;
pub mod orchestrator;
pub mod profile;
pub mod score;
pub mod types;

pub use grounded::{EdgarSource, GroundedSource, MockGroundedSource};
pub use interpret::{Interpreter, TemplateInterpreter};
pub use interpret_llm::AnthropicInterpreter;
pub use measure::{ChartSource, EngineChartSource, SYNASTRY_ORB};
pub use orchestrator::{is_advice_seeking, Answer, ApprovalRequest, ApprovalToken, Session};
pub use profile::{export_profile, import_profile, make_profile, ProfileError};
pub use score::synastry_score;
pub use types::{
    AspectHit, BirthMoment, Briefing, Choice, Fit, GateError, GroundedSignals, Measures,
    Recommendation, ToolCall,
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

/// Five real US IPOs as choices, with CIKs for the live SEC EDGAR pull. Undated listings
/// (KO, JNJ) carry `time = None` and are charted without angles — honestly, not invented.
pub fn demo_choices() -> Vec<Choice> {
    vec![
        Choice {
            ticker: "AAPL".into(),
            name: "Apple".into(),
            birth: ny(1980, 12, 12, 9, 30),
            cik: Some(320193),
        },
        Choice {
            ticker: "MSFT".into(),
            name: "Microsoft".into(),
            birth: ny(1986, 3, 13, 9, 30),
            cik: Some(789019),
        },
        Choice {
            ticker: "TSLA".into(),
            name: "Tesla".into(),
            birth: ny(2010, 6, 29, 9, 30),
            cik: Some(1318605),
        },
        Choice {
            ticker: "KO".into(),
            name: "Coca-Cola".into(),
            birth: ny_dateonly(1919, 9, 5),
            cik: Some(21344),
        },
        Choice {
            ticker: "JNJ".into(),
            name: "Johnson & Johnson".into(),
            birth: ny_dateonly(1944, 9, 24),
            cik: Some(200406),
        },
    ]
}
