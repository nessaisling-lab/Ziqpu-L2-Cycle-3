//! Natal chart assembly — turn a birth moment into signs, degrees, and angles.
//!
//! Generic over any [`ephemeris::Ephemeris`] backend. Bodies a backend cannot provide are
//! skipped (not faked); angles are computed only when the birth time is known — the honesty
//! rule from the PRD, enforced by construction.

use ephemeris::{ascendant_mc, Body, Ephemeris, Zodiac};

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

/// Which lunar node a chart carries.
///
/// A choice, never a pair. The mean and true nodes are the same point in the sky measured two ways,
/// ~1.5° apart, so a chart holding both double-counts every node contact — and because synastry is a
/// cross-product, node-to-node contacts count four times. That is not hypothetical: this codebase
/// shipped it, when a phantom `TrueNode` returned the mean node's own longitude under a second name
/// and Apple's chart listed sixteen node contacts that were seven facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeMode {
    /// The classical smoothed formula. **The default**, because it is what every chart this project
    /// has produced used, it needs no lunar latitude, and it is therefore the only node the analytic
    /// floor can supply.
    #[default]
    Mean,
    /// The true (osculating) node — the real crossing. Requires a backend that models the Moon's
    /// latitude; on the analytic floor the body errors and the chart comes out without a node rather
    /// than with a fabricated one.
    True,
}

impl NodeMode {
    pub fn body(self) -> Body {
        match self {
            NodeMode::Mean => Body::MeanNode,
            NodeMode::True => Body::TrueNode,
        }
    }
}

/// How a chart is drawn — which node, and which zodiac.
///
/// A struct rather than more parameters. `compute_chart_with` already took a `NodeMode`, and adding
/// a zodiac beside it would make seven positional arguments where two of them are `f64` coordinates
/// — an easy pair to transpose and a hard mistake to see. Both fields default to what every chart
/// this project has produced so far, so `Default::default()` is the existing behaviour by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChartOptions {
    pub node: NodeMode,
    pub zodiac: Zodiac,
}

/// The bodies Ziqpu charts, in canonical order — with the **mean** node.
///
/// Kept as a constant because it is part of the shipped contract and several tests count it. For a
/// chart with the true node, use [`compute_chart_with`] and [`NodeMode::True`], which substitutes
/// rather than appends.
pub const CHART_BODIES: [Body; 12] = [
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
    /// Which zodiac these longitudes are expressed in. Carried on the chart itself so a reading can
    /// never present positions without being able to say what they are measured from — the two
    /// zodiacs differ by nearly a whole sign.
    pub zodiac: Zodiac,
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
pub fn compute_chart<E: Ephemeris + ?Sized>(
    eph: &E,
    jd_ut: f64,
    latitude: f64,
    longitude: f64,
    time_known: bool,
) -> NatalChart {
    compute_chart_with(
        eph,
        jd_ut,
        latitude,
        longitude,
        time_known,
        ChartOptions::default(),
    )
}

/// [`compute_chart`], choosing which lunar node the chart carries.
///
/// The node is **substituted**, not added: the body list is the same length either way. A chart with
/// both nodes would double-count every node contact, which is the defect [`NodeMode`] exists to make
/// impossible to reintroduce by accident.
pub fn compute_chart_with<E: Ephemeris + ?Sized>(
    eph: &E,
    jd_ut: f64,
    latitude: f64,
    longitude: f64,
    time_known: bool,
    opts: ChartOptions,
) -> NatalChart {
    let mut bodies = Vec::new();
    for &body in &CHART_BODIES {
        let body = if body == Body::MeanNode {
            opts.node.body()
        } else {
            body
        };
        if let Ok(p) = eph.position(body, jd_ut) {
            // The ephemeris always speaks tropical; the zodiac is applied here, once, at the single
            // point where a longitude becomes a sign.
            let longitude = opts.zodiac.from_tropical(p.longitude, jd_ut);
            let (sign, degree) = sign_of(longitude);
            bodies.push(BodyPosition {
                body,
                longitude,
                sign,
                degree,
                retrograde: p.speed_lon < 0.0,
                speed: p.speed_lon,
            });
        }
    }
    let (ascendant, midheaven) = if time_known {
        let (asc, mc) = ascendant_mc(jd_ut, latitude, longitude);
        // The angles are longitudes too, so they take the same conversion — an ascendant left
        // tropical in a sidereal chart would sit a whole sign away from the bodies around it.
        (
            Some(opts.zodiac.from_tropical(asc, jd_ut)),
            Some(opts.zodiac.from_tropical(mc, jd_ut)),
        )
    } else {
        (None, None)
    };
    NatalChart {
        zodiac: opts.zodiac,
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
        // Analytic backend supplies all but Pluto → 11 of 12 (Chiron via the bundled table).
        //
        // It was 12 of 13 while a phantom `TrueNode` sat in `CHART_BODIES` returning the mean node's
        // own longitude. This assertion is what caught its removal, which is the point of counting
        // bodies at all: a chart that quietly gains or loses one is a chart whose scores moved.
        assert_eq!(chart.bodies.len(), 11);
        assert!(chart.ascendant.is_some() && chart.midheaven.is_some());
        let sun = chart.bodies.iter().find(|b| b.body == Body::Sun).unwrap();
        assert_eq!(sun.sign, "Sagittarius");

        // Exactly one lunar node. Two identical ones double-counted every node contact and
        // quadrupled node-to-node contacts, because the synastry cross-product squares a duplicate.
        let nodes = chart
            .bodies
            .iter()
            .filter(|b| b.body.name().contains("Node"))
            .count();
        assert_eq!(nodes, 1, "one node body, or every score is inflated");
    }

    /// The true node is a DIFFERENT number from the mean node, and the chart still holds one node.
    ///
    /// The negative half is the point: this is where a phantom `TrueNode` would be caught. It once
    /// existed and returned `mean_node()`, so the two were bit-identical — a test that only asserted
    /// "the body is present" passed happily on a body that was a lie.
    #[test]
    fn the_true_node_differs_from_the_mean_node_and_does_not_join_it() {
        let jd = julian_day(2024, 6, 1, 12.0);
        let engine = ephemeris::shared();
        if !engine.source.is_authoritative() {
            eprintln!("SKIPPED the true-node comparison: no DE440 kernel — the analytic floor has no lunar latitude");
            // The floor must REFUSE rather than substitute. That half is checkable everywhere.
            assert!(ephemeris::AnalyticBackend
                .position(Body::TrueNode, jd)
                .is_err());
            return;
        }

        let mean = compute_chart_with(
            engine.backend.as_ref(),
            jd,
            40.7,
            -74.0,
            true,
            ChartOptions {
                node: NodeMode::Mean,
                ..Default::default()
            },
        );
        let true_ = compute_chart_with(
            engine.backend.as_ref(),
            jd,
            40.7,
            -74.0,
            true,
            ChartOptions {
                node: NodeMode::True,
                ..Default::default()
            },
        );

        // Substituted, not appended — same body count, exactly one node in each.
        assert_eq!(mean.bodies.len(), true_.bodies.len());
        for chart in [&mean, &true_] {
            assert_eq!(
                chart
                    .bodies
                    .iter()
                    .filter(|b| b.body.name().contains("Node"))
                    .count(),
                1,
                "one node, or every node contact is counted twice"
            );
        }

        let m = mean
            .bodies
            .iter()
            .find(|b| b.body == Body::MeanNode)
            .unwrap();
        let t = true_
            .bodies
            .iter()
            .find(|b| b.body == Body::TrueNode)
            .unwrap();

        let mut sep = (t.longitude - m.longitude).abs();
        if sep > 180.0 {
            sep = 360.0 - sep;
        }
        assert!(
            sep > 1e-6,
            "the true node is not a relabelled mean node — that was the deleted bug"
        );
        assert!(
            sep < 2.5,
            "but it librates AROUND the mean node, so a large gap means the derivation is wrong: {sep}deg"
        );
    }

    /// A whole chart drawn sidereally sits about a sign behind the same chart drawn tropically —
    /// bodies AND angles together.
    ///
    /// The angles are the half worth testing. They come from `ascendant_mc`, which always speaks
    /// tropical, so they are converted separately from the bodies — two code paths that must agree,
    /// and the failure mode if they do not is an ascendant a full sign away from the planets sitting
    /// beside it in the same chart.
    #[test]
    fn a_sidereal_chart_moves_bodies_and_angles_together() {
        use ephemeris::{Ayanamsa, Zodiac};
        let jd = julian_day(1980, 12, 12, 14.5);
        let sid = ChartOptions {
            zodiac: Zodiac::Sidereal(Ayanamsa::Lahiri),
            ..Default::default()
        };

        let trop = compute_chart_with(
            &AnalyticBackend,
            jd,
            40.7589,
            -73.9851,
            true,
            ChartOptions::default(),
        );
        let vedic = compute_chart_with(&AnalyticBackend, jd, 40.7589, -73.9851, true, sid);

        // Same bodies, same count — only the frame moved.
        assert_eq!(trop.bodies.len(), vedic.bodies.len());
        assert_eq!(trop.zodiac.label(), "tropical");
        assert_eq!(vedic.zodiac.label(), "sidereal (Lahiri)");

        let gap = |a: f64, b: f64| {
            let mut d = a - b;
            if d < 0.0 {
                d += 360.0;
            }
            d
        };

        // 1980: the Lahiri offset is a little over 23 degrees. Every body shifts by the SAME amount.
        for (t, v) in trop.bodies.iter().zip(vedic.bodies.iter()) {
            assert_eq!(t.body, v.body);
            let d = gap(t.longitude, v.longitude);
            assert!(
                (23.0..24.0).contains(&d),
                "{}: expected about a 23 degree shift, got {d}",
                t.body.name()
            );
        }

        // And the angles moved with them, by the same offset — this is the part a separate code
        // path could silently get wrong.
        let d_asc = gap(trop.ascendant.unwrap(), vedic.ascendant.unwrap());
        let d_mc = gap(trop.midheaven.unwrap(), vedic.midheaven.unwrap());
        assert!((23.0..24.0).contains(&d_asc), "ascendant shift {d_asc}");
        assert!((23.0..24.0).contains(&d_mc), "midheaven shift {d_mc}");

        // The concrete consequence: AAPL's Sun leaves Sagittarius for Scorpio. Nearly a whole sign
        // is exactly why a chart must say which zodiac drew it.
        let sun = |c: &NatalChart| c.bodies.iter().find(|b| b.body == Body::Sun).unwrap().sign;
        assert_eq!(sun(&trop), "Sagittarius");
        assert_eq!(sun(&vedic), "Scorpio");
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
