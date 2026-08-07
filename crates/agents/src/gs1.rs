//! GS1 element strings — reading a scanned code's **own** record, including the one field that can
//! date an individual object: `AI (11) PROD DATE`.
//!
//! Everything else in this crate dates an *archetype*. [`ProductSource`](crate::ProductSource) says
//! the PlayStation 5 launched on 2020-11-12 — true of every console ever made. A GS1 production date
//! is different in kind: it is stamped on **the one item in your hand**, so it is the only birth
//! moment we can offer that is genuinely *yours*.
//!
//! # What can and cannot be read
//!
//! The retail barcode on ordinary packaging (UPC/EAN) is a bare GTIN — identity, no date. Only the
//! **GS1 2D** family (DataMatrix, GS1-128, Digital Link QR) carries Application Identifiers, and only
//! some of those include `(11)`. Encoding it is optional and driven mostly by perishables and pharma,
//! so a real production date is **uncommon but not rare** — a power feature, not the default path.
//!
//! Decoding is unencumbered: the AI standard is openly published, and a GS1 licence is required to
//! *issue* codes, not to *read* one somebody else printed.
//!
//! # The honesty rules, which are the reason this module is careful
//!
//! - **Only `(11)` is a birth.** `(17)` expiry, `(15)` best-before and `(10)` batch describe the same
//!   object but are not when it was made. They are surfaced as context and never as the moment.
//! - **A `DD` of `00` is not a day.** GS1 allows it to mean "no specific day in this month". Treating
//!   it as the 1st would be the January-the-first fabrication this project deleted 904 charts over,
//!   in miniature — so it is reported as a month and marked unchartable.
//! - **An unrecognised AI stops the parse.** Application Identifiers are 2–4 digits, and this module
//!   deliberately understands a documented subset. Guessing the length of an unknown AI would
//!   silently mis-attribute the digits that follow it to the wrong field — a wrong date is worse than
//!   a missing one — so parsing stops and returns what was read with certainty.

use chrono::NaiveDate;

/// One `(AI, value)` pair from a GS1 element string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gs1Element {
    pub ai: String,
    pub value: String,
}

/// A production date as the code actually stated it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannedDate {
    /// A real, day-precise manufacture date — the strongest birth moment this project can offer,
    /// because it belongs to the individual object rather than to its model.
    Day(NaiveDate),
    /// `DD` was `00`: the code names a month and declines to name a day. Reported, never charted.
    Month(i32, u32),
}

/// The AIs this module reads, with their **fixed** value lengths. Anything not listed is treated as
/// unknown and stops the parse rather than being guessed at (see the module note).
///
/// Date AIs are all `N6` (`YYMMDD`). `01` is the 14-digit GTIN. `10` (batch) and `21` (serial) are
/// variable-length and handled separately — they run to a separator or to the end of the string.
const FIXED_LEN: &[(&str, usize)] = &[
    ("00", 18), // SSCC
    ("01", 14), // GTIN
    ("02", 14), // GTIN of contained trade items
    ("11", 6),  // PRODUCTION DATE  ← the one that can be a birth
    ("12", 6),  // due date
    ("13", 6),  // packaging date
    ("15", 6),  // best before
    ("16", 6),  // sell by
    ("17", 6),  // expiry
    ("20", 2),  // variant
];

/// AIs that are variable-length: they run to the next separator (`FNC1`, transmitted as ASCII 29) or
/// to the end of the string.
const VARIABLE: &[&str] = &["10", "21", "22", "30", "37"];

/// The GS1 separator character — `FNC1` in the symbology, ASCII **group separator** on the wire.
const GS: char = '\u{1d}';

/// Parse a GS1 element string into its `(AI, value)` pairs.
///
/// Accepts both shapes a person can actually produce: the human-readable form with parentheses
/// (`(01)00614141000012(11)200315`), which is how codes are printed and quoted, and the raw
/// scanner transmission, where AIs run together and variable-length values end at a separator.
///
/// Returns what could be read **with certainty** — an unknown AI ends the parse rather than being
/// guessed past.
pub fn parse_gs1(input: &str) -> Vec<Gs1Element> {
    let s = input.trim();
    if s.contains('(') {
        parse_parenthesised(s)
    } else {
        parse_raw(s)
    }
}

/// `(01)00614141000012(11)200315(10)ABC` — each value runs to the next `(`.
fn parse_parenthesised(s: &str) -> Vec<Gs1Element> {
    let mut out = Vec::new();
    for chunk in s.split('(').filter(|c| !c.is_empty()) {
        let Some((ai, value)) = chunk.split_once(')') else {
            break;
        };
        let ai = ai.trim();
        if ai.is_empty() || !ai.chars().all(|c| c.is_ascii_digit()) {
            break;
        }
        out.push(Gs1Element {
            ai: ai.to_string(),
            value: value.trim_matches(GS).trim().to_string(),
        });
    }
    out
}

/// `010061414100001211200315` (+ separators before variable-length values). Walks the string, taking
/// each AI's documented length; stops at the first AI it does not know.
fn parse_raw(s: &str) -> Vec<Gs1Element> {
    let chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    let mut out = Vec::new();
    let mut i = 0usize;

    while i + 2 <= chars.len() {
        // A separator may precede the next AI; step over it.
        if chars[i] == GS {
            i += 1;
            continue;
        }
        let ai: String = chars[i..i + 2].iter().collect();
        if !ai.chars().all(|c| c.is_ascii_digit()) {
            break;
        }
        i += 2;

        if let Some((_, len)) = FIXED_LEN.iter().find(|(a, _)| *a == ai) {
            if i + len > chars.len() {
                break; // truncated — better to stop than to read a short value as complete
            }
            let value: String = chars[i..i + len].iter().collect();
            i += len;
            out.push(Gs1Element { ai, value });
        } else if VARIABLE.contains(&ai.as_str()) {
            let end = chars[i..]
                .iter()
                .position(|c| *c == GS)
                .map(|p| i + p)
                .unwrap_or(chars.len());
            let value: String = chars[i..end].iter().collect();
            i = end;
            out.push(Gs1Element { ai, value });
        } else {
            // An AI we do not model. Its value length is unknown, so every digit after it would be
            // guesswork — stop and keep only what is certain.
            break;
        }
    }
    out
}

/// The GTIN (`AI 01`) if the code carries one — what the item *is*.
pub fn gtin(elements: &[Gs1Element]) -> Option<&str> {
    elements
        .iter()
        .find(|e| e.ai == "01")
        .map(|e| e.value.as_str())
}

/// The **production date** (`AI 11`), the only field here that can be a birth moment.
///
/// `current_year` is passed in rather than read from the clock so this stays a pure function: the
/// two-digit year needs a century, and GS1 resolves that against *now*.
pub fn production_date(elements: &[Gs1Element], current_year: i32) -> Option<ScannedDate> {
    let raw = elements.iter().find(|e| e.ai == "11")?;
    parse_yymmdd(&raw.value, current_year)
}

/// `YYMMDD` → a dated value, or `None` if it isn't six digits or names an impossible month.
///
/// A `DD` of `00` is legal in GS1 and means the month without a specific day, so it yields
/// [`ScannedDate::Month`] — never a silently invented 1st.
pub fn parse_yymmdd(raw: &str, current_year: i32) -> Option<ScannedDate> {
    let raw = raw.trim();
    if raw.len() != 6 || !raw.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let yy: i32 = raw[0..2].parse().ok()?;
    let mm: u32 = raw[2..4].parse().ok()?;
    let dd: u32 = raw[4..6].parse().ok()?;
    if !(1..=12).contains(&mm) {
        return None;
    }
    let year = expand_year(yy, current_year);
    if dd == 0 {
        return Some(ScannedDate::Month(year, mm));
    }
    NaiveDate::from_ymd_opt(year, mm, dd).map(ScannedDate::Day)
}

/// Expand a two-digit year against GS1's sliding century window: the date is taken to lie within
/// roughly fifty years either side of now, so `20` reads as 2020 today and would read as 2120 in a
/// century's time. Without this, every date before 2000 or after 2099 silently lands in the wrong
/// century.
fn expand_year(yy: i32, current_year: i32) -> i32 {
    let century = (current_year / 100) * 100;
    [century - 100 + yy, century + yy, century + 100 + yy]
        .into_iter()
        .find(|y| *y >= current_year - 49 && *y <= current_year + 50)
        .unwrap_or(century + yy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_printed_parenthesised_form() {
        let els = parse_gs1("(01)00614141000012(11)200315(10)ABC123");
        assert_eq!(
            els,
            vec![
                Gs1Element {
                    ai: "01".into(),
                    value: "00614141000012".into()
                },
                Gs1Element {
                    ai: "11".into(),
                    value: "200315".into()
                },
                Gs1Element {
                    ai: "10".into(),
                    value: "ABC123".into()
                },
            ]
        );
        assert_eq!(gtin(&els), Some("00614141000012"));
    }

    #[test]
    fn parses_the_raw_scanner_transmission() {
        // Fixed-length values run together; the variable-length batch ends at the separator.
        let raw = format!("010061414100001211200315{}10ABC123", GS);
        let els = parse_gs1(&raw);
        assert_eq!(els.len(), 3, "{els:?}");
        assert_eq!(gtin(&els), Some("00614141000012"));
        assert_eq!(
            production_date(&els, 2026),
            Some(ScannedDate::Day(
                NaiveDate::from_ymd_opt(2020, 3, 15).unwrap()
            ))
        );
    }

    /// A variable-length value at the very end has no separator to stop it — it runs to the end.
    #[test]
    fn a_trailing_variable_value_runs_to_the_end() {
        let els = parse_gs1("010061414100001221SERIAL9");
        assert_eq!(gtin(&els), Some("00614141000012"));
        assert_eq!(
            els[1],
            Gs1Element {
                ai: "21".into(),
                value: "SERIAL9".into()
            }
        );
    }

    /// The load-bearing safety rule: an AI whose length we don't know ends the parse. Guessing would
    /// hand the following digits to the wrong field, and a *wrong* date is worse than none.
    #[test]
    fn an_unknown_ai_stops_the_parse_instead_of_guessing() {
        // 3103 (net weight) is a 4-digit AI this module doesn't model.
        let els = parse_gs1("01006141410000123103000189");
        assert_eq!(els.len(), 1, "only the certain GTIN survives: {els:?}");
        assert_eq!(gtin(&els), Some("00614141000012"));
        assert!(production_date(&els, 2026).is_none());
    }

    /// GS1 allows `DD = 00` for "this month, no particular day". It must not become the 1st.
    #[test]
    fn a_day_of_zero_is_a_month_not_an_invented_first() {
        let els = parse_gs1("(11)200300");
        assert_eq!(
            production_date(&els, 2026),
            Some(ScannedDate::Month(2020, 3)),
            "DD=00 names a month; inventing a day here is the Jan-1st fabrication in miniature"
        );
    }

    #[test]
    fn only_production_date_counts_as_a_moment() {
        // Expiry and best-before describe the same object but are not when it was made.
        let els = parse_gs1("(17)271231(15)270601");
        assert!(
            production_date(&els, 2026).is_none(),
            "an expiry date is not a birth: {els:?}"
        );
    }

    #[test]
    fn two_digit_years_land_in_the_right_century() {
        // Near dates resolve forward/back around now.
        assert_eq!(expand_year(20, 2026), 2020);
        assert_eq!(
            expand_year(99, 2026),
            1999,
            "just outside the forward window"
        );
        assert_eq!(expand_year(70, 2026), 2070, "inside the +50 window");
        // The same code read a century later resolves differently — which is the point of the rule.
        assert_eq!(expand_year(20, 2126), 2120);
    }

    #[test]
    fn malformed_dates_are_refused_rather_than_half_read() {
        assert!(parse_yymmdd("2003", 2026).is_none(), "too short");
        assert!(parse_yymmdd("20AB15", 2026).is_none(), "not digits");
        assert!(parse_yymmdd("201315", 2026).is_none(), "month 13");
        assert!(parse_yymmdd("200231", 2026).is_none(), "31 February");
    }

    #[test]
    fn a_bare_retail_barcode_carries_no_date() {
        // A plain GTIN — the ordinary UPC/EAN case. Identity, and nothing else.
        let els = parse_gs1("0100614141000012");
        assert_eq!(gtin(&els), Some("00614141000012"));
        assert!(production_date(&els, 2026).is_none());
    }
}
