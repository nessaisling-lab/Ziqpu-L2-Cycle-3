//! **Agentic, multi-source grounding** — the agent researches like a real research agent instead of
//! running a fixed fan of sources. The model, over the OpenAI-compatible tool loop ([`crate::tools`]),
//! *decides which real sources to query* for a given choice (and, when the deeper reach is approved,
//! crafts an open-web search). It runs on any tool-capable endpoint: a **local** `llama-server`
//! started with `--jinja`, or a hosted OpenAI-compatible model — so a seeker's own machine can research
//! offline. This is the "grow the agent" path that sits on top of the provenance-clean fetchers from
//! [`crate::grounded`].
//!
//! # The honesty spine is preserved exactly
//!
//! The whole product refuses to present anything unmeasured as measured. A model let loose to "research
//! the web" could *claim* a grounded fact it never fetched. This module makes that structurally
//! impossible: every tool records the **real signals its source returned** into a shared sink, and the
//! grounding is built **from that sink** — the model's own final prose is *discarded*. The model
//! orchestrates (which tools, what query); it never authors a grounded fact. So exactly as in the
//! deterministic path, only what a source actually fetched can carry the `GROUNDED` badge, and
//! [`has_real_signals`](crate::) still gates it.
//!
//! # Tiers
//!
//! - **Provenance-clean (always on):** SEC filings, SEC financials, Wikidata — the same keyless,
//!   public-domain/CC0 fetchers the deterministic composite uses.
//! - **Open web (only when `deep`, i.e. explicitly approved):** a keyless news search (GDELT). Higher
//!   blast radius and noisier, so it is gated behind the human's deeper-reach approval and every hit is
//!   tagged with its real source domain + date. A keyed search API (Brave/Serp/Bing) is a clean future
//!   swap behind the same tool interface.

use std::sync::{Arc, Mutex};

use serde_json::{json, Value};

use crate::grounded::{
    is_placeholder, sec_user_agent, urlencoding_min, EdgarSource, GroundedSource, SecFactsSource,
    WikidataSource, NO_SIGNALS,
};
use crate::tools::{run_tool_loop, Tool, DEFAULT_MAX_STEPS};
use crate::types::{Choice, GroundedSignals};

/// Where every tool deposits the **real** (non-placeholder) signals it fetched, as
/// `(source_label, items)`. The grounding is assembled from this — never from the model's prose.
type Sink = Arc<Mutex<Vec<(String, Vec<String>)>>>;

/// Which OpenAI-compatible endpoint the research loop drives, and how hard it may work.
pub struct ResearchConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub max_steps: usize,
}

impl ResearchConfig {
    /// Point at the local `llama-server` (served with `--jinja`) by default, honoring the same
    /// `ZIQPU_LLM_URL` / `ZIQPU_LOCAL_MODEL` env the local interpreter uses — so "let Local research"
    /// needs no extra wiring. A hosted OpenAI-compatible endpoint (e.g. OpenRouter) can be passed by
    /// constructing this directly.
    pub fn local_from_env() -> Self {
        Self {
            base_url: std::env::var("ZIQPU_LLM_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:1234/v1".to_string()),
            api_key: std::env::var("ZIQPU_LLM_KEY").unwrap_or_default(),
            model: std::env::var("ZIQPU_LOCAL_MODEL").unwrap_or_else(|_| "local-model".to_string()),
            max_steps: DEFAULT_MAX_STEPS,
        }
    }
}

/// Wraps a [`GroundedSource`] as a model-callable [`Tool`] bound to one choice. On call it fetches,
/// records the source's **real** items into the shared sink, and hands the model a readable summary
/// (so the model can decide what else to pull) — but the summary is *not* what becomes the grounding;
/// the sink is.
struct GroundedTool {
    name: &'static str,
    description: &'static str,
    source: Box<dyn GroundedSource + Send + Sync>,
    choice: Choice,
    sink: Sink,
}

impl Tool for GroundedTool {
    fn name(&self) -> &str {
        self.name
    }

    fn spec(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": { "type": "object", "properties": {} }
            }
        })
    }

    fn call(&self, _args: &Value) -> String {
        let sig = self.source.fetch(&self.choice);
        let real: Vec<String> = sig
            .items
            .into_iter()
            .filter(|i| !is_placeholder(i))
            .collect();
        let summary = if real.is_empty() {
            format!("{}: no public data found for this entity.", self.name)
        } else {
            format!("{} returned:\n- {}", sig.source, real.join("\n- "))
        };
        // Record the REAL signals (may be empty). This — not the summary, and never the model's
        // prose — is what the grounding is built from.
        self.sink
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((sig.source, real));
        summary
    }
}

/// The open-web reach — a keyless news search over GDELT. Gated behind the deeper-reach approval
/// because it is broader and noisier than the structured sources. The model supplies the `query`
/// (crafting the search is exactly the judgment an agent adds). Every hit is a **real** article
/// tagged with its source domain + date; a throttle/non-JSON body degrades to "no web signals"
/// rather than inventing one.
struct WebNewsTool {
    default_query: String,
    user_agent: String,
    sink: Sink,
}

impl Tool for WebNewsTool {
    fn name(&self) -> &str {
        "web_news_search"
    }

    fn spec(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "web_news_search",
                "description": "Search recent news across the open web for what is being reported about \
                    this entity right now. Use a focused query (the company's full name). Returns real \
                    dated articles with their source; may return nothing if throttled.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "The search query, e.g. the company's full name." }
                    }
                }
            }
        })
    }

    fn call(&self, args: &Value) -> String {
        let query = args
            .get("query")
            .and_then(|q| q.as_str())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&self.default_query);
        let items = fetch_news(query, &self.user_agent);
        let summary = if items.is_empty() {
            "web_news_search: no news signals available right now.".to_string()
        } else {
            format!("Web news returned:\n- {}", items.join("\n- "))
        };
        self.sink
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(("Web news (GDELT)".to_string(), items));
        summary
    }
}

/// Up to 4 recent articles for `query` from GDELT's keyless DOC API, each as
/// `"<title> — <domain>, <YYYY-MM-DD>"`. `Vec::new()` on any throttle/transport/parse failure — GDELT
/// rate-limits hard and answers a throttle with a *plaintext* notice, so a failed JSON parse must
/// degrade to "no signals", never an invented one.
fn fetch_news(query: &str, user_agent: &str) -> Vec<String> {
    let url = format!(
        "https://api.gdeltproject.org/api/v2/doc/doc?query={}&mode=artlist&maxrecords=4&format=json&sort=datedesc",
        urlencoding_min(&format!("\"{query}\""))
    );
    let Some(bytes) = crate::grounded::http_get(&url, user_agent) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
        return Vec::new();
    };
    let Some(articles) = value["articles"].as_array() else {
        return Vec::new();
    };
    articles
        .iter()
        .filter_map(|a| {
            let title = a["title"].as_str()?.trim();
            if title.is_empty() {
                return None;
            }
            let domain = a["domain"].as_str().unwrap_or("");
            let date = a["seendate"]
                .as_str()
                .and_then(|d| d.get(0..8))
                .map(|d| format!("{}-{}-{}", &d[0..4], &d[4..6], &d[6..8]))
                .unwrap_or_default();
            Some(format!("news: {title} — {domain}, {date}"))
        })
        .collect()
}

/// Merge everything the tools deposited into one honest [`GroundedSignals`], the same way the
/// deterministic composite does: drop sources that contributed nothing, name only the contributors,
/// and mark the whole set unsourced ([`NO_SIGNALS`]) only when *every* source came up empty.
fn merge_sink(ticker: &str, sink: Sink) -> GroundedSignals {
    let collected = sink.lock().unwrap_or_else(|e| e.into_inner());
    let mut items: Vec<String> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    for (label, real) in collected.iter() {
        if !real.is_empty() {
            labels.push(label.clone());
            items.extend(real.iter().cloned());
        }
    }
    if items.is_empty() {
        return GroundedSignals {
            choice: ticker.to_string(),
            source: "(no public signals)".to_string(),
            items: vec![NO_SIGNALS.to_string()],
        };
    }
    labels.dedup();
    GroundedSignals {
        choice: ticker.to_string(),
        source: labels.join(" · "),
        items,
    }
}

/// The system prompt for the research loop. Names the job, forbids invention, and keeps the loop
/// short (call each relevant tool once, then stop).
const RESEARCH_SYSTEM: &str = "You are a research assistant gathering REAL public signals about an \
    entity for a grounded briefing. Call the tools that fit this entity to collect facts — filings, \
    financials, structured facts, and (if available) recent news. Do NOT invent, estimate, or infer \
    any data; only report what a tool returns. Call each relevant tool at most once. When you have \
    gathered the available signals, reply with the single word DONE.";

/// Research a choice's grounding by letting the model drive the tool loop, then build the grounding
/// from the **real signals the tools returned** (never the model's prose). `deep` adds the gated
/// open-web (news) tool. Falls back to nothing-collected → [`NO_SIGNALS`]; a caller that wants a
/// safety net can fall back to the deterministic [`CompositeSource`](crate::CompositeSource).
pub fn research_grounded(choice: &Choice, cfg: &ResearchConfig, deep: bool) -> GroundedSignals {
    let sink: Sink = Arc::new(Mutex::new(Vec::new()));
    let ua = sec_user_agent();

    let mut tools: Vec<Box<dyn Tool>> = vec![
        Box::new(GroundedTool {
            name: "sec_filings",
            description: "Recent SEC EDGAR filings and the company's SIC industry. For US public filers.",
            source: Box::new(EdgarSource::default()),
            choice: choice.clone(),
            sink: sink.clone(),
        }),
        Box::new(GroundedTool {
            name: "sec_financials",
            description: "Latest reported revenue and total assets from SEC XBRL company facts. For US public filers.",
            source: Box::new(SecFactsSource::default()),
            choice: choice.clone(),
            sink: sink.clone(),
        }),
        Box::new(GroundedTool {
            name: "company_facts",
            description: "Structured CC0 facts from Wikidata: founding year and employee count.",
            source: Box::new(WikidataSource::default()),
            choice: choice.clone(),
            sink: sink.clone(),
        }),
    ];
    if deep {
        tools.push(Box::new(WebNewsTool {
            default_query: choice.name.clone(),
            user_agent: ua,
            sink: sink.clone(),
        }));
    }

    let refs: Vec<&dyn Tool> = tools.iter().map(|t| t.as_ref()).collect();
    let user = format!(
        "Gather grounding signals for {} (ticker {}).",
        choice.name, choice.ticker
    );
    // The return value (the model's prose) is deliberately ignored — the grounding is the sink.
    let _ = run_tool_loop(
        &cfg.base_url,
        &cfg.api_key,
        &cfg.model,
        RESEARCH_SYSTEM,
        &user,
        &refs,
        cfg.max_steps,
    );

    merge_sink(&choice.ticker, sink)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BirthMoment;

    fn demo_choice() -> Choice {
        Choice {
            ticker: "TEST".to_string(),
            name: "Test Co".to_string(),
            birth: BirthMoment {
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

    /// The honesty invariant, unit-tested without a network: a tool records real signals into the
    /// sink; `merge_sink` builds the grounding from the sink; a placeholder-only source is dropped;
    /// and the model's prose plays no part.
    #[test]
    fn merge_builds_grounding_from_the_sink_not_from_prose() {
        let sink: Sink = Arc::new(Mutex::new(vec![
            (
                "SEC financials (XBRL)".to_string(),
                vec!["revenue: $1.0B (over 2025-01-01 → 2025-12-31, 10-K)".to_string()],
            ),
            // A source that found nothing must not be named.
            ("Wikidata".to_string(), vec![]),
            (
                "Web news (GDELT)".to_string(),
                vec!["news: Test Co expands — example.com, 2026-07-18".to_string()],
            ),
        ]));
        let sig = merge_sink("TEST", sink);
        assert_eq!(
            sig.items.len(),
            2,
            "both real items survive: {:?}",
            sig.items
        );
        assert_eq!(
            sig.source, "SEC financials (XBRL) · Web news (GDELT)",
            "the empty Wikidata source is not named"
        );
    }

    /// An all-empty sink (the loop failed / no model served) degrades to the honest unsourced marker,
    /// which [`has_real_signals`] will treat as unsourced — never a fabricated GROUNDED badge.
    #[test]
    fn an_empty_sink_degrades_to_the_unsourced_marker() {
        let sink: Sink = Arc::new(Mutex::new(vec![("Wikidata".to_string(), vec![])]));
        let sig = merge_sink("TEST", sink);
        assert_eq!(sig.items, vec![NO_SIGNALS.to_string()]);
        assert!(is_placeholder(&sig.items[0]));
    }

    /// A `GroundedTool` deposits its source's real signals into the sink (stripping placeholders) and
    /// hands the model a readable summary — the mechanism the loop relies on. Uses a mock source, no
    /// network.
    #[test]
    fn a_grounded_tool_records_real_signals_and_summarizes() {
        struct FakeSource;
        impl GroundedSource for FakeSource {
            fn fetch(&self, choice: &Choice) -> GroundedSignals {
                GroundedSignals {
                    choice: choice.ticker.clone(),
                    source: "SEC financials (XBRL)".to_string(),
                    items: vec![
                        "revenue: $1.0B (over 2025-01-01 → 2025-12-31, 10-K)".to_string(),
                        NO_SIGNALS.to_string(), // a placeholder that must be stripped
                    ],
                }
            }
        }
        let sink: Sink = Arc::new(Mutex::new(Vec::new()));
        let tool = GroundedTool {
            name: "sec_financials",
            description: "d",
            source: Box::new(FakeSource),
            choice: demo_choice(),
            sink: sink.clone(),
        };
        let summary = tool.call(&json!({}));
        assert!(summary.contains("revenue: $1.0B"), "summary: {summary}");
        assert!(
            !summary.contains(NO_SIGNALS),
            "placeholder must not reach the model"
        );
        let recorded = sink.lock().unwrap();
        assert_eq!(recorded.len(), 1);
        assert_eq!(
            recorded[0].1,
            vec!["revenue: $1.0B (over 2025-01-01 → 2025-12-31, 10-K)".to_string()]
        );
    }

    /// LIVE end-to-end: a REAL served `llama-server --jinja` decides which source tools to call and
    /// grounds a real ticker; the grounding is the tools' real returns. Ignored (needs a GPU/served
    /// model). Serve first, then:
    /// `cargo test -p agents research -- --ignored --nocapture live_research`
    #[test]
    #[ignore = "needs a served tool-capable llama-server (--jinja)"]
    fn live_research_grounds_a_real_ticker() {
        let manh = Choice {
            ticker: "MANH".to_string(),
            name: "Manhattan Associates Inc".to_string(),
            birth: BirthMoment {
                date: chrono::NaiveDate::from_ymd_opt(1998, 4, 24).unwrap(),
                time: None,
                tz: chrono_tz::America::New_York,
                lat: 0.0,
                lon: 0.0,
            },
            cik: Some(1_056_696),
            wiki: None,
        };
        let cfg = ResearchConfig::local_from_env();
        let sig = research_grounded(&manh, &cfg, false);
        eprintln!("source: {}", sig.source);
        for item in &sig.items {
            eprintln!("  - {item}");
        }
        assert!(
            sig.items.iter().any(|i| !is_placeholder(i)),
            "the model should have gathered real signals, got: {:?}",
            sig.items
        );
    }
}
