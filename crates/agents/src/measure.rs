//! Hamun-ana's tools — where charts come from. The default source computes charts directly from the
//! reused engine over whichever ephemeris [`ephemeris::shared`] resolved (real math, no database or
//! network), so the loop is deterministic and testable offline. A deployment can implement
//! [`ChartSource`] over a remote service instead, unchanged above it — the seam is the point.
//!
//! This file used to name `AnalyticBackend` in both `chart` and `transits`, which is what made the
//! DE440 backend unreachable: it could be compiled in, the kernel could be on disk, and every chart
//! still came from VSOP87 series with no Pluto in it. The engine is resolved once per process now,
//! and [`EngineChartSource::engine`] reports which one so a surface can say so.

use crate::types::{AspectHit, BirthMoment, ToolCall};
use chrono::{Datelike, NaiveDate};
use engine::{compute_chart, find_aspect, score_synastry_aspect, NatalChart};
use ephemeris::julian_day;

/// Orb (degrees) for counting a cross-aspect.
pub const SYNASTRY_ORB: f64 = 6.0;

/// The seam Hamun-ana measures through. Three operations: build a chart, cross-aspect two charts,
/// and build the transiting sky for a day.
pub trait ChartSource {
    fn chart(&self, birth: &BirthMoment) -> NatalChart;
    fn synastry(&self, a: &NatalChart, b: &NatalChart) -> Vec<AspectHit>;

    /// The transiting sky for a calendar day — bodies only (no angles), at noon UT. Transit
    /// longitudes are geocentric and location-independent, so a planet-to-natal-planet read needs
    /// no birth place or time.
    fn transits(&self, date: NaiveDate) -> NatalChart;
}

/// Default source: the reused interpretation engine over the best ephemeris available — JPL DE440
/// when the verified kernel is present, the analytic floor otherwise.
pub struct EngineChartSource {
    pub orb: f64,
}

impl Default for EngineChartSource {
    fn default() -> Self {
        Self { orb: SYNASTRY_ORB }
    }
}

impl EngineChartSource {
    /// Which ephemeris actually computed these charts, and — when it is the floor — why.
    ///
    /// Exposed rather than logged once at startup: the caveat belongs beside the chart it applies
    /// to. A line in a log the seeker never opens is not a disclosure.
    pub fn engine(&self) -> &'static ephemeris::EngineSource {
        &ephemeris::shared().source
    }
}

impl ChartSource for EngineChartSource {
    fn chart(&self, birth: &BirthMoment) -> NatalChart {
        let (jd, time_known) = birth.julian_day_ut();
        compute_chart(
            ephemeris::shared().backend.as_ref(),
            jd,
            birth.lat,
            birth.lon,
            time_known,
        )
    }

    fn synastry(&self, a: &NatalChart, b: &NatalChart) -> Vec<AspectHit> {
        let mut hits = Vec::new();
        for pa in &a.bodies {
            for pb in &b.bodies {
                if let Some((aspect, orb)) = find_aspect(pa.longitude, pb.longitude, self.orb) {
                    // Signed contribution computed where the bodies and signs are live, then cached
                    // on the hit so scoring/theming stay pure vector reductions downstream.
                    let weight = score_synastry_aspect(
                        pa.body, pa.sign, pb.body, pb.sign, aspect, orb, self.orb,
                    );
                    hits.push(AspectHit {
                        body_a: pa.body.name().to_string(),
                        body_b: pb.body.name().to_string(),
                        aspect: aspect.name().to_string(),
                        orb: (orb * 100.0).round() / 100.0,
                        harmonious: aspect.is_harmonious(),
                        weight,
                    });
                }
            }
        }
        hits.sort_by(|x, y| x.orb.total_cmp(&y.orb));
        hits
    }

    fn transits(&self, date: NaiveDate) -> NatalChart {
        // Noon UT is fixed by the date parameter — nothing reads the system clock. Location and
        // time are irrelevant to planet-to-planet transits, so lat/lon are 0 and angles withheld.
        let jd = julian_day(date.year(), date.month(), date.day(), 12.0);
        compute_chart(ephemeris::shared().backend.as_ref(), jd, 0.0, 0.0, false)
    }
}

/// Hamun-ana's sequencing decision — *which* tools to call, in order, to measure a choice. This is
/// the seam a real local model (Qwen via Ollama) sits in. The chart math is always exact and
/// deterministic; the measurer only decides (and records) the plan, so it can never corrupt a number.
/// The one true contract is `get_chart(you) → get_chart(choice) → get_synastry(you, choice)`.
pub trait Measurer {
    fn sequence(&self, ticker: &str) -> Vec<ToolCall>;
}

/// The one correct measurement sequence for a choice.
pub fn expected_sequence(ticker: &str) -> Vec<ToolCall> {
    vec![
        ToolCall::GetChart("you".to_string()),
        ToolCall::GetChart(ticker.to_string()),
        ToolCall::GetSynastry("you".to_string(), ticker.to_string()),
    ]
}

/// The default measurer — emits the fixed, correct sequence with no model. Used in CI and by default.
#[derive(Default)]
pub struct DeterministicMeasurer;

impl Measurer for DeterministicMeasurer {
    fn sequence(&self, ticker: &str) -> Vec<ToolCall> {
        expected_sequence(ticker)
    }
}

/// Lets a boxed measurer be used wherever a `Measurer` is expected (runtime selection).
impl Measurer for Box<dyn Measurer> {
    fn sequence(&self, ticker: &str) -> Vec<ToolCall> {
        (**self).sequence(ticker)
    }
}
