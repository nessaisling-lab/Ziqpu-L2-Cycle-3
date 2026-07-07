//! Ziqpu interpretation engine — the author's IP (PolyForm Noncommercial 1.0.0).
//!
//! This crate is backend-agnostic: everything here operates on positions produced
//! through the [`ephemeris::Ephemeris`] trait, so no ephemeris licensing (permissive
//! or copyleft) ever reaches the interpretation logic. That separation is what keeps
//! a future commercial edition clean.

pub mod chart;
pub use chart::{compute_chart, sign_of, BodyPosition, NatalChart};

/// The five Ptolemaic aspects Ziqpu scores, with their exact angles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aspect {
    Conjunction,
    Sextile,
    Square,
    Trine,
    Opposition,
}

impl Aspect {
    /// Exact separation angle (degrees) that defines the aspect.
    pub fn angle(self) -> f64 {
        match self {
            Aspect::Conjunction => 0.0,
            Aspect::Sextile => 60.0,
            Aspect::Square => 90.0,
            Aspect::Trine => 120.0,
            Aspect::Opposition => 180.0,
        }
    }

    /// Whether tradition reads the aspect as flowing (vs. frictional).
    pub fn is_harmonious(self) -> bool {
        matches!(self, Aspect::Conjunction | Aspect::Sextile | Aspect::Trine)
    }
}

/// Angular separation of two ecliptic longitudes, normalized to `0.0..=180.0` degrees.
///
/// Direction-agnostic — the property that lets one routine serve transits and synastry
/// alike (the same [`find_aspect`] works for a chart-vs-chart comparison unchanged).
pub fn separation(lon_a: f64, lon_b: f64) -> f64 {
    let mut d = (lon_a - lon_b).rem_euclid(360.0);
    if d > 180.0 {
        d = 360.0 - d;
    }
    d
}

/// If two longitudes form an aspect within `orb` degrees, return the aspect and the
/// exact orb (how far from partile). Returns the tightest aspect when several match.
pub fn find_aspect(lon_a: f64, lon_b: f64, orb: f64) -> Option<(Aspect, f64)> {
    let sep = separation(lon_a, lon_b);
    [
        Aspect::Conjunction,
        Aspect::Sextile,
        Aspect::Square,
        Aspect::Trine,
        Aspect::Opposition,
    ]
    .into_iter()
    .filter_map(|a| {
        let delta = (sep - a.angle()).abs();
        (delta <= orb).then_some((a, delta))
    })
    .min_by(|x, y| x.1.total_cmp(&y.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separation_wraps_around_zero() {
        assert_eq!(separation(10.0, 350.0), 20.0);
        assert_eq!(separation(0.0, 180.0), 180.0);
        assert!((separation(224.0, 104.0) - 120.0).abs() < 1e-9);
    }

    #[test]
    fn finds_partile_trine() {
        assert_eq!(find_aspect(224.0, 104.0, 6.0), Some((Aspect::Trine, 0.0)));
    }

    #[test]
    fn no_aspect_outside_orb() {
        assert_eq!(find_aspect(0.0, 45.0, 6.0), None);
    }

    #[test]
    fn harmony_classification() {
        assert!(Aspect::Trine.is_harmonious());
        assert!(!Aspect::Square.is_harmonious());
    }
}
