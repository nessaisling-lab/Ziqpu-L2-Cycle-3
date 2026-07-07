//! Pluggable ephemeris layer for Ziqpu.
//!
//! The [`Ephemeris`] trait is the seam that keeps the public tree free of copyleft.
//! The default `anise-backend` (wired in Phase 0) computes positions from permissive
//! JPL DE440 data; the high-accuracy Swiss Ephemeris backend lives behind the private
//! `swisseph` feature and never ships in the public repository.

/// A geocentric ecliptic position of a body at an instant.
///
/// `longitude` is normalized to `0.0..360.0` degrees; `speed_lon` is degrees/day
/// (negative when the body is retrograde).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EclipticPos {
    pub longitude: f64,
    pub latitude: f64,
    pub distance_au: f64,
    pub speed_lon: f64,
}

/// The bodies Ziqpu charts.
///
/// `Chiron` is the reason the backend choice matters: analytic planetary theories do
/// not include it, so a permissive backend must source it from a small-body ephemeris
/// while Swiss Ephemeris provides it natively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Body {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    MeanNode,
    TrueNode,
    Chiron,
}

/// The backend seam. Implemented by the ANISE backend (default) and, privately, by the
/// Swiss Ephemeris backend behind the `swisseph` feature.
pub trait Ephemeris {
    /// Geocentric ecliptic position of `body` at Julian day (UT).
    fn position(&self, body: Body, jd_ut: f64) -> Result<EclipticPos, EphemerisError>;
}

/// Failure to compute a position (for example, a date outside a backend's valid range).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EphemerisError(pub String);

impl core::fmt::Display for EphemerisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ephemeris error: {}", self.0)
    }
}

impl std::error::Error for EphemerisError {}

/// Convert a UT calendar date + fractional hour to a Julian Day (Meeus, Gregorian calendar).
pub fn julian_day(year: i32, month: u32, day: u32, hour_ut: f64) -> f64 {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    let day_frac = day as f64 + hour_ut / 24.0;
    (365.25 * (y as f64 + 4716.0)).floor() + (30.6001 * (m as f64 + 1.0)).floor() + day_frac + b
        - 1524.5
}

/// Julian centuries from J2000.0 (JD 2451545.0).
pub fn jd_to_t(jd: f64) -> f64 {
    (jd - 2_451_545.0) / 36_525.0
}

/// Normalize an angle in degrees to `0.0..360.0`.
pub fn norm360(deg: f64) -> f64 {
    deg.rem_euclid(360.0)
}

/// Ascendant and Midheaven ecliptic longitudes (degrees) for a UT instant at a geographic
/// latitude/longitude (degrees; longitude East-positive). Uses mean sidereal time + mean
/// obliquity — well inside astrology tolerance. Angles depend on location, so they live here
/// rather than behind the body-only `Ephemeris::position`.
pub fn ascendant_mc(jd_ut: f64, lat_deg: f64, lon_east_deg: f64) -> (f64, f64) {
    let d2r = core::f64::consts::PI / 180.0;
    let t = jd_to_t(jd_ut);
    // Greenwich mean sidereal time (Meeus 12.4), degrees.
    let gmst = norm360(
        280.460_618_37
            + 360.985_647_366_29 * (jd_ut - 2_451_545.0)
            + t * t * (0.000_387_933 - t / 38_710_000.0),
    );
    let ramc = norm360(gmst + lon_east_deg) * d2r; // right ascension of the meridian
    let eps = (23.439_291 - 0.013_004_2 * t) * d2r; // mean obliquity of the ecliptic
    let lat = lat_deg * d2r;
    let mc = norm360(ramc.sin().atan2(ramc.cos() * eps.cos()) / d2r);
    let asc = norm360(
        ramc.cos()
            .atan2(-(ramc.sin() * eps.cos() + lat.tan() * eps.sin()))
            / d2r,
    );
    (asc, mc)
}

#[cfg(feature = "analytic")]
pub mod analytic;
#[cfg(feature = "analytic")]
pub use analytic::AnalyticBackend;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bodies_are_distinct_hash_keys() {
        let all = [
            Body::Sun,
            Body::Moon,
            Body::Chiron,
            Body::TrueNode,
            Body::MeanNode,
        ];
        let set: HashSet<Body> = all.iter().copied().collect();
        assert_eq!(set.len(), all.len());
    }

    #[test]
    fn ecliptic_pos_is_copy() {
        let p = EclipticPos {
            longitude: 224.0,
            latitude: 0.0,
            distance_au: 1.0,
            speed_lon: 0.98,
        };
        let q = p; // relies on Copy
        assert_eq!(p, q);
    }

    #[test]
    fn mc_is_latitude_independent_but_ascendant_is_not() {
        let jd = 2_451_545.0; // 2000-01-01 12:00 UT
        let (asc_a, mc_a) = ascendant_mc(jd, 20.0, -74.0);
        let (asc_b, mc_b) = ascendant_mc(jd, 60.0, -74.0);
        // The Midheaven is fixed by sidereal time + obliquity; latitude must not move it.
        assert!((mc_a - mc_b).abs() < 1e-6, "MC must not depend on latitude");
        // The Ascendant rides the horizon, so latitude must move it.
        assert!(
            (asc_a - asc_b).abs() > 1.0,
            "Ascendant must depend on latitude"
        );
        for v in [asc_a, mc_a, asc_b, mc_b] {
            assert!((0.0..360.0).contains(&v), "angle {v} out of range");
        }
    }
}
