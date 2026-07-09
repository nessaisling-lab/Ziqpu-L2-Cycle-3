//! The grounded-signal tool — the loop's one action that reaches the outside world. It is the
//! reason the checkpoint exists (external, gated, costed). Default is a deterministic mock for
//! CI and offline demos; the live source pulls real SEC EDGAR filings by shelling out to `curl`
//! (so the crate adds no HTTP dependency and CI never touches the network).

use crate::types::{Choice, GroundedSignals};

/// A source of real-world signals about a choice.
pub trait GroundedSource {
    fn fetch(&self, choice: &Choice) -> GroundedSignals;
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
        let source = "SEC EDGAR".to_string();
        let Some(cik) = choice.cik else {
            return GroundedSignals {
                choice: choice.ticker.clone(),
                source,
                items: vec!["no CIK on file — cannot fetch filings".to_string()],
            };
        };
        let url = format!("https://data.sec.gov/submissions/CIK{cik:010}.json");
        let output = std::process::Command::new("curl")
            .args([
                "-sS",
                "-H",
                &format!("User-Agent: {}", self.user_agent),
                &url,
            ])
            .output();
        let items = match output {
            Ok(o) if o.status.success() => parse_recent_filings(&o.stdout),
            Ok(o) => vec![format!(
                "EDGAR fetch failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            )],
            Err(e) => vec![format!("could not run curl: {e}")],
        };
        GroundedSignals {
            choice: choice.ticker.clone(),
            source,
            items,
        }
    }
}

/// Pull the five most recent filings (form + date) from an EDGAR submissions JSON payload.
fn parse_recent_filings(bytes: &[u8]) -> Vec<String> {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(bytes) else {
        return vec!["could not parse EDGAR response".to_string()];
    };
    let recent = &value["filings"]["recent"];
    match (recent["form"].as_array(), recent["filingDate"].as_array()) {
        (Some(forms), Some(dates)) => forms
            .iter()
            .zip(dates)
            .take(5)
            .map(|(form, date)| {
                format!(
                    "{} filed {}",
                    form.as_str().unwrap_or("?"),
                    date.as_str().unwrap_or("?")
                )
            })
            .collect(),
        _ => vec!["no recent filings in EDGAR response".to_string()],
    }
}
