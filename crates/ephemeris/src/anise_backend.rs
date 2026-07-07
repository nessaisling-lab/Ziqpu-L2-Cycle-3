//! ANISE + JPL DE440 ephemeris backend (feature `anise-backend`).
//!
//! High-accuracy positions from a JPL Development Ephemeris kernel (e.g. `de440s.bsp`), loaded
//! at runtime. ANISE yields geocentric ecliptic-of-J2000 coordinates; we advance them to
//! ecliptic-of-date (tropical) by adding the general precession in longitude. Because it needs a
//! kernel file, this backend is constructed explicitly (unlike the data-free analytic default).

use crate::{jd_to_t, norm360, Body, EclipticPos, Ephemeris, EphemerisError};
use anise::almanac::Almanac;
use anise::constants::frames::{
    EARTH_J2000, JUPITER_BARYCENTER_J2000, MARS_BARYCENTER_J2000, MERCURY_J2000, MOON_J2000,
    NEPTUNE_BARYCENTER_J2000, PLUTO_BARYCENTER_J2000, SATURN_BARYCENTER_J2000, SUN_J2000,
    URANUS_BARYCENTER_J2000, VENUS_J2000,
};
use anise::prelude::{Epoch, Frame};

const AU_KM: f64 = 149_597_870.7;
/// Mean obliquity of the ecliptic at J2000.0 (degrees) — the fixed ICRF/equatorial→ecliptic tilt.
const OBLIQUITY_J2000_DEG: f64 = 23.439_291_1;

/// ANISE-backed ephemeris. Load a DE440-compatible kernel with [`AniseBackend::from_kernel`].
pub struct AniseBackend {
    almanac: Almanac,
}

impl AniseBackend {
    /// Load from a JPL SPK/BSP kernel path (e.g. `data/ephemeris/de440s.bsp`).
    pub fn from_kernel(path: &str) -> Result<Self, EphemerisError> {
        let almanac =
            Almanac::new(path).map_err(|e| EphemerisError(format!("load kernel {path}: {e}")))?;
        Ok(Self { almanac })
    }

    /// Geocentric ecliptic-of-date (longitude°, latitude°, distance AU) for a DE440 body.
    fn ecliptic(&self, body: Body, jd_ut: f64) -> Result<(f64, f64, f64), EphemerisError> {
        let frame = body_frame(body).ok_or_else(|| {
            EphemerisError(format!("{} not provided by the anise backend", body.name()))
        })?;
        let epoch = Epoch::from_jde_utc(jd_ut);
        // translate() does translation only (no frame rotation → no orientation kernel needed),
        // giving the geocentric position in J2000 *equatorial* (ICRF) coordinates.
        let st = self
            .almanac
            .translate(frame, EARTH_J2000, epoch, None)
            .map_err(|e| EphemerisError(format!("translate {}: {e}", body.name())))?;
        let (x, y_eq, z_eq) = (st.radius_km.x, st.radius_km.y, st.radius_km.z);
        // Rotate equatorial → ecliptic about X by the J2000 obliquity.
        let eps = OBLIQUITY_J2000_DEG.to_radians();
        let (ce, se) = (eps.cos(), eps.sin());
        let y = y_eq * ce + z_eq * se;
        let z = -y_eq * se + z_eq * ce;
        let dist = (x * x + y_eq * y_eq + z_eq * z_eq).sqrt();
        let lon_j2000 = norm360(y.atan2(x).to_degrees());
        let lat = (z / dist).asin().to_degrees();
        // Ecliptic-of-J2000 → ecliptic-of-date: general precession in longitude (Meeus/IAU).
        let t = jd_to_t(jd_ut);
        let precession = (5028.796195 * t + 1.1054348 * t * t) / 3600.0;
        Ok((norm360(lon_j2000 + precession), lat, dist / AU_KM))
    }
}

fn body_frame(body: Body) -> Option<Frame> {
    Some(match body {
        Body::Sun => SUN_J2000,
        Body::Moon => MOON_J2000,
        Body::Mercury => MERCURY_J2000,
        Body::Venus => VENUS_J2000,
        Body::Mars => MARS_BARYCENTER_J2000,
        Body::Jupiter => JUPITER_BARYCENTER_J2000,
        Body::Saturn => SATURN_BARYCENTER_J2000,
        Body::Uranus => URANUS_BARYCENTER_J2000,
        Body::Neptune => NEPTUNE_BARYCENTER_J2000,
        Body::Pluto => PLUTO_BARYCENTER_J2000,
        _ => return None,
    })
}

/// Mean lunar node (Ω), Meeus 47.7 — DE440 kernels carry no node, so it stays analytic.
fn mean_node(t: f64) -> f64 {
    norm360(125.0445 - 1934.1362 * t + 0.0020762 * t * t)
}

fn signed_daily_motion(a: f64, b: f64) -> f64 {
    let mut d = a - b;
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    d
}

impl Ephemeris for AniseBackend {
    fn name(&self) -> &'static str {
        "ANISE/DE440"
    }

    fn position(&self, body: Body, jd_ut: f64) -> Result<EclipticPos, EphemerisError> {
        let (longitude, latitude, distance_au) = match body {
            Body::MeanNode | Body::TrueNode => (mean_node(jd_to_t(jd_ut)), 0.0, 0.0),
            Body::Chiron => {
                return Err(EphemerisError(
                    "Chiron requires a JPL Horizons small-body kernel (not yet loaded)".into(),
                ))
            }
            _ => self.ecliptic(body, jd_ut)?,
        };
        let speed_lon = match body {
            Body::MeanNode | Body::TrueNode => signed_daily_motion(
                mean_node(jd_to_t(jd_ut + 0.5)),
                mean_node(jd_to_t(jd_ut - 0.5)),
            ),
            _ => match (
                self.ecliptic(body, jd_ut + 0.5),
                self.ecliptic(body, jd_ut - 0.5),
            ) {
                (Ok(a), Ok(b)) => signed_daily_motion(a.0, b.0),
                _ => 0.0,
            },
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
    use crate::{julian_day, AnalyticBackend};

    #[test]
    #[ignore = "requires data/ephemeris/de440s.bsp — run scripts/fetch-ephemeris.sh"]
    fn analytic_agrees_with_anise_within_one_degree() {
        // Cargo runs tests with CWD at the crate dir, so resolve the kernel from the manifest.
        let kernel = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/ephemeris/de440s.bsp"
        );
        let anise = AniseBackend::from_kernel(kernel).expect("load DE440s kernel");
        let jd = julian_day(1980, 12, 12, 14.5); // AAPL IPO
                                                 // Sun + planets validate the VSOP87 + precession + ANISE wiring to <1°. The Moon uses a
                                                 // truncated Meeus series (~1° error), still far inside astrology's 6–8° orbs.
        let cases = [
            (Body::Sun, 1.0),
            (Body::Mercury, 1.0),
            (Body::Venus, 1.0),
            (Body::Mars, 1.0),
            (Body::Jupiter, 1.0),
            (Body::Saturn, 1.0),
            (Body::Uranus, 1.0),
            (Body::Neptune, 1.0),
            (Body::Moon, 1.5),
        ];
        for (body, tol) in cases {
            let a = AnalyticBackend.position(body, jd).unwrap().longitude;
            let b = anise.position(body, jd).unwrap().longitude;
            let diff = ((a - b + 540.0).rem_euclid(360.0) - 180.0).abs();
            assert!(
                diff < tol,
                "{}: analytic {a:.3}° vs anise {b:.3}° = {diff:.3}° (want <{tol}°)",
                body.name()
            );
        }
    }
}
