//! Ziqpu offline ticker index — search a listed universe and turn an entity into a datable
//! [`agents::Choice`].
//!
//! One index serves three universes ([`Universe`]): the **stock** market (dated by IPO) plus two
//! industry universes contributed by teammates — **airlines** and **insurers** — dated by their
//! FOUNDING/origin, not their IPO. Each ships its committed CSV compiled in via [`include_str!`]
//! and parsed **once, lazily** into its own in-memory index — **no network, no build-time codegen,
//! no filesystem at runtime.**
//!
//! - **Stocks** (`company_metadata.csv`, ≈5.3k US-listed symbols): every symbol is dated by its IPO
//!   on a US exchange, so a choice's birth moment is `ipo_date @ 09:30 America/New_York` at the
//!   exchange's coordinates (both already in the CSV).
//! - **Airlines / Insurers** (`aviation.csv` / `insurance.csv`): each entity is dated by its
//!   founding `birth_date`, at the time in `birth_time` **if present, else left unknown** (industry
//!   birth times are usually blank — honest time-unknown, never invented), with `tz` + `latitude` +
//!   `longitude` taken straight from the CSV.
//!
//! [`search_in`] is a deterministic linear scan: lowercase the query and each field, then rank
//! exact-ticker → ticker-prefix → substring, tie-broken lexicographically by ticker and capped.
//! Same input → byte-identical output, so the UI on top of it is testable. The market-only [`search`],
//! [`choice`], and [`find_in_text`] are thin wrappers over the [`Universe::Stocks`] index.

use std::sync::OnceLock;

use agents::{BirthMoment, Choice};
use chrono::{NaiveDate, NaiveTime};
use chrono_tz::Tz;

/// The committed stock table (a copy of the repo-root `company_metadata.csv`). Header:
/// `ticker,company_name,ipo_date,ipo_time,exchange,latitude,longitude,founding_date,data_source,notes`.
static CSV: &str = include_str!("../data/company_metadata.csv");

/// The committed airline table (a copy of `datasets/aviation/entities.csv`). Header:
/// `id,name,birth_date,birth_time,location,tz,latitude,longitude,data_source,notes`.
static AVIATION_CSV: &str = include_str!("../data/aviation.csv");

/// The committed insurer table (a copy of `datasets/insurance/entities.csv`), same industry header
/// as [`AVIATION_CSV`].
static INSURANCE_CSV: &str = include_str!("../data/insurance.csv");

/// Cap on how many candidates a single search returns.
const MAX_RESULTS: usize = 30;

/// A listed universe this index can search and date. Every public entry point comes in a
/// universe-taking form ([`search_in`], [`choice_in`]); the bare [`search`]/[`choice`]/
/// [`find_in_text`] are [`Universe::Stocks`] wrappers kept for existing callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Universe {
    /// US-listed stocks, dated by IPO at the opening bell (`company_metadata.csv`).
    Stocks,
    /// Airlines, dated by their founding/origin (`aviation.csv`).
    Airlines,
    /// Insurers, dated by their founding/origin (`insurance.csv`).
    Insurance,
}

impl Universe {
    /// Human-facing label for a universe tab/toggle.
    pub fn label(&self) -> &'static str {
        match self {
            Universe::Stocks => "Stocks",
            Universe::Airlines => "Airlines",
            Universe::Insurance => "Insurers",
        }
    }

    /// Stable machine slug (URL/state key) for a universe.
    pub fn slug(&self) -> &'static str {
        match self {
            Universe::Stocks => "stocks",
            Universe::Airlines => "airlines",
            Universe::Insurance => "insurance",
        }
    }

    /// Parse a [`slug`](Self::slug) back into its universe; `None` for anything unrecognized.
    pub fn from_slug(s: &str) -> Option<Universe> {
        match s {
            "stocks" => Some(Universe::Stocks),
            "airlines" => Some(Universe::Airlines),
            "insurance" => Some(Universe::Insurance),
            _ => None,
        }
    }
}

/// The opening bell — every symbol's birth moment is its listing day at `09:30 America/New_York`.
const MARKET_OPEN: NaiveTime = match NaiveTime::from_hms_opt(9, 30, 0) {
    Some(t) => t,
    None => unreachable!(),
};

/// Neutral listing date used when a row has neither a parseable `ipo_date` nor `founding_date`, so
/// every symbol in the table is still datable (and thus addable) rather than silently dropped.
const DEFAULT_LISTING_DATE: NaiveDate = match NaiveDate::from_ymd_opt(2000, 1, 1) {
    Some(d) => d,
    None => unreachable!(),
};

/// Trading-floor coordinates used to backfill a row whose `latitude`/`longitude` are missing or out
/// of range, keyed by exchange; `NEUTRAL_US_MARKET` (lower Manhattan) covers anything else.
const NYSE_COORDS: (f64, f64) = (40.707, -74.011);
const NASDAQ_COORDS: (f64, f64) = (40.757, -73.986);
const NEUTRAL_US_MARKET: (f64, f64) = (40.7128, -74.0060);

/// A search hit: the symbol and the company's display name. Just enough to render a result row and
/// to feed [`choice`] when the seeker picks it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickerRow {
    pub ticker: String,
    pub name: String,
}

/// One parsed row from any universe, normalized into a common shape: the display fields, their
/// lowercased search keys (plus a space-bracketed normalized name for whole-word matching), and the
/// pre-resolved birth-moment inputs [`choice_in`] needs. `lat`/`lon` are `None` when the CSV value
/// is missing or out of range, in which case `choice_in` falls back to [`fallback`](Self::fallback).
struct Entity {
    ticker: String,
    name: String,
    ticker_lc: String,
    name_lc: String,
    name_norm: String,
    /// Primary birth date string: `ipo_date` (Stocks) or `birth_date` (industry).
    date: String,
    /// Secondary date tried when `date` is blank: `founding_date` for Stocks, empty for industry.
    alt_date: String,
    /// Raw birth-time column: `ipo_time` (Stocks) or `birth_time` (industry).
    time: String,
    /// Timezone: fixed `America/New_York` for Stocks; the CSV `tz` column for industry.
    tz: Tz,
    lat: Option<f64>,
    lon: Option<f64>,
    /// Coordinates used when `lat`/`lon` are absent: the exchange's trading floor (Stocks) or a
    /// neutral point (industry).
    fallback: (f64, f64),
    /// Time to use when the `time` column is blank/unparseable: `Some(09:30)` for Stocks (the
    /// opening bell), `None` for industry (honest time-unknown — never invented).
    blank_time: Option<NaiveTime>,
}

/// Parse and cache a universe's table once, on first use.
fn entities(u: Universe) -> &'static Vec<Entity> {
    match u {
        Universe::Stocks => {
            static ROWS: OnceLock<Vec<Entity>> = OnceLock::new();
            ROWS.get_or_init(|| parse_stocks(CSV))
        }
        Universe::Airlines => {
            static ROWS: OnceLock<Vec<Entity>> = OnceLock::new();
            ROWS.get_or_init(|| parse_industry(AVIATION_CSV))
        }
        Universe::Insurance => {
            static ROWS: OnceLock<Vec<Entity>> = OnceLock::new();
            ROWS.get_or_init(|| parse_industry(INSURANCE_CSV))
        }
    }
}

/// The stock index — kept as the default for the market-only wrappers and iteration in tests.
fn rows() -> &'static Vec<Entity> {
    entities(Universe::Stocks)
}

/// Search one universe's index. Case-insensitive; an entity matches when its ticker/id equals, is
/// prefixed by, or contains the query, or when the name contains it. Results rank
/// **exact ticker → ticker-prefix → substring**, tie-broken lexicographically by ticker, and are
/// capped at [`MAX_RESULTS`]. An empty or whitespace-only query returns an empty list. Pure and
/// deterministic — the order is total (rank, then ticker), so it never depends on sort stability.
pub fn search_in(u: Universe, query: &str) -> Vec<TickerRow> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(u8, &Entity)> = entities(u)
        .iter()
        .filter_map(|r| rank(r, &q).map(|t| (t, r)))
        .collect();
    hits.sort_by(|(ta, ra), (tb, rb)| ta.cmp(tb).then_with(|| ra.ticker.cmp(&rb.ticker)));
    hits.into_iter()
        .take(MAX_RESULTS)
        .map(|(_, r)| TickerRow {
            ticker: r.ticker.clone(),
            name: r.name.clone(),
        })
        .collect()
}

/// Search the stock market — [`search_in`] over [`Universe::Stocks`]. Kept for existing callers.
pub fn search(query: &str) -> Vec<TickerRow> {
    search_in(Universe::Stocks, query)
}

/// Classify a row against a lowercased query into a rank tier (lower is better), or `None` if it
/// does not match at all: `0` exact ticker · `1` ticker-prefix · `2` whole-word company-name match
/// · `3` mid-substring (ticker or name). Ranking a whole-word name hit above a mid-substring one is
/// what lets a plain company query ("micron", "oracle") surface its symbol within the cap instead of
/// being buried behind incidental substring collisions.
fn rank(r: &Entity, q: &str) -> Option<u8> {
    if r.ticker_lc == q {
        Some(0)
    } else if r.ticker_lc.starts_with(q) {
        Some(1)
    } else if name_has_whole_word(r, q) {
        Some(2)
    } else if r.ticker_lc.contains(q) || r.name_lc.contains(q) {
        Some(3)
    } else {
        None
    }
}

/// Whether the query appears as a whole word (or contiguous whole-word phrase) in the company name,
/// e.g. "micron" in "Micron Technology, Inc." or "under armour" in "Under Armour, Inc." — matched
/// against the row's pre-normalized, space-bracketed name so punctuation never blocks a hit.
fn name_has_whole_word(r: &Entity, q: &str) -> bool {
    let needle = normalize(q);
    let needle = needle.trim();
    !needle.is_empty() && phrase_contains(&r.name_norm, needle)
}

/// Common short English words that must never resolve to a ticker, even though some are real symbols
/// (e.g. `AM`, `IT`, `A`) — so free text like "am I compatible with …" never spuriously matches a
/// company on a stray function word.
const STOPWORDS: &[&str] = &[
    "a", "am", "an", "and", "any", "are", "as", "at", "be", "but", "buy", "by", "can", "do", "for",
    "go", "he", "her", "here", "him", "his", "how", "i", "if", "in", "is", "it", "its", "me", "my",
    "no", "not", "of", "on", "or", "out", "sell", "she", "should", "so", "the", "them", "then",
    "this", "to", "up", "us", "we", "what", "when", "with", "you", "your", "about", "nothing",
];

/// Corporate-name noise tokens stripped from a company name's ends to reach its distinctive core, so
/// "Coca-Cola Company" matches the text "coca-cola" and "Apple Inc." matches "apple".
const NAME_NOISE: &[&str] = &[
    "inc",
    "incorporated",
    "corp",
    "corporation",
    "co",
    "company",
    "companies",
    "ltd",
    "limited",
    "plc",
    "llc",
    "lp",
    "holdings",
    "holding",
    "group",
    "common",
    "stock",
    "ordinary",
    "shares",
    "share",
    "class",
    "sa",
    "nv",
    "ag",
    "se",
    "the",
    "and",
];

/// Find a company in free text — the seam that lets a chat surface ("am I compatible with
/// Coca-Cola?") resolve to a datable symbol. It considers two kinds of evidence and prefers the
/// more specific one:
///
/// - a **whole-word ticker** (e.g. `KO`, `AAPL`), case-insensitive, in reading order — short/common
///   words ([`STOPWORDS`], and anything under two characters) are ignored so a stray "I"/"a"/"am"
///   never resolves to a symbol;
/// - a **company-name** phrase — the row whose distinctive core name (corporate suffixes stripped)
///   appears as a whole phrase in the text, preferring the longest such name and tie-breaking
///   lexicographically by ticker.
///
/// A **multi-word** company phrase ("advanced micro devices", "under armour") is the strongest,
/// least-ambiguous signal, so it wins even over a bare ticker word elsewhere in the sentence — this
/// is what stops an incidental common-word ticker (LOVE, ALL, KEY…) in prose from hijacking a
/// clearly-named company. A ticker still beats a *single*-word name match, preserving "how about
/// AAPL". Returns `None` when nothing distinctive matches. Pure, dependency-free, and deterministic.
pub fn find_in_text(text: &str) -> Option<TickerRow> {
    let ticker_hit = ticker_in_text(text);
    let name_hit = name_in_text(text);

    // A multi-word company phrase is specific enough to override an incidental ticker word.
    let name_is_multiword = name_hit
        .map(|r| r.name_norm.trim().contains(' '))
        .unwrap_or(false);

    let chosen = match (ticker_hit, name_hit) {
        (Some(t), Some(n)) => {
            if name_is_multiword {
                n
            } else {
                t
            }
        }
        (Some(t), None) => t,
        (None, Some(n)) => n,
        (None, None) => return None,
    };
    Some(TickerRow {
        ticker: chosen.ticker.clone(),
        name: chosen.name.clone(),
    })
}

/// The first whole-word ticker symbol in `text` (reading order), skipping [`STOPWORDS`] and tokens
/// under two characters so bare function words never resolve.
fn ticker_in_text(text: &str) -> Option<&'static Entity> {
    for raw in text.split_whitespace() {
        let tok = raw.trim_matches(|c: char| !c.is_alphanumeric());
        if tok.len() < 2 {
            continue;
        }
        let lc = tok.to_lowercase();
        if STOPWORDS.contains(&lc.as_str()) {
            continue;
        }
        if let Some(r) = rows().iter().find(|r| r.ticker_lc == lc) {
            return Some(r);
        }
    }
    None
}

/// The company whose distinctive core name appears as a whole phrase in `text`, preferring the
/// longest (most specific) core and tie-breaking by lexicographically smallest ticker.
fn name_in_text(text: &str) -> Option<&'static Entity> {
    let hay = normalize(text);
    let mut best: Option<(usize, &Entity)> = None;
    for r in rows() {
        let core = core_name(&r.name_lc);
        if core.len() < 4 {
            continue;
        }
        if !phrase_contains(&hay, &core) {
            continue;
        }
        let better = match best {
            None => true,
            Some((blen, brow)) => {
                core.len() > blen || (core.len() == blen && r.ticker < brow.ticker)
            }
        };
        if better {
            best = Some((core.len(), r));
        }
    }
    best.map(|(_, r)| r)
}

/// Lowercase `s`, fold every run of non-alphanumeric characters to a single space, and bracket the
/// result with spaces — so a needle padded the same way matches only on whole-word boundaries.
fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push(' ');
    let mut prev_space = true;
    for c in s.chars() {
        if c.is_alphanumeric() {
            out.extend(c.to_lowercase());
            prev_space = false;
        } else if !prev_space {
            out.push(' ');
            prev_space = true;
        }
    }
    if !prev_space {
        out.push(' ');
    }
    out
}

/// The distinctive core of a (already-lowercased) company name: normalized, with a leading "the" and
/// any trailing corporate-noise tokens ([`NAME_NOISE`]) removed. Returned space-trimmed.
fn core_name(name_lc: &str) -> String {
    let norm = normalize(name_lc);
    let mut toks: Vec<&str> = norm.split_whitespace().collect();
    while toks.first() == Some(&"the") {
        toks.remove(0);
    }
    while toks.last().is_some_and(|t| NAME_NOISE.contains(t)) {
        toks.pop();
    }
    toks.join(" ")
}

/// Whether `core` appears as a whole-word phrase in the space-bracketed haystack `hay`.
fn phrase_contains(hay: &str, core: &str) -> bool {
    if core.is_empty() {
        return false;
    }
    hay.contains(&format!(" {core} "))
}

/// Resolve an entity (case-insensitive ticker/id) in a given universe to a datable [`Choice`],
/// building its birth moment from THAT universe's data:
/// - **Stocks**: the listing day at `09:30 America/New_York` at the exchange's coordinates — current
///   market behavior.
/// - **Airlines / Insurers**: the founding `birth_date`, at `birth_time` if present else `None`
///   (honest time-unknown), with `tz` + `latitude` + `longitude` straight from the CSV.
///
/// **Every listed entity yields `Some`**, so the UI's `if let Some(choice) = …` can always add it.
/// Missing or unparseable data degrades gracefully rather than dropping the row:
/// - date: primary date, else the secondary (`founding_date` for Stocks), else
///   [`DEFAULT_LISTING_DATE`] (2000-01-01);
/// - time: the time column when parseable, else the universe's blank-time default
///   ([`MARKET_OPEN`] for Stocks, `None` for industry);
/// - coordinates: the CSV `latitude`/`longitude` when both are present and in range, else the
///   universe's fallback (the exchange's trading floor for Stocks, a neutral point for industry).
///
/// `cik`/`wiki` are `None` (these universes carry no curated grounding handles; the seeded demo's
/// five stocks do). Returns `None` only for an id not present in that universe's table.
pub fn choice_in(u: Universe, id: &str) -> Option<Choice> {
    let key = id.trim().to_lowercase();
    let r = entities(u).iter().find(|r| r.ticker_lc == key)?;
    let date = parse_date(&r.date)
        .or_else(|| parse_date(&r.alt_date))
        .unwrap_or(DEFAULT_LISTING_DATE);
    let time = parse_time(&r.time).or(r.blank_time);
    let (lat, lon) = match (r.lat, r.lon) {
        (Some(lat), Some(lon)) => (lat, lon),
        _ => r.fallback,
    };
    Some(Choice {
        ticker: r.ticker.clone(),
        name: r.name.clone(),
        birth: BirthMoment {
            date,
            time,
            tz: r.tz,
            lat,
            lon,
        },
        cik: None,
        wiki: None,
    })
}

/// Resolve a stock ticker — [`choice_in`] over [`Universe::Stocks`]. Kept for existing callers.
pub fn choice(ticker: &str) -> Option<Choice> {
    choice_in(Universe::Stocks, ticker)
}

/// Parse a `YYYY-MM-DD` date, tolerating surrounding whitespace; `None` if blank or malformed.
fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()
}

/// Parse a birth-time column, accepting `HH:MM:SS` (the stock CSV) or `HH:MM`; `None` if blank or
/// malformed, so [`choice_in`] can apply the universe's blank-time default.
fn parse_time(s: &str) -> Option<NaiveTime> {
    let s = s.trim();
    NaiveTime::parse_from_str(s, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M"))
        .ok()
}

/// Trading-floor coordinates for a row whose own latitude/longitude are missing or out of range.
fn fallback_coords(exchange: &str) -> (f64, f64) {
    match exchange.trim().to_ascii_uppercase().as_str() {
        "NYSE" => NYSE_COORDS,
        "NASDAQ" => NASDAQ_COORDS,
        _ => NEUTRAL_US_MARKET,
    }
}

/// Parse the committed stock CSV into entities, skipping the header and any blank lines. Columns:
/// `ticker,company_name,ipo_date,ipo_time,exchange,latitude,longitude,founding_date,…`. Every
/// stock's timezone is `America/New_York`, blank times fall back to the opening bell, and missing
/// coordinates fall back to the exchange's trading floor. Malformed rows (too few columns, empty
/// ticker) are dropped rather than panicking — the committed artifact is well-formed, but a
/// resilient parse keeps the UI alive if a future regeneration slips.
fn parse_stocks(csv: &str) -> Vec<Entity> {
    csv.lines()
        .skip(1)
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let f = split_csv(line);
            if f.len() < 7 {
                return None;
            }
            let ticker = f[0].trim().to_string();
            if ticker.is_empty() {
                return None;
            }
            let name = f[1].trim().to_string();
            let name_lc = name.to_lowercase();
            let name_norm = normalize(&name_lc);
            Some(Entity {
                ticker_lc: ticker.to_lowercase(),
                name_norm,
                date: f[2].trim().to_string(),
                alt_date: f.get(7).map(|s| s.trim().to_string()).unwrap_or_default(),
                time: f[3].trim().to_string(),
                tz: chrono_tz::America::New_York,
                lat: parse_coord(f[5].trim(), 90.0),
                lon: parse_coord(f[6].trim(), 180.0),
                fallback: fallback_coords(f[4].trim()),
                blank_time: Some(MARKET_OPEN),
                ticker,
                name,
                name_lc,
            })
        })
        .collect()
}

/// Parse a committed industry CSV (airlines / insurers) into entities, skipping the header and any
/// blank lines. Columns: `id,name,birth_date,birth_time,location,tz,latitude,longitude,…`. `id`
/// becomes the ticker field, the founding `birth_date` is the primary date (no secondary), the
/// timezone and coordinates come straight from the CSV, and a blank `birth_time` stays unknown
/// (`None`) rather than being invented. An unparseable `tz` falls back to `America/New_York`, and
/// missing coordinates fall back to a neutral point, so every listed entity stays datable. Malformed
/// rows (too few columns, empty id) are dropped rather than panicking.
fn parse_industry(csv: &str) -> Vec<Entity> {
    csv.lines()
        .skip(1)
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let f = split_csv(line);
            if f.len() < 8 {
                return None;
            }
            let ticker = f[0].trim().to_string();
            if ticker.is_empty() {
                return None;
            }
            let name = f[1].trim().to_string();
            let name_lc = name.to_lowercase();
            let name_norm = normalize(&name_lc);
            let tz = f[5]
                .trim()
                .parse::<Tz>()
                .unwrap_or(chrono_tz::America::New_York);
            Some(Entity {
                ticker_lc: ticker.to_lowercase(),
                name_norm,
                date: f[2].trim().to_string(),
                alt_date: String::new(),
                time: f[3].trim().to_string(),
                tz,
                lat: parse_coord(f[6].trim(), 90.0),
                lon: parse_coord(f[7].trim(), 180.0),
                fallback: NEUTRAL_US_MARKET,
                blank_time: None,
                ticker,
                name,
                name_lc,
            })
        })
        .collect()
}

/// Parse a latitude/longitude field, returning `None` when it is blank, non-numeric, or outside
/// `[-limit, limit]` (90 for latitude, 180 for longitude) — so [`choice`] can fall back to the
/// exchange's coordinates instead of trusting a bad value.
fn parse_coord(s: &str, limit: f64) -> Option<f64> {
    s.parse::<f64>()
        .ok()
        .filter(|v| v.is_finite() && v.abs() <= limit)
}

/// Split one CSV line into fields, honoring RFC-4180 double-quoting: a `,` inside quotes is
/// literal, and a doubled `""` inside a quoted field is an escaped quote. Enough for this dataset
/// (company names like `"Applied Optoelectronics, Inc."`); not a general CSV library.
fn split_csv(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => fields.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    fields.push(cur);
    fields
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn empty_query_returns_empty() {
        assert!(search("").is_empty());
        assert!(search("   ").is_empty());
    }

    #[test]
    fn exact_ticker_resolves_first() {
        let r = search("AAPL");
        assert!(!r.is_empty(), "expected an AAPL hit");
        assert_eq!(r[0].ticker, "AAPL", "exact ticker must rank first");
        assert!(r[0].name.starts_with("Apple"));
    }

    #[test]
    fn name_substring_finds_apple() {
        // "apple" is not a ticker; it only matches Apple Inc. by company name.
        let r = search("apple");
        assert!(
            r.iter()
                .any(|t| t.ticker == "AAPL" && t.name.starts_with("Apple")),
            "name-substring search should surface Apple Inc., got {r:?}"
        );
    }

    #[test]
    fn choice_carries_ipo_date_and_a_known_time() {
        let c = choice("AAPL").expect("AAPL resolves to a choice");
        assert_eq!(c.ticker, "AAPL");
        assert_eq!(c.name, "Apple Inc.");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(1980, 12, 12).unwrap());
        assert_eq!(
            c.birth.time,
            Some(NaiveTime::from_hms_opt(9, 30, 0).unwrap())
        );
        assert_eq!(c.birth.tz, chrono_tz::America::New_York);
        assert!(c.cik.is_none() && c.wiki.is_none());
    }

    #[test]
    fn choice_is_case_insensitive_and_none_for_unknown() {
        assert!(choice("aapl").is_some(), "lookup must be case-insensitive");
        assert!(choice("__no_such_ticker__").is_none());
    }

    #[test]
    fn find_in_text_matches_company_name() {
        // "Coca-Cola" is not a ticker token; it resolves by company name to KO — and the leading
        // "am"/"I" (both real-ish function words; AM is even a real ticker) must not steal the match.
        let r = find_in_text("am I compatible with Coca-Cola").expect("should find KO by name");
        assert_eq!(r.ticker, "KO");
    }

    #[test]
    fn find_in_text_prefers_whole_word_ticker() {
        let r = find_in_text("how about AAPL").expect("should find AAPL by ticker");
        assert_eq!(r.ticker, "AAPL");
    }

    #[test]
    fn find_in_text_returns_none_when_nothing_matches() {
        // Only function words and a common word ("nothing") — no ticker, no company name.
        assert!(find_in_text("should I buy nothing here").is_none());
    }

    #[test]
    fn find_in_text_ignores_bare_function_words() {
        // "I"/"a"/"it" are (or map to) real tickers, but as stray words they must never resolve.
        assert!(find_in_text("I a it").is_none());
    }

    #[test]
    fn find_in_text_finds_apple_by_name() {
        let r = find_in_text("does apple fit me").expect("apple resolves by name");
        assert_eq!(r.ticker, "AAPL");
    }

    /// The route-3 (SEC Form 8-A) enrichment is present in the shipped dataset.
    ///
    /// It lived only in `db/init/02_seed.sql` for nine days while the CSV the binary actually
    /// compiles in stayed at the pre-enrichment Polygon export — 2,354 dated instead of 4,507. MU
    /// is one of the 2,156 rows the backfill dated; it is pinned here so a regenerated CSV that
    /// silently loses the enrichment fails loudly instead of quietly halving the product.
    #[test]
    fn the_sec_8a_enrichment_is_in_the_shipped_dataset() {
        let c = choice("MU").expect("MU must resolve");
        assert_eq!(c.ticker, "MU");
        assert_ne!(
            c.birth.date, DEFAULT_LISTING_DATE,
            "MU fell back to the fabricated date — the sec-8a enrichment is missing from the CSV. \
             Run scripts/gen-tickers-csv.sh."
        );
        assert_eq!(c.birth.time, Some(MARKET_OPEN));
        assert_eq!(c.birth.tz, chrono_tz::America::New_York);
    }

    /// **Characterizes a known defect — it does not endorse one.** (Nathan's review, P2.)
    ///
    /// 764 rows have no date at all: not in Polygon, and no Form 8-A on record. `choice_in` hands
    /// them [`DEFAULT_LISTING_DATE`] — **2000-01-01, a date that never happened** — and the app then
    /// computes a full synastry reading from it: a score, a band, a "why", closing on *"measured,
    /// not fate"*. That reading is fiction presented as measurement, which is the one thing this
    /// product exists to refuse. `UNGASAGA_SYSTEM` even forbids it in words ("never invent a measure
    /// you were not given") while the data layer does it upstream.
    ///
    /// It was not malice: the constant was introduced to fix a UI bug where undated symbols "didn't
    /// hold" when picked. A UX bug got fixed by inventing data.
    ///
    /// The harvest cut the blast radius from 2,917 rows to 764. The remaining fix is a product
    /// decision the owner has to make — drop the undated from search, or list them marked as
    /// unchartable (his stated preference elsewhere: *indicate, don't hide*) — so this test records
    /// today's behaviour honestly rather than pretending it is correct.
    #[test]
    fn a_dateless_row_is_charted_on_a_fabricated_date_known_defect() {
        // AAUC: Polygon has no listing date and there is no Form 8-A on record.
        let c = choice("AAUC").expect("a blank ipo_date does not currently drop the symbol");
        assert_eq!(c.ticker, "AAUC");
        assert_eq!(
            c.birth.date, DEFAULT_LISTING_DATE,
            "the fabrication path changed — if undated rows are now handled honestly, delete this \
             test and DEFAULT_LISTING_DATE with it"
        );
        assert!(
            c.birth.lat.abs() <= 90.0 && c.birth.lon.abs() <= 180.0,
            "fallback coordinates must be in range, got {},{}",
            c.birth.lat,
            c.birth.lon
        );
    }

    #[test]
    fn every_symbol_in_the_table_is_addable() {
        // The core robustness guarantee: choice() returns Some for every ticker that exists.
        for r in rows() {
            assert!(
                choice(&r.ticker).is_some(),
                "choice({}) returned None — symbol would silently fail to add",
                r.ticker
            );
        }
    }

    #[test]
    fn choice_leaves_a_normal_row_unchanged() {
        // A fully-populated row (AAPL) must be byte-for-byte what it was before the fallbacks.
        let c = choice("AAPL").expect("AAPL resolves");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(1980, 12, 12).unwrap());
        assert_eq!(
            c.birth.time,
            Some(NaiveTime::from_hms_opt(9, 30, 0).unwrap())
        );
        assert_eq!(c.birth.lat, 40.7589);
        assert_eq!(c.birth.lon, -73.9851);
    }

    #[test]
    fn search_surfaces_well_known_names_by_whole_word() {
        // "micron" and "oracle" are company names, not tickers; a whole-word name match must lift
        // their symbols into the capped result set.
        let m = search("micron");
        assert!(
            m.iter().any(|t| t.ticker == "MU"),
            "search(\"micron\") should surface MU, got {m:?}"
        );
        let o = search("oracle");
        assert!(
            o.iter().any(|t| t.ticker == "ORCL"),
            "search(\"oracle\") should surface ORCL, got {o:?}"
        );
    }

    #[test]
    fn find_in_text_resolves_plain_company_names() {
        assert_eq!(
            find_in_text("how do I fit with oracle")
                .expect("oracle resolves")
                .ticker,
            "ORCL"
        );
        // "advanced micro devices" spelled out resolves to AMD by its core name…
        assert_eq!(
            find_in_text("i love advanced micro devices")
                .expect("AMD resolves by name")
                .ticker,
            "AMD"
        );
        // …and the bare ticker resolves via the whole-word ticker pass.
        assert_eq!(
            find_in_text("what about amd")
                .expect("AMD resolves by ticker")
                .ticker,
            "AMD"
        );
        // "under armour" resolves to one of the two Under Armour share classes.
        let ua = find_in_text("do I match under armour").expect("Under Armour resolves");
        assert!(
            ua.ticker == "UAA" || ua.ticker == "UA",
            "expected UAA/UA, got {}",
            ua.ticker
        );
    }

    #[test]
    fn search_is_capped_ordered_and_deterministic() {
        // A one-letter query matches thousands of symbols → the cap must bind exactly.
        let broad = search("a");
        assert_eq!(
            broad.len(),
            MAX_RESULTS,
            "a broad query is capped at {MAX_RESULTS}"
        );

        // "aa" has an exact ticker (Alcoa = AA), then prefixes (AACB, AACI, …). Exact ranks first,
        // and within a tier results are lexicographic by ticker.
        let r = search("aa");
        assert_eq!(r[0].ticker, "AA", "exact ticker outranks prefixes");
        let prefixes: Vec<&str> = r
            .iter()
            .filter(|t| t.ticker.to_lowercase().starts_with("aa") && t.ticker != "AA")
            .map(|t| t.ticker.as_str())
            .collect();
        let mut sorted = prefixes.clone();
        sorted.sort_unstable();
        assert_eq!(prefixes, sorted, "same-tier hits are ordered by ticker");

        assert_eq!(
            search("aa"),
            search("aa"),
            "same query is byte-identical across runs"
        );
    }

    #[test]
    fn universe_slugs_and_labels_round_trip() {
        for u in [Universe::Stocks, Universe::Airlines, Universe::Insurance] {
            assert_eq!(Universe::from_slug(u.slug()), Some(u));
        }
        assert_eq!(Universe::Stocks.label(), "Stocks");
        assert_eq!(Universe::Airlines.label(), "Airlines");
        assert_eq!(Universe::Insurance.label(), "Insurers");
        assert_eq!(Universe::from_slug("nope"), None);
    }

    #[test]
    fn airlines_search_surfaces_delta() {
        // "delta" is a company-name whole-word match → Delta Air Lines (id DAL) must surface.
        let r = search_in(Universe::Airlines, "delta");
        assert!(
            r.iter()
                .any(|t| t.ticker == "DAL" && t.name.contains("Delta")),
            "search_in(Airlines, \"delta\") should surface DAL/Delta, got {r:?}"
        );
    }

    #[test]
    fn airlines_choice_dates_by_founding_year() {
        // American Airlines was founded 1926-04-15 — the founding date, not the 1994 IPO.
        let c = choice_in(Universe::Airlines, "AAL").expect("AAL resolves in Airlines");
        assert_eq!(c.ticker, "AAL");
        assert_eq!(c.birth.date.year(), 1926);
        // birth_time is blank → honest time-unknown.
        assert_eq!(c.birth.time, None);
        // tz + coords come straight from the CSV (America/Chicago at DFW).
        assert_eq!(c.birth.tz, chrono_tz::America::Chicago);
        assert!(c.cik.is_none() && c.wiki.is_none());
    }

    #[test]
    fn insurance_search_finds_state_farm() {
        // "state farm" is a whole-word company-name phrase → State Farm's NAIC id (25178).
        let r = search_in(Universe::Insurance, "state farm");
        assert!(
            r.iter().any(|t| t.ticker == "25178"),
            "search_in(Insurance, \"state farm\") should return id 25178, got {r:?}"
        );
    }

    #[test]
    fn industry_blank_time_yields_time_none() {
        // Every industry row here ships a blank birth_time → choice_in must leave time unknown,
        // never inventing an opening bell the way the stock path does.
        let c = choice_in(Universe::Insurance, "25178").expect("State Farm resolves");
        assert_eq!(
            c.birth.time, None,
            "blank industry birth_time must stay None"
        );
    }

    #[test]
    fn every_industry_entity_is_addable() {
        // The robustness guarantee extends to both industry universes.
        for u in [Universe::Airlines, Universe::Insurance] {
            for r in entities(u) {
                assert!(
                    choice_in(u, &r.ticker).is_some(),
                    "choice_in({:?}, {}) returned None",
                    u,
                    r.ticker
                );
            }
        }
    }

    #[test]
    fn stocks_path_is_unchanged_via_universe() {
        // The Stocks wrappers and the universe-taking form must agree, and still match the old
        // spot-check: AAPL @ 1980-12-12, 09:30, at Apple's coordinates.
        let via_wrapper = choice("AAPL").expect("AAPL resolves");
        let via_universe = choice_in(Universe::Stocks, "AAPL").expect("AAPL resolves in Stocks");
        assert_eq!(via_wrapper.birth, via_universe.birth);
        assert_eq!(
            via_universe.birth.date,
            NaiveDate::from_ymd_opt(1980, 12, 12).unwrap()
        );
        assert_eq!(via_universe.birth.time, Some(MARKET_OPEN));
        assert_eq!(via_universe.birth.tz, chrono_tz::America::New_York);
        assert_eq!(via_universe.birth.lat, 40.7589);
        assert_eq!(via_universe.birth.lon, -73.9851);
        assert_eq!(search("aapl"), search_in(Universe::Stocks, "aapl"));
    }
}
