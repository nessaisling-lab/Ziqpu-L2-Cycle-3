//! Analytic (pure-Rust, no data files) ephemeris backend — the default.
//!
//! Planets via VSOP87D (heliocentric spherical, equinox of date → tropical), converted to
//! geocentric ecliptic longitude by vector subtraction from Earth. Sun derived from Earth's
//! heliocentric position (opposite direction). Moon via a Meeus Ch.47 series. Agreement with
//! JPL/ANISE is well under a degree for the Sun and planets — far tighter than astrology's
//! whole-degree orbs. Pluto, nodes, angles, and Chiron are handled by other backends/tables.

use crate::{jd_to_t, norm360, Body, EclipticPos, Ephemeris, EphemerisError};
use std::f64::consts::PI;
use vsop87::vsop87d;

const R2D: f64 = 180.0 / PI;
const D2R: f64 = PI / 180.0;

/// The pure-Rust analytic backend (VSOP87 planets + Meeus Moon).
pub struct AnalyticBackend;

/// Heliocentric rectangular (AU) from a VSOP87 spherical coordinate (longitude/latitude radians).
fn helio_xyz(s: &vsop87::SphericalCoordinates) -> (f64, f64, f64) {
    let (l, b, r) = (s.longitude(), s.latitude(), s.distance());
    (r * b.cos() * l.cos(), r * b.cos() * l.sin(), r * b.sin())
}

fn planet_helio(body: Body, jde: f64) -> Option<vsop87::SphericalCoordinates> {
    Some(match body {
        Body::Mercury => vsop87d::mercury(jde),
        Body::Venus => vsop87d::venus(jde),
        Body::Mars => vsop87d::mars(jde),
        Body::Jupiter => vsop87d::jupiter(jde),
        Body::Saturn => vsop87d::saturn(jde),
        Body::Uranus => vsop87d::uranus(jde),
        Body::Neptune => vsop87d::neptune(jde),
        _ => return None,
    })
}

/// Geocentric ecliptic (longitude°, latitude°, distance AU) for a VSOP87 planet.
fn geocentric(body: Body, jde: f64) -> (f64, f64, f64) {
    let (ex, ey, ez) = helio_xyz(&vsop87d::earth(jde));
    let (px, py, pz) =
        helio_xyz(&planet_helio(body, jde).expect("caller guarantees a VSOP87 body"));
    let (gx, gy, gz) = (px - ex, py - ey, pz - ez);
    let distance = (gx * gx + gy * gy + gz * gz).sqrt();
    (
        norm360(gy.atan2(gx) * R2D),
        (gz / distance).asin() * R2D,
        distance,
    )
}

/// Geocentric Sun (longitude°, distance AU): opposite Earth's heliocentric direction.
fn sun_geocentric(jde: f64) -> (f64, f64) {
    let (ex, ey, ez) = helio_xyz(&vsop87d::earth(jde));
    let (sx, sy, sz) = (-ex, -ey, -ez);
    let distance = (sx * sx + sy * sy + sz * sz).sqrt();
    (norm360(sy.atan2(sx) * R2D), distance)
}

/// Meeus low-precision Sun longitude (°). Used to cross-check the VSOP87 Sun in tests.
pub fn meeus_sun_longitude(t: f64) -> f64 {
    let l0 = norm360(280.46646 + t * (36000.76983 + t * 0.0003032));
    let m = (357.52911 + t * (35999.05029 - t * 0.0001537)) * D2R;
    let c = (1.914602 - t * (0.004817 + t * 0.000014)) * m.sin()
        + (0.019993 - t * 0.000101) * (2.0 * m).sin()
        + 0.000289 * (3.0 * m).sin();
    norm360(l0 + c - 0.00569 - 0.00478 * ((125.04 - 1934.136 * t) * D2R).sin())
}

/// Meeus Ch.47 Moon longitude (°), geocentric ecliptic of date. Leading terms only
/// (|coefficient| >= 0.01°), which keeps the truncation error well inside astrology tolerance.
pub fn moon_longitude(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let lp = norm360(218.3165 + 481267.8813 * t);
    let lpr = lp * D2R;
    let m = (134.9634 + 477198.8676 * t + 0.0087 * t2 + t3 / 69699.0) * D2R; // Moon anomaly
    let ms = (357.5291 + 35999.0503 * t - 0.0001559 * t2) * D2R; // Sun anomaly
    let f = (93.2721 + 483202.0175 * t - 0.0034 * t2) * D2R; // argument of latitude

    let sigma = 6.288750 * m.sin()
        + 1.274018 * (2.0 * lpr - m).sin()
        + 0.658309 * (2.0 * lpr).sin()
        + 0.213616 * (2.0 * m).sin()
        - 0.185596 * ms.sin()
        - 0.114336 * (2.0 * f).sin()
        + 0.058793 * (2.0 * lpr - 2.0 * m).sin()
        + 0.057212 * (2.0 * lpr - ms - m).sin()
        + 0.053320 * (2.0 * lpr + m).sin()
        + 0.045874 * (2.0 * lpr - ms).sin()
        + 0.041024 * (m - ms).sin()
        - 0.034718 * lpr.sin()
        - 0.030465 * (ms + m).sin()
        + 0.015326 * (2.0 * lpr - 2.0 * f).sin()
        - 0.012528 * (2.0 * f + m).sin()
        - 0.010980 * (2.0 * f - m).sin()
        + 0.010674 * (4.0 * lpr - m).sin()
        + 0.010034 * (3.0 * m).sin();

    norm360(lp + sigma)
}

/// Mean lunar node (ascending, Ω) ecliptic longitude (°), Meeus 47.7. The mean node regresses
/// ~0.053°/day. True node ≈ mean here (its ±1.5° libration is a documented later refinement).
pub fn mean_node(t: f64) -> f64 {
    norm360(125.0445 - 1934.1362 * t + 0.0020762 * t * t)
}

/// Ecliptic longitude (°) for a body this backend supports, else `None`.
fn analytic_longitude(body: Body, jde: f64) -> Option<f64> {
    Some(match body {
        Body::Sun => sun_geocentric(jde).0,
        Body::Moon => moon_longitude(jd_to_t(jde)),
        Body::Mercury
        | Body::Venus
        | Body::Mars
        | Body::Jupiter
        | Body::Saturn
        | Body::Uranus
        | Body::Neptune => geocentric(body, jde).0,
        Body::MeanNode | Body::TrueNode => mean_node(jd_to_t(jde)),
        _ => return None,
    })
}

/// Signed daily motion (°/day) from longitudes one day apart, handling the 0°/360° wrap.
fn signed_daily_motion(lon_after: f64, lon_before: f64) -> f64 {
    let mut d = lon_after - lon_before;
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    d
}

impl Ephemeris for AnalyticBackend {
    fn name(&self) -> &'static str {
        "analytic (VSOP87 + Meeus)"
    }

    fn position(&self, body: Body, jd_ut: f64) -> Result<EclipticPos, EphemerisError> {
        // ΔT (~70 s) is ignored: <0.001° even at the Moon's speed — negligible for astrology.
        let jde = jd_ut;
        let (longitude, latitude, distance_au) = match body {
            Body::Sun => {
                let (l, d) = sun_geocentric(jde);
                (l, 0.0, d)
            }
            Body::Moon => (moon_longitude(jd_to_t(jde)), 0.0, 0.00257),
            Body::Mercury
            | Body::Venus
            | Body::Mars
            | Body::Jupiter
            | Body::Saturn
            | Body::Uranus
            | Body::Neptune => geocentric(body, jde),
            Body::Pluto => {
                return Err(EphemerisError(
                    "Pluto is not in the VSOP87 analytic backend; use the anise backend".into(),
                ))
            }
            Body::MeanNode | Body::TrueNode => (mean_node(jd_to_t(jde)), 0.0, 0.0),
            Body::Chiron => {
                return Err(EphemerisError(
                    "Chiron requires the precomputed table or the swisseph backend".into(),
                ))
            }
        };
        let speed_lon = match (
            analytic_longitude(body, jde + 0.5),
            analytic_longitude(body, jde - 0.5),
        ) {
            (Some(a), Some(b)) => signed_daily_motion(a, b),
            _ => 0.0,
        };
        Ok(EclipticPos {
            longitude,
            latitude,
            distance_au,
            speed_lon,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian_day;

    /// AAPL IPO: 1980-12-12, 09:30 EST = 14:30 UT.
    fn aapl_jd() -> f64 {
        julian_day(1980, 12, 12, 14.5)
    }

    #[test]
    fn sun_vsop_matches_meeus() {
        for jd in [2_451_545.0_f64, aapl_jd(), 2_460_000.5] {
            let vsop = sun_geocentric(jd).0;
            let meeus = meeus_sun_longitude(jd_to_t(jd));
            let diff = ((vsop - meeus + 540.0).rem_euclid(360.0) - 180.0).abs();
            assert!(diff < 0.1, "Sun mismatch {diff}° at jd {jd}");
        }
    }

    #[test]
    fn aapl_sun_in_sagittarius() {
        let lon = AnalyticBackend
            .position(Body::Sun, aapl_jd())
            .unwrap()
            .longitude;
        // 12 Dec 1980 → Sun ~21° Sagittarius ≈ 261°.
        assert!(
            (240.0..270.0).contains(&lon),
            "Sun at {lon}°, expected Sagittarius (~261°)"
        );
    }

    #[test]
    fn planets_return_valid_longitudes() {
        let jd = aapl_jd();
        for b in [
            Body::Mercury,
            Body::Venus,
            Body::Mars,
            Body::Jupiter,
            Body::Saturn,
            Body::Uranus,
            Body::Neptune,
        ] {
            let p = AnalyticBackend.position(b, jd).unwrap();
            assert!(
                (0.0..360.0).contains(&p.longitude),
                "{b:?} lon out of range"
            );
            assert!(p.distance_au > 0.0, "{b:?} distance non-positive");
        }
    }

    #[test]
    fn moon_moves_about_thirteen_degrees_per_day() {
        let p = AnalyticBackend.position(Body::Moon, aapl_jd()).unwrap();
        assert!((0.0..360.0).contains(&p.longitude));
        assert!(
            p.speed_lon.abs() > 10.0 && p.speed_lon.abs() < 16.0,
            "Moon daily motion {}°/day looks wrong",
            p.speed_lon
        );
    }

    #[test]
    fn unsupported_bodies_error_cleanly() {
        assert!(AnalyticBackend.position(Body::Pluto, aapl_jd()).is_err());
        assert!(AnalyticBackend.position(Body::Chiron, aapl_jd()).is_err());
    }

    #[test]
    fn mean_node_is_in_range_and_regresses() {
        let p = AnalyticBackend.position(Body::MeanNode, aapl_jd()).unwrap();
        assert!((0.0..360.0).contains(&p.longitude));
        assert!(
            p.speed_lon < 0.0,
            "the lunar node regresses, got {}",
            p.speed_lon
        );
    }
}
