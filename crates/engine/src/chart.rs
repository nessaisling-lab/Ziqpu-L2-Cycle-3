//! Natal chart assembly — turn a birth moment into signs, degrees, and angles.
//!
//! Generic over any [`ephemeris::Ephemeris`] backend. Bodies a backend cannot provide are
//! skipped (not faked); angles are computed only when the birth time is known — the honesty
//! rule from the PRD, enforced by construction.

use ephemeris::{ascendant_mc, Body, Ephemeris};

/// Tropical zodiac signs, Aries first.
pub const ZODIAC: [&str; 12] = [
    "Aries",
    "Taurus",
    "Gemini",
    "Cancer",
    "Leo",
    "Virgo",
    "Libra",
    "Scorpio",
    "Sagittarius",
    "Capricorn",
    "Aquarius",
    "Pisces",
];

/// The bodies Ziqpu charts, in canonical order.
pub const CHART_BODIES: [Body; 13] = [
    Body::Sun,
    Body::Moon,
    Body::Mercury,
    Body::Venus,
    Body::Mars,
    Body::Jupiter,
    Body::Saturn,
    Body::Uranus,
    Body::Neptune,
    Body::Pluto,
    Body::MeanNode,
    Body::TrueNode,
    Body::Chiron,
];

/// Sign name and degree-within-sign (0..30) for an ecliptic longitude.
pub fn sign_of(longitude: f64) -> (&'static str, f64) {
    let lon = longitude.rem_euclid(360.0);
    let idx = (lon / 30.0).floor() as usize % 12;
    (ZODIAC[idx], lon - 30.0 * idx as f64)
}

/// One body placed in the chart.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyPosition {
    pub body: Body,
    pub longitude: f64,
    pub sign: &'static str,
    pub degree: f64,
    pub retrograde: bool,
    pub speed: f64,
}

/// A computed natal chart. `ascendant`/`midheaven` are `None` when the birth time is unknown.
#[derive(Debug, Clone)]
pub struct NatalChart {
    pub jd_ut: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub time_known: bool,
    pub bodies: Vec<BodyPosition>,
    pub ascendant: Option<f64>,
    pub midheaven: Option<f64>,
}

/// Compute a natal chart from a birth moment (Julian day UT) and location. Bodies the backend
/// cannot supply are silently skipped; angles are included only when `time_known` is true.
pub fn compute_chart<E: Ephemeris>(
    eph: &E,
    jd_ut: f64,
    latitude: f64,
    longitude: f64,
    time_known: bool,
) -> NatalChart {
    let mut bodies = Vec::new();
    for &body in &CHART_BODIES {
        if let Ok(p) = eph.position(body, jd_ut) {
            let (sign, degree) = sign_of(p.longitude);
            bodies.push(BodyPosition {
                body,
                longitude: p.longitude,
                sign,
                degree,
                retrograde: p.speed_lon < 0.0,
                speed: p.speed_lon,
            });
        }
    }
    let (ascendant, midheaven) = if time_known {
        let (asc, mc) = ascendant_mc(jd_ut, latitude, longitude);
        (Some(asc), Some(mc))
    } else {
        (None, None)
    };
    NatalChart {
        jd_ut,
        latitude,
        longitude,
        time_known,
        bodies,
        ascendant,
        midheaven,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ephemeris::{julian_day, AnalyticBackend};

    #[test]
    fn sign_split_is_correct() {
        assert_eq!(sign_of(0.0), ("Aries", 0.0));
        let (sign, degree) = sign_of(224.0);
        assert_eq!(sign, "Scorpio");
        assert!((degree - 14.0).abs() < 1e-9);
        assert_eq!(sign_of(359.9).0, "Pisces");
    }

    #[test]
    fn aapl_chart_has_expected_bodies_and_angles() {
        let jd = julian_day(1980, 12, 12, 14.5);
        let chart = compute_chart(&AnalyticBackend, jd, 40.7589, -73.9851, true);
        // Analytic backend supplies all but Pluto + Chiron → 11 of 13.
        assert_eq!(chart.bodies.len(), 11);
        assert!(chart.ascendant.is_some() && chart.midheaven.is_some());
        let sun = chart.bodies.iter().find(|b| b.body == Body::Sun).unwrap();
        assert_eq!(sun.sign, "Sagittarius");
    }

    #[test]
    fn unknown_time_omits_angles() {
        // KO: 1919 with unknown first-trade time — angles must be withheld, not faked.
        let jd = julian_day(1919, 9, 5, 12.0);
        let chart = compute_chart(&AnalyticBackend, jd, 40.7069, -74.0089, false);
        assert!(chart.ascendant.is_none() && chart.midheaven.is_none());
        assert!(!chart.bodies.is_empty());
    }
}
