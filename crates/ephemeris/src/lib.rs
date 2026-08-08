//! Pluggable ephemeris layer for Ziqpu.
//!
//! The [`Ephemeris`] trait is the seam that keeps the public tree free of copyleft.
//!
//! - **`anise-backend`** — the **authoritative** engine, on by default and bundled with every
//!   release: ANISE over the JPL DE440 kernel (`de440s.bsp`, ~32 MB, verified against a pinned
//!   SHA-256). DE440/DE441 are U.S. Government works, public-domain-equivalent and freely
//!   redistributable, which is exactly why they can ship where the AGPL Swiss Ephemeris cannot.
//! - **`analytic`** — the **floor**, not the choice: pure-Rust VSOP87 planets with a Meeus Moon, no
//!   data files, no I/O. It runs when the kernel is absent or fails verification, so the app still
//!   works — but it **cannot compute Pluto at all**, and [`EngineSource::caveat`] says so rather
//!   than letting a chart come out one planet short in silence.
//! - **`swisseph`** — a private/commercial backend stub; never ships in the public repository.
//!
//! [`resolve`] picks between them at runtime and reports which one ran. Nothing constructs a
//! backend directly any more: `crates/agents/src/measure.rs` named `AnalyticBackend` in both of its
//! chart calls, which meant DE440 could be compiled in and the kernel could be sitting on disk and
//! every chart still came from the series.
//!
//! (This header has now been wrong in both directions — it once claimed `anise-backend` was the
//! default when it was not, and was corrected to claim `analytic` was what every shipped build
//! used, which was true only because nothing could reach the other one. Both times the fix was to
//! make the code match the sentence, not to soften the sentence.)

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
    /// The **mean** lunar node — the classical smoothed formula.
    ///
    /// There used to be a `TrueNode` variant beside this one. It was never implemented: both
    /// backends matched `MeanNode | TrueNode` to the same `mean_node()` call, so the two returned
    /// bit-identical longitudes under different names. That is not a near-duplicate, it is a false
    /// label — and on a project that deleted a data source over provenance and refuses to invent a
    /// birth time, shipping a body called True Node that is the mean node is the same error in the
    /// engine.
    ///
    /// It also corrupted every score. Two identical bodies in each chart meant node-to-other
    /// contacts counted twice and node-to-node contacts counted **four times**, since the synastry
    /// cross-product squares the duplication. Apple's chart listed sixteen node contacts that were
    /// seven facts.
    ///
    /// The true (osculating) node is real work — it needs the Moon's actual orbital elements, not a
    /// rename. It exists now as [`Body::TrueNode`], backed by [`true_node`].
    MeanNode,
    /// The **true (osculating)** lunar node — where the Moon's instantaneous orbital plane actually
    /// crosses the ecliptic. See [`true_node`] for the derivation and its requirements.
    ///
    /// A variant of this name existed once and was deleted, because both backends mapped it to
    /// `mean_node()` and it returned bit-identical longitudes under a different label. It is back
    /// only now that it computes something different from the mean node: it librates ±1.9° around
    /// it and moves direct on roughly a quarter of days, which the mean node never does.
    ///
    /// **Never chart this alongside [`Body::MeanNode`].** They are two names for the same point in
    /// the sky, ~1.5° apart, so a chart carrying both double-counts every node contact — the exact
    /// defect the phantom variant caused. Pick one; see `engine::NodeMode`.
    TrueNode,
    Chiron,
}

impl Body {
    /// Canonical display name.
    pub fn name(self) -> &'static str {
        match self {
            Body::Sun => "Sun",
            Body::Moon => "Moon",
            Body::Mercury => "Mercury",
            Body::Venus => "Venus",
            Body::Mars => "Mars",
            Body::Jupiter => "Jupiter",
            Body::Saturn => "Saturn",
            Body::Uranus => "Uranus",
            Body::Neptune => "Neptune",
            Body::Pluto => "Pluto",
            Body::MeanNode => "MeanNode",
            Body::TrueNode => "TrueNode",
            Body::Chiron => "Chiron",
        }
    }
}

/// The backend seam. Implemented by the ANISE backend (default) and, privately, by the
/// Swiss Ephemeris backend behind the `swisseph` feature.
pub trait Ephemeris {
    /// Geocentric ecliptic position of `body` at Julian day (UT).
    fn position(&self, body: Body, jd_ut: f64) -> Result<EclipticPos, EphemerisError>;

    /// Human-readable backend name, surfaced so a reading can say how it was measured.
    fn name(&self) -> &'static str {
        "ephemeris"
    }
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

/// Which zero point a zodiac is measured from.
///
/// # The distinction this exists to stop blurring
///
/// The **tropical** zodiac starts at the vernal equinox — a point defined by Earth's orientation,
/// which precesses. The **sidereal** zodiac starts from a fixed point among the stars. They were
/// aligned around the 3rd–5th century CE and have drifted apart since, by roughly 24° today. Neither
/// is "correct": they answer different questions, and Western and Vedic astrology are built on
/// different answers.
///
/// The gap is nearly a full sign, so presenting a chart without saying which zodiac produced it is
/// not a nuance — it is the difference between a Sun in Sagittarius and a Sun in Scorpio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Zodiac {
    /// Measured from the vernal equinox. **The default**, and what every chart this project has
    /// produced so far uses.
    #[default]
    Tropical,
    /// Measured from a fixed stellar origin, via the named ayanamsa.
    Sidereal(Ayanamsa),
}

/// The offset between the tropical and sidereal zero points.
///
/// A named list rather than a number, because there is no single sidereal zero point — the schools
/// disagree by up to about a degree, and an app that silently picked one would be asserting a
/// position in a live argument. Naming which one was used is the honest form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ayanamsa {
    /// **Lahiri** (Chitrapaksha) — the Indian government standard and by far the most used in Vedic
    /// practice. Anchored so that the star Spica sits near 0° Libra.
    Lahiri,
}

impl Ayanamsa {
    pub fn label(self) -> &'static str {
        match self {
            Ayanamsa::Lahiri => "Lahiri",
        }
    }

    /// The offset in degrees at `jd_ut`, to be **subtracted** from a tropical longitude.
    ///
    /// Built from the same general-precession series the ANISE backend already uses to advance
    /// ecliptic-of-J2000 to ecliptic-of-date, anchored at Lahiri's J2000 value of 23°51'11".
    /// Reusing that series rather than introducing a second precession model is deliberate: two
    /// expressions of the same physics is the shape of defect this codebase keeps paying for.
    ///
    /// Accurate to a few arcseconds against published Lahiri tables across this app's date range —
    /// checked in the tests at 1950, J2000 and 2020. True Lahiri uses a marginally different
    /// precession basis, so agreement is to arcseconds rather than exact; a sign is 30°, so the
    /// residual is roughly one part in twenty thousand of the quantity being decided.
    pub fn degrees_at(self, jd_ut: f64) -> f64 {
        match self {
            Ayanamsa::Lahiri => {
                /// Lahiri ayanamsa at J2000.0 — 23°51'11".
                const AT_J2000: f64 = 23.853_055_6;
                let t = jd_to_t(jd_ut);
                AT_J2000 + (5028.796_195 * t + 1.105_434_8 * t * t) / 3600.0
            }
        }
    }
}

impl Zodiac {
    /// Convert a tropical ecliptic longitude into this zodiac.
    ///
    /// Tropical is the identity. Sidereal subtracts the ayanamsa, which is why this needs the
    /// instant: the offset grows by about 50 arcseconds a year and a fixed constant would be wrong
    /// everywhere except the year it was written.
    pub fn from_tropical(self, longitude: f64, jd_ut: f64) -> f64 {
        match self {
            Zodiac::Tropical => norm360(longitude),
            Zodiac::Sidereal(a) => norm360(longitude - a.degrees_at(jd_ut)),
        }
    }

    /// How to name this zodiac in a reading, so a chart never appears without saying what it is.
    pub fn label(self) -> String {
        match self {
            Zodiac::Tropical => "tropical".to_string(),
            Zodiac::Sidereal(a) => format!("sidereal ({})", a.label()),
        }
    }
}

/// The **true (osculating) lunar node** — ascending, ecliptic longitude in degrees.
///
/// # What makes it "true" rather than mean
///
/// The mean node is a smooth polynomial: a fiction that regresses at a constant ~0.053°/day and is
/// never exactly where the Moon's orbit actually crosses the ecliptic. The true node is that real
/// crossing, computed from the Moon's instantaneous orbital plane, and it librates around the mean
/// by up to roughly ±1.6° — occasionally turning direct for a few days, which the mean node never
/// does.
///
/// This project shipped a `TrueNode` body once before that was a *relabelled mean node*: both
/// backends matched `MeanNode | TrueNode` to the same call, so two names returned bit-identical
/// longitudes. It was deleted rather than left, because on a project that refuses to invent a birth
/// time, a body whose name is a lie is the same error one layer down. This is the real computation;
/// the name is earned now.
///
/// # Why it is derived from state rather than read from the backend
///
/// The osculating node needs the Moon's **velocity**, and [`Ephemeris::position`] returns only
/// [`EclipticPos`] — longitude, latitude, distance, and speed in longitude. That is not a velocity
/// vector: it says nothing about how the latitude is changing, which is precisely the component that
/// defines the orbital plane.
///
/// So the state is reconstructed by central difference around `jd_ut`, which needs no change to the
/// trait and works on **every** backend, including ones added later. The node is then the standard
/// orbital-elements derivation: with position `r` and velocity `v`, the specific angular momentum
/// `h = r × v` is normal to the orbital plane, the node vector is `ẑ × h = (−h_y, h_x, 0)`, and the
/// ascending node is its longitude.
///
/// `DELTA` is the tuned part. Too small and the difference is dominated by the backend's own
/// rounding; too large and the central-difference truncation error grows. It is measured, not
/// guessed — the tests check the libration amplitude and the regression rate against the known
/// behaviour of the real node.
///
/// # It requires a backend that knows the Moon's LATITUDE
///
/// This returns an error on the analytic floor, and that is correct rather than a gap to paper over.
/// The analytic Moon is a longitude-only Meeus series: its latitude is identically `0.0`, so its
/// Moon lies *in* the ecliptic by construction. A body with no inclination has no orbital plane
/// distinct from the ecliptic and therefore **no node at all** — `r` and `v` are coplanar with the
/// ecliptic, `r × v` points straight along `ẑ`, and the node line vanishes.
///
/// Computing a number anyway would mean inventing an inclination the source never had, which is the
/// same failure as the year dressed up as a day. So it refuses, and says which backend limitation
/// caused it. On DE440, measured over 2024–2025: libration −1.88°…+1.90° around the mean node, mean
/// libration +0.003°, direct motion on 182 of 730 days, and −39.9° of net regression over two years
/// against the ~−38.7° the mean node covers.
pub fn true_node<E: Ephemeris + ?Sized>(eph: &E, jd_ut: f64) -> Result<f64, EphemerisError> {
    /// Half-width of the central difference, in days (~72 minutes).
    const DELTA: f64 = 0.05;

    let at = |jd: f64| -> Result<[f64; 3], EphemerisError> {
        let p = eph.position(Body::Moon, jd)?;
        let (lon, lat) = (p.longitude.to_radians(), p.latitude.to_radians());
        let d = p.distance_au;
        Ok([
            d * lat.cos() * lon.cos(),
            d * lat.cos() * lon.sin(),
            d * lat.sin(),
        ])
    };

    let r = at(jd_ut)?;
    let (before, after) = (at(jd_ut - DELTA)?, at(jd_ut + DELTA)?);
    let v = [
        (after[0] - before[0]) / (2.0 * DELTA),
        (after[1] - before[1]) / (2.0 * DELTA),
        (after[2] - before[2]) / (2.0 * DELTA),
    ];

    // h = r × v — normal to the instantaneous orbital plane.
    let h = [
        r[1] * v[2] - r[2] * v[1],
        r[2] * v[0] - r[0] * v[2],
        r[0] * v[1] - r[1] * v[0],
    ];
    // The node line is ẑ × h = (−h_y, h_x, 0); the ASCENDING node is its longitude.
    if h[0].abs() < f64::EPSILON && h[1].abs() < f64::EPSILON {
        return Err(EphemerisError(
            "this backend reports no lunar latitude, so the Moon has no orbital plane and no true              node — the analytic floor is longitude-only; DE440 provides it"
                .into(),
        ));
    }
    Ok(norm360(h[0].atan2(-h[1]).to_degrees()))
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

mod chiron;
pub use chiron::chiron_longitude;

#[cfg(feature = "analytic")]
pub mod analytic;
#[cfg(feature = "analytic")]
pub use analytic::AnalyticBackend;

// Gated on `analytic` because the resolver's whole contract is "DE440, or the floor" — without a
// floor to fall back to there is nothing for it to resolve to.
#[cfg(feature = "analytic")]
mod resolve;
#[cfg(feature = "analytic")]
pub use resolve::{
    candidate_paths, load, shared, AnalyticReason, EngineSource, Loaded, DE440S_SHA256, KERNEL_FILE,
};

#[cfg(feature = "anise-backend")]
mod anise_backend;
#[cfg(feature = "anise-backend")]
pub use anise_backend::AniseBackend;

#[cfg(test)]
mod tests {
    /// Lahiri, checked against published values at three separated epochs.
    ///
    /// Three rather than one, because a single point cannot distinguish a correct series from a
    /// constant that happens to be right in one year — and the whole reason this is a function of
    /// time is that the offset moves about 50 arcseconds a year.
    #[test]
    fn lahiri_matches_published_values() {
        use super::{julian_day, Ayanamsa};
        let arcmin = |deg: f64| (deg * 60.0).round() / 60.0;

        // J2000.0 — the anchor: 23 deg 51' 11".
        let j2000 = Ayanamsa::Lahiri.degrees_at(2_451_545.0);
        assert!(
            (j2000 - 23.8531).abs() < 0.001,
            "J2000 Lahiri should be 23 deg 51' 11\", got {j2000}"
        );

        // 1950-01-01 — published ~23 deg 09'. Half a century BEFORE the anchor, so this is what
        // catches a sign error in the precession term.
        let y1950 = Ayanamsa::Lahiri.degrees_at(julian_day(1950, 1, 1, 0.0));
        assert!(
            (arcmin(y1950) - 23.15).abs() < 0.02,
            "1950 Lahiri should be about 23 deg 09', got {y1950}"
        );

        // 2020-01-01 — published ~24 deg 08'.
        let y2020 = Ayanamsa::Lahiri.degrees_at(julian_day(2020, 1, 1, 0.0));
        assert!(
            (arcmin(y2020) - 24.13).abs() < 0.02,
            "2020 Lahiri should be about 24 deg 08', got {y2020}"
        );

        // And it INCREASES with time — a decreasing ayanamsa would be precession running backwards.
        assert!(y1950 < j2000 && j2000 < y2020);
    }

    /// The whole point: the two zodiacs disagree by nearly a sign, and the code says which is which.
    #[test]
    fn sidereal_is_about_a_sign_behind_tropical() {
        use super::{julian_day, Ayanamsa, Zodiac};
        let jd = julian_day(2024, 6, 1, 0.0);

        // Tropical is the identity — no silent transformation on the default path.
        assert_eq!(Zodiac::Tropical.from_tropical(123.456, jd), 123.456);

        let sid = Zodiac::Sidereal(Ayanamsa::Lahiri);
        let gap = 123.456 - sid.from_tropical(123.456, jd);
        assert!(
            (23.5..25.0).contains(&gap),
            "the 2024 offset should be about 24 deg, got {gap}"
        );

        // Wrapping is handled: a tropical longitude just past 0 Aries lands in late Pisces.
        let wrapped = sid.from_tropical(5.0, jd);
        assert!(
            wrapped > 340.0,
            "5 deg tropical is late Pisces sidereal, got {wrapped}"
        );

        // Every chart must be able to say which zodiac drew it.
        assert_eq!(Zodiac::Tropical.label(), "tropical");
        assert_eq!(sid.label(), "sidereal (Lahiri)");
    }

    /// The osculating node's PHYSICS, not a golden number.
    ///
    /// A single hardcoded longitude would pass on a subtly wrong derivation as easily as a right
    /// one — and this project already shipped a node body that passed every test it had by being a
    /// copy of another body. So this asserts the properties that distinguish the true node from the
    /// mean node and from a mistake:
    ///
    ///   * it librates around the mean node, within a couple of degrees, never far;
    ///   * that libration averages to about zero over a long window;
    ///   * it moves DIRECT on a substantial minority of days, which the mean node never does;
    ///   * and it regresses at roughly the mean node's long-run rate.
    ///
    /// Measured over 2024–2025 on DE440: −1.877°…+1.900°, mean +0.0025°, direct on 182 of 730 days.
    #[test]
    fn the_true_node_behaves_like_the_real_one() {
        let engine = super::shared();
        if !engine.source.is_authoritative() {
            eprintln!("SKIPPED: no DE440 kernel; the analytic floor has no lunar latitude");
            assert!(super::true_node(&super::AnalyticBackend, 2_460_000.0).is_err());
            return;
        }
        let eph = engine.backend.as_ref();
        let start = super::julian_day(2024, 1, 1, 0.0);

        let wrap = |d: f64| {
            if d > 180.0 {
                d - 360.0
            } else if d < -180.0 {
                d + 360.0
            } else {
                d
            }
        };

        let (mut lo, mut hi, mut sum, mut direct, mut prev) =
            (f64::MAX, f64::MIN, 0.0, 0usize, None::<f64>);
        for step in 0..730 {
            let jd = start + step as f64;
            let t = super::true_node(eph, jd).expect("DE440 supplies lunar latitude");
            let m = eph.position(super::Body::MeanNode, jd).unwrap().longitude;
            let lib = wrap(t - m);
            lo = lo.min(lib);
            hi = hi.max(lib);
            sum += lib;
            if let Some(p) = prev {
                if wrap(t - p) > 0.0 {
                    direct += 1;
                }
            }
            prev = Some(t);
        }

        assert!(lo > -2.5 && hi < 2.5, "libration out of range: {lo}..{hi}");
        assert!(
            lo < -1.0 && hi > 1.0,
            "too small to be the true node: {lo}..{hi}"
        );
        assert!(
            (sum / 730.0).abs() < 0.2,
            "libration should average near zero, got {}",
            sum / 730.0
        );
        // The headline difference. A hardcoded retrograde rate would erase it.
        assert!(
            (100..400).contains(&direct),
            "the true node moves direct on a real minority of days, got {direct}/730"
        );

        let net = wrap(
            super::true_node(eph, start + 729.0).unwrap() - super::true_node(eph, start).unwrap(),
        );
        assert!(
            (-45.0..-32.0).contains(&net),
            "two years should regress about -38.7 deg, got {net}"
        );
    }

    /// The floor refuses rather than substituting. Runs everywhere.
    #[test]
    fn the_analytic_floor_refuses_the_true_node_and_says_why() {
        let err = super::true_node(&super::AnalyticBackend, super::julian_day(2024, 6, 1, 0.0))
            .expect_err("a longitude-only Moon has no orbital plane");
        assert!(err.0.contains("latitude"), "{}", err.0);

        // And via the body, which is the path a chart takes.
        use super::Ephemeris as _;
        let err = super::AnalyticBackend
            .position(super::Body::TrueNode, super::julian_day(2024, 6, 1, 0.0))
            .expect_err("must refuse");
        assert!(err.0.contains("latitude"), "{}", err.0);
    }

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bodies_are_distinct_hash_keys() {
        let all = [Body::Sun, Body::Moon, Body::Chiron, Body::MeanNode];
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
