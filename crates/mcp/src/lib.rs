//! Ziqpu MCP server — drives the two-vizier loop over the Model Context Protocol so the agent
//! runs *outside* the Ziqpu app and travels with the user (PRD §16/§20 portability, Phase 1b).
//!
//! MCP's stdio transport is newline-delimited JSON-RPC 2.0, so this is a tiny hand-rolled handler
//! over `serde_json` — no extra dependency and no pre-1.0 SDK. [`handle`] is pure (request in,
//! response out), which keeps it unit-testable without any stdio plumbing.
//!
//! Tools exposed to the host:
//! - `make_profile` — build a portable birth profile (the thing the agent travels with).
//! - `chart` — a real natal chart for a seeded choice (real ephemeris).
//! - `recommend` — OBSERVE + DECIDE: ranked fit reads for a profile, then proposes grounding.
//! - `pull_grounded_signals` — the checkpoint: without `approved:true` it returns `PENDING_APPROVAL`
//!   and touches nothing; the host must confirm before the gated, costed SEC EDGAR pull runs.

use serde_json::{json, Value};

/// Handle one JSON-RPC message. Returns `Some(response)` for requests and `None` for notifications.
pub fn handle(req: &Value) -> Option<Value> {
    let id = req.get("id").cloned();
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    match method {
        "initialize" => Some(result(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "ziqpu-mcp", "version": env!("CARGO_PKG_VERSION") },
            }),
        )),
        // Notifications carry no id and expect no response.
        m if m.starts_with("notifications/") => None,
        "ping" => Some(result(id, json!({}))),
        "tools/list" => Some(result(id, json!({ "tools": tools() }))),
        "tools/call" => Some(handle_call(id, req.get("params"))),
        _ if id.is_some() => Some(error(id, -32601, "method not found")),
        _ => None,
    }
}

fn result(id: Option<Value>, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "result": result })
}

fn error(id: Option<Value>, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "error": { "code": code, "message": message } })
}

/// The tool catalogue (name · description · JSON-Schema of arguments).
fn tools() -> Value {
    json!([
        {
            "name": "make_profile",
            "description": "Build a portable Ziqpu birth profile (birth data only) the agent can travel with. Returns a small JSON string to pass to `recommend`.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "date": { "type": "string", "description": "Birth date, YYYY-MM-DD" },
                    "time": { "type": "string", "description": "Birth time, HH:MM (omit if unknown)" },
                    "tz": { "type": "string", "description": "IANA timezone, e.g. America/New_York" },
                    "lat": { "type": "number" },
                    "lon": { "type": "number" }
                },
                "required": ["date", "tz", "lat", "lon"]
            }
        },
        {
            "name": "chart",
            "description": "The natal chart of a seeded choice (real ephemeris). Tickers: AAPL, MSFT, TSLA, KO, JNJ.",
            "inputSchema": {
                "type": "object",
                "properties": { "ticker": { "type": "string" } },
                "required": ["ticker"]
            }
        },
        {
            "name": "recommend",
            "description": "Observe + decide: ranked synastry fit reads for a profile against the seeded choices, then proposes grounding. Never advice.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile": { "type": "string", "description": "A profile string from make_profile" },
                    "tickers": { "type": "array", "items": { "type": "string" }, "description": "Optional subset; defaults to all seeded choices" }
                },
                "required": ["profile"]
            }
        },
        {
            "name": "pull_grounded_signals",
            "description": "The human-in-the-loop checkpoint. Without approved=true it returns PENDING_APPROVAL and fetches nothing; with approved=true it makes the gated, costed external pull (SEC EDGAR filings and Wikipedia).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "ticker": { "type": "string" },
                    "approved": { "type": "boolean", "description": "Must be true to run the external pull" }
                },
                "required": ["ticker"]
            }
        }
    ])
}

fn handle_call(id: Option<Value>, params: Option<&Value>) -> Value {
    let params = params.cloned().unwrap_or(Value::Null);
    let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let (text, is_error) = match name {
        "make_profile" => call_make_profile(&args),
        "chart" => call_chart(&args),
        "recommend" => call_recommend(&args),
        "pull_grounded_signals" => call_pull(&args),
        other => (format!("unknown tool: {other}"), true),
    };
    result(
        id,
        json!({ "content": [{ "type": "text", "text": text }], "isError": is_error }),
    )
}

// --- tool implementations (each returns (text, is_error)) -------------------

fn call_make_profile(args: &Value) -> (String, bool) {
    let (Some(date), Some(tz), Some(lat), Some(lon)) = (
        args.get("date").and_then(|d| d.as_str()),
        args.get("tz").and_then(|t| t.as_str()),
        args.get("lat").and_then(|x| x.as_f64()),
        args.get("lon").and_then(|x| x.as_f64()),
    ) else {
        return (
            "make_profile needs date (YYYY-MM-DD), tz (IANA), lat, lon".to_string(),
            true,
        );
    };
    let time = args.get("time").and_then(|t| t.as_str());
    match agents::make_profile(date, time, tz, lat, lon) {
        Ok(profile) => (profile, false),
        Err(e) => (format!("could not build profile: {e}"), true),
    }
}

fn call_chart(args: &Value) -> (String, bool) {
    let Some(ticker) = args.get("ticker").and_then(|t| t.as_str()) else {
        return ("chart needs a ticker".to_string(), true);
    };
    let ticker = ticker.to_uppercase();
    let Some(choice) = agents::demo_choices()
        .into_iter()
        .find(|c| c.ticker == ticker)
    else {
        return (
            format!("unknown ticker {ticker}; try AAPL, MSFT, TSLA, KO, or JNJ"),
            true,
        );
    };
    let lines = agents::chart_summary(&choice.birth);
    (
        format!(
            "{} ({}) — natal chart:\n{}",
            choice.name,
            choice.ticker,
            lines.join("\n")
        ),
        false,
    )
}

fn call_recommend(args: &Value) -> (String, bool) {
    let Some(profile) = args.get("profile").and_then(|p| p.as_str()) else {
        return (
            "recommend needs a profile string (from make_profile)".to_string(),
            true,
        );
    };
    let seeker = match agents::import_profile(profile) {
        Ok(s) => s,
        Err(e) => return (format!("bad profile: {e}"), true),
    };
    let choices = match args.get("tickers").and_then(|t| t.as_array()) {
        Some(arr) => {
            let want: Vec<String> = arr
                .iter()
                .filter_map(|t| t.as_str())
                .map(|t| t.to_uppercase())
                .collect();
            let filtered: Vec<_> = agents::demo_choices()
                .into_iter()
                .filter(|c| want.contains(&c.ticker))
                .collect();
            if filtered.is_empty() {
                return ("no known tickers in that list".to_string(), true);
            }
            filtered
        }
        None => agents::demo_choices(),
    };

    let mut session = agents::Session::new(
        agents::EngineChartSource::default(),
        grounded(),
        agents::build_interpreter(),
    );
    let recs = session.recommend(&seeker, &choices);

    let mut out = String::from("Ranked fit (measured, not fate):\n");
    for r in &recs {
        out.push_str(&format!(
            "  {:<18} {:<16} {:>3}/100\n",
            r.name,
            r.fit.label(),
            r.score
        ));
    }
    out.push_str(
        "\nCHECKPOINT — to ground a pick against real data, call \
         pull_grounded_signals { ticker, approved: true }.\n\nTop read:\n",
    );
    if let Some(top) = recs.first() {
        out.push_str(&top.reading);
    }
    (out, false)
}

fn call_pull(args: &Value) -> (String, bool) {
    let Some(ticker) = args.get("ticker").and_then(|t| t.as_str()) else {
        return ("pull_grounded_signals needs a ticker".to_string(), true);
    };
    let ticker = ticker.to_uppercase();
    let Some(choice) = agents::demo_choices()
        .into_iter()
        .find(|c| c.ticker == ticker)
    else {
        return (format!("unknown ticker {ticker}"), true);
    };
    let approved = args
        .get("approved")
        .and_then(|a| a.as_bool())
        .unwrap_or(false);
    if !approved {
        // The checkpoint, surfaced to the host: nothing external ran.
        return (
            // Names every source the approval would spend — this text *is* the consent an MCP host
            // shows before authorizing the pull, so it can't understate what runs.
            format!(
                "PENDING_APPROVAL — grounding {ticker} makes gated, costed external calls \
                 (SEC EDGAR filings and Wikipedia). Nothing was fetched. Re-call with \
                 {{ \"approved\": true }} to proceed, or keep the symbolic read."
            ),
            false,
        );
    }
    use agents::GroundedSource;
    let signals = grounded().fetch(&choice);
    let mut out = format!("GROUNDED ({}) for {}:\n", signals.source, ticker);
    for item in &signals.items {
        out.push_str(&format!("  - {item}\n"));
    }
    out.push_str("REMINDER: measured, not fate — not financial advice.");
    (out, false)
}

/// The grounded source: real SEC EDGAR when `ZIQPU_LIVE` is set, else the deterministic mock.
fn grounded() -> Box<dyn agents::GroundedSource> {
    if std::env::var("ZIQPU_LIVE").is_ok() {
        Box::new(agents::EdgarSource::default())
    } else {
        Box::new(agents::MockGroundedSource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_tool(name: &str, arguments: Value) -> String {
        let req = json!({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": { "name": name, "arguments": arguments }
        });
        let resp = handle(&req).unwrap();
        resp["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn initialize_advertises_the_server() {
        let resp = handle(&json!({ "jsonrpc": "2.0", "id": 0, "method": "initialize" })).unwrap();
        assert_eq!(resp["result"]["serverInfo"]["name"], "ziqpu-mcp");
        assert!(resp["result"]["capabilities"]["tools"].is_object());
    }

    #[test]
    fn notifications_get_no_response() {
        assert!(
            handle(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).is_none()
        );
    }

    #[test]
    fn tools_list_has_the_four_tools() {
        let resp = handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" })).unwrap();
        let names: Vec<&str> = resp["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "make_profile",
                "chart",
                "recommend",
                "pull_grounded_signals"
            ]
        );
    }

    #[test]
    fn full_flow_profile_recommend_checkpoint() {
        // 1. make a portable profile
        let profile = call_tool(
            "make_profile",
            json!({ "date": "1990-05-15", "time": "14:30", "tz": "America/New_York", "lat": 40.71, "lon": -74.0 }),
        );
        assert!(profile.contains("ziqpu.profile"));

        // 2. recommend over it → ranked fit + the checkpoint proposal
        let recs = call_tool("recommend", json!({ "profile": profile }));
        assert!(recs.contains("Ranked fit"));
        assert!(recs.contains("CHECKPOINT"));

        // 3. the checkpoint: no approval → nothing fetched
        let pending = call_tool("pull_grounded_signals", json!({ "ticker": "AAPL" }));
        assert!(pending.contains("PENDING_APPROVAL"));

        // 4. approved → the (mock, in CI) grounded pull runs
        let grounded = call_tool(
            "pull_grounded_signals",
            json!({ "ticker": "AAPL", "approved": true }),
        );
        assert!(grounded.contains("GROUNDED"));
        assert!(grounded.contains("not financial advice"));
    }

    #[test]
    fn chart_returns_bodies_for_a_known_ticker() {
        let chart = call_tool("chart", json!({ "ticker": "aapl" }));
        assert!(chart.contains("Apple"));
        assert!(chart.contains("Sun"));
    }

    #[test]
    fn unknown_method_is_a_json_rpc_error() {
        let resp = handle(&json!({ "jsonrpc": "2.0", "id": 9, "method": "nope" })).unwrap();
        assert_eq!(resp["error"]["code"], -32601);
    }
}
