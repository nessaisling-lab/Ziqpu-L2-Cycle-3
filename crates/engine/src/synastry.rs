//! Weighted, symmetric synastry scoring (PRD §7.2).
//!
//! A single cross-aspect between two charts is scored to a signed contribution: harmonious
//! families lift, hard families weigh down, and a conjunction's polarity follows the combined
//! nature of the two bodies. The magnitude folds in aspect family, body weight, essential
//! dignity, and how tight the orb is. The function is **symmetric** in `(a, b)` — a contact
//! reads the same regardless of which chart owns which planet — and carries **no**
//! applying/separating term, so the same number falls out on any platform.
//!
//! Determinism rule: every operation here is pure f64 add/mul/clamp plus integer combinatorics.
//! No `tanh`, no transcendental functions — the score is bit-identical across operating systems,
//! which is what lets the graded evals pin exact Fit bands.

use crate::Aspect;
use ephemeris::Body;

/// The traditional temperament of a body, used to sign a conjunction and to amplify a contact
/// that involves a strongly-charged planet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nature {
    /// Sun, Moon — the lights (neutral valence, but weighty).
    Luminary,
    /// Venus, Jupiter — the classical benefics.
    Benefic,
    /// Mars, Saturn — the classical malefics.
    Malefic,
    /// Everything else — Mercury, the outers, the nodes, Chiron.
    Neutral,
}

impl Nature {
    /// Signed valence: `+1` benefic, `−1` malefic, `0` otherwise.
    pub fn valence(self) -> f64 {
        match self {
            Nature::Benefic => 1.0,
            Nature::Malefic => -1.0,
            Nature::Luminary | Nature::Neutral => 0.0,
        }
    }
}

/// The traditional temperament of a body.
pub fn planet_nature(body: Body) -> Nature {
    match body {
        Body::Venus | Body::Jupiter => Nature::Benefic,
        Body::Mars | Body::Saturn => Nature::Malefic,
        Body::Sun | Body::Moon => Nature::Luminary,
        _ => Nature::Neutral,
    }
}

/// How much a body counts, by tradition: the lights weigh most, the personal planets next, the
/// social planets less, the outers least, and the calculated points (nodes, Chiron) least of all.
pub fn body_weight(body: Body) -> f64 {
    match body {
        Body::Sun | Body::Moon => 1.0,
        Body::Mercury | Body::Venus | Body::Mars => 0.9,
        Body::Jupiter | Body::Saturn => 0.7,
        Body::Uranus | Body::Neptune | Body::Pluto => 0.4,
        Body::MeanNode | Body::TrueNode | Body::Chiron => 0.3,
    }
}

/// Essential-dignity multiplier for a body in a sign: domicile 1.15, exaltation 1.10, peregrine
/// 1.00, detriment 0.90, fall 0.85. The outers, the nodes, and Chiron have no classical rulerships
/// and always return 1.0. Where a sign is both a body's exaltation and domicile (Mercury in Virgo),
/// domicile wins; where it is both detriment and fall (Mercury in Pisces), detriment wins — the
/// lookup checks in that fixed priority so the result is unambiguous.
pub fn dignity_modifier(body: Body, sign: &str) -> f64 {
    // (domicile, exaltation, detriment, fall) sign lists for the seven classical bodies.
    let (dom, exalt, detr, fall): (&[&str], &[&str], &[&str], &[&str]) = match body {
        Body::Sun => (&["Leo"], &["Aries"], &["Aquarius"], &["Libra"]),
        Body::Moon => (&["Cancer"], &["Taurus"], &["Capricorn"], &["Scorpio"]),
        Body::Mercury => (
            &["Gemini", "Virgo"],
            &["Virgo"],
            &["Sagittarius", "Pisces"],
            &["Pisces"],
        ),
        Body::Venus => (
            &["Taurus", "Libra"],
            &["Pisces"],
            &["Aries", "Scorpio"],
            &["Virgo"],
        ),
        Body::Mars => (
            &["Aries", "Scorpio"],
            &["Capricorn"],
            &["Taurus", "Libra"],
            &["Cancer"],
        ),
        Body::Jupiter => (
            &["Sagittarius", "Pisces"],
            &["Cancer"],
            &["Gemini", "Virgo"],
            &["Capricorn"],
        ),
        Body::Saturn => (
            &["Capricorn", "Aquarius"],
            &["Libra"],
            &["Cancer", "Leo"],
            &["Aries"],
        ),
        // Outers, nodes, Chiron: no essential dignity.
        _ => return 1.0,
    };
    if dom.contains(&sign) {
        1.15
    } else if exalt.contains(&sign) {
        1.10
    } else if detr.contains(&sign) {
        0.90
    } else if fall.contains(&sign) {
        0.85
    } else {
        1.0
    }
}

/// Calibration constant: a partile (orb 0) Trine between the two peregrine luminaries lands at
/// exactly this value (amp 1, weight 1, dignity 1, base 1) — squarely in the intended +7–8 band,
/// which keeps the 75/60/40 Fit thresholds meaningful.
const SCALE: f64 = 7.5;

/// Signed contribution of one cross-aspect. Positive lifts the fit, negative lowers it.
///
/// Symmetric in `(a, b)`; no applying/separating term. Magnitude is
/// `base · amp · tight · w · d · SCALE`, where:
/// - `base` is the aspect family's strength (Trine/Square 1.0, Conjunction/Opposition 0.9, Sextile 0.7),
/// - `w = body_weight(a) · body_weight(b)`,
/// - `d = (dignity(a) + dignity(b)) / 2`,
/// - `amp = 1 + 0.25·(|valence(a)| + |valence(b)|)` (a charged planet sharpens the contact),
/// - `tight = (1 − orb/orb_limit)` clamped to `[0.2, 1.0]`.
///
/// The sign follows the aspect family, except a Conjunction takes its sign from the combined
/// nature valence: benefic-leaning → positive, malefic-leaning → negative, and a neutral balance
/// (e.g. two luminaries, or a benefic with a malefic) gets a mild `+0.3` default so a real contact
/// never silently vanishes to zero.
#[allow(clippy::too_many_arguments)]
pub fn score_synastry_aspect(
    body_a: Body,
    sign_a: &str,
    body_b: Body,
    sign_b: &str,
    aspect: Aspect,
    orb: f64,
    orb_limit: f64,
) -> f64 {
    let base = match aspect {
        Aspect::Trine | Aspect::Square => 1.0,
        Aspect::Conjunction | Aspect::Opposition => 0.9,
        Aspect::Sextile => 0.7,
    };
    let va = planet_nature(body_a).valence();
    let vb = planet_nature(body_b).valence();
    let sign = match aspect {
        Aspect::Trine | Aspect::Sextile => 1.0,
        Aspect::Square | Aspect::Opposition => -1.0,
        Aspect::Conjunction => {
            let net = va + vb;
            if net > 0.0 {
                1.0
            } else if net < 0.0 {
                -1.0
            } else {
                0.3
            }
        }
    };
    let w = body_weight(body_a) * body_weight(body_b);
    let d = (dignity_modifier(body_a, sign_a) + dignity_modifier(body_b, sign_b)) / 2.0;
    let amp = 1.0 + 0.25 * (va.abs() + vb.abs());
    let tight = (1.0 - orb / orb_limit).clamp(0.2, 1.0);
    sign * base * amp * tight * w * d * SCALE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natures_and_valence() {
        assert_eq!(planet_nature(Body::Venus), Nature::Benefic);
        assert_eq!(planet_nature(Body::Mars), Nature::Malefic);
        assert_eq!(planet_nature(Body::Sun), Nature::Luminary);
        assert_eq!(planet_nature(Body::Mercury), Nature::Neutral);
        assert_eq!(Nature::Benefic.valence(), 1.0);
        assert_eq!(Nature::Malefic.valence(), -1.0);
        assert_eq!(Nature::Luminary.valence(), 0.0);
    }

    #[test]
    fn weights_and_dignities_are_bounded() {
        assert_eq!(body_weight(Body::Sun), 1.0);
        assert_eq!(body_weight(Body::Pluto), 0.4);
        assert_eq!(body_weight(Body::Chiron), 0.3);
        assert_eq!(dignity_modifier(Body::Venus, "Taurus"), 1.15);
        assert_eq!(dignity_modifier(Body::Venus, "Scorpio"), 0.90);
        assert_eq!(dignity_modifier(Body::Venus, "Pisces"), 1.10);
        assert_eq!(dignity_modifier(Body::Venus, "Virgo"), 0.85);
        // Outers/nodes/Chiron are always peregrine.
        assert_eq!(dignity_modifier(Body::Uranus, "Aries"), 1.0);
        assert_eq!(dignity_modifier(Body::MeanNode, "Leo"), 1.0);
        // Domicile beats exaltation (Mercury in Virgo), detriment beats fall (Mercury in Pisces).
        assert_eq!(dignity_modifier(Body::Mercury, "Virgo"), 1.15);
        assert_eq!(dignity_modifier(Body::Mercury, "Pisces"), 0.90);
    }

    #[test]
    fn swap_symmetry() {
        let ab = score_synastry_aspect(
            Body::Sun,
            "Leo",
            Body::Moon,
            "Taurus",
            Aspect::Trine,
            1.0,
            6.0,
        );
        let ba = score_synastry_aspect(
            Body::Moon,
            "Taurus",
            Body::Sun,
            "Leo",
            Aspect::Trine,
            1.0,
            6.0,
        );
        assert_eq!(ab, ba, "scoring must be symmetric in (a, b)");
    }

    #[test]
    fn polarity_by_family() {
        let trine = score_synastry_aspect(
            Body::Sun,
            "Leo",
            Body::Venus,
            "Libra",
            Aspect::Trine,
            0.0,
            6.0,
        );
        let square = score_synastry_aspect(
            Body::Sun,
            "Leo",
            Body::Venus,
            "Libra",
            Aspect::Square,
            0.0,
            6.0,
        );
        assert!(trine > 0.0, "a Trine must lift");
        assert!(square < 0.0, "a Square must weigh down");
    }

    #[test]
    fn conjunction_nature_teeth() {
        // Two malefics conjunct read hard; two benefics conjunct read soft.
        let hard = score_synastry_aspect(
            Body::Mars,
            "Aries",
            Body::Saturn,
            "Capricorn",
            Aspect::Conjunction,
            0.0,
            6.0,
        );
        let soft = score_synastry_aspect(
            Body::Venus,
            "Taurus",
            Body::Jupiter,
            "Cancer",
            Aspect::Conjunction,
            0.0,
            6.0,
        );
        assert!(hard < 0.0, "Mars conj Saturn must be negative");
        assert!(soft > 0.0, "Venus conj Jupiter must be positive");
    }

    #[test]
    fn neutral_conjunction_never_vanishes() {
        // Two luminaries: net valence 0 → the mild positive default, not 0.
        let lights = score_synastry_aspect(
            Body::Sun,
            "Gemini",
            Body::Moon,
            "Gemini",
            Aspect::Conjunction,
            0.0,
            6.0,
        );
        assert!(lights > 0.0, "a neutral conjunction must not vanish to 0");
    }

    #[test]
    fn dignity_direction() {
        // Same Trine, same partner: Venus dignified (Taurus) beats Venus debilitated (Scorpio).
        let strong = score_synastry_aspect(
            Body::Venus,
            "Taurus",
            Body::Sun,
            "Leo",
            Aspect::Trine,
            0.0,
            6.0,
        );
        let weak = score_synastry_aspect(
            Body::Venus,
            "Scorpio",
            Body::Sun,
            "Leo",
            Aspect::Trine,
            0.0,
            6.0,
        );
        assert!(strong > weak, "dignity must raise the contribution");
    }

    #[test]
    fn partile_luminary_trine_lands_in_band() {
        // The calibration anchor: two peregrine luminaries, partile Trine → ~7.5 (the +7–8 band).
        let v = score_synastry_aspect(
            Body::Sun,
            "Gemini",
            Body::Moon,
            "Gemini",
            Aspect::Trine,
            0.0,
            6.0,
        );
        assert!((7.0..=8.0).contains(&v), "expected +7–8, got {v}");
    }
}
