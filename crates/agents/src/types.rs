//! Shared value types for the loop: birth moments, measures, the fit scale, tool-call records.

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use ephemeris::julian_day;

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
}

/// The structured measures Hamun-ana returns (never prose).
#[derive(Debug, Clone)]
pub struct Measures {
    pub choice: String,
    pub aspects: Vec<AspectHit>,
    pub score: u8,
    /// The tightest few contacts, for the reading's "what was measured" beat.
    pub top: Vec<AspectHit>,
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
    pub reading: String,
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
