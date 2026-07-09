//! Shared value types for the loop: birth moments, measures, the fit scale, tool-call records.

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use ephemeris::julian_day;

// The chart-tagged aspect pattern type lives in the engine (the author's IP); the agents layer
// carries it through `Measures`/`SynastryReport` and re-exports it for surfaces above.
pub use engine::Pattern;

/// A birth moment — a local date/time at a place. The time is optional: an unknown birth
/// time is honestly flagged (never invented), mirroring the sidecar and the PRD's honesty rule.
#[derive(Debug, Clone, PartialEq)]
pub struct BirthMoment {
    pub date: NaiveDate,
    pub time: Option<NaiveTime>,
    pub tz: Tz,
    pub lat: f64,
    pub lon: f64,
}

impl BirthMoment {
    /// `(Julian day UT, time_known)`. DST-aware, matching the sidecar's `birth_jd`. An unknown
    /// time uses local noon and reports `time_known = false` so angles are withheld downstream.
    pub fn julian_day_ut(&self) -> (f64, bool) {
        let (t, known) = match self.time {
            Some(t) => (t, true),
            None => (NaiveTime::from_hms_opt(12, 0, 0).unwrap(), false),
        };
        let local = NaiveDateTime::new(self.date, t);
        let utc = self
            .tz
            .from_local_datetime(&local)
            .earliest()
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|| Utc.from_utc_datetime(&local));
        let hour = utc.hour() as f64 + utc.minute() as f64 / 60.0 + utc.second() as f64 / 3600.0;
        (julian_day(utc.year(), utc.month(), utc.day(), hour), known)
    }
}

/// A choice the seeker is weighing — a datable entity. In v1 these are companies dated by IPO.
#[derive(Debug, Clone)]
pub struct Choice {
    pub ticker: String,
    pub name: String,
    pub birth: BirthMoment,
    /// SEC CIK, for the live grounded pull (EDGAR). `None` → no filings can be fetched.
    pub cik: Option<u32>,
    /// Wikipedia page title, for the keyless "what this is" grounded signal. `None` → skip.
    pub wiki: Option<String>,
}

/// One cross-aspect between the seeker's chart and a choice's chart.
#[derive(Debug, Clone, PartialEq)]
pub struct AspectHit {
    pub body_a: String,
    pub body_b: String,
    pub aspect: String,
    pub orb: f64,
    pub harmonious: bool,
    /// The cached signed contribution to the score (from `engine::score_synastry_aspect`). A
    /// custom [`crate::ChartSource`] that leaves this at `0.0` degrades gracefully to a neutral 50.
    pub weight: f64,
}

/// Whether a set of contacts reads as net-flowing, net-friction, or balanced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Flowing,
    Friction,
    Balanced,
}

impl Tone {
    pub fn label(self) -> &'static str {
        match self {
            Tone::Flowing => "flowing",
            Tone::Friction => "friction",
            Tone::Balanced => "balanced",
        }
    }
}

/// The single dominant contact of a read — the heaviest axis, its aspect, and the overall tone.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// The two bodies of the heaviest contact (seeker body, choice body).
    pub axis: (String, String),
    pub aspect: String,
    pub tone: Tone,
    /// The heaviest contact's share of the total absolute weight, in `0.0..=1.0`.
    pub share: f64,
}

/// How much to trust a read, from the number of tight contacts and whether birth times are known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    High,
    Moderate,
    Low,
}

impl Confidence {
    pub fn label(self) -> &'static str {
        match self {
            Confidence::High => "High",
            Confidence::Moderate => "Moderate",
            Confidence::Low => "Low",
        }
    }

    /// Drop one notch (used when a birth time is unknown). `Low` is the floor.
    pub fn notch_down(self) -> Confidence {
        match self {
            Confidence::High => Confidence::Moderate,
            Confidence::Moderate => Confidence::Low,
            Confidence::Low => Confidence::Low,
        }
    }
}

/// The structured measures Hamun-ana returns (never prose).
#[derive(Debug, Clone)]
pub struct Measures {
    pub choice: String,
    pub aspects: Vec<AspectHit>,
    pub score: u8,
    /// The tightest few contacts, for the reading's "what was measured" beat.
    pub top: Vec<AspectHit>,
    /// The dominant contact of the read (heaviest axis + tone), if there is any contact.
    pub theme: Option<Theme>,
    /// Cross-chart aspect patterns (Grand Trine, T-Square, Yod, Stellium).
    pub patterns: Vec<Pattern>,
    /// How much to trust this read.
    pub confidence: Confidence,
}

/// The four-band fit scale — the same bands and thresholds as the PRD's Verdict mode (§5, §12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fit {
    StronglyAligned,
    Aligned,
    Mixed,
    Misaligned,
}

impl Fit {
    /// Map a 0–100 score to a band: Strongly Aligned ≥ 75 · Aligned 60–74 · Mixed 40–59 · else Misaligned.
    pub fn from_score(score: u8) -> Fit {
        match score {
            75..=u8::MAX => Fit::StronglyAligned,
            60..=74 => Fit::Aligned,
            40..=59 => Fit::Mixed,
            _ => Fit::Misaligned,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Fit::StronglyAligned => "Strongly Aligned",
            Fit::Aligned => "Aligned",
            Fit::Mixed => "Mixed",
            Fit::Misaligned => "Misaligned",
        }
    }
}

/// A record of a tool the loop invoked, in order — the basis of the CI tool-order eval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolCall {
    GetChart(String),
    GetSynastry(String, String),
    /// The loop proposed grounding and paused for approval (the checkpoint).
    Propose,
    /// The gated grounded pull actually ran (only reachable with an approval token).
    PullGrounded(String),
}

/// Real external signals about a choice (v1: SEC EDGAR filings).
#[derive(Debug, Clone, PartialEq)]
pub struct GroundedSignals {
    pub choice: String,
    pub source: String,
    pub items: Vec<String>,
}

/// A ranked fit read for one choice (the DECIDE output).
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub choice: String,
    pub name: String,
    pub fit: Fit,
    pub score: u8,
    pub theme: Option<Theme>,
    pub confidence: Confidence,
    pub reading: String,
}

/// The full synastry report for one choice (the flagship "Report" mode) — everything the measure
/// produced, plus the interpreter's prose.
#[derive(Debug, Clone)]
pub struct SynastryReport {
    pub choice: String,
    pub name: String,
    pub score: u8,
    pub fit: Fit,
    pub theme: Option<Theme>,
    pub patterns: Vec<Pattern>,
    pub aspects: Vec<AspectHit>,
    pub top: Vec<AspectHit>,
    pub confidence: Confidence,
    pub reading: String,
}

/// A one-line fit call for one choice (the "Verdict" mode) — band, score, confidence, and a
/// single measured→meaning line that ends in the guardrail.
#[derive(Debug, Clone)]
pub struct Verdict {
    pub choice: String,
    pub name: String,
    pub fit: Fit,
    pub score: u8,
    pub confidence: Confidence,
    pub why: String,
}

/// The grounded briefing (the ACT output).
#[derive(Debug, Clone)]
pub struct Briefing {
    pub reading: String,
}

/// Why the grounded pull was refused at the checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    /// No approval token was supplied — the human has not approved the costed pull.
    NotApproved,
    /// The approval token was minted for a different choice.
    WrongChoice,
}

impl std::fmt::Display for GateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateError::NotApproved => write!(f, "grounded pull not approved by a human"),
            GateError::WrongChoice => write!(f, "approval token is for a different choice"),
        }
    }
}

impl std::error::Error for GateError {}
