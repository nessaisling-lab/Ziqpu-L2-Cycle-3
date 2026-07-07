//! Chiron via a committed table of JPL Horizons geocentric ecliptic longitudes.
//!
//! Horizons emits Chiron (asteroid 2060) as an SPK "Type 21" that ANISE cannot read, and no
//! analytic planetary theory includes it — so we bundle sampled geocentric ecliptic-of-date
//! longitudes (1900–2100, 2-day steps) and linearly interpolate. Works on every backend, no
//! runtime kernel. Regenerate with `scripts/gen-chiron-table.sh`.

use crate::{julian_day, norm360};

/// f32 little-endian ObsEcLon samples (degrees) at 2-day steps from 1900-01-01, from JPL Horizons.
const CHIRON_LONS: &[u8] = include_bytes!("../data/chiron.bin");
const STEP_DAYS: f64 = 2.0;

fn sample_count() -> usize {
    CHIRON_LONS.len() / 4
}

fn sample(i: usize) -> f64 {
    let o = i * 4;
    f32::from_le_bytes([
        CHIRON_LONS[o],
        CHIRON_LONS[o + 1],
        CHIRON_LONS[o + 2],
        CHIRON_LONS[o + 3],
    ]) as f64
}

/// Geocentric ecliptic-of-date longitude (°) of Chiron for a Julian day (UT), or `None` if the
/// date is outside the table's 1900–2100 range.
pub fn chiron_longitude(jd_ut: f64) -> Option<f64> {
    let start = julian_day(1900, 1, 1, 0.0);
    let n = sample_count();
    let x = (jd_ut - start) / STEP_DAYS;
    if x < 0.0 || x > (n - 1) as f64 {
        return None;
    }
    let i = x.floor() as usize;
    if i + 1 >= n {
        return Some(norm360(sample(n - 1)));
    }
    let frac = x - i as f64;
    let a = sample(i);
    let b = sample(i + 1);
    // Shortest-arc linear interpolation across the 0°/360° seam.
    let mut d = b - a;
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    Some(norm360(a + d * frac))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chiron_matches_horizons_reference() {
        // Horizons ObsEcLon for asteroid 2060 at 1980-12-12 14:30 UT = 44.2277°.
        let jd = julian_day(1980, 12, 12, 14.5);
        let lon = chiron_longitude(jd).expect("in table range");
        let diff = ((lon - 44.2277 + 540.0).rem_euclid(360.0) - 180.0).abs();
        assert!(
            diff < 0.1,
            "Chiron {lon:.3}° vs Horizons 44.228° = {diff:.3}°"
        );
    }

    #[test]
    fn out_of_range_is_none() {
        assert!(chiron_longitude(julian_day(1850, 1, 1, 0.0)).is_none());
        assert!(chiron_longitude(julian_day(2200, 1, 1, 0.0)).is_none());
    }

    #[test]
    fn endpoints_present() {
        assert!(chiron_longitude(julian_day(1900, 1, 2, 0.0)).is_some());
        assert!(chiron_longitude(julian_day(2099, 1, 1, 0.0)).is_some());
    }
}
