//! The Vedic layer — whole-sign houses, nakshatras, and the Vimshottari dasha cycle.
//!
//! # What makes this Vedic rather than a shifted Western chart
//!
//! `ephemeris::Zodiac::Sidereal` gave charts the right *frame* — positions measured from a fixed
//! stellar origin rather than the precessing equinox. That alone is not a Vedic reading; it is a
//! Western chart with every degree moved back about 24°.
//!
//! Three things here are structurally different, not merely offset:
//!
//! - **Whole-sign houses.** The first house is the *entire sign* the ascendant falls in, not a
//!   30° arc starting at the ascendant's exact degree. A planet in the same sign as the ascendant is
//!   in the first house whether it sits at 1° or 29°. This is the oldest house system in use and the
//!   standard one in Vedic practice, and it makes house placement independent of birth-time
//!   precision in a way the quadrant systems are not — which matters here, because most of this
//!   app's subjects have no recorded clock time at all.
//! - **Nakshatras.** The ecliptic divided into 27 lunar mansions of 13°20' each, tracking the Moon's
//!   daily motion rather than the Sun's monthly. There is no Western equivalent.
//! - **Vimshottari dasha.** A 120-year cycle of planetary periods whose *starting point* is fixed by
//!   the Moon's exact position at birth. It is the reason nakshatra precision matters: the Moon
//!   moves ~13°/day, so a day's error moves the whole timeline by years.
//!
//! # Precision, honestly
//!
//! The dasha depends on the Moon's longitude, which depends on the birth moment. Where the clock
//! time is unknown — most of this app's subjects — the dasha computed from a noon assumption can be
//! wrong by a meaningful fraction of a year. [`DashaTimeline::from_moon`] takes the known-ness with
//! it and [`DashaTimeline::caveat`] says so, on the same principle as every other unknown here: the
//! number is still computed, and it arrives labelled.

use ephemeris::norm360;

/// Degrees per nakshatra: 360 / 27.
pub const NAKSHATRA_ARC: f64 = 360.0 / 27.0;

/// The 27 nakshatras in zodiacal order, starting from 0° sidereal Aries.
pub const NAKSHATRAS: [&str; 27] = [
    "Ashwini",
    "Bharani",
    "Krittika",
    "Rohini",
    "Mrigashira",
    "Ardra",
    "Punarvasu",
    "Pushya",
    "Ashlesha",
    "Magha",
    "Purva Phalguni",
    "Uttara Phalguni",
    "Hasta",
    "Chitra",
    "Swati",
    "Vishakha",
    "Anuradha",
    "Jyeshtha",
    "Mula",
    "Purva Ashadha",
    "Uttara Ashadha",
    "Shravana",
    "Dhanishta",
    "Shatabhisha",
    "Purva Bhadrapada",
    "Uttara Bhadrapada",
    "Revati",
];

/// The nine dasha lords, in Vimshottari order, with their period lengths in years.
///
/// The nine sum to **120**, which is the whole point of the system — the cycle is a human lifetime
/// idealised, and the order repeats across the 27 nakshatras exactly three times.
pub const VIMSHOTTARI: [(&str, f64); 9] = [
    ("Ketu", 7.0),
    ("Venus", 20.0),
    ("Sun", 6.0),
    ("Moon", 10.0),
    ("Mars", 7.0),
    ("Rahu", 18.0),
    ("Jupiter", 16.0),
    ("Saturn", 19.0),
    ("Mercury", 17.0),
];

/// Total years in one full Vimshottari cycle.
pub const VIMSHOTTARI_YEARS: f64 = 120.0;

/// Where a longitude falls among the 27 lunar mansions.
#[derive(Debug, Clone, PartialEq)]
pub struct Nakshatra {
    /// 0-based index into [`NAKSHATRAS`].
    pub index: usize,
    pub name: &'static str,
    /// How far into this nakshatra the point sits, in degrees (`0.0..13.333…`).
    pub degrees_in: f64,
    /// The same, as a fraction (`0.0..1.0`) — what the dasha balance is computed from.
    pub fraction: f64,
    /// Which quarter (pada) of the nakshatra, 1–4. Each pada is 3°20'.
    pub pada: u8,
    /// The Vimshottari lord ruling this nakshatra.
    pub lord: &'static str,
}

/// Which nakshatra a **sidereal** ecliptic longitude falls in.
///
/// Takes a sidereal longitude by contract. Handing it a tropical one produces a confident, wrong
/// answer roughly two nakshatras away — the caller converts via `ephemeris::Zodiac`, and
/// [`crate::chart::NatalChart::zodiac`] records which frame a chart is already in.
pub fn nakshatra_of(sidereal_longitude: f64) -> Nakshatra {
    let lon = norm360(sidereal_longitude);
    let index = ((lon / NAKSHATRA_ARC).floor() as usize).min(26);
    let degrees_in = lon - index as f64 * NAKSHATRA_ARC;
    let fraction = degrees_in / NAKSHATRA_ARC;
    Nakshatra {
        index,
        name: NAKSHATRAS[index],
        degrees_in,
        fraction,
        // 4 padas per nakshatra; clamped so a value landing exactly on the boundary reads as 4
        // rather than a nonexistent 5th quarter.
        pada: ((fraction * 4.0).floor() as u8 + 1).min(4),
        // The nine lords repeat every nine nakshatras — three full cycles across the 27.
        lord: VIMSHOTTARI[index % 9].0,
    }
}

/// One planetary period in the timeline.
#[derive(Debug, Clone, PartialEq)]
pub struct DashaPeriod {
    pub lord: &'static str,
    /// Years from birth at which this period begins. The first is negative when the birth falls
    /// part-way through its opening period — which it almost always does.
    pub starts_years: f64,
    pub length_years: f64,
}

impl DashaPeriod {
    pub fn ends_years(&self) -> f64 {
        self.starts_years + self.length_years
    }
}

/// The Vimshottari sequence for one birth moment.
#[derive(Debug, Clone, PartialEq)]
pub struct DashaTimeline {
    pub moon_nakshatra: Nakshatra,
    /// Years of the opening period still to run at birth.
    pub balance_years: f64,
    /// One full 120-year cycle from birth, in order.
    pub periods: Vec<DashaPeriod>,
    /// Whether the birth moment carried a real clock time.
    pub time_known: bool,
}

impl DashaTimeline {
    /// Build the timeline from the Moon's **sidereal** longitude at birth.
    ///
    /// The opening period is not entered at its start: the birth lands part-way through whichever
    /// period rules the Moon's nakshatra, and what remains is that lord's full length times the
    /// portion of the nakshatra still ahead of the Moon. Everything after follows the fixed order.
    pub fn from_moon(moon_sidereal_longitude: f64, time_known: bool) -> Self {
        let moon_nakshatra = nakshatra_of(moon_sidereal_longitude);
        let start = moon_nakshatra.index % 9;
        let (_, first_len) = VIMSHOTTARI[start];

        // What is LEFT of the opening period, not what has elapsed.
        let balance_years = first_len * (1.0 - moon_nakshatra.fraction);

        let mut periods = Vec::with_capacity(9);
        // The opening period began before birth; recording its true start as a negative offset keeps
        // every later period's arithmetic a plain running sum instead of a special case.
        let mut cursor = balance_years - first_len;
        for step in 0..9 {
            let (lord, length_years) = VIMSHOTTARI[(start + step) % 9];
            periods.push(DashaPeriod {
                lord,
                starts_years: cursor,
                length_years,
            });
            cursor += length_years;
        }

        Self {
            moon_nakshatra,
            balance_years,
            periods,
            time_known,
        }
    }

    /// The lord ruling at `years_after_birth`, or `None` past the end of the cycle.
    pub fn lord_at(&self, years_after_birth: f64) -> Option<&DashaPeriod> {
        self.periods
            .iter()
            .find(|p| years_after_birth >= p.starts_years && years_after_birth < p.ends_years())
    }

    /// What an unknown birth time costs this timeline, or `None` when the time is known.
    ///
    /// Quantified rather than hand-waved: the Moon moves about 13.2°/day, a nakshatra is 13°20', so
    /// a full day of uncertainty is very nearly one whole nakshatra — which changes the opening
    /// lord, not merely the balance. That is a different timeline, not a slightly shifted one.
    pub fn caveat(&self) -> Option<&'static str> {
        (!self.time_known).then_some(
            "This dasha rests on a date without a recorded clock time. The Moon covers about one \
             whole nakshatra per day, so the opening period — and therefore every date below it — \
             could belong to a different lord entirely. Read it as the shape of a cycle, not as \
             dated predictions.",
        )
    }
}

/// A whole-sign house division.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WholeSignHouses {
    /// 0-based index of the sign holding the ascendant — the first house.
    pub ascendant_sign: usize,
}

impl WholeSignHouses {
    /// Build from an ascendant longitude in whatever zodiac the chart uses.
    pub fn from_ascendant(ascendant_longitude: f64) -> Self {
        Self {
            ascendant_sign: (norm360(ascendant_longitude) / 30.0).floor() as usize % 12,
        }
    }

    /// Which house (1–12) a longitude falls in.
    ///
    /// The whole sign, not a 30° arc from the ascendant's degree — a planet at 1° of the rising sign
    /// and one at 29° are both in the first house. That is the substantive difference from the
    /// quadrant systems, and it is why this survives an unknown birth time better: the house only
    /// changes when the ascendant crosses a whole sign boundary, roughly every two hours, rather
    /// than drifting continuously.
    pub fn house_of(&self, longitude: f64) -> u8 {
        let sign = (norm360(longitude) / 30.0).floor() as usize % 12;
        ((sign + 12 - self.ascendant_sign) % 12) as u8 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_twenty_seven_mansions_tile_the_circle() {
        // Boundaries land where they should, and nothing falls off either end.
        assert_eq!(nakshatra_of(0.0).name, "Ashwini");
        assert_eq!(nakshatra_of(0.0).index, 0);
        assert_eq!(nakshatra_of(13.0).name, "Ashwini");
        assert_eq!(nakshatra_of(13.34).name, "Bharani");
        assert_eq!(nakshatra_of(359.99).name, "Revati");
        assert_eq!(nakshatra_of(359.99).index, 26);

        // Wrapping is handled rather than panicking.
        assert_eq!(nakshatra_of(360.0).name, "Ashwini");
        assert_eq!(nakshatra_of(-1.0).name, "Revati");

        // Every degree of the circle lands in exactly one, and the arcs are equal.
        for i in 0..27 {
            let mid = i as f64 * NAKSHATRA_ARC + NAKSHATRA_ARC / 2.0;
            let n = nakshatra_of(mid);
            assert_eq!(n.index, i);
            assert!(
                (n.fraction - 0.5).abs() < 1e-9,
                "{} fraction {}",
                n.name,
                n.fraction
            );
            // The midpoint is EXACTLY the pada 2/3 boundary, so which side it falls on is decided
            // by the last bit of the division — my first version asserted 3 and got 2. Asserting
            // either is asserting a coin flip; the real property is that it is one of the two
            // adjacent quarters and never anything else. Interior points are pinned exactly in
            // `padas_quarter_each_mansion`.
            assert!(
                n.pada == 2 || n.pada == 3,
                "{} midpoint landed in pada {}",
                n.name,
                n.pada
            );
        }
    }

    #[test]
    fn padas_quarter_each_mansion() {
        let at = |deg: f64| nakshatra_of(deg).pada;
        assert_eq!(at(0.0), 1);
        assert_eq!(at(3.0), 1);
        assert_eq!(at(3.4), 2); // 3°20' boundary
        assert_eq!(at(6.7), 3);
        assert_eq!(at(10.1), 4);
        // Exactly on the closing boundary reads as the 4th quarter, never a 5th.
        assert_eq!(nakshatra_of(NAKSHATRA_ARC - 1e-9).pada, 4);
    }

    /// The nine lords must sum to 120 years, or every date in every timeline is wrong.
    #[test]
    fn the_cycle_is_one_hundred_and_twenty_years() {
        let total: f64 = VIMSHOTTARI.iter().map(|(_, y)| y).sum();
        assert!((total - VIMSHOTTARI_YEARS).abs() < 1e-9, "got {total}");
        assert_eq!(VIMSHOTTARI.len(), 9);
        // The lord order repeats exactly three times across the 27 nakshatras.
        assert_eq!(nakshatra_of(0.0).lord, "Ketu");
        assert_eq!(nakshatra_of(9.0 * NAKSHATRA_ARC).lord, "Ketu");
        assert_eq!(nakshatra_of(18.0 * NAKSHATRA_ARC).lord, "Ketu");
    }

    /// The opening period is the REMAINDER, not the whole thing — the classic way to get this wrong.
    #[test]
    fn the_opening_period_is_what_is_left_not_what_has_passed() {
        // Moon exactly at the start of Ashwini (Ketu, 7 years): the whole 7 years remain.
        let t = DashaTimeline::from_moon(0.0, true);
        assert_eq!(t.moon_nakshatra.name, "Ashwini");
        assert_eq!(t.periods[0].lord, "Ketu");
        assert!((t.balance_years - 7.0).abs() < 1e-9, "{}", t.balance_years);

        // Exactly halfway through Ashwini: half of Ketu's 7 years remain.
        let half = DashaTimeline::from_moon(NAKSHATRA_ARC / 2.0, true);
        assert!(
            (half.balance_years - 3.5).abs() < 1e-9,
            "{}",
            half.balance_years
        );

        // Three quarters through: a quarter left — NOT three quarters.
        let late = DashaTimeline::from_moon(NAKSHATRA_ARC * 0.75, true);
        assert!(
            (late.balance_years - 1.75).abs() < 1e-9,
            "{}",
            late.balance_years
        );
    }

    #[test]
    fn the_timeline_runs_a_full_cycle_in_order_without_gaps() {
        let t = DashaTimeline::from_moon(NAKSHATRA_ARC * 3.5, true); // mid-Rohini → Moon lord
        assert_eq!(t.moon_nakshatra.name, "Rohini");
        assert_eq!(t.periods[0].lord, "Moon");

        assert_eq!(t.periods.len(), 9);
        // Contiguous: each period starts exactly where the last ended.
        for pair in t.periods.windows(2) {
            assert!(
                (pair[0].ends_years() - pair[1].starts_years).abs() < 1e-9,
                "gap between {} and {}",
                pair[0].lord,
                pair[1].lord
            );
        }
        // The nine cover 120 years, and the sequence follows Vimshottari order from the Moon's lord.
        let span: f64 = t.periods.iter().map(|p| p.length_years).sum();
        assert!((span - VIMSHOTTARI_YEARS).abs() < 1e-9);
        assert_eq!(t.periods[1].lord, "Mars");
        assert_eq!(t.periods[2].lord, "Rahu");

        // Birth sits inside the opening period, which began before it.
        assert!(t.periods[0].starts_years < 0.0);
        assert_eq!(t.lord_at(0.0).unwrap().lord, "Moon");
        // And the balance is where the second lord takes over.
        assert_eq!(t.lord_at(t.balance_years + 0.001).unwrap().lord, "Mars");
        assert!(
            t.lord_at(200.0).is_none(),
            "past the cycle, no lord is claimed"
        );
    }

    /// An unknown clock time is quantified, not waved at.
    #[test]
    fn an_untimed_birth_says_the_opening_lord_may_be_wrong() {
        let known = DashaTimeline::from_moon(100.0, true);
        assert_eq!(known.caveat(), None);

        let unknown = DashaTimeline::from_moon(100.0, false);
        let c = unknown.caveat().expect("an untimed dasha owes a caveat");
        assert!(c.contains("nakshatra"), "{c}");
        assert!(
            c.contains("different lord"),
            "names the real consequence: {c}"
        );
    }

    /// Whole-sign: the whole sign, not a 30° arc from the ascendant's degree.
    #[test]
    fn whole_sign_houses_use_the_sign_not_the_degree() {
        // Ascendant at 25° Aries — late in the sign.
        let h = WholeSignHouses::from_ascendant(25.0);
        assert_eq!(h.ascendant_sign, 0);

        // A planet at 1° Aries is STILL first house, though it is 24° "behind" the ascendant.
        // A quadrant system would place it in the twelfth.
        assert_eq!(h.house_of(1.0), 1);
        assert_eq!(h.house_of(29.9), 1);
        // The second house begins at the next sign boundary, not 30° after the ascendant.
        assert_eq!(h.house_of(30.0), 2);
        assert_eq!(h.house_of(55.0), 2);

        // And it wraps correctly all the way round.
        assert_eq!(h.house_of(330.0), 12);
        assert_eq!(h.house_of(359.9), 12);

        // Ascendant in a late sign wraps the same way.
        let late = WholeSignHouses::from_ascendant(340.0); // Pisces
        assert_eq!(late.house_of(345.0), 1);
        assert_eq!(late.house_of(5.0), 2); // Aries
        assert_eq!(late.house_of(315.0), 12); // Aquarius
    }

    /// Every longitude lands in exactly one house, for every possible ascendant.
    #[test]
    fn the_twelve_houses_tile_the_circle_for_any_ascendant() {
        for asc_sign in 0..12 {
            let h = WholeSignHouses::from_ascendant(asc_sign as f64 * 30.0 + 15.0);
            let mut seen = [0u32; 13];
            for sign in 0..12 {
                let house = h.house_of(sign as f64 * 30.0 + 15.0);
                assert!((1..=12).contains(&house), "house {house} out of range");
                seen[house as usize] += 1;
            }
            for (house, count) in seen.iter().enumerate().skip(1) {
                assert_eq!(*count, 1, "house {house} claimed {count} signs");
            }
        }
    }
}
