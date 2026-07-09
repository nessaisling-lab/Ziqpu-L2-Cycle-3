//! A thin, symmetric synastry scorer (0–100). v1 keeps it deliberately simple: harmonious
//! contacts add, hard contacts subtract, each weighted by how tight the orb is. Phase 3 will
//! reuse the engine's dignity/weight helpers (PRD §7.2); this is enough to drive the fit band.

use crate::types::AspectHit;

/// Score a set of cross-aspects on a 0–100 scale, centred at 50 (neutral). Symmetric: an
/// aspect contributes by its own strength regardless of which chart each planet belongs to.
pub fn synastry_score(aspects: &[AspectHit]) -> u8 {
    let mut acc = 50.0_f64;
    for a in aspects {
        let base = match a.aspect.as_str() {
            "Trine" => 8.0,
            "Conjunction" => 6.0,
            "Sextile" => 5.0,
            "Square" => -6.0,
            "Opposition" => -5.0,
            _ => 0.0,
        };
        // Tighter orb → fuller weight (orb 0 → 1.0; orb 6 → 0.25), floored so wide aspects still count.
        let tightness = (1.0 - a.orb / 8.0).clamp(0.2, 1.0);
        acc += base * tightness;
    }
    acc.clamp(0.0, 100.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(aspect: &str, orb: f64) -> AspectHit {
        AspectHit {
            body_a: "Sun".into(),
            body_b: "Sun".into(),
            aspect: aspect.into(),
            orb,
            harmonious: matches!(aspect, "Trine" | "Sextile" | "Conjunction"),
        }
    }

    #[test]
    fn empty_is_neutral() {
        assert_eq!(synastry_score(&[]), 50);
    }

    #[test]
    fn harmonious_lifts_and_hard_lowers() {
        let harmonious = vec![hit("Trine", 0.5), hit("Sextile", 1.0), hit("Trine", 2.0)];
        let hard = vec![
            hit("Square", 0.5),
            hit("Opposition", 1.0),
            hit("Square", 2.0),
        ];
        assert!(synastry_score(&harmonious) > 55);
        assert!(synastry_score(&hard) < 45);
    }

    #[test]
    fn stays_in_range() {
        let many = vec![hit("Trine", 0.1); 40];
        assert!(synastry_score(&many) <= 100);
    }
}
