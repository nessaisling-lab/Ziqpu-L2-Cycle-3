//! Ziqpu offline ticker index — search a listed universe and turn an entity into a datable
//! [`agents::Choice`].
//!
//! One index serves three universes ([`Universe`]): the **stock** market (dated by IPO) plus two
//! industry universes contributed by teammates — **airlines** and **insurers** — dated by their
//! FOUNDING/origin, not their IPO. Each ships its committed CSV compiled in via [`include_str!`]
//! and parsed **once, lazily** into its own in-memory index — **no network, no build-time codegen,
//! no filesystem at runtime.**
//!
//! - **Stocks** (`company_metadata.csv`, ~7.7k US-listed symbols): each row carries TWO moments —
//!   a **conception** (founding, Wikidata P571) and a **birth** (listing, SEC 424B4). A stock is
//!   charted on its listing at the opening bell when known, else on a day-precise founding without a
//!   bell. **4,253 are chartable**; the rest are searchable but **unchartable** ([`TickerRow`])
//!   rather than charted on a date we made up. All sources are CC0 / public-domain (Polygon purged
//!   for licence + because its `list_date` was a founding field mislabelled as a listing).
//!   Regenerate with `scripts/derive-dates.py` then `scripts/build-tickers-csv.py`.
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

/// The committed stock table, re-derived from CC0 + public-domain sources by
/// `scripts/build-tickers-csv.py`. Header: `ticker,name,exchange,cik,conception_date,
/// conception_prec,conception_src,birth_date,birth_prec,birth_src,note`.
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

// There used to be a `DEFAULT_LISTING_DATE` here — 2000-01-01, handed to any row with no date at
// all "so every symbol is still datable (and thus addable) rather than silently dropped". It was
// added to fix a UI bug where undated symbols didn't hold when picked: a UX problem solved by
// inventing data.
//
// It applied to 2,917 of 5,271 rows (the SEC 8-A harvest cut that to 764), and each one produced a
// full synastry reading — a score, a band, a "why" — computed from a date that never happened and
// closed with "measured, not fate". That is fiction presented as measurement, which is the one thing
// this product exists to refuse; `UNGASAGA_SYSTEM` forbids inventing a measure in words while the
// data layer did it upstream, and the Postgres seed the data came from was more honest than we were
// (db/README: implausible dates are NULLed "so not fabricated into a bogus chart").
//
// Undated rows are now unchartable and say so. See [`TickerRow::chartable`] and [`choice_in`].

/// A stock's chart location — the exchange's trading floor, where the listing actually happened.
/// `NEUTRAL_US_MARKET` (lower Manhattan) covers an unrecognized exchange.
const NYSE_COORDS: (f64, f64) = (40.707, -74.011);
const NASDAQ_COORDS: (f64, f64) = (40.757, -73.986);
const CBOE_COORDS: (f64, f64) = (41.8789, -87.6359); // Chicago — CBOE Global Markets
const NEUTRAL_US_MARKET: (f64, f64) = (40.7128, -74.0060);

/// A search hit: the symbol, the company's display name, and whether we can actually chart it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickerRow {
    pub ticker: String,
    pub name: String,
    /// Whether we hold a **day-precise** moment for this entity, and can therefore chart it.
    ///
    /// `false` for the 3,418 rows (of 7,671) with no SEC listing date and no day-precise Wikidata
    /// founding — or a founding *conflict*, like Coca-Cola. They are still returned from a search on
    /// purpose — a seeker who types "Coca-Cola" should be told we don't know its birth moment, not
    /// silently handed nothing and left to wonder whether they misspelled it. Absence of data is
    /// data; hiding it is its own small lie.
    ///
    /// [`choice_in`] returns `None` for these, so an unchartable row can never become a reading.
    pub chartable: bool,
    /// Which lifecycle moment this row would be charted on — `Some(Listing)` / `Some(Founding)` —
    /// or `None` when it isn't chartable. Lets the UI say *which* birth it is reading.
    pub moment: Option<Moment>,
}

/// Which moment of an entity's lifecycle a chart is cast for. An entity has several (founding,
/// incorporation, listing, …); Ziqpu names the one it read rather than pretending there is only one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Moment {
    /// The company became publicly tradeable — its IPO/listing. The v1 stock framing, and preferred
    /// when known because it is day-precise and carries a real time (the opening bell).
    Listing,
    /// The entity was founded. Used for a stock only when no listing date exists (most pre-1994
    /// companies), and always for the industry universes. No opening bell — a founding has no bell.
    Founding,
}

impl Moment {
    /// A short human label for the UI — "charted on its listing" / "charted on its founding".
    pub fn label(self) -> &'static str {
        match self {
            Moment::Listing => "listing",
            Moment::Founding => "founding",
        }
    }
}

/// One parsed row from any universe. The date logic is **resolved at parse time** into a single
/// chart moment, so nothing downstream re-derives it (that is how a stock's founding date used to
/// leak into a listing slot). `chart_date` is `Some` only when we hold a **day-precise** moment — a
/// chart needs a day for its angles, and a year-only founding is honestly unchartable, not rounded
/// up to January 1st.
struct Entity {
    ticker: String,
    name: String,
    ticker_lc: String,
    name_lc: String,
    name_norm: String,
    /// The day-precise moment to cast the chart for, or `None` when the entity is unchartable
    /// (no listing date and no day-precise founding). Search still lists it; it just can't be picked.
    chart_date: Option<NaiveDate>,
    /// The time of that moment: `Some(09:30)` for a listing (the opening bell), `None` for a
    /// founding (no bell — charts without angles, honestly flagged). Never invented.
    chart_time: Option<NaiveTime>,
    /// Which moment `chart_date` names, for the UI. Meaningful only when `chart_date` is `Some`.
    moment: Moment,
    tz: Tz,
    /// The chart location — the exchange's trading floor (stocks) or the CSV point (industry).
    lat: f64,
    lon: f64,
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
            chartable: r.chart_date.is_some(),
            moment: r.chart_date.map(|_| r.moment),
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
        chartable: chosen.chart_date.is_some(),
        moment: chosen.chart_date.map(|_| chosen.moment),
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
        // A single-word core name that is an ordinary English word must not resolve from free text —
        // the same protection the ticker path gets from [`STOPWORDS`], which the expanded SEC
        // universe made load-bearing: it lists real companies named "Here Group", "Nothing", etc.,
        // so "buy nothing here" would otherwise chart a holding company. A multi-word name
        // ("under armour") is specific enough to keep.
        if !core.contains(' ') && STOPWORDS.contains(&core.as_str()) {
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
/// Returns `None` when the id isn't in that universe's table, **or when we have no real birth date
/// for it** — 764 rows have neither a Polygon listing date nor a Form 8-A on record, and a chart
/// cannot be computed from a date we don't have. Those rows are still *searchable*; they are marked
/// unchartable ([`TickerRow::chartable`]) rather than hidden, so the seeker learns we don't know
/// instead of quietly getting a reading built on an invented moment.
///
/// Everything softer than the date still degrades gracefully, because these are honest partials
/// rather than inventions:
/// - time: the time column when parseable, else the universe's blank-time default ([`MARKET_OPEN`]
///   for Stocks — the real moment trading opens; `None` for industry, which charts without angles
///   and is flagged);
/// - coordinates: the CSV `latitude`/`longitude` when present and in range, else the universe's
///   fallback (the exchange's trading floor for Stocks — where the listing actually happened; a
///   neutral point for industry).
///
/// `cik`/`wiki` are `None` (these universes carry no curated grounding handles; the seeded demo's
/// five stocks do).
pub fn choice_in(u: Universe, id: &str) -> Option<Choice> {
    let key = id.trim().to_lowercase();
    let r = entities(u).iter().find(|r| r.ticker_lc == key)?;
    // No day-precise moment, no chart. The one thing we will not do is make one up. `chart_date` was
    // resolved once, at parse time, so this can never disagree with what search marked chartable.
    let date = r.chart_date?;
    Some(Choice {
        ticker: r.ticker.clone(),
        name: r.name.clone(),
        birth: BirthMoment {
            date,
            time: r.chart_time,
            tz: r.tz,
            lat: r.lat,
            lon: r.lon,
        },
        cik: None,
        wiki: None,
    })
}

/// Resolve a stock ticker — [`choice_in`] over [`Universe::Stocks`]. Kept for existing callers.
pub fn choice(ticker: &str) -> Option<Choice> {
    choice_in(Universe::Stocks, ticker)
}

/// Parse a **full-day** `YYYY-MM-DD` date, tolerating surrounding whitespace; `None` if blank,
/// malformed, or reduced-precision (`1978` or `1978-10`).
///
/// The reduced-precision rejection is the point, not an accident: the dataset stores year-only and
/// month-only facts as exactly that (`1978`, `1978-10`), and this returning `None` for them is what
/// makes such an entity **unchartable** rather than charted on a rounded-up January 1st. The 904
/// Jan-1 charts we deleted came from a pipeline that had no way to express "year"; here the year is
/// expressible and simply doesn't yield a castable moment.
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
        "CBOE" => CBOE_COORDS,
        _ => NEUTRAL_US_MARKET,
    }
}

/// Parse the committed stock CSV into entities, skipping the header and any blank lines. Columns:
/// `ticker,name,exchange,cik,conception_date,conception_prec,conception_src,birth_date,birth_prec,
/// birth_src,note` (regenerate with `scripts/build-tickers-csv.py`).
///
/// **The chart moment is resolved here, once.** A stock is charted on its **listing** when we hold
/// one — day-precise, at the opening bell — because that is the v1 "born onto the market" framing.
/// Otherwise it falls back to a **day-precise founding** (no bell — charts without angles, flagged),
/// which is the only date most pre-1994 companies have. A year-only or month-only founding yields no
/// castable moment, so the entity is unchartable: listed in search, marked, never invented.
///
/// Coordinates are the exchange's trading floor (where the listing happened), mapped from the
/// exchange column — not stored per row. Malformed rows (too few columns, empty ticker) are dropped
/// rather than panicking.
fn parse_stocks(csv: &str) -> Vec<Entity> {
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
            let (lat, lon) = fallback_coords(f[2].trim());

            // Listing first (day-precise, opening bell), else a day-precise founding (no bell).
            let birth = parse_date(f[7].trim());
            let conception = parse_date(f[4].trim());
            let (chart_date, chart_time, moment) = match (birth, conception) {
                (Some(b), _) => (Some(b), Some(MARKET_OPEN), Moment::Listing),
                (None, Some(c)) => (Some(c), None, Moment::Founding),
                (None, None) => (None, None, Moment::Founding),
            };

            Some(Entity {
                ticker_lc: ticker.to_lowercase(),
                name_norm,
                chart_date,
                chart_time,
                moment,
                tz: chrono_tz::America::New_York,
                lat,
                lon,
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
            let (lat, lon) = match (
                parse_coord(f[6].trim(), 90.0),
                parse_coord(f[7].trim(), 180.0),
            ) {
                (Some(la), Some(lo)) => (la, lo),
                _ => NEUTRAL_US_MARKET,
            };
            // Industry entities are dated by their FOUNDING — always a Founding moment. A blank or
            // reduced-precision founding date means unchartable (listed, marked), never invented. A
            // blank founding TIME stays unknown (charts without angles), never the opening bell.
            let chart_date = parse_date(f[2].trim());
            let chart_time = chart_date.and_then(|_| parse_time(f[3].trim()));
            Some(Entity {
                ticker_lc: ticker.to_lowercase(),
                name_norm,
                chart_date,
                chart_time,
                moment: Moment::Founding,
                tz,
                lat,
                lon,
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
    fn a_listing_charts_at_the_opening_bell() {
        // Tesla has a real SEC listing date (2010-06-29). A listing is charted AT the opening bell,
        // because that is a real fact about when trading began.
        let c = choice("TSLA").expect("TSLA has a listing date and resolves");
        assert_eq!(c.ticker, "TSLA");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(2010, 6, 29).unwrap());
        assert_eq!(
            c.birth.time,
            Some(MARKET_OPEN),
            "a listing carries the bell"
        );
        assert_eq!(c.birth.tz, chrono_tz::America::New_York);
        assert!(c.cik.is_none() && c.wiki.is_none());
    }

    #[test]
    fn a_founding_charts_without_a_bell() {
        // Apple listed in 1980, but that IPO predates EDGAR, so we have no listing date — only its
        // day-precise FOUNDING (1976-04-01). It charts on the founding, and a founding has NO opening
        // bell (that would be inventing a time), so it charts without angles, honestly.
        let c = choice("AAPL").expect("AAPL has a day-precise founding and resolves");
        assert_eq!(c.name, "Apple Inc.");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(1976, 4, 1).unwrap());
        assert_eq!(c.birth.time, None, "a founding has no opening bell");
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

    /// The listing dates are the CC0/public-domain re-derivation, not Polygon — and a listing is
    /// day-precise from the SEC prospectus.
    #[test]
    fn a_listing_is_day_precise_from_sec() {
        // UAA/UA (Under Armour) listed 2005-11-18 — an EDGAR-era IPO the 424B4 discriminator dates
        // exactly. This is the birth path, sourced from public-domain SEC filings.
        let c = choice("UAA").expect("UAA has a listing date");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(2005, 11, 18).unwrap());
        assert_eq!(c.birth.time, Some(MARKET_OPEN));
    }

    /// **A row we hold no day-precise moment for is never charted, and never invented.** (Nathan's
    /// P2, and the whole point of the CC0 re-derivation.)
    ///
    /// This is the honest end state of a long fight. Undated rows once got a fabricated 2000-01-01;
    /// then a `founding_date` fallback that stamped the opening bell; now: no day-precise moment ->
    /// no chart. Coca-Cola is the sharpest case — its Wikidata inception is a genuine CONFLICT
    /// (1892 incorporation vs 1886 "first Coke sold"), so we refuse to guess, and its 1919 NYSE
    /// listing predates EDGAR. Unchartable, and honest about it.
    #[test]
    fn a_row_with_no_castable_moment_is_unchartable_not_invented() {
        assert!(
            choice("KO").is_none(),
            "KO's founding is a conflict and its listing is pre-EDGAR — it must not resolve, \
             because the only way to chart it is to pick one of two contested dates or invent one"
        );

        // ...but it is still findable, and search knows it isn't chartable.
        let hit = search("KO")
            .into_iter()
            .find(|r| r.ticker == "KO")
            .expect("an unchartable row must still be searchable — hiding it is its own small lie");
        assert!(!hit.chartable, "KO has no castable moment");
        assert!(hit.moment.is_none(), "an unchartable row names no moment");
    }

    /// Every symbol search marks chartable must actually resolve, and every one it doesn't must not.
    ///
    /// The invariant is agreement, not totality. `chart_date` is resolved once at parse time and
    /// both `search` and `choice` read it, so they cannot disagree — a row shown as pickable that
    /// then refuses to be picked is worse than either answer alone. The counts are pinned so a
    /// regenerated CSV that silently loses the SEC/Wikidata enrichment fails loudly.
    #[test]
    fn chartable_and_resolvable_never_disagree() {
        let (mut chartable, mut unchartable) = (0, 0);
        for r in rows() {
            let flag = r.chart_date.is_some();
            let resolves = choice(&r.ticker).is_some();
            assert_eq!(
                flag,
                resolves,
                "{}: chartable={flag} but choice() {} — search and the resolver disagree",
                r.ticker,
                if resolves {
                    "resolved"
                } else {
                    "returned None"
                }
            );
            if flag {
                chartable += 1
            } else {
                unchartable += 1
            }
        }
        assert_eq!(chartable, 4_253, "chartable stock count drifted");
        assert_eq!(unchartable, 3_418, "unchartable stock count drifted");
    }

    #[test]
    fn a_founding_row_carries_the_founding_moment_and_exchange_coords() {
        // Apple: founding moment, NASDAQ trading-floor coordinates (where it would have listed).
        let c = choice("AAPL").expect("AAPL resolves on its founding");
        assert_eq!(c.birth.date, NaiveDate::from_ymd_opt(1976, 4, 1).unwrap());
        assert_eq!(c.birth.time, None);
        assert_eq!((c.birth.lat, c.birth.lon), NASDAQ_COORDS);
        // Search reports which moment it is.
        let hit = search("AAPL")
            .into_iter()
            .find(|r| r.ticker == "AAPL")
            .unwrap();
        assert_eq!(hit.moment, Some(Moment::Founding));
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

    /// The same agreement rule across the industry universes — where the gap is proportionally
    /// worse: 27 of 61 insurers ship a blank `birth_date`. Every one of them used to be charted on
    /// an invented 2000-01-01, which for a company founded in 1922 is not a rounding error, it is a
    /// different company.
    #[test]
    fn industry_chartable_and_resolvable_never_disagree() {
        for u in [Universe::Airlines, Universe::Insurance] {
            for r in entities(u) {
                assert_eq!(
                    r.chart_date.is_some(),
                    choice_in(u, &r.ticker).is_some(),
                    "{:?} {}: the search flag and the resolver disagree",
                    u,
                    r.ticker
                );
            }
        }
        // Airlines are fully dated; insurance has a real, known hole. Named so a future data drop
        // that quietly widens it has to argue with a number.
        let undated = |u| {
            entities(u)
                .iter()
                .filter(|r| r.chart_date.is_none())
                .count()
        };
        assert_eq!(
            undated(Universe::Airlines),
            0,
            "airlines gained an undated row"
        );
        assert_eq!(
            undated(Universe::Insurance),
            27,
            "insurance date coverage drifted"
        );
    }

    #[test]
    fn stocks_path_is_unchanged_via_universe() {
        // The Stocks wrappers and the universe-taking form must agree: AAPL charts on its founding
        // (1976-04-01, no bell), at NASDAQ's coordinates.
        let via_wrapper = choice("AAPL").expect("AAPL resolves");
        let via_universe = choice_in(Universe::Stocks, "AAPL").expect("AAPL resolves in Stocks");
        assert_eq!(via_wrapper.birth, via_universe.birth);
        assert_eq!(
            via_universe.birth.date,
            NaiveDate::from_ymd_opt(1976, 4, 1).unwrap()
        );
        assert_eq!(via_universe.birth.time, None);
        assert_eq!(via_universe.birth.tz, chrono_tz::America::New_York);
        assert_eq!(
            (via_universe.birth.lat, via_universe.birth.lon),
            NASDAQ_COORDS
        );
        assert_eq!(search("aapl"), search_in(Universe::Stocks, "aapl"));
    }
}
