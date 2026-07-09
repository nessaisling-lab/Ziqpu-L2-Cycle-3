//! Weighted, symmetric synastry scoring (0–100) plus the two derived read-outs the flagship needs:
//! the dominant theme and a confidence assessment. The per-contact weights are computed once, in
//! the engine, and cached on each [`AspectHit`]; everything here is a pure reduction over them —
//! `50 + Σ weight`, clamped and rounded. An empty set stays neutral at 50.

use crate::types::{AspectHit, Confidence, Theme, Tone};

/// Score a set of cross-aspects on a 0–100 scale, centred at 50 (neutral). Each contact's cached
/// signed `weight` (from `engine::score_synastry_aspect`) lifts or lowers the total; symmetry and
/// determinism come from the weights themselves.
pub fn synastry_score(aspects: &[AspectHit]) -> u8 {
    let acc: f64 = 50.0 + aspects.iter().map(|a| a.weight).sum::<f64>();
    acc.clamp(0.0, 100.0).round() as u8
}

/// The dominant theme of a read: the single heaviest contact (by absolute weight) as an axis +
/// aspect, the net tone across all contacts (with a 15% dead-band that reads as `Balanced`), and
/// the heaviest contact's share of the total absolute weight. `None` for an empty set.
///
/// Ties on absolute weight break to the tighter orb, then to the lexicographically smaller
/// `(body_a, body_b, aspect)` — so the choice is deterministic.
pub fn dominant_theme(aspects: &[AspectHit]) -> Option<Theme> {
    if aspects.is_empty() {
        return None;
    }
    let total_abs: f64 = aspects.iter().map(|a| a.weight.abs()).sum();
    let net: f64 = aspects.iter().map(|a| a.weight).sum();

    let top = aspects.iter().max_by(|x, y| {
        x.weight
            .abs()
            .total_cmp(&y.weight.abs())
            // tie: the tighter (smaller) orb should win, so it must compare as "greater"
            .then_with(|| y.orb.total_cmp(&x.orb))
            // tie: the lexicographically smaller name should win
            .then_with(|| (&y.body_a, &y.body_b, &y.aspect).cmp(&(&x.body_a, &x.body_b, &x.aspect)))
    })?;

    let tone = if total_abs == 0.0 || net.abs() < 0.15 * total_abs {
        Tone::Balanced
    } else if net > 0.0 {
        Tone::Flowing
    } else {
        Tone::Friction
    };
    let share = if total_abs > 0.0 {
        top.weight.abs() / total_abs
    } else {
        0.0
    };

    Some(Theme {
        axis: (top.body_a.clone(), top.body_b.clone()),
        aspect: top.aspect.clone(),
        tone,
        share,
    })
}

/// Assess confidence from the number of tight contacts (orb ≤ 3.0°): `High` at 3+, `Moderate` at
/// 1–2, else `Low`. Drop one notch when a birth time is unknown (angles/timing are softer then).
pub fn assess_confidence(aspects: &[AspectHit], time_known: bool) -> Confidence {
    let tight = aspects.iter().filter(|a| a.orb <= 3.0).count();
    let base = if tight >= 3 {
        Confidence::High
    } else if tight >= 1 {
        Confidence::Moderate
    } else {
        Confidence::Low
    };
    if time_known {
        base
    } else {
        base.notch_down()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::Aspect;
    use ephemeris::Body;

    fn aspect_of(name: &str) -> Aspect {
        match name {
            "Trine" => Aspect::Trine,
            "Sextile" => Aspect::Sextile,
            "Conjunction" => Aspect::Conjunction,
            "Square" => Aspect::Square,
            "Opposition" => Aspect::Opposition,
            other => panic!("unknown aspect {other}"),
        }
    }

    /// A Sun–Sun contact in a peregrine sign, with its weight computed by the real engine scorer.
    fn hit(aspect: &str, orb: f64) -> AspectHit {
        let asp = aspect_of(aspect);
        let weight =
            engine::score_synastry_aspect(Body::Sun, "Gemini", Body::Sun, "Gemini", asp, orb, 8.0);
        AspectHit {
            body_a: "Sun".into(),
            body_b: "Sun".into(),
            aspect: aspect.into(),
            orb,
            harmonious: asp.is_harmonious(),
            weight,
        }
    }

    /// A named contact between real bodies/signs, weight from the engine — for the band-lock test.
    fn contact(ba: Body, sa: &str, bb: Body, sb: &str, aspect: &str, orb: f64) -> AspectHit {
        let asp = aspect_of(aspect);
        let weight = engine::score_synastry_aspect(ba, sa, bb, sb, asp, orb, 8.0);
        AspectHit {
            body_a: ba.name().into(),
            body_b: bb.name().into(),
            aspect: aspect.into(),
            orb,
            harmonious: asp.is_harmonious(),
            weight,
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

    #[test]
    fn bands_are_locked_by_known_clusters() {
        // A dignified, benefic-rich harmonious cluster must reach Strongly Aligned (≥75)...
        let harmonious = vec![
            contact(Body::Venus, "Taurus", Body::Jupiter, "Cancer", "Trine", 0.0),
            contact(Body::Sun, "Leo", Body::Moon, "Taurus", "Trine", 0.0),
            contact(Body::Venus, "Libra", Body::Mars, "Aries", "Trine", 0.5),
            contact(Body::Sun, "Leo", Body::Venus, "Libra", "Sextile", 1.0),
        ];
        // ...and a malefic, debilitated hard cluster must fall to Misaligned (<40).
        let hard = vec![
            contact(
                Body::Mars,
                "Aries",
                Body::Saturn,
                "Capricorn",
                "Square",
                0.0,
            ),
            contact(
                Body::Sun,
                "Aquarius",
                Body::Saturn,
                "Leo",
                "Opposition",
                0.5,
            ),
            contact(Body::Moon, "Scorpio", Body::Mars, "Cancer", "Square", 1.0),
            contact(
                Body::Mars,
                "Scorpio",
                Body::Saturn,
                "Taurus",
                "Opposition",
                0.0,
            ),
        ];
        assert!(
            synastry_score(&harmonious) >= 75,
            "harmonious cluster scored {}",
            synastry_score(&harmonious)
        );
        assert!(
            synastry_score(&hard) < 40,
            "hard cluster scored {}",
            synastry_score(&hard)
        );
    }

    #[test]
    fn dominant_theme_picks_heaviest_axis_and_tone() {
        // Empty → None.
        assert!(dominant_theme(&[]).is_none());

        // A heavy positive Venus–Jupiter trine dominates a lighter negative contact → Flowing.
        let flowing = vec![
            contact(Body::Venus, "Taurus", Body::Jupiter, "Cancer", "Trine", 0.0),
            contact(
                Body::Mercury,
                "Gemini",
                Body::Saturn,
                "Aries",
                "Square",
                4.0,
            ),
        ];
        let theme = dominant_theme(&flowing).unwrap();
        assert_eq!(theme.axis, ("Venus".to_string(), "Jupiter".to_string()));
        assert_eq!(theme.aspect, "Trine");
        assert_eq!(theme.tone, Tone::Flowing);
        assert!(theme.share > 0.0 && theme.share <= 1.0);

        // A net-negative set → Friction.
        let friction = vec![
            contact(
                Body::Mars,
                "Aries",
                Body::Saturn,
                "Capricorn",
                "Square",
                0.0,
            ),
            contact(Body::Sun, "Leo", Body::Moon, "Taurus", "Sextile", 5.0),
        ];
        assert_eq!(dominant_theme(&friction).unwrap().tone, Tone::Friction);
    }

    #[test]
    fn confidence_reads_tight_contacts_and_time() {
        let three_tight = vec![hit("Trine", 0.5), hit("Sextile", 1.0), hit("Square", 2.0)];
        assert_eq!(assess_confidence(&three_tight, true), Confidence::High);
        // Same contacts but unknown time drops a notch.
        assert_eq!(assess_confidence(&three_tight, false), Confidence::Moderate);

        // No tight contacts + unknown time → Low.
        let none_tight = vec![hit("Trine", 5.5)];
        assert_eq!(assess_confidence(&none_tight, false), Confidence::Low);
        assert_eq!(assess_confidence(&[], true), Confidence::Low);
    }
}
