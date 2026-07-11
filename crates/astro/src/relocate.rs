//! Relocation charts — re-house an existing chart for a different place at the *same* instant.
//!
//! Positions and aspects are **time-only**, so they are copied unchanged; only the Ascendant and
//! Midheaven are recomputed for the new location, via the already-shipping `ephemeris::ascendant_mc`.
//! Angles stay withheld when the source chart's time is unknown (the project's honesty rule, honored
//! by construction). This crate only *calls* `engine`/`ephemeris` — it never modifies them.

use engine::NatalChart;
use ephemeris::ascendant_mc;

/// Re-house `chart` for a different `latitude` / `longitude` (east-positive degrees) at the SAME
/// instant. Bodies (positions + aspects) are copied verbatim; only ASC/MC move. When the source
/// chart's time is unknown the angles stay `None` — a date-only entity relocates without inventing
/// angles it cannot know.
pub fn relocate(chart: &NatalChart, latitude: f64, longitude: f64) -> NatalChart {
    let (ascendant, midheaven) = if chart.time_known {
        let (asc, mc) = ascendant_mc(chart.jd_ut, latitude, longitude);
        (Some(asc), Some(mc))
    } else {
        (None, None)
    };
    NatalChart {
        jd_ut: chart.jd_ut,
        latitude,
        longitude,
        time_known: chart.time_known,
        bodies: chart.bodies.clone(), // time-only → do not move
        ascendant,
        midheaven,
    }
}

/// Which whole-house system to derive cusps in. Placidus (quadrant) is out of scope on purpose:
/// it needs trig that would make asserted cusp values OS-dependent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseSystem {
    /// House 1 is the whole sign containing the Ascendant; cusps land on 0° of each sign.
    WholeSign,
    /// House 1 begins exactly at the Ascendant; cusps are ASC + 30°·n.
    Equal,
}

/// The twelve house-cusp longitudes derived from the Ascendant alone. Pure arithmetic
/// (`rem_euclid` / `floor` / `+` / `*`) — no transcendentals, so results are bit-identical across
/// operating systems and safe to assert as exact values.
pub fn house_cusps(ascendant: f64, system: HouseSystem) -> [f64; 12] {
    let start = match system {
        HouseSystem::Equal => ascendant.rem_euclid(360.0),
        HouseSystem::WholeSign => (ascendant.rem_euclid(360.0) / 30.0).floor() * 30.0,
    };
    let mut cusps = [0.0_f64; 12];
    for (i, c) in cusps.iter_mut().enumerate() {
        *c = (start + 30.0 * i as f64).rem_euclid(360.0);
    }
    cusps
}

#[cfg(test)]
mod tests {
    use super::*;
    use ephemeris::{julian_day, AnalyticBackend};

    // A time-known chart in NYC (1990-05-15 ≈ 18:30 UT); pass `time_known=false` for the date-only case.
    fn nyc_chart(time_known: bool) -> NatalChart {
        let jd = julian_day(1990, 5, 15, 18.5);
        engine::compute_chart(&AnalyticBackend, jd, 40.7128, -74.0060, time_known)
    }

    const BERLIN_LAT: f64 = 52.52;
    const BERLIN_LON: f64 = 13.405;

    #[test]
    fn positions_do_not_move_on_relocation() {
        let natal = nyc_chart(true);
        let moved = relocate(&natal, BERLIN_LAT, BERLIN_LON);
        assert_eq!(moved.bodies, natal.bodies); // time-only → frozen
    }

    #[test]
    fn relocation_preserves_instant_and_records_place() {
        let natal = nyc_chart(true);
        let moved = relocate(&natal, BERLIN_LAT, BERLIN_LON);
        assert_eq!(moved.jd_ut, natal.jd_ut);
        assert_eq!(moved.latitude, BERLIN_LAT);
        assert_eq!(moved.longitude, BERLIN_LON);
    }

    #[test]
    fn time_unknown_relocation_withholds_angles() {
        let natal = nyc_chart(false);
        assert!(natal.ascendant.is_none());
        let moved = relocate(&natal, BERLIN_LAT, BERLIN_LON);
        assert!(moved.ascendant.is_none() && moved.midheaven.is_none());
    }

    #[test]
    fn relocation_shifts_the_ascendant() {
        // Same instant, ~87° of longitude apart → the Ascendant must move noticeably.
        let natal = nyc_chart(true);
        let moved = relocate(&natal, BERLIN_LAT, BERLIN_LON);
        let (a, b) = (natal.ascendant.unwrap(), moved.ascendant.unwrap());
        assert!((a - b).abs() > 1.0, "ascendant {a} vs {b} barely moved");
    }

    #[test]
    fn whole_sign_cusps_land_on_sign_boundaries() {
        // 215.7° = 5.7° Scorpio → the containing sign starts at 210°.
        let cusps = house_cusps(215.7, HouseSystem::WholeSign);
        assert_eq!(cusps[0], 210.0);
        assert_eq!(cusps[1], 240.0);
        assert_eq!(cusps[11], 180.0);
    }

    #[test]
    fn equal_cusps_start_exactly_at_the_ascendant() {
        let asc = 215.7;
        let cusps = house_cusps(asc, HouseSystem::Equal);
        assert_eq!(cusps[0], asc);
        assert_eq!(cusps[3], (asc + 90.0).rem_euclid(360.0));
        assert_eq!(cusps[11], (asc + 330.0).rem_euclid(360.0));
    }
}
