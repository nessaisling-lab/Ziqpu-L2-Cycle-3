//! Ziqpu offline gazetteer — turn a place name into `lat`, `lon`, and an **IANA timezone**.
//!
//! Birth charts need the timezone *of the birthplace*, so this crate ships a compact binary
//! gazetteer (derived from GeoNames `cities5000`, CC BY 4.0) and resolves a query entirely
//! in-memory: **no network, no filesystem, no clock at runtime.** The blob is compiled in via
//! [`include_bytes!`] and parsed once, lazily. The generator that produces the blob lives in
//! `src/bin/gen.rs` and is the *only* code that reads GeoNames text — the library never does.
//!
//! Search is a deterministic linear scan: ASCII-fold + lowercase the query and each name, match
//! by prefix (then substring), rank by population descending with lexicographic tiebreaks, and
//! cap the result list. Same input → byte-identical output, so the UI on top of it is testable.

use std::sync::OnceLock;

/// The committed gazetteer blob. Fixed little-endian layout (see `src/bin/gen.rs`):
/// `magic "ZGEO" | u32 version | u32 tz_count | u32 row_count`, a tz string table
/// (`u16 len + UTF-8` each), then rows (`u8 ascii_len + ascii | u8 name_len + name |
/// f32 lat | f32 lon | u16 tz_idx | [u8;2] country | u32 population`).
static BLOB: &[u8] = include_bytes!("../data/cities.bin");

/// Cap on how many candidates a single [`lookup`] returns.
const MAX_RESULTS: usize = 20;

/// A resolved place: a display name, its country, coordinates, and the IANA timezone name.
///
/// `tz` is the canonical IANA id (e.g. `"America/New_York"`) precisely so it round-trips into
/// `chrono_tz::Tz` via `FromStr` at the call site.
#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub name: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz: String,
}

/// One parsed gazetteer row. `name`/`tz` borrow the `'static` blob; `key` is the folded,
/// lowercased search key derived from the ASCII name (allocated once at index build).
struct Row {
    key: String,
    name: &'static str,
    country: [u8; 2],
    lat: f32,
    lon: f32,
    tz: &'static str,
    population: u32,
}

/// Look up places by name. ASCII-folds and lowercases the query, matches by prefix first then
/// substring, ranks prefix-before-substring and then by population (descending), and returns at
/// most [`MAX_RESULTS`]. An empty or whitespace-only query returns an empty list. Pure and
/// deterministic — the order is total (population, then key, then name), so it never depends on
/// sort stability.
pub fn lookup(query: &str) -> Vec<Place> {
    let q = fold(query);
    if q.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(&Row, u8)> = Vec::new();
    for r in index() {
        if r.key.starts_with(&q) {
            hits.push((r, 0));
        } else if r.key.contains(&q) {
            hits.push((r, 1));
        }
    }
    hits.sort_by(|(ra, ta), (rb, tb)| {
        ta.cmp(tb)
            .then_with(|| rb.population.cmp(&ra.population))
            .then_with(|| ra.key.cmp(&rb.key))
            .then_with(|| ra.name.cmp(rb.name))
    });
    hits.into_iter()
        .take(MAX_RESULTS)
        .map(|(r, _)| Place {
            name: r.name.to_string(),
            country: country_name(r.country),
            lat: r.lat as f64,
            lon: r.lon as f64,
            tz: r.tz.to_string(),
        })
        .collect()
}

/// Parse and cache the gazetteer once, on first use.
fn index() -> &'static Vec<Row> {
    static IDX: OnceLock<Vec<Row>> = OnceLock::new();
    IDX.get_or_init(|| parse(BLOB))
}

/// A tiny forward-only reader over the `'static` blob. Panics on a malformed blob — which can
/// only happen if the committed artifact is corrupt, a build-time (not user-facing) failure.
struct Cursor {
    buf: &'static [u8],
    pos: usize,
}

impl Cursor {
    fn new(buf: &'static [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    fn take(&mut self, n: usize) -> &'static [u8] {
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        s
    }
    fn u8(&mut self) -> u8 {
        self.take(1)[0]
    }
    fn u16(&mut self) -> u16 {
        u16::from_le_bytes(self.take(2).try_into().unwrap())
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    fn f32(&mut self) -> f32 {
        f32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    fn str(&mut self, n: usize) -> &'static str {
        std::str::from_utf8(self.take(n)).expect("gazetteer blob holds valid UTF-8")
    }
}

fn parse(blob: &'static [u8]) -> Vec<Row> {
    let mut c = Cursor::new(blob);
    assert_eq!(c.take(4), b"ZGEO", "gazetteer blob has a bad magic header");
    assert_eq!(c.u32(), 1, "unsupported gazetteer blob version");
    let tz_count = c.u32() as usize;
    let row_count = c.u32() as usize;

    let mut tzs: Vec<&'static str> = Vec::with_capacity(tz_count);
    for _ in 0..tz_count {
        let len = c.u16() as usize;
        tzs.push(c.str(len));
    }

    let mut rows = Vec::with_capacity(row_count);
    for _ in 0..row_count {
        let ascii_len = c.u8() as usize;
        let ascii = c.str(ascii_len);
        let name_len = c.u8() as usize;
        let name = c.str(name_len);
        let lat = c.f32();
        let lon = c.f32();
        let tz_idx = c.u16() as usize;
        let country = [c.u8(), c.u8()];
        let population = c.u32();
        rows.push(Row {
            key: fold(ascii),
            name,
            country,
            lat,
            lon,
            tz: tzs[tz_idx],
            population,
        });
    }
    rows
}

/// ASCII-fold + lowercase a string for matching: strip common Latin diacritics, treat
/// hyphens/apostrophes/punctuation as spaces, drop anything else non-alphanumeric, and collapse
/// whitespace. Applied identically to the query and to each stored name so they align.
fn fold(s: &str) -> String {
    let mut raw = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'a'..='z' | '0'..='9' | ' ' => raw.push(c),
            'A'..='Z' => raw.push(c.to_ascii_lowercase()),
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => {
                raw.push('a')
            }
            'è' | 'é' | 'ê' | 'ë' | 'È' | 'É' | 'Ê' | 'Ë' => raw.push('e'),
            'ì' | 'í' | 'î' | 'ï' | 'Ì' | 'Í' | 'Î' | 'Ï' => raw.push('i'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => {
                raw.push('o')
            }
            'ù' | 'ú' | 'û' | 'ü' | 'Ù' | 'Ú' | 'Û' | 'Ü' => raw.push('u'),
            'ñ' | 'Ñ' => raw.push('n'),
            'ç' | 'Ç' => raw.push('c'),
            'ý' | 'ÿ' | 'Ý' => raw.push('y'),
            'ß' => raw.push_str("ss"),
            'æ' | 'Æ' => raw.push_str("ae"),
            'œ' | 'Œ' => raw.push_str("oe"),
            '-' | '\'' | '’' | '.' | ',' | '/' => raw.push(' '),
            _ => {
                if c.is_ascii_alphanumeric() {
                    raw.push(c.to_ascii_lowercase());
                } else if c.is_whitespace() {
                    raw.push(' ');
                }
            }
        }
    }
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// English name for an ISO 3166-1 alpha-2 country code, falling back to the raw code when the
/// code is unknown. Keeps [`Place::country`] human-readable without a network lookup.
fn country_name(code: [u8; 2]) -> String {
    for (c, name) in COUNTRIES {
        if *c == code {
            return (*name).to_string();
        }
    }
    String::from_utf8_lossy(&code).into_owned()
}

/// ISO 3166-1 alpha-2 → English short name. Not exhaustive of every territory, but covers the
/// countries GeoNames tags; unknown codes fall back to the raw two letters.
#[rustfmt::skip]
const COUNTRIES: &[([u8; 2], &str)] = &[
    (*b"AD", "Andorra"), (*b"AE", "United Arab Emirates"), (*b"AF", "Afghanistan"),
    (*b"AG", "Antigua and Barbuda"), (*b"AI", "Anguilla"), (*b"AL", "Albania"),
    (*b"AM", "Armenia"), (*b"AO", "Angola"), (*b"AQ", "Antarctica"), (*b"AR", "Argentina"),
    (*b"AS", "American Samoa"), (*b"AT", "Austria"), (*b"AU", "Australia"), (*b"AW", "Aruba"),
    (*b"AX", "Åland Islands"), (*b"AZ", "Azerbaijan"), (*b"BA", "Bosnia and Herzegovina"),
    (*b"BB", "Barbados"), (*b"BD", "Bangladesh"), (*b"BE", "Belgium"), (*b"BF", "Burkina Faso"),
    (*b"BG", "Bulgaria"), (*b"BH", "Bahrain"), (*b"BI", "Burundi"), (*b"BJ", "Benin"),
    (*b"BL", "Saint Barthélemy"), (*b"BM", "Bermuda"), (*b"BN", "Brunei"), (*b"BO", "Bolivia"),
    (*b"BQ", "Caribbean Netherlands"), (*b"BR", "Brazil"), (*b"BS", "Bahamas"), (*b"BT", "Bhutan"),
    (*b"BV", "Bouvet Island"), (*b"BW", "Botswana"), (*b"BY", "Belarus"), (*b"BZ", "Belize"),
    (*b"CA", "Canada"), (*b"CC", "Cocos Islands"), (*b"CD", "DR Congo"),
    (*b"CF", "Central African Republic"), (*b"CG", "Congo"), (*b"CH", "Switzerland"),
    (*b"CI", "Côte d'Ivoire"), (*b"CK", "Cook Islands"), (*b"CL", "Chile"), (*b"CM", "Cameroon"),
    (*b"CN", "China"), (*b"CO", "Colombia"), (*b"CR", "Costa Rica"), (*b"CU", "Cuba"),
    (*b"CV", "Cape Verde"), (*b"CW", "Curaçao"), (*b"CX", "Christmas Island"), (*b"CY", "Cyprus"),
    (*b"CZ", "Czechia"), (*b"DE", "Germany"), (*b"DJ", "Djibouti"), (*b"DK", "Denmark"),
    (*b"DM", "Dominica"), (*b"DO", "Dominican Republic"), (*b"DZ", "Algeria"), (*b"EC", "Ecuador"),
    (*b"EE", "Estonia"), (*b"EG", "Egypt"), (*b"EH", "Western Sahara"), (*b"ER", "Eritrea"),
    (*b"ES", "Spain"), (*b"ET", "Ethiopia"), (*b"FI", "Finland"), (*b"FJ", "Fiji"),
    (*b"FK", "Falkland Islands"), (*b"FM", "Micronesia"), (*b"FO", "Faroe Islands"),
    (*b"FR", "France"), (*b"GA", "Gabon"), (*b"GB", "United Kingdom"), (*b"GD", "Grenada"),
    (*b"GE", "Georgia"), (*b"GF", "French Guiana"), (*b"GG", "Guernsey"), (*b"GH", "Ghana"),
    (*b"GI", "Gibraltar"), (*b"GL", "Greenland"), (*b"GM", "Gambia"), (*b"GN", "Guinea"),
    (*b"GP", "Guadeloupe"), (*b"GQ", "Equatorial Guinea"), (*b"GR", "Greece"),
    (*b"GS", "South Georgia"), (*b"GT", "Guatemala"), (*b"GU", "Guam"), (*b"GW", "Guinea-Bissau"),
    (*b"GY", "Guyana"), (*b"HK", "Hong Kong"), (*b"HM", "Heard and McDonald Islands"),
    (*b"HN", "Honduras"), (*b"HR", "Croatia"), (*b"HT", "Haiti"), (*b"HU", "Hungary"),
    (*b"ID", "Indonesia"), (*b"IE", "Ireland"), (*b"IL", "Israel"), (*b"IM", "Isle of Man"),
    (*b"IN", "India"), (*b"IO", "British Indian Ocean Territory"), (*b"IQ", "Iraq"),
    (*b"IR", "Iran"), (*b"IS", "Iceland"), (*b"IT", "Italy"), (*b"JE", "Jersey"),
    (*b"JM", "Jamaica"), (*b"JO", "Jordan"), (*b"JP", "Japan"), (*b"KE", "Kenya"),
    (*b"KG", "Kyrgyzstan"), (*b"KH", "Cambodia"), (*b"KI", "Kiribati"), (*b"KM", "Comoros"),
    (*b"KN", "Saint Kitts and Nevis"), (*b"KP", "North Korea"), (*b"KR", "South Korea"),
    (*b"KW", "Kuwait"), (*b"KY", "Cayman Islands"), (*b"KZ", "Kazakhstan"), (*b"LA", "Laos"),
    (*b"LB", "Lebanon"), (*b"LC", "Saint Lucia"), (*b"LI", "Liechtenstein"), (*b"LK", "Sri Lanka"),
    (*b"LR", "Liberia"), (*b"LS", "Lesotho"), (*b"LT", "Lithuania"), (*b"LU", "Luxembourg"),
    (*b"LV", "Latvia"), (*b"LY", "Libya"), (*b"MA", "Morocco"), (*b"MC", "Monaco"),
    (*b"MD", "Moldova"), (*b"ME", "Montenegro"), (*b"MF", "Saint Martin"), (*b"MG", "Madagascar"),
    (*b"MH", "Marshall Islands"), (*b"MK", "North Macedonia"), (*b"ML", "Mali"), (*b"MM", "Myanmar"),
    (*b"MN", "Mongolia"), (*b"MO", "Macao"), (*b"MP", "Northern Mariana Islands"),
    (*b"MQ", "Martinique"), (*b"MR", "Mauritania"), (*b"MS", "Montserrat"), (*b"MT", "Malta"),
    (*b"MU", "Mauritius"), (*b"MV", "Maldives"), (*b"MW", "Malawi"), (*b"MX", "Mexico"),
    (*b"MY", "Malaysia"), (*b"MZ", "Mozambique"), (*b"NA", "Namibia"), (*b"NC", "New Caledonia"),
    (*b"NE", "Niger"), (*b"NF", "Norfolk Island"), (*b"NG", "Nigeria"), (*b"NI", "Nicaragua"),
    (*b"NL", "Netherlands"), (*b"NO", "Norway"), (*b"NP", "Nepal"), (*b"NR", "Nauru"),
    (*b"NU", "Niue"), (*b"NZ", "New Zealand"), (*b"OM", "Oman"), (*b"PA", "Panama"),
    (*b"PE", "Peru"), (*b"PF", "French Polynesia"), (*b"PG", "Papua New Guinea"),
    (*b"PH", "Philippines"), (*b"PK", "Pakistan"), (*b"PL", "Poland"),
    (*b"PM", "Saint Pierre and Miquelon"), (*b"PN", "Pitcairn"), (*b"PR", "Puerto Rico"),
    (*b"PS", "Palestine"), (*b"PT", "Portugal"), (*b"PW", "Palau"), (*b"PY", "Paraguay"),
    (*b"QA", "Qatar"), (*b"RE", "Réunion"), (*b"RO", "Romania"), (*b"RS", "Serbia"),
    (*b"RU", "Russia"), (*b"RW", "Rwanda"), (*b"SA", "Saudi Arabia"), (*b"SB", "Solomon Islands"),
    (*b"SC", "Seychelles"), (*b"SD", "Sudan"), (*b"SE", "Sweden"), (*b"SG", "Singapore"),
    (*b"SH", "Saint Helena"), (*b"SI", "Slovenia"), (*b"SJ", "Svalbard and Jan Mayen"),
    (*b"SK", "Slovakia"), (*b"SL", "Sierra Leone"), (*b"SM", "San Marino"), (*b"SN", "Senegal"),
    (*b"SO", "Somalia"), (*b"SR", "Suriname"), (*b"SS", "South Sudan"),
    (*b"ST", "São Tomé and Príncipe"), (*b"SV", "El Salvador"), (*b"SX", "Sint Maarten"),
    (*b"SY", "Syria"), (*b"SZ", "Eswatini"), (*b"TC", "Turks and Caicos Islands"), (*b"TD", "Chad"),
    (*b"TF", "French Southern Territories"), (*b"TG", "Togo"), (*b"TH", "Thailand"),
    (*b"TJ", "Tajikistan"), (*b"TK", "Tokelau"), (*b"TL", "Timor-Leste"), (*b"TM", "Turkmenistan"),
    (*b"TN", "Tunisia"), (*b"TO", "Tonga"), (*b"TR", "Turkey"), (*b"TT", "Trinidad and Tobago"),
    (*b"TV", "Tuvalu"), (*b"TW", "Taiwan"), (*b"TZ", "Tanzania"), (*b"UA", "Ukraine"),
    (*b"UG", "Uganda"), (*b"UM", "U.S. Minor Outlying Islands"), (*b"US", "United States"),
    (*b"UY", "Uruguay"), (*b"UZ", "Uzbekistan"), (*b"VA", "Vatican City"),
    (*b"VC", "Saint Vincent and the Grenadines"), (*b"VE", "Venezuela"),
    (*b"VG", "British Virgin Islands"), (*b"VI", "U.S. Virgin Islands"), (*b"VN", "Vietnam"),
    (*b"VU", "Vanuatu"), (*b"WF", "Wallis and Futuna"), (*b"WS", "Samoa"), (*b"XK", "Kosovo"),
    (*b"YE", "Yemen"), (*b"YT", "Mayotte"), (*b"ZA", "South Africa"), (*b"ZM", "Zambia"),
    (*b"ZW", "Zimbabwe"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_returns_empty() {
        assert!(lookup("").is_empty());
        assert!(lookup("    ").is_empty());
    }

    #[test]
    fn new_york_ranks_first() {
        let r = lookup("new york");
        assert!(!r.is_empty(), "expected New York results");
        assert!(
            r[0].name.to_lowercase().starts_with("new york"),
            "New York should rank first, got {:?}",
            r[0].name
        );
        assert_eq!(r[0].tz, "America/New_York");
        assert_eq!(r[0].country, "United States");
    }

    #[test]
    fn tokyo_prefix_resolves() {
        let r = lookup("toky");
        assert!(!r.is_empty(), "expected a Tokyo prefix hit");
        assert_eq!(r[0].name, "Tokyo");
        assert_eq!(r[0].tz, "Asia/Tokyo");
    }

    #[test]
    fn midsize_town_resolves_with_its_iana_tz() {
        // Reykjavík: a unique, mid-size city — pins that the folded search and stored tz agree.
        let r = lookup("reykjavik");
        assert!(!r.is_empty(), "expected Reykjavik");
        assert_eq!(r[0].tz, "Atlantic/Reykjavik");
        assert_eq!(r[0].country, "Iceland");
    }

    #[test]
    fn results_are_bounded_and_deterministic() {
        let a = lookup("san");
        assert!(a.len() <= MAX_RESULTS, "capped at {MAX_RESULTS}");
        let b = lookup("san");
        assert_eq!(a, b, "same query must be byte-identical across runs");
    }

    #[test]
    fn every_stored_tz_parses_to_chrono_tz() {
        use chrono_tz::Tz;
        for row in index() {
            assert!(
                row.tz.parse::<Tz>().is_ok(),
                "committed blob holds a tz that chrono_tz cannot parse: {}",
                row.tz
            );
        }
    }
}
