//! Ziqpu offline ticker index — search the full listed universe and turn a symbol into a datable
//! [`agents::Choice`].
//!
//! The seeded demo weighs five hand-picked IPOs; this crate opens the door to the whole market. It
//! ships the committed `company_metadata.csv` (≈5.3k US-listed symbols) compiled in via
//! [`include_str!`] and parsed **once, lazily** into an in-memory index — **no network, no
//! build-time codegen, no filesystem at runtime.** Every symbol is dated by its IPO on a US
//! exchange, so a choice's birth moment is `ipo_date @ 09:30 America/New_York` at the exchange's
//! coordinates (both already in the CSV).
//!
//! [`search`] is a deterministic linear scan: lowercase the query and each field, then rank
//! exact-ticker → ticker-prefix → substring, tie-broken lexicographically by ticker and capped.
//! Same input → byte-identical output, so the UI on top of it is testable.

use std::sync::OnceLock;

use agents::{BirthMoment, Choice};
use chrono::{NaiveDate, NaiveTime};

/// The committed ticker table (a copy of the repo-root `company_metadata.csv`). Header:
/// `ticker,company_name,ipo_date,ipo_time,exchange,latitude,longitude,founding_date,data_source,notes`.
static CSV: &str = include_str!("../data/company_metadata.csv");

/// Cap on how many candidates a single [`search`] returns.
const MAX_RESULTS: usize = 30;

/// A search hit: the symbol and the company's display name. Just enough to render a result row and
/// to feed [`choice`] when the seeker picks it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickerRow {
    pub ticker: String,
    pub name: String,
}

/// One parsed CSV row: the display fields, their lowercased search keys, and the raw IPO/location
/// columns [`choice`] needs to build a [`BirthMoment`].
struct Row {
    ticker: String,
    name: String,
    ticker_lc: String,
    name_lc: String,
    ipo_date: String,
    ipo_time: String,
    lat: f64,
    lon: f64,
}

/// Parse and cache the ticker table once, on first use.
fn rows() -> &'static Vec<Row> {
    static ROWS: OnceLock<Vec<Row>> = OnceLock::new();
    ROWS.get_or_init(|| parse(CSV))
}

/// Search the ticker universe. Case-insensitive; a symbol matches when its ticker equals, is
/// prefixed by, or contains the query, or when the company name contains it. Results rank
/// **exact ticker → ticker-prefix → substring**, tie-broken lexicographically by ticker, and are
/// capped at [`MAX_RESULTS`]. An empty or whitespace-only query returns an empty list. Pure and
/// deterministic — the order is total (rank, then ticker), so it never depends on sort stability.
pub fn search(query: &str) -> Vec<TickerRow> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(u8, &Row)> = rows()
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

/// Classify a row against a lowercased query into a rank tier (lower is better), or `None` if it
/// does not match at all: `0` exact ticker · `1` ticker-prefix · `2` ticker/name substring.
fn rank(r: &Row, q: &str) -> Option<u8> {
    if r.ticker_lc == q {
        Some(0)
    } else if r.ticker_lc.starts_with(q) {
        Some(1)
    } else if r.ticker_lc.contains(q) || r.name_lc.contains(q) {
        Some(2)
    } else {
        None
    }
}

/// Resolve a ticker (case-insensitive) to a datable [`Choice`]. The birth moment is the IPO:
/// `ipo_date` at `ipo_time` (all rows list `09:30:00`) in `America/New_York` — every symbol here is
/// US-listed — at the exchange's coordinates. `cik`/`wiki` are `None` (the full universe has no
/// curated grounding handles; the seeded demo's five do). Returns `None` for an unknown symbol or an
/// unparseable IPO date.
pub fn choice(ticker: &str) -> Option<Choice> {
    let key = ticker.trim().to_lowercase();
    let r = rows().iter().find(|r| r.ticker_lc == key)?;
    let date = NaiveDate::parse_from_str(&r.ipo_date, "%Y-%m-%d").ok()?;
    let time = NaiveTime::parse_from_str(&r.ipo_time, "%H:%M:%S").ok();
    Some(Choice {
        ticker: r.ticker.clone(),
        name: r.name.clone(),
        birth: BirthMoment {
            date,
            time,
            tz: chrono_tz::America::New_York,
            lat: r.lat,
            lon: r.lon,
        },
        cik: None,
        wiki: None,
    })
}

/// Parse the committed CSV into rows, skipping the header and any blank lines. Malformed rows
/// (too few columns, empty ticker) are dropped rather than panicking — the committed artifact is
/// well-formed, but a resilient parse keeps the UI alive if a future regeneration slips.
fn parse(csv: &str) -> Vec<Row> {
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
            Some(Row {
                ticker_lc: ticker.to_lowercase(),
                name_lc: name.to_lowercase(),
                ipo_date: f[2].trim().to_string(),
                ipo_time: f[3].trim().to_string(),
                lat: f[5].trim().parse::<f64>().unwrap_or(0.0),
                lon: f[6].trim().parse::<f64>().unwrap_or(0.0),
                ticker,
                name,
            })
        })
        .collect()
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
}
