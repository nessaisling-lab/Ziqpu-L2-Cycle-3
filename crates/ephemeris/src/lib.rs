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
}
