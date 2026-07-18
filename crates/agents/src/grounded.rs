//! The grounded-signal tool — the loop's one action that reaches the outside world. It is the
//! reason the checkpoint exists (external, gated, costed). Default is a deterministic mock for
//! CI and offline demos; the live source pulls real SEC EDGAR filings by shelling out to `curl`
//! (so the crate adds no HTTP dependency and CI never touches the network).

use crate::types::{Choice, GroundedSignals};

/// A source of real-world signals about a choice.
pub trait GroundedSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals;
}

/// Lets a boxed source be used wherever a `GroundedSource` is expected (runtime selection).
impl GroundedSource for Box<dyn GroundedSource> {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        (**self).fetch(choice)
    }
}

/// Deterministic stand-in — no network. Used in CI and as the default demo source.
#[derive(Default)]
pub struct MockGroundedSource;

impl GroundedSource for MockGroundedSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        GroundedSignals {
            choice: choice.ticker.clone(),
            source: "mock (recorded fixture)".to_string(),
            items: vec![
                format!("recent filings for {} would appear here", choice.ticker),
                "grounded-source mock — no live network in CI".to_string(),
            ],
        }
    }
}

/// The contact address sent to SEC EDGAR, overridable with `ZIQPU_EDGAR_UA`.
///
/// SEC's fair-access policy requires a User-Agent that reaches a human, and this string ships in
/// the binary — so **every** grounded pull, by every person who installs Ziqpu, carries it. That
/// makes it a *role* address by necessity, not a preference: a maintainer's personal or
/// institutional address here would hand strangers' traffic a name that never consented to it, and
/// would rot the moment that person moved on. Whoever answers this mailbox answers for the fleet.
///
/// Keep it a real, monitored address. A plausible-looking dead mailbox is worse than none: it
/// satisfies the format while quietly breaking the policy's actual purpose, and SEC can block the
/// User-Agent for the whole fleet at once.
///
/// It is also **public** — in a public repo, in every shipped binary, on every request — so expect
/// it to be scraped. That's the trade SEC's policy asks for; it's a reason to use a mailbox that
/// can absorb spam, not a reason to omit the contact.
const DEFAULT_EDGAR_UA: &str = "Ziqpu research (ness.aisling@nisabacapitalcharting.com)";

/// Real, keyless SEC EDGAR submissions pull. Live only. SEC policy requires a contact
/// User-Agent — see [`DEFAULT_EDGAR_UA`].
pub struct EdgarSource {
    pub user_agent: String,
}

impl Default for EdgarSource {
    /// Uses [`DEFAULT_EDGAR_UA`] unless `ZIQPU_EDGAR_UA` overrides it — so a fork, a partner
    /// deployment, or a heavy researcher can identify as themselves (which SEC's policy actually
    /// wants) without patching the source. Blank or whitespace is treated as unset: an empty
    /// contact would be a policy violation dressed as a configuration.
    fn default() -> Self {
        let ua = std::env::var("ZIQPU_EDGAR_UA")
            .map(|s| s.trim().to_string())
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_EDGAR_UA.to_string());
        Self { user_agent: ua }
    }
}

impl GroundedSource for EdgarSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        // 1) Try the live SEC EDGAR filings pull. `None` = transport error / blocked / rate-limited.
        let live_filings = choice.cik.and_then(|cik| self.fetch_filings(cik));
        // 2) Wikipedia — a keyless "what this actually is" reality check that works on most networks.
        let wiki = choice
            .wiki
            .as_deref()
            .and_then(|title| self.fetch_wikipedia(title));

        // Live filings succeeded → the real briefing takes precedence.
        if let Some(mut items) = live_filings {
            if let Some(summary) = wiki {
                items.push(format!("what it is: {summary}"));
            }
            if items.is_empty() {
                items.push(NO_SIGNALS.to_string());
            }
            return GroundedSignals {
                choice: choice.ticker.clone(),
                source: "SEC EDGAR + Wikipedia".to_string(),
                items,
            };
        }

        // Live filings failed/blocked. For a known demo ticker, fall back to a bundled recorded
        // fixture so the demo's grounded ACT beat still lands (labelled honestly as a fixture).
        if let Some(mut items) = fixture_filings(&choice.ticker) {
            if let Some(summary) = wiki {
                items.push(format!("what it is: {summary}"));
            }
            return GroundedSignals {
                choice: choice.ticker.clone(),
                source: "SEC EDGAR (recorded fixture — live pull unavailable)".to_string(),
                items,
            };
        }

        // Unknown ticker and no live data — degrade honestly rather than invent signals.
        let mut items = Vec::new();
        if let Some(summary) = wiki {
            items.push(format!("what it is: {summary}"));
        }
        if items.is_empty() {
            items.push(NO_SIGNALS.to_string());
        }
        GroundedSignals {
            choice: choice.ticker.clone(),
            source: "SEC EDGAR + Wikipedia".to_string(),
            items,
        }
    }
}

/// Bundled recorded-fixture filings for the five demo tickers (AAPL, MSFT, TSLA, KO, JNJ), used
/// only when a live SEC EDGAR pull fails or is blocked — so the demo's grounded beat still lands
/// with realistic recent-filing lines instead of an empty/blocked note. These are recorded
/// snapshots, not live data; the fetch labels them as a fixture. Unknown tickers return `None`
/// (honest degrade).
fn fixture_filings(ticker: &str) -> Option<Vec<String>> {
    let lines: &[&str] = match ticker {
        "AAPL" => &[
            "recent filing: 10-K on 2025-11-01",
            "recent filing: 8-K on 2026-01-30",
            "recent filing: 10-Q on 2026-01-31",
            "industry: Electronic Computers",
        ],
        "MSFT" => &[
            "recent filing: 10-K on 2025-07-30",
            "recent filing: 8-K on 2026-01-27",
            "recent filing: 10-Q on 2026-01-28",
            "industry: Services-Prepackaged Software",
        ],
        "TSLA" => &[
            "recent filing: 10-K on 2026-01-27",
            "recent filing: 8-K on 2026-01-22",
            "recent filing: 10-Q on 2025-10-23",
            "industry: Motor Vehicles & Passenger Car Bodies",
        ],
        "KO" => &[
            "recent filing: 10-K on 2026-02-20",
            "recent filing: 8-K on 2026-02-11",
            "recent filing: 10-Q on 2025-10-28",
            "industry: Beverages",
        ],
        "JNJ" => &[
            "recent filing: 10-K on 2026-02-13",
            "recent filing: 8-K on 2026-01-21",
            "recent filing: 10-Q on 2025-10-14",
            "industry: Pharmaceutical Preparations",
        ],
        _ => return None,
    };
    Some(lines.iter().map(|s| s.to_string()).collect())
}

impl EdgarSource {
    /// GET `url` with the contact User-Agent; `--compressed` so gzip'd bodies decode. `None` on a
    /// transport error, a timeout, or a non-2xx status (`curl` returns exit 0 on HTTP errors, so we
    /// don't rely on that alone — callers treat a non-JSON body as "unavailable").
    ///
    /// **The bounds are load-bearing, not hygiene.** `curl` has no default *transfer* timeout, and
    /// this call is made from a worker thread whose only exit is a result: the checkpoint renders a
    /// "grounding…" view with no cancel and no error path, so a `curl` that never returns is a
    /// permanent silent spinner and a wedged thread. A refused connection is fine — that errors and
    /// falls back to the recorded fixture. The killer is a connection that is *accepted and then
    /// stalls*: a captive portal that answers the handshake and blackholes, a Wi-Fi roam, a laptop
    /// resuming from sleep with a half-open socket. Without `--max-time` those hang forever.
    ///
    /// `--max-filesize` bounds the other end: a large filer's submissions JSON is multi-MB, and we
    /// only ever read the first few entries. Both values match the convention the rest of the tree
    /// already follows (see `model::system_cmd` callers, requirement SEC-004).
    fn get(&self, url: &str) -> Option<Vec<u8>> {
        http_get(url, &self.user_agent)
    }

    /// Three most recent filings (form + date) plus the SIC industry, from the submissions payload.
    /// Returns `None` on a transport error, a non-JSON body (a 403/HTML block or throttle on some
    /// networks), or a malformed payload — the caller then falls back to a recorded fixture for
    /// known tickers rather than surfacing a bare error note.
    fn fetch_filings(&self, cik: u32) -> Option<Vec<String>> {
        let url = format!("https://data.sec.gov/submissions/CIK{cik:010}.json");
        let bytes = self.get(&url)?;
        let value = serde_json::from_slice::<serde_json::Value>(&bytes).ok()?;
        let recent = &value["filings"]["recent"];
        let (Some(forms), Some(dates)) =
            (recent["form"].as_array(), recent["filingDate"].as_array())
        else {
            return None;
        };
        let mut out: Vec<String> = forms
            .iter()
            .zip(dates)
            .take(3)
            .map(|(form, date)| {
                format!(
                    "recent filing: {} on {}",
                    form.as_str().unwrap_or("?"),
                    date.as_str().unwrap_or("?")
                )
            })
            .collect();
        if let Some(sic) = value["sicDescription"].as_str() {
            out.push(format!("industry: {sic}"));
        }
        Some(out)
    }

    /// A short blurb from the Wikipedia summary for `title` (keyless REST API). Capped at ~200
    /// chars on a word boundary — avoids over-eager sentence splitting on abbreviations ("Inc.").
    fn fetch_wikipedia(&self, title: &str) -> Option<String> {
        let url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{title}");
        let bytes = self.get(&url)?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        let extract = value["extract"].as_str()?.trim();
        if extract.is_empty() {
            return None;
        }
        if extract.chars().count() <= 200 {
            return Some(extract.to_string());
        }
        let mut short: String = extract.chars().take(200).collect();
        if let Some(idx) = short.rfind(' ') {
            short.truncate(idx);
        }
        Some(format!("{short}…"))
    }
}

/// The honest "nothing to show" marker. [`has_real_signals`](crate::) and the composite both key on
/// this exact string, so it is defined once here and reused rather than spelled out at each site.
pub(crate) const NO_SIGNALS: &str = "no public signals available";

/// Bounded, contact-identified HTTP GET shared by every live grounded source.
///
/// The timeouts are load-bearing, not hygiene — see [`EdgarSource::get`]'s history: this runs on a
/// worker thread whose only exit is a result, behind a checkpoint view with no cancel path, so a
/// stalled socket is a permanent silent spinner. `--max-time` bounds a connection that is accepted
/// then blackholed; `--max-filesize` bounds a multi-MB payload we only skim the head of.
fn http_get(url: &str, user_agent: &str) -> Option<Vec<u8>> {
    let output = crate::no_window(std::process::Command::new("curl"))
        .args([
            "-sS",
            "--compressed",
            "--max-time",
            "8",
            "--max-filesize",
            "5000000",
            "-H",
            &format!("User-Agent: {user_agent}"),
            url,
        ])
        .output()
        .ok()?;
    output.status.success().then_some(output.stdout)
}

/// Is this item an empty/placeholder marker rather than a real fetched fact? The composite strips
/// these before merging so one source's "nothing here" never pollutes another's real signals, and so
/// the merged set is only marked unsourced when *every* source came up empty. Kept in lockstep with
/// [`has_real_signals`](crate::)'s marker list.
fn is_placeholder(item: &str) -> bool {
    let i = item.trim().to_lowercase();
    i.is_empty()
        || i.contains("no public signals available")
        || i.contains("no recent signals")
        || i.contains("grounded-source mock")
        || i.contains("no live network")
        || i.contains("would appear here")
}

/// The default SEC contact User-Agent, honoring `ZIQPU_EDGAR_UA` — shared by every SEC-hitting
/// source so the whole fleet identifies one way. See [`DEFAULT_EDGAR_UA`] for why it is a role
/// address by necessity.
fn sec_user_agent() -> String {
    std::env::var("ZIQPU_EDGAR_UA")
        .map(|s| s.trim().to_string())
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_EDGAR_UA.to_string())
}

/// Render a USD figure the way a person reads a balance sheet — `$1.02B`, `$282.2M`, else the raw
/// dollars. Grounding is a reading, not a spreadsheet; two significant figures is the register.
fn format_usd(val: f64) -> String {
    let a = val.abs();
    if a >= 1e9 {
        format!("${:.2}B", val / 1e9)
    } else if a >= 1e6 {
        format!("${:.1}M", val / 1e6)
    } else if a >= 1e3 {
        format!("${:.0}K", val / 1e3)
    } else {
        format!("${val:.0}")
    }
}

/// Revenue lives under several us-gaap concept names across filers and eras (the plain `Revenues`
/// concept was largely retired after the 2018 revenue-recognition standard). Tried in order; the
/// newest datapoint across whichever ones return wins.
const REVENUE_CONCEPTS: &[&str] = &[
    "RevenueFromContractWithCustomerExcludingAssessedTax",
    "Revenues",
    "RevenueFromContractWithCustomerIncludingAssessedTax",
    "SalesRevenueNet",
];

/// Real, keyless SEC financial fundamentals from the XBRL company-facts API (`data.sec.gov`). Adds a
/// dimension EDGAR's filings list does not: the company's most recently *reported numbers* — latest
/// revenue and total assets — each dated to the period end and the form it came from. Public domain
/// (17 USC 105). Live only; needs a CIK, so it no-ops for the industry universes.
pub struct SecFactsSource {
    pub user_agent: String,
}

impl Default for SecFactsSource {
    fn default() -> Self {
        Self {
            user_agent: sec_user_agent(),
        }
    }
}

impl GroundedSource for SecFactsSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        let mut items = Vec::new();
        if let Some(cik) = choice.cik {
            if let Some(line) = self.latest_usd(cik, REVENUE_CONCEPTS, "revenue") {
                items.push(line);
            }
            if let Some(line) = self.latest_usd(cik, &["Assets"], "total assets") {
                items.push(line);
            }
        }
        GroundedSignals {
            choice: choice.ticker.clone(),
            source: "SEC financials (XBRL)".to_string(),
            items,
        }
    }
}

impl SecFactsSource {
    /// The newest USD datapoint across `concepts` for `cik`, formatted as one grounded line, or
    /// `None` on any transport/parse failure or if no concept returned data. "Newest" is by period
    /// `end`, so a fresh 10-Q beats a stale 10-K and a retired concept never shadows a live one.
    fn latest_usd(&self, cik: u32, concepts: &[&str], label: &str) -> Option<String> {
        // (end, val, form, start). `start` is `None` for an instant fact (Assets — a point in time)
        // and `Some` for a duration fact (revenue — a span). It is load-bearing for honesty: a 10-Q
        // reports several revenue durations ending on the same date (the 3-month quarter AND the
        // cumulative year-to-date), so a bare "revenue: $254.94B" would silently present a
        // multi-quarter total as if it were the period's revenue. We label the real span instead.
        let mut best: Option<(String, f64, String, Option<String>)> = None;
        for concept in concepts {
            let url = format!(
                "https://data.sec.gov/api/xbrl/companyconcept/CIK{cik:010}/us-gaap/{concept}.json"
            );
            let Some(bytes) = http_get(&url, &self.user_agent) else {
                continue;
            };
            let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
                continue;
            };
            let Some(units) = value["units"]["USD"].as_array() else {
                continue;
            };
            for pt in units {
                let (Some(end), Some(val)) = (pt["end"].as_str(), pt["val"].as_f64()) else {
                    continue;
                };
                let form = pt["form"].as_str().unwrap_or("").to_string();
                let start = pt["start"].as_str().map(str::to_string);
                // Prefer the newest period end; among facts sharing that end, prefer the *shortest*
                // span (the later start) — i.e. the clean quarter over the year-to-date blob.
                let take = match best.as_ref() {
                    None => true,
                    Some((b_end, _, _, b_start)) => {
                        end > b_end.as_str()
                            || (end == b_end.as_str()
                                && start.as_deref().unwrap_or("")
                                    > b_start.as_deref().unwrap_or(""))
                    }
                };
                if take {
                    best = Some((end.to_string(), val, form, start));
                }
            }
        }
        let (end, val, form, start) = best?;
        let form_note = if form.is_empty() {
            String::new()
        } else {
            format!(", {form}")
        };
        // A duration fact names its span (so the reader sees a quarter, not an implied year); an
        // instant fact (Assets) is a point-in-time balance.
        let period = match start {
            Some(s) => format!("over {s} → {end}"),
            None => format!("as of {end}"),
        };
        Some(format!(
            "{label}: {} ({period}{form_note})",
            format_usd(val)
        ))
    }
}

/// Instance-of (P31) QIDs that mark an entity as a company/organization. Used to reject a
/// mis-resolved Wikidata hit — if a name search lands on something that is *not* an organization
/// (the fruit rather than the company), we refuse to attach its facts rather than risk a wrong one.
const ORG_TYPES: &[&str] = &[
    "Q4830453",  // business
    "Q891723",   // public company
    "Q6881511",  // enterprise
    "Q783794",   // company
    "Q167037",   // corporation
    "Q43229",    // organization
    "Q18388277", // technology company
    "Q219577",   // holding company
];

/// Real, CC0 structured facts from Wikidata — the founding year and headcount a filings list never
/// carries. The single-source that spans every future N3 domain (products/films/games via `P577`),
/// which is why it earns its place now even though it is thin for some tickers. Never fabricates: a
/// name that resolves to a non-organization, or an entity with none of the wanted claims, yields no
/// items rather than a guessed one.
pub struct WikidataSource {
    pub user_agent: String,
}

impl Default for WikidataSource {
    fn default() -> Self {
        Self {
            user_agent: sec_user_agent(),
        }
    }
}

impl GroundedSource for WikidataSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        let mut items = Vec::new();
        if let Some((qid, label)) = self.resolve_qid(&choice.name) {
            // Guard against a name search that landed on the wrong entity: the resolved label must
            // share a real word with the company name.
            if label_overlaps(&choice.name, &label) {
                if let Some(entity) = self.entity_json(&qid) {
                    if is_organization(&entity, &qid) {
                        if let Some(year) = inception_year(&entity, &qid) {
                            items.push(format!("founded: {year}"));
                        }
                        if let Some(n) = employees(&entity, &qid) {
                            items.push(format!("employees: {n}"));
                        }
                    }
                }
            }
        }
        GroundedSignals {
            choice: choice.ticker.clone(),
            source: "Wikidata".to_string(),
            items,
        }
    }
}

impl WikidataSource {
    /// Resolve a company name to its Wikidata `(QID, label)` via the keyless entity search. Searches
    /// on the name with its corporate suffix stripped — Wikidata labels are the bare trade name
    /// ("Manhattan Associates"), so an unstripped "MANHATTAN ASSOCIATES INC" returns *no hit* and
    /// silently kills coverage for nearly every ticker. Takes the top hit; the caller's label-overlap
    /// and organization guards catch the rare miss.
    fn resolve_qid(&self, name: &str) -> Option<(String, String)> {
        let q = urlencoding_min(&search_name(name));
        let url = format!(
            "https://www.wikidata.org/w/api.php?action=wbsearchentities&search={q}&language=en&format=json&limit=1"
        );
        let bytes = http_get(&url, &self.user_agent)?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        let hit = value["search"].as_array()?.first()?;
        let id = hit["id"].as_str()?.to_string();
        let label = hit["label"].as_str().unwrap_or("").to_string();
        Some((id, label))
    }

    /// The full entity JSON (`Special:EntityData/{qid}.json`) — one call carries every claim we read.
    fn entity_json(&self, qid: &str) -> Option<serde_json::Value> {
        let url = format!("https://www.wikidata.org/wiki/Special:EntityData/{qid}.json");
        let bytes = http_get(&url, &self.user_agent)?;
        serde_json::from_slice(&bytes).ok()
    }
}

/// The `claims` object of an entity, indexed by property id.
fn claims<'a>(entity: &'a serde_json::Value, qid: &str) -> Option<&'a serde_json::Value> {
    Some(&entity["entities"][qid]["claims"])
}

/// Does the entity's instance-of (P31) place it among [`ORG_TYPES`]? A resolved hit with a P31 that
/// matches nothing organizational is treated as a mis-resolution and its facts are refused.
fn is_organization(entity: &serde_json::Value, qid: &str) -> bool {
    let Some(cl) = claims(entity, qid) else {
        return false;
    };
    let Some(p31) = cl["P31"].as_array() else {
        return false;
    };
    p31.iter().any(|c| {
        c["mainsnak"]["datavalue"]["value"]["id"]
            .as_str()
            .is_some_and(|id| ORG_TYPES.contains(&id))
    })
}

/// Founding year from P571 (inception). Year precision (`9`) is enough for a fact line — this is a
/// *reading*, not a chart moment (the chart casts from the CSV date, never from this). Reduced to a
/// bare year regardless of the stored day, because a Wikidata "January 1" is nearly always a
/// year-precision placeholder, and we will not present a placeholder as a day.
fn inception_year(entity: &serde_json::Value, qid: &str) -> Option<i64> {
    let cl = claims(entity, qid)?;
    let time =
        cl["P571"].as_array()?.first()?["mainsnak"]["datavalue"]["value"]["time"].as_str()?;
    // "+1990-01-01T00:00:00Z" → 1990
    time.trim_start_matches('+')
        .split('-')
        .next()?
        .parse::<i64>()
        .ok()
        .filter(|y| *y > 0)
}

/// Latest employee count from P1128 (quantity). Wikidata stores the amount as a string like
/// `"+2600"`; we surface the integer.
fn employees(entity: &serde_json::Value, qid: &str) -> Option<String> {
    let cl = claims(entity, qid)?;
    let amount = cl["P1128"].as_array()?.last()?["mainsnak"]["datavalue"]["value"]["amount"]
        .as_str()?
        .trim_start_matches('+');
    let n: f64 = amount.parse().ok()?;
    Some(format!("{}", n as i64))
}

/// Does the resolved Wikidata label share a significant word with the company name? A cheap guard
/// (no extra request) against a search landing on an unrelated entity. Ignores the corporate-suffix
/// noise words so "Manhattan Associates Inc" still matches the label "Manhattan Associates".
fn label_overlaps(name: &str, label: &str) -> bool {
    let stop = [
        "inc", "corp", "co", "ltd", "llc", "the", "company", "plc", "group", "holdings",
    ];
    let words = |s: &str| -> Vec<String> {
        s.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 2 && !stop.contains(w))
            .map(|w| w.to_string())
            .collect()
    };
    let name_words = words(name);
    let label_words = words(label);
    name_words.iter().any(|w| label_words.contains(w))
}

/// Strip the corporate-form suffix from a company name for a Wikidata name search. Wikidata's label
/// is the bare trade name, so "MANHATTAN ASSOCIATES INC" must become "MANHATTAN ASSOCIATES" or the
/// search returns nothing. Drops standalone corporate-form tokens and their surrounding punctuation
/// ("Tesla, Inc." → "Tesla"); never returns empty (falls back to the trimmed original).
fn search_name(name: &str) -> String {
    const SUFFIX: &[&str] = &[
        "inc",
        "incorporated",
        "corp",
        "corporation",
        "co",
        "company",
        "ltd",
        "limited",
        "llc",
        "plc",
        "lp",
        "holdings",
        "holding",
        "group",
        "sa",
        "nv",
        "ag",
        "the",
        "class",
    ];
    let mut kept: Vec<String> = Vec::new();
    for w in name.split_whitespace() {
        let clean = w.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.is_empty() || SUFFIX.contains(&clean.to_lowercase().as_str()) {
            continue;
        }
        kept.push(clean.to_string());
    }
    if kept.is_empty() {
        name.trim().to_string()
    } else {
        kept.join(" ")
    }
}

/// Minimal percent-encoding for a query value — enough for company names in a Wikidata/EDGAR query
/// string. Keeps the crate dependency-free (no `urlencoding` crate) the way the rest of `agents`
/// shells out rather than pulling HTTP/encoding deps.
fn urlencoding_min(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Fans a choice out across several [`GroundedSource`]s and merges what they return into one honest
/// briefing. Each source's placeholder lines are stripped before merging (so one source's "nothing
/// here" never dilutes another's real facts); the merged `source` label names only the sources that
/// actually contributed; and the whole set is marked unsourced ([`NO_SIGNALS`]) only when *every*
/// source came up empty. This is the deterministic multi-source grounding — the same fetchers become
/// the tools the agentic loop selects from in the next increment.
pub struct CompositeSource {
    sources: Vec<Box<dyn GroundedSource>>,
}

impl CompositeSource {
    /// The shipped live grounding: SEC filings + Wikipedia (with the demo fixture fallback), plus the
    /// two new provenance-clean dimensions — SEC financials and Wikidata structured facts.
    pub fn live_default() -> Self {
        Self {
            sources: vec![
                Box::new(EdgarSource::default()),
                Box::new(SecFactsSource::default()),
                Box::new(WikidataSource::default()),
            ],
        }
    }

    /// Build a composite from an explicit source list — used by tests to fan deterministic in-memory
    /// sources and assert the merge behavior without touching the network.
    pub fn from_sources(sources: Vec<Box<dyn GroundedSource>>) -> Self {
        Self { sources }
    }
}

impl GroundedSource for CompositeSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals {
        let mut items: Vec<String> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        for source in &self.sources {
            let sig = source.fetch(choice);
            let real: Vec<String> = sig
                .items
                .into_iter()
                .filter(|i| !is_placeholder(i))
                .collect();
            if !real.is_empty() {
                labels.push(sig.source);
                items.extend(real);
            }
        }
        if items.is_empty() {
            return GroundedSignals {
                choice: choice.ticker.clone(),
                source: "(no public signals)".to_string(),
                items: vec![NO_SIGNALS.to_string()],
            };
        }
        labels.dedup();
        GroundedSignals {
            choice: choice.ticker.clone(),
            source: labels.join(" · "),
            items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deterministic in-memory source, so the composite's merge logic is testable without a
    /// network. Returns the same fixed signals for any choice.
    struct FakeSource {
        source: &'static str,
        items: Vec<String>,
    }
    impl GroundedSource for FakeSource {
        fn fetch(&self, choice: &Choice) -> GroundedSignals {
            GroundedSignals {
                choice: choice.ticker.clone(),
                source: self.source.to_string(),
                items: self.items.clone(),
            }
        }
    }
    fn demo_choice() -> Choice {
        Choice {
            ticker: "TEST".to_string(),
            name: "Test Co".to_string(),
            birth: crate::BirthMoment {
                date: chrono::NaiveDate::from_ymd_opt(2000, 1, 2).unwrap(),
                time: None,
                tz: chrono_tz::America::New_York,
                lat: 0.0,
                lon: 0.0,
            },
            cik: None,
            wiki: None,
        }
    }

    #[test]
    fn composite_merges_real_items_and_names_only_contributing_sources() {
        let composite = CompositeSource::from_sources(vec![
            Box::new(FakeSource {
                source: "SEC EDGAR + Wikipedia",
                items: vec!["recent filing: 10-K on 2026-01-01".to_string()],
            }),
            Box::new(FakeSource {
                source: "SEC financials (XBRL)",
                items: vec!["revenue: $282.2M (ended 2026-03-31, 10-Q)".to_string()],
            }),
            // Contributes nothing → its label must NOT appear.
            Box::new(FakeSource {
                source: "Wikidata",
                items: vec![],
            }),
        ]);
        let sig = composite.fetch(&demo_choice());
        assert_eq!(
            sig.items.len(),
            2,
            "both real items survive: {:?}",
            sig.items
        );
        assert_eq!(
            sig.source, "SEC EDGAR + Wikipedia · SEC financials (XBRL)",
            "empty Wikidata source must not be named"
        );
    }

    #[test]
    fn composite_strips_placeholders_so_they_never_dilute_real_signals() {
        let composite = CompositeSource::from_sources(vec![
            Box::new(FakeSource {
                source: "SEC EDGAR + Wikipedia",
                items: vec![NO_SIGNALS.to_string()],
            }),
            Box::new(FakeSource {
                source: "Wikidata",
                items: vec!["founded: 1990".to_string()],
            }),
        ]);
        let sig = composite.fetch(&demo_choice());
        assert_eq!(sig.items, vec!["founded: 1990".to_string()]);
        assert_eq!(sig.source, "Wikidata", "the empty EDGAR label is dropped");
    }

    #[test]
    fn composite_marks_unsourced_only_when_every_source_is_empty() {
        let composite = CompositeSource::from_sources(vec![
            Box::new(FakeSource {
                source: "SEC EDGAR + Wikipedia",
                items: vec![NO_SIGNALS.to_string()],
            }),
            Box::new(FakeSource {
                source: "Wikidata",
                items: vec![],
            }),
        ]);
        let sig = composite.fetch(&demo_choice());
        assert_eq!(sig.items, vec![NO_SIGNALS.to_string()]);
        // is_placeholder(NO_SIGNALS) is true → the reading layer will (correctly) mark this unsourced.
        assert!(is_placeholder(&sig.items[0]));
    }

    #[test]
    fn usd_formats_at_a_readers_precision() {
        assert_eq!(format_usd(1_020_000_000.0), "$1.02B");
        assert_eq!(format_usd(282_215_000.0), "$282.2M");
        assert_eq!(format_usd(740_538_000.0), "$740.5M");
        assert_eq!(format_usd(9_500.0), "$10K");
        assert_eq!(format_usd(512.0), "$512");
    }

    #[test]
    fn inception_reduces_to_a_bare_year_never_a_placeholder_day() {
        let entity = serde_json::json!({
            "entities": { "Q1": { "claims": {
                "P571": [ { "mainsnak": { "datavalue": { "value": { "time": "+1990-01-01T00:00:00Z" } } } } ]
            }}}
        });
        assert_eq!(inception_year(&entity, "Q1"), Some(1990));
    }

    #[test]
    fn organization_guard_rejects_a_non_org_hit() {
        // An entity whose P31 is "fruit" (Q3249551, not in ORG_TYPES) must be refused.
        let fruit = serde_json::json!({
            "entities": { "Q89": { "claims": {
                "P31": [ { "mainsnak": { "datavalue": { "value": { "id": "Q3249551" } } } } ]
            }}}
        });
        assert!(!is_organization(&fruit, "Q89"));
        let company = serde_json::json!({
            "entities": { "Q3285819": { "claims": {
                "P31": [ { "mainsnak": { "datavalue": { "value": { "id": "Q891723" } } } } ]
            }}}
        });
        assert!(is_organization(&company, "Q3285819"));
    }

    #[test]
    fn label_overlap_ignores_corporate_suffix_noise() {
        assert!(label_overlaps(
            "MANHATTAN ASSOCIATES INC",
            "Manhattan Associates"
        ));
        assert!(label_overlaps("Apple Inc.", "Apple"));
        // A search that lands on something unrelated shares no significant word.
        assert!(!label_overlaps(
            "Manhattan Associates Inc",
            "Oracle Corporation"
        ));
        // A shared word ("Manhattan") passes the overlap guard by design — it is a lenient backstop;
        // the P31 organization guard is what rejects a same-word-but-wrong-kind hit (a research
        // project, a borough) from actually contributing facts.
        assert!(label_overlaps(
            "Manhattan Associates Inc",
            "Manhattan Project"
        ));
        // Suffix-only overlap ("Inc"/"Company") must not count as a match.
        assert!(!label_overlaps("Delta Air Lines Inc", "Acme Company Inc"));
    }

    #[test]
    fn search_name_strips_corporate_suffixes() {
        // The exact failure that killed Wikidata coverage: the suffixed CSV name returns no hit.
        assert_eq!(
            search_name("MANHATTAN ASSOCIATES INC"),
            "MANHATTAN ASSOCIATES"
        );
        assert_eq!(search_name("Tesla, Inc."), "Tesla");
        assert_eq!(search_name("Apple Inc."), "Apple");
        assert_eq!(search_name("The Coca-Cola Company"), "Coca-Cola");
        // A name that is ALL suffix-like tokens must not collapse to empty.
        assert_eq!(search_name("The Company"), "The Company");
    }

    #[test]
    fn url_encoding_covers_spaces_and_punctuation() {
        assert_eq!(
            urlencoding_min("Manhattan Associates"),
            "Manhattan%20Associates"
        );
        assert_eq!(urlencoding_min("AT&T Inc."), "AT%26T%20Inc.");
    }

    /// Live end-to-end: the exact ticker that came back empty before the CIK fix now grounds across
    /// all three provenance-clean sources. Network + SEC/Wikidata availability; run explicitly:
    /// `cargo test -p agents grounded -- --ignored --nocapture live_composite`.
    #[test]
    #[ignore = "hits SEC EDGAR + Wikidata live"]
    fn live_composite_grounds_manhattan_associates() {
        let manh = Choice {
            ticker: "MANH".to_string(),
            name: "Manhattan Associates Inc".to_string(),
            birth: crate::BirthMoment {
                date: chrono::NaiveDate::from_ymd_opt(1998, 4, 24).unwrap(),
                time: None,
                tz: chrono_tz::America::New_York,
                lat: 0.0,
                lon: 0.0,
            },
            cik: Some(1_056_696),
            wiki: None,
        };
        let sig = CompositeSource::live_default().fetch(&manh);
        eprintln!("source: {}", sig.source);
        for item in &sig.items {
            eprintln!("  - {item}");
        }
        let real: Vec<&String> = sig.items.iter().filter(|i| !is_placeholder(i)).collect();
        assert!(
            !real.is_empty(),
            "MANH must ground with real signals now, got: {:?}",
            sig.items
        );
    }

    #[test]
    fn fixture_fallback_yields_items_for_every_demo_ticker() {
        for ticker in ["AAPL", "MSFT", "TSLA", "KO", "JNJ"] {
            let items =
                fixture_filings(ticker).unwrap_or_else(|| panic!("no fixture for {ticker}"));
            assert!(!items.is_empty(), "fixture for {ticker} is empty");
            assert!(
                items.iter().any(|i| i.starts_with("recent filing:")),
                "fixture for {ticker} has no filing line: {items:?}"
            );
            assert!(
                items.iter().any(|i| i.starts_with("industry:")),
                "fixture for {ticker} has no industry line: {items:?}"
            );
        }
        // Unknown tickers degrade honestly — no invented fixture.
        assert!(fixture_filings("ZZZZ").is_none());
    }

    /// The shipped contact must be a mailbox **the project owns**.
    ///
    /// This string rides every SEC request every installed copy ever makes, so the harm in
    /// `aisling.ld@pursuit.org` — the address that was hardcoded here and shipped — was never that
    /// it named a person. It was that it named a *third party*: the fellowship's domain, made the
    /// point of contact for strangers' traffic without Pursuit ever agreeing to it, and dead the
    /// moment that affiliation ends. A personal-looking local part on the project's own domain has
    /// neither problem, which is why this checks the domain rather than banning names.
    ///
    /// Forks and partner deployments identify as themselves via `ZIQPU_EDGAR_UA` (tested below);
    /// this guard only constrains what *we* ship as the default.
    #[test]
    fn the_contact_address_belongs_to_the_project() {
        let ua = DEFAULT_EDGAR_UA.to_ascii_lowercase();
        // SEC's policy is about reachability, so the format has to actually be a contact.
        assert!(
            ua.contains('@'),
            "SEC requires a contact in the User-Agent: {DEFAULT_EDGAR_UA}"
        );
        assert!(
            ua.contains("@nisabacapitalcharting.com"),
            "DEFAULT_EDGAR_UA must be a mailbox this project owns, not a third party's \
             (school, employer, a maintainer's private account): {DEFAULT_EDGAR_UA}"
        );
        // The specific leak, named so it cannot quietly return.
        assert!(
            !ua.contains("pursuit.org"),
            "DEFAULT_EDGAR_UA is back to the fellowship's domain: {DEFAULT_EDGAR_UA}"
        );
    }

    /// `ZIQPU_EDGAR_UA` overrides the default, and a blank value is treated as unset rather than
    /// sending SEC an empty contact — a policy violation dressed as a configuration.
    ///
    /// Serialized with the test below: both mutate the same process-wide env var, and Rust runs
    /// tests in parallel threads by default.
    #[test]
    fn env_override_wins_but_blank_does_not() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var("ZIQPU_EDGAR_UA", "Acme research (ops@acme.example)");
        assert_eq!(
            EdgarSource::default().user_agent,
            "Acme research (ops@acme.example)"
        );
        // Whitespace-only must fall back, not send an empty contact.
        std::env::set_var("ZIQPU_EDGAR_UA", "   ");
        assert_eq!(EdgarSource::default().user_agent, DEFAULT_EDGAR_UA);
        std::env::remove_var("ZIQPU_EDGAR_UA");
    }

    /// With nothing set, the shipped default is what goes out.
    #[test]
    fn unset_falls_back_to_the_role_address() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var("ZIQPU_EDGAR_UA");
        assert_eq!(EdgarSource::default().user_agent, DEFAULT_EDGAR_UA);
    }

    /// Guards the two env tests against each other — `set_var`/`remove_var` are process-wide, so
    /// without this they race and flake.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
}
