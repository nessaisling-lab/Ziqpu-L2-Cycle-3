//! Hamun-ana's tools — where charts come from. The default source computes charts directly
//! from the reused engine + analytic ephemeris (real math, no database or network), so the
//! loop is deterministic and testable offline. A deployment can implement [`ChartSource`] over
//! the read-only sidecar instead, unchanged above it.

use crate::types::{AspectHit, BirthMoment};
use engine::{compute_chart, find_aspect, NatalChart};
use ephemeris::AnalyticBackend;

/// Orb (degrees) for counting a cross-aspect — matches the sidecar's synastry orb.
pub const SYNASTRY_ORB: f64 = 6.0;

/// The seam Hamun-ana measures through. Two operations: build a chart, and cross-aspect two charts.
pub trait ChartSource {
    fn chart(&self, birth: &BirthMoment) -> NatalChart;
    fn synastry(&self, a: &NatalChart, b: &NatalChart) -> Vec<AspectHit>;
}

/// Default source: the reused interpretation engine over the pure-Rust analytic ephemeris.
pub struct EngineChartSource {
    pub orb: f64,
}

impl Default for EngineChartSource {
    fn default() -> Self {
        Self { orb: SYNASTRY_ORB }
    }
}

impl ChartSource for EngineChartSource {
    fn chart(&self, birth: &BirthMoment) -> NatalChart {
        let (jd, time_known) = birth.julian_day_ut();
        compute_chart(&AnalyticBackend, jd, birth.lat, birth.lon, time_known)
    }

    fn synastry(&self, a: &NatalChart, b: &NatalChart) -> Vec<AspectHit> {
        let mut hits = Vec::new();
        for pa in &a.bodies {
            for pb in &b.bodies {
                if let Some((aspect, orb)) = find_aspect(pa.longitude, pb.longitude, self.orb) {
                    hits.push(AspectHit {
                        body_a: pa.body.name().to_string(),
                        body_b: pb.body.name().to_string(),
                        aspect: aspect.name().to_string(),
                        orb: (orb * 100.0).round() / 100.0,
                        harmonious: aspect.is_harmonious(),
                    });
                }
            }
        }
        hits.sort_by(|x, y| x.orb.total_cmp(&y.orb));
        hits
    }
}
