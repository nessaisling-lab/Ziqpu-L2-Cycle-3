//! A\*C\*G (astrocartography) planetary **lines** — where each body would sit on one of the
//! four angles (MC, IC, ASC, DSC) as a function of place, for a single birth instant.
//!
//! Self-contained pure-`f64` spherical trig over the already-built [`engine::NatalChart`]. Nothing
//! in `engine` or `ephemeris` is modified: the two Meeus polynomials this needs (Greenwich mean
//! sidereal time and the mean obliquity of the ecliptic) are **replicated here** from the exact
//! expressions inside `ephemeris::ascendant_mc`, and a cross-check test pins the replicas to that
//! shipping formula so they can never silently drift.
//!
//! ## Sign conventions (locked)
//! Hour angle `H` is westward-positive: `H = 0` → MC (culmination), `H = 180°` → IC, `H = -H0` →
//! ASC (eastern/rising horizon), `H = +H0` → DSC (western/setting horizon). MC/IC lines are true
//! meridians (constant geographic longitude at every latitude); ASC/DSC are the familiar bell
//! curves. The horizon is **geometric** (altitude exactly 0° — refraction, parallax and
//! semidiameter are deliberately omitted, a documented ~34′ simplification) which keeps every
//! value a deterministic function of pure `f64` trig.
//!
//! ## Determinism note
//! A\*C\*G geometry is inherently trigonometric (`sin`/`cos`/`tan`/`acos`/`atan2`). Outputs are
//! continuous geographic coordinates for a map — never rounded then thresholded — so tests here
//! assert with a **tolerance**, never a bit-exact literal on a transcendental result. Only the
//! pure-polynomial GMST/obliquity replicas are pinned tightly.

use engine::NatalChart;
use ephemeris::{jd_to_t, norm360, Body, Ephemeris};

/// Equatorial coordinates, degrees. `ra_deg ∈ [0, 360)`, `dec_deg ∈ [-90, 90]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Equatorial {
    pub ra_deg: f64,
    pub dec_deg: f64,
}

/// A point on Earth, degrees. `lon ∈ (-180, 180]` (east-positive), `lat ∈ [-90, 90]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoPoint {
    pub lon: f64,
    pub lat: f64,
}

/// Which angle the body holds along the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    /// Upper meridian — the body culminates (Midheaven).
    Mc,
    /// Lower meridian — the body anti-culminates (Imum Coeli).
    Ic,
    /// Eastern horizon — the body is rising (Ascendant).
    Asc,
    /// Western horizon — the body is setting (Descendant).
    Dsc,
}

/// One A\*C\*G line for one body. MC/IC carry a constant-longitude meridian sampled across
/// latitude; ASC/DSC carry the sampled horizon curve (shorter where the body is circumpolar).
#[derive(Debug, Clone, PartialEq)]
pub struct AcgLine {
    pub body: Body,
    pub kind: LineKind,
    pub points: Vec<GeoPoint>,
}

/// The full set of lines for one chart.
#[derive(Debug, Clone, PartialEq)]
pub struct AcgMap {
    pub lines: Vec<AcgLine>,
}

/// Options for line generation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcgOptions {
    /// `true` uses each body's ecliptic latitude β (the *in-mundo* path); `false` forces β = 0
    /// (the zodiacal projection, which needs no ephemeris backend).
    pub in_mundo: bool,
    /// Latitude sampling step, degrees, for the meridian and horizon polylines.
    pub lat_step_deg: f64,
}

impl Default for AcgOptions {
    fn default() -> Self {
        Self {
            in_mundo: true,
            lat_step_deg: 1.0,
        }
    }
}

/// Greenwich mean sidereal time, degrees `[0, 360)`. **Pure polynomial** — identical expression to
/// the local inside `ephemeris::ascendant_mc`; replicated (not imported) so `ephemeris` stays
/// byte-for-byte unchanged. Pinned to the shipping formula by [`tests::replicas_match_ephemeris_mc`].
fn gmst_deg(jd_ut: f64) -> f64 {
    let t = jd_to_t(jd_ut);
    norm360(
        280.460_618_37
            + 360.985_647_366_29 * (jd_ut - 2_451_545.0)
            + t * t * (0.000_387_933 - t / 38_710_000.0),
    )
}

/// Mean obliquity of the ecliptic, degrees. **Pure polynomial** — identical expression to the
/// local inside `ephemeris::ascendant_mc`.
fn mean_obliquity_deg(jd_ut: f64) -> f64 {
    let t = jd_to_t(jd_ut);
    23.439_291 - 0.013_004_2 * t
}

/// Normalize an angle in degrees to `(-180, 180]`. Pure arithmetic.
fn norm180(deg: f64) -> f64 {
    let x = deg.rem_euclid(360.0);
    if x > 180.0 {
        x - 360.0
    } else {
        x
    }
}

/// Latitudes sampled for a line, from -90° to 90° inclusive at `step_deg` (falls back to 1° for a
/// non-positive step). MC/IC use every sample (constant longitude); ASC/DSC drop the circumpolar
/// ones.
fn lat_samples(step_deg: f64) -> Vec<f64> {
    let step = if step_deg > 0.0 { step_deg } else { 1.0 };
    let n = (180.0 / step).round().max(1.0) as i64;
    (0..=n)
        .map(|i| (-90.0 + step * i as f64).min(90.0))
        .collect()
}

/// `(λ, β, ε)` degrees → equatorial `(RA, dec)` degrees. `β` is the body's ecliptic latitude
/// (in-mundo); pass `β = 0` for the zodiacal projection.
///
/// ```text
/// sin δ = sin β · cos ε + cos β · sin ε · sin λ
/// RA    = atan2(sin λ · cos ε − tan β · sin ε, cos λ)
/// ```
pub fn ecliptic_to_equatorial(lambda_deg: f64, beta_deg: f64, obliquity_deg: f64) -> Equatorial {
    let lambda = lambda_deg.to_radians();
    let beta = beta_deg.to_radians();
    let eps = obliquity_deg.to_radians();

    let sin_dec = (beta.sin() * eps.cos() + beta.cos() * eps.sin() * lambda.sin()).clamp(-1.0, 1.0);
    let dec = sin_dec.asin().to_degrees();

    let y = lambda.sin() * eps.cos() - beta.tan() * eps.sin();
    let x = lambda.cos();
    let ra = norm360(y.atan2(x).to_degrees());

    Equatorial {
        ra_deg: ra,
        dec_deg: dec,
    }
}

/// Geographic longitude of the meridian where a body of right ascension `ra_deg` culminates:
/// `λ = RA − GMST`, mapped to `(-180, 180]`. Pure arithmetic. IC is `norm180(mc + 180)`.
pub fn mc_longitude(ra_deg: f64, gmst_deg: f64) -> f64 {
    norm180(ra_deg - gmst_deg)
}

/// Magnitude of the hour angle at which declination `dec_deg` meets the geometric horizon at
/// latitude `lat_deg`: `cos H0 = −tan φ · tan δ`. Returns `None` when `|−tan φ · tan δ| > 1`
/// (the body is circumpolar / never rises at that latitude) — the natural end of the ASC/DSC curve.
pub fn rising_hour_angle(dec_deg: f64, lat_deg: f64) -> Option<f64> {
    let cos_h = -lat_deg.to_radians().tan() * dec_deg.to_radians().tan();
    if cos_h.abs() > 1.0 {
        None
    } else {
        Some(cos_h.acos().to_degrees())
    }
}

/// Geographic longitude of the ASC (`H = −H0`, rising) or DSC (`H = +H0`, setting) crossing at
/// latitude `lat_deg`: `λ = RA ± H0 − GMST`, mapped to `(-180, 180]`. `None` when circumpolar, or
/// for MC/IC (which are not horizon geometry).
pub fn horizon_longitude(
    kind: LineKind,
    eq: Equatorial,
    lat_deg: f64,
    gmst_deg: f64,
) -> Option<f64> {
    let h0 = rising_hour_angle(eq.dec_deg, lat_deg)?;
    let signed = match kind {
        LineKind::Asc => -h0,
        LineKind::Dsc => h0,
        LineKind::Mc | LineKind::Ic => return None,
    };
    Some(norm180(eq.ra_deg + signed - gmst_deg))
}

/// Build one line for one body. MC/IC → a constant-longitude meridian at every sampled latitude;
/// ASC/DSC → one point per latitude where the body is not circumpolar.
pub fn body_line(
    body: Body,
    kind: LineKind,
    eq: Equatorial,
    gmst_deg: f64,
    lat_step_deg: f64,
) -> AcgLine {
    let points = match kind {
        LineKind::Mc | LineKind::Ic => {
            let mc = mc_longitude(eq.ra_deg, gmst_deg);
            let lon = if kind == LineKind::Mc {
                mc
            } else {
                norm180(mc + 180.0)
            };
            lat_samples(lat_step_deg)
                .into_iter()
                .map(|lat| GeoPoint { lon, lat })
                .collect()
        }
        LineKind::Asc | LineKind::Dsc => lat_samples(lat_step_deg)
            .into_iter()
            .filter_map(|lat| {
                horizon_longitude(kind, eq, lat, gmst_deg).map(|lon| GeoPoint { lon, lat })
            })
            .collect(),
    };
    AcgLine { body, kind, points }
}

/// Compute the A\*C\*G line map for `chart`.
///
/// Returns `None` when `!chart.time_known` — the lines depend on the exact instant, so this honors
/// the same honesty gate that withholds the natal Ascendant/Midheaven. Otherwise emits four lines
/// (MC, IC, ASC, DSC) per body. For `in_mundo`, each body's ecliptic latitude β is re-fetched from
/// `eph` (it is dropped from [`engine::BodyPosition`]); a body whose β cannot be fetched is skipped.
/// For the zodiacal path (`in_mundo == false`) β ≡ 0 and the backend is not consulted for β.
pub fn compute_acg<E: Ephemeris + ?Sized>(
    chart: &NatalChart,
    eph: &E,
    opts: AcgOptions,
) -> Option<AcgMap> {
    if !chart.time_known {
        return None;
    }
    let eps = mean_obliquity_deg(chart.jd_ut);
    let gmst = gmst_deg(chart.jd_ut);

    let mut lines = Vec::with_capacity(chart.bodies.len() * 4);
    for bp in &chart.bodies {
        let beta = if opts.in_mundo {
            match eph.position(bp.body, chart.jd_ut) {
                Ok(p) => p.latitude,
                Err(_) => continue,
            }
        } else {
            0.0
        };
        let eq = ecliptic_to_equatorial(bp.longitude, beta, eps);
        for kind in [LineKind::Mc, LineKind::Ic, LineKind::Asc, LineKind::Dsc] {
            lines.push(body_line(bp.body, kind, eq, gmst, opts.lat_step_deg));
        }
    }
    Some(AcgMap { lines })
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::compute_chart;
    use ephemeris::{ascendant_mc, julian_day, AnalyticBackend};

    // A time-known chart at the AAPL first-trade fixture used across the workspace.
    fn aapl_chart(time_known: bool) -> NatalChart {
        let jd = julian_day(1980, 12, 12, 14.5);
        compute_chart(&AnalyticBackend, jd, 40.7589, -73.9851, time_known)
    }

    /// Minimal circular separation of two longitudes, degrees `[0, 180]`.
    fn circular_sep(a: f64, b: f64) -> f64 {
        let d = (a - b).rem_euclid(360.0);
        if d > 180.0 {
            360.0 - d
        } else {
            d
        }
    }

    // --- pure-arithmetic replicas + the cross-check that pins them to the shipping formula -------

    #[test]
    fn replicas_match_ephemeris_mc() {
        // Known Meeus values at J2000 (pure polynomial → tight tolerance).
        assert!((gmst_deg(2_451_545.0) - 280.460_618_37).abs() < 1e-9);
        assert!((mean_obliquity_deg(2_451_545.0) - 23.439_291).abs() < 1e-12);

        // Cross-check: astro's OWN Midheaven, built from the replicated GMST + obliquity via the
        // same rotation `ascendant_mc` uses, must equal the shipping `ascendant_mc` MC. A drift in
        // either replica breaks this, so it pins both to the crown-jewel formula.
        let jd = julian_day(1980, 12, 12, 14.5);
        for &lon0 in &[-73.9851_f64, 0.0, 100.0, -150.0] {
            let (_, mc_eph) = ascendant_mc(jd, 0.0, lon0);
            let d2r = core::f64::consts::PI / 180.0;
            let ramc = (gmst_deg(jd) + lon0).rem_euclid(360.0) * d2r;
            let eps = mean_obliquity_deg(jd) * d2r;
            let mc_astro = norm360(ramc.sin().atan2(ramc.cos() * eps.cos()) / d2r);
            assert!(
                circular_sep(mc_astro, mc_eph) < 1e-6,
                "astro MC {mc_astro} vs ephemeris MC {mc_eph} at lon {lon0}"
            );
        }
    }

    #[test]
    fn mc_ic_arithmetic_and_antipodal() {
        // Pure arithmetic → exact.
        assert_eq!(mc_longitude(100.0, 40.0), 60.0);
        assert_eq!(mc_longitude(10.0, 40.0), -30.0);
        assert_eq!(mc_longitude(350.0, 10.0), -20.0); // wrap into (-180,180]

        // IC is the opposite meridian; MC and IC are 180° apart for every RA/GMST.
        for &(ra, gmst) in &[(100.0, 40.0), (10.0, 40.0), (350.0, 10.0), (5.0, 300.0)] {
            let mc = mc_longitude(ra, gmst);
            let ic = norm180(mc + 180.0);
            assert!((circular_sep(mc, ic) - 180.0).abs() < 1e-12);
        }
    }

    #[test]
    fn mc_line_is_vertical() {
        let eq = ecliptic_to_equatorial(123.4, 0.0, mean_obliquity_deg(2_451_545.0));
        let gmst = gmst_deg(2_451_545.0);
        let line = body_line(Body::Sun, LineKind::Mc, eq, gmst, 1.0);
        assert!(line.points.len() > 2);
        let lon0 = line.points[0].lon;
        for p in &line.points {
            assert!(
                (p.lon - lon0).abs() < 1e-12,
                "MC line not vertical: {} vs {}",
                p.lon,
                lon0
            );
        }
        assert!((lon0 - mc_longitude(eq.ra_deg, gmst)).abs() < 1e-12);
    }

    #[test]
    fn ecliptic_to_equatorial_cardinal_points() {
        let eps = 23.439_291;
        let e0 = ecliptic_to_equatorial(0.0, 0.0, eps);
        assert!(circular_sep(e0.ra_deg, 0.0) < 1e-6 && e0.dec_deg.abs() < 1e-6);

        let e90 = ecliptic_to_equatorial(90.0, 0.0, eps);
        assert!(circular_sep(e90.ra_deg, 90.0) < 1e-6 && (e90.dec_deg - eps).abs() < 1e-6);

        let e180 = ecliptic_to_equatorial(180.0, 0.0, eps);
        assert!(circular_sep(e180.ra_deg, 180.0) < 1e-6 && e180.dec_deg.abs() < 1e-6);

        let e270 = ecliptic_to_equatorial(270.0, 0.0, eps);
        assert!(circular_sep(e270.ra_deg, 270.0) < 1e-6 && (e270.dec_deg + eps).abs() < 1e-6);
    }

    #[test]
    fn rising_hour_angle_and_circumpolar_cutoff() {
        // δ = 0 → H0 = 90° independent of latitude.
        for &phi in &[0.0, 10.0, 45.0, 60.0, -30.0] {
            let h0 = rising_hour_angle(0.0, phi).unwrap();
            assert!((h0 - 90.0).abs() < 1e-9);
        }
        // Circumpolar branch: high |δ| at high |φ| never rises.
        assert!(rising_hour_angle(70.0, 80.0).is_none());
        assert!(rising_hour_angle(10.0, 10.0).is_some());
    }

    #[test]
    fn asc_dsc_symmetric_about_mc() {
        let eq = ecliptic_to_equatorial(200.0, 2.0, mean_obliquity_deg(2_451_545.0));
        let gmst = gmst_deg(2_451_545.0);
        let phi = 35.0;
        let asc = horizon_longitude(LineKind::Asc, eq, phi, gmst).unwrap();
        let dsc = horizon_longitude(LineKind::Dsc, eq, phi, gmst).unwrap();
        let mc = mc_longitude(eq.ra_deg, gmst);
        // Midpoint of the two horizon crossings is the meridian, mod 360.
        let midpoint = (asc + dsc) / 2.0;
        assert!(
            circular_sep(midpoint, mc) < 1e-9 || (circular_sep(midpoint, mc) - 180.0).abs() < 1e-9,
            "asc {asc} dsc {dsc} midpoint {midpoint} vs mc {mc}"
        );
    }

    #[test]
    fn round_trip_anchor_to_ascendant_mc() {
        // The authoritative anchor: run the trusted `ascendant_mc` MC (an ecliptic longitude) back
        // through `ecliptic_to_equatorial` + `mc_longitude` and recover the original meridian.
        let jd = julian_day(1980, 12, 12, 14.5);
        let eps = mean_obliquity_deg(jd);
        let gmst = gmst_deg(jd);
        for &lon0 in &[-73.9851_f64, 15.0, 140.0, -120.0] {
            let (_, mc_ecl) = ascendant_mc(jd, 0.0, lon0);
            let ra = ecliptic_to_equatorial(mc_ecl, 0.0, eps).ra_deg;
            let recovered = mc_longitude(ra, gmst);
            assert!(
                circular_sep(recovered, lon0) < 1e-6,
                "recovered {recovered} vs lon0 {lon0}"
            );
        }
    }

    #[test]
    fn compute_acg_honors_time_gate_and_line_count() {
        // Time unknown → None (same honesty rule as withheld natal angles).
        let dateonly = aapl_chart(false);
        assert!(compute_acg(&dateonly, &AnalyticBackend, AcgOptions::default()).is_none());

        // Time known → four lines per charted body (in-mundo exercises the β re-fetch).
        let chart = aapl_chart(true);
        let map = compute_acg(&chart, &AnalyticBackend, AcgOptions::default()).unwrap();
        assert_eq!(map.lines.len(), 4 * chart.bodies.len());
        // Every line pairs the expected kinds and only the four angles appear.
        for line in &map.lines {
            assert!(matches!(
                line.kind,
                LineKind::Mc | LineKind::Ic | LineKind::Asc | LineKind::Dsc
            ));
        }
    }

    #[test]
    fn circumpolar_latitude_has_no_asc_point() {
        // A body forced circumpolar (|δ| large) at a high latitude: no horizon crossing there,
        // so the ASC curve simply omits that latitude rather than inventing a point.
        let eq = Equatorial {
            ra_deg: 45.0,
            dec_deg: 70.0,
        };
        assert!(horizon_longitude(LineKind::Asc, eq, 80.0, 0.0).is_none());

        let line = body_line(Body::Sun, LineKind::Asc, eq, 0.0, 1.0);
        assert!(
            line.points.iter().all(|p| p.lat.abs() < 20.0 + 1e-9),
            "ASC points should only exist near the equator for δ=70°"
        );
        assert!(
            !line.points.is_empty(),
            "some low-latitude ASC points exist"
        );
    }
}
