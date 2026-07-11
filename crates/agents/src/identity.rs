//! Anonymous identity — a generated handle, **no email and no PII** (PRD N4 direction: "anonymous
//! but with real security"). The seeker is known by a two-word cosmic handle that *feels like a
//! gaming avatar*, never by a login.
//!
//! Two ways to get one, both offline and dependency-free:
//! - [`anon_handle_for`] derives a **stable** handle from the birth moment — the same chart always
//!   yields the same name, so a re-import or a relaunch shows the same identity ("your cosmic
//!   handle"). This ties into the N4 chart-derived avatar.
//! - [`anon_handle_reroll`] mixes in a caller-supplied nonce so the seeker can **re-roll** for a
//!   different name — the gaming-avatar affordance. The `agents` crate stays deterministic (no
//!   `rand`); the UI supplies the entropy.
//!
//! The hash is a hand-rolled **FNV-1a**, deliberately *not* `std::hash::DefaultHasher` — the std
//! hasher's output is explicitly unstable across Rust versions, which would reshuffle every
//! seeker's handle on a toolchain bump. FNV-1a is fixed forever, so identities are stable.

use crate::types::BirthMoment;

/// Evocative first words — sky, night, measure, and a little Sumerian colour (the Nisaba lineage).
/// A power of two (16) so the index is a clean mask. Owner-tunable: this is the product's voice.
const EPITHETS: [&str; 16] = [
    "Quiet",
    "Amber",
    "Lapis",
    "Waxing",
    "Distant",
    "Gilded",
    "Nightfall",
    "Ashen",
    "Cobalt",
    "Wandering",
    "Silent",
    "Umber",
    "Hollow",
    "Sable",
    "First",
    "Long",
];

/// Second words — celestial bodies, Sumerian figures, and horned/field creatures (wheat + stars).
/// Also 16, decorrelated from the epithet by using a different slice of the seed.
const NOUNS: [&str; 16] = [
    "Auroch", "Lamassu", "Herald", "Scribe", "Comet", "Zenith", "Meridian", "Reed", "Sheaf",
    "Oryx", "Sentinel", "Lodestar", "Warden", "Mooncalf", "Furrow", "Tablet",
];

/// FNV-1a 64-bit over raw bytes — small, allocation-free, and **stable across compiler versions**
/// (unlike `DefaultHasher`), which is the whole point: a birth moment must always hash the same.
fn fnv1a(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut h = OFFSET;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(PRIME);
    }
    h
}

/// A stable seed for the seeker's default handle, derived from every field of the birth moment.
/// The time is folded in with a present/absent flag so an unknown-time chart still hashes cleanly
/// and distinctly from the same date at midnight. Lat/lon are quantised to 1e-4° (~11 m) so tiny
/// float noise in a re-picked place doesn't flip the identity.
pub fn handle_seed(birth: &BirthMoment) -> u64 {
    use chrono::{Datelike, Timelike};
    let mut buf = Vec::with_capacity(48);
    buf.extend_from_slice(&birth.date.year().to_le_bytes());
    buf.push(birth.date.month() as u8);
    buf.push(birth.date.day() as u8);
    match birth.time {
        Some(t) => {
            buf.push(1);
            buf.push(t.hour() as u8);
            buf.push(t.minute() as u8);
        }
        None => buf.extend_from_slice(&[0, 0, 0]),
    }
    buf.extend_from_slice(birth.tz.name().as_bytes());
    buf.extend_from_slice(&((birth.lat * 1e4).round() as i64).to_le_bytes());
    buf.extend_from_slice(&((birth.lon * 1e4).round() as i64).to_le_bytes());
    fnv1a(&buf)
}

/// A splitmix64 finalizer — avalanches a seed so that even small, sequential inputs (0, 1, 2, …)
/// spread across the whole 64-bit range. Without this, low seeds share high bits and the epithet
/// (drawn from those bits) would never vary.
fn avalanche(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

/// Map a seed to a two-word handle (e.g. `"Lapis Scribe"`). The seed is avalanched first, then the
/// epithet and noun are indexed from **different nibbles** of the mixed value so they vary
/// independently — no lock-step pairing, and adjacent seeds land on unrelated names.
pub fn anon_handle(seed: u64) -> String {
    let m = avalanche(seed);
    let epithet = EPITHETS[((m >> 8) as usize) & (EPITHETS.len() - 1)];
    let noun = NOUNS[(m as usize) & (NOUNS.len() - 1)];
    format!("{epithet} {noun}")
}

/// The seeker's **default** handle — derived from, and stable for, their birth chart.
pub fn anon_handle_for(birth: &BirthMoment) -> String {
    anon_handle(handle_seed(birth))
}

/// A re-rolled handle: the seed mixed with a caller-supplied `nonce` (a click counter, a timestamp
/// — the UI's choice) through the golden-ratio constant so successive nonces spread across the
/// table rather than marching in step. `nonce = 0` returns the chart-default handle unchanged.
pub fn anon_handle_reroll(seed: u64, nonce: u64) -> String {
    let mixed = seed ^ nonce.wrapping_mul(0x9E3779B97F4A7C15);
    anon_handle(mixed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    fn demo() -> BirthMoment {
        BirthMoment {
            date: NaiveDate::from_ymd_opt(1990, 5, 15).unwrap(),
            time: Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
            tz: chrono_tz::America::New_York,
            lat: 40.7128,
            lon: -74.0060,
        }
    }

    #[test]
    fn handle_is_deterministic_for_a_seed_and_a_chart() {
        assert_eq!(anon_handle(42), anon_handle(42));
        assert_eq!(anon_handle_for(&demo()), anon_handle_for(&demo()));
        // Nonce 0 == the chart default (documented contract).
        let s = handle_seed(&demo());
        assert_eq!(anon_handle_reroll(s, 0), anon_handle(s));
    }

    #[test]
    fn handle_is_two_titlecase_words() {
        for seed in [0u64, 1, 42, 999, u64::MAX, 0x1234_5678_9abc_def0] {
            let h = anon_handle(seed);
            let words: Vec<&str> = h.split_whitespace().collect();
            assert_eq!(words.len(), 2, "handle {h:?} should be two words");
            for w in words {
                assert!(!w.is_empty());
                assert!(
                    w.chars().next().unwrap().is_ascii_uppercase(),
                    "word {w:?} should be title-case"
                );
            }
        }
    }

    #[test]
    fn a_missing_birth_time_still_yields_a_stable_handle() {
        let dateonly = BirthMoment {
            time: None,
            ..demo()
        };
        assert_eq!(anon_handle_for(&dateonly), anon_handle_for(&dateonly));
        // Unknown-time and midnight-of-the-same-date are distinct identities.
        let midnight = BirthMoment {
            time: Some(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            ..demo()
        };
        assert_ne!(handle_seed(&dateonly), handle_seed(&midnight));
    }

    #[test]
    fn handles_spread_across_the_table() {
        // 256 distinct pairings exist (16×16); a run of seeds should visit a healthy chunk of them,
        // proving the two indices really are decorrelated rather than collapsing onto a diagonal.
        let set: std::collections::HashSet<String> = (0u64..600).map(anon_handle).collect();
        assert!(
            set.len() > 120,
            "only {} distinct handles — too clumped",
            set.len()
        );
    }

    #[test]
    fn reroll_actually_changes_the_name() {
        // Over a handful of nonces the re-roll should produce several different names from one seed.
        let seed = handle_seed(&demo());
        let rolled: std::collections::HashSet<String> =
            (1u64..=12).map(|n| anon_handle_reroll(seed, n)).collect();
        assert!(
            rolled.len() >= 5,
            "re-roll too sticky: {} names",
            rolled.len()
        );
    }

    #[test]
    fn quantises_negligible_place_jitter_to_the_same_identity() {
        // A re-picked place a few microdegrees off must not flip the handle (11 m quantisation).
        let jittered = BirthMoment {
            lat: 40.7128 + 1e-6,
            lon: -74.0060 - 2e-6,
            ..demo()
        };
        assert_eq!(handle_seed(&jittered), handle_seed(&demo()));
    }
}
