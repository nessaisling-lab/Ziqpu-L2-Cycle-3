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
const DEFAULT_EDGAR_UA: &str = "Ziqpu research (nisaba.ziqpu@gmail.com)";

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
        let output = crate::no_window(std::process::Command::new("curl"))
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

    /// No maintainer's personal or institutional address may ship in the User-Agent.
    ///
    /// This string rides every SEC request every installed copy ever makes, so a personal address
    /// here quietly attributes strangers' traffic to a named individual — and breaks when that
    /// person moves on. The addresses below are the ones actually reachable from this project's
    /// history, which is exactly why they're the ones worth naming: `aisling.ld@pursuit.org` was
    /// hardcoded here and shipped.
    #[test]
    fn the_contact_address_is_a_role_not_a_person() {
        for personal in ["pursuit.org", "aisling"] {
            assert!(
                !DEFAULT_EDGAR_UA.to_ascii_lowercase().contains(personal),
                "DEFAULT_EDGAR_UA ships a personal address ({personal}): {DEFAULT_EDGAR_UA}. \
                 Every user's SEC traffic carries this — use a role mailbox."
            );
        }
        // SEC's policy is about reachability, so the format has to actually be a contact.
        assert!(
            DEFAULT_EDGAR_UA.contains('@'),
            "SEC requires a contact in the User-Agent: {DEFAULT_EDGAR_UA}"
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
