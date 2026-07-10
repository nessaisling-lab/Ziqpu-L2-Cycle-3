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

/// Real, keyless SEC EDGAR submissions pull. Live only. SEC policy requires a contact
/// User-Agent. Shells out to `curl` so no HTTP crate enters the dependency tree.
pub struct EdgarSource {
    pub user_agent: String,
}

impl Default for EdgarSource {
    fn default() -> Self {
        Self {
            user_agent: "Ziqpu research (aisling.ld@pursuit.org)".to_string(),
        }
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
                items.push("no public signals available".to_string());
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
            items.push("no public signals available".to_string());
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
    /// transport error or a non-2xx status (`curl` returns exit 0 on HTTP errors, so we don't rely
    /// on that alone — callers treat a non-JSON body as "unavailable").
    fn get(&self, url: &str) -> Option<Vec<u8>> {
        let output = std::process::Command::new("curl")
            .args([
                "-sS",
                "--compressed",
                "-H",
                &format!("User-Agent: {}", self.user_agent),
                url,
            ])
            .output()
            .ok()?;
        output.status.success().then_some(output.stdout)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
