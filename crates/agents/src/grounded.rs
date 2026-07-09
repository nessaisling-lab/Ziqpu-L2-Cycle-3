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
        let mut items = Vec::new();
        // 1) SEC EDGAR recent filings + industry (may be blocked/rate-limited on some networks —
        //    degrade to a clean note rather than an error).
        if let Some(cik) = choice.cik {
            items.extend(self.fetch_filings(cik));
        }
        // 2) Wikipedia — a keyless "what this actually is" reality check that works everywhere.
        if let Some(title) = choice.wiki.as_deref() {
            if let Some(summary) = self.fetch_wikipedia(title) {
                items.push(format!("what it is: {summary}"));
            }
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
    fn fetch_filings(&self, cik: u32) -> Vec<String> {
        let url = format!("https://data.sec.gov/submissions/CIK{cik:010}.json");
        let Some(bytes) = self.get(&url) else {
            return vec!["SEC EDGAR unreachable".to_string()];
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            // A 403/HTML body (some networks block SEC) or throttle — say so, don't crash.
            return vec!["SEC EDGAR returned no data (blocked or rate-limited)".to_string()];
        };
        let recent = &value["filings"]["recent"];
        let mut out: Vec<String> =
            match (recent["form"].as_array(), recent["filingDate"].as_array()) {
                (Some(forms), Some(dates)) => forms
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
                    .collect(),
                _ => vec!["no recent filings in SEC response".to_string()],
            };
        if let Some(sic) = value["sicDescription"].as_str() {
            out.push(format!("industry: {sic}"));
        }
        out
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
