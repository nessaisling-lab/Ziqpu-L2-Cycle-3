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
        "initialize" => {
            note_client_capabilities(req.get("params"));
            Some(result(
                id,
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "ziqpu-mcp", "version": env!("CARGO_PKG_VERSION") },
                }),
            ))
        }
        // Notifications carry no id and expect no response.
        m if m.starts_with("notifications/") => None,
        "ping" => Some(result(id, json!({}))),
        "tools/list" => Some(result(id, json!({ "tools": tools() }))),
        "tools/call" => Some(handle_call(id, req.get("params"))),
        _ if id.is_some() => Some(error(id, -32601, "method not found")),
        _ => None,
    }
}

/// Whether the connected host declared support for **elicitation** — the MCP request that lets a
/// server ask the *host* to put a question to the human and return their answer.
///
/// It is recorded at `initialize` and read later, which is why this is a process-global rather than
/// a parameter: `handle` is deliberately pure (request in, response out) so it stays unit-testable
/// without stdio plumbing, and one server process serves exactly one session.
///
/// Nothing uses it yet, and that is deliberate rather than forgotten. Elicitation is a
/// **server-initiated request**: the server writes a request and waits for the host's response,
/// interleaved with the host's own traffic. This transport is strictly request-in/response-out, so
/// using it means rebuilding the loop and giving up the purity that makes `handle` testable. The
/// detection lands now because it is the cheap prerequisite and because it makes the gap
/// measurable — see `ELICITATION_NOTE`.
static HOST_SUPPORTS_ELICITATION: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// Record what the host said it can do, from the `initialize` params.
fn note_client_capabilities(params: Option<&Value>) {
    let supports = params
        .and_then(|p| p.get("capabilities"))
        .and_then(|c| c.get("elicitation"))
        .is_some();
    HOST_SUPPORTS_ELICITATION.store(supports, std::sync::atomic::Ordering::Relaxed);
}

/// Whether the host can be asked to prompt the human directly.
pub fn host_supports_elicitation() -> bool {
    HOST_SUPPORTS_ELICITATION.load(std::sync::atomic::Ordering::Relaxed)
}

/// The one sentence this server may honestly say about approval on the MCP surface.
///
/// **The server cannot verify that a human approved anything.** The `acknowledged` argument is set
/// by the calling model, which also reads this response — so a token minted here and required back
/// would be the model shaking hands with itself, not a gate. Returning `PENDING_APPROVAL` is still
/// worth doing, because it forces a second round trip and puts the consent text where the host's own
/// approval dialog will show it to a person. That dialog is the real gate on this surface, and
/// saying so plainly is the difference between a limitation and a lie.
const ELICITATION_NOTE: &str =
    "Note: this server cannot verify that a human approved this. Your MCP host's own approval      prompt is the gate; `acknowledged` records that the text above was surfaced, not that anyone      agreed to it.";

/// Read the human-acknowledgement flag, accepting the older `approved` spelling.
///
/// Renamed because `approved` claimed something untrue: nothing here can establish that a human
/// approved. The old name is still accepted so existing host configurations keep working, and is
/// documented as deprecated rather than silently honoured.
fn acknowledged(args: &Value) -> bool {
    ["acknowledged", "approved"]
        .iter()
        .any(|k| args.get(*k).and_then(|a| a.as_bool()).unwrap_or(false))
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
            "annotations": { "title": "Build a birth profile", "readOnlyHint": true, "openWorldHint": false, "idempotentHint": true },
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
            "annotations": { "title": "Natal chart of a seeded choice", "readOnlyHint": true, "openWorldHint": false, "idempotentHint": true },
            "description": "The natal chart of a seeded choice (real ephemeris). Tickers: AAPL, MSFT, TSLA, KO, JNJ.",
            "inputSchema": {
                "type": "object",
                "properties": { "ticker": { "type": "string" } },
                "required": ["ticker"]
            }
        },
        {
            "name": "recommend",
            "annotations": { "title": "Ranked fit reads — SPENDS a live model call per choice", "readOnlyHint": false, "openWorldHint": true, "idempotentHint": false },
            "description": "Observe + decide: ranked synastry fit reads for a profile against the seeded choices, then proposes grounding. Never advice. COSTS MONEY: calls the configured hosted model ONCE PER CHOICE, billed to the user's own API key. Without acknowledged=true it returns PENDING_APPROVAL naming the number of calls and spends nothing. This server cannot verify a human approved — the host's approval prompt is the gate.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile": { "type": "string", "description": "A profile string from make_profile" },
                    "tickers": { "type": "array", "items": { "type": "string" }, "description": "Optional subset; defaults to all seeded choices" },
                    "acknowledged": { "type": "boolean", "description": "Set only after the PENDING_APPROVAL text — which names how many billed model calls this runs — has been shown to a human. The server cannot verify this." }
                },
                "required": ["profile"]
            }
        },
        {
            "name": "pull_grounded_signals",
            "description": "The human-in-the-loop checkpoint. Without approved=true it returns PENDING_APPROVAL — naming the exact sources that call would spend — and fetches nothing; with acknowledged=true it makes the gated external pull. This server CANNOT verify that a human approved — the host's own approval prompt is the gate, and `acknowledged` only records that the consent text was surfaced. Which sources run depends on what the entity is: SEC EDGAR filings, SEC XBRL financials, Wikidata and Wikipedia for a public filer; NHTSA vPIC for a vehicle; Wikidata, Wikipedia and the FDA drug register for a bare name.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "ticker": { "type": "string" },
                    "acknowledged": { "type": "boolean", "description": "Set only after the PENDING_APPROVAL text has been shown to a human. The server cannot verify this; `approved` is accepted as a deprecated alias." }
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

    // The checkpoint this tool never had. `recommend` calls the live hosted model ONCE PER CHOICE,
    // on the user's own API key — so on this surface it is the expensive tool, not
    // `pull_grounded_signals`, whose sources are all keyless public endpoints. The gate was on the
    // one that spends quota and absent from the one that spends money.
    if !acknowledged(args) {
        return (
            format!(
                "PENDING_APPROVAL — ranking {} choice(s) runs {} live model call(s) via {}, billed                  to the configured key. Nothing has been spent. Re-call with                  {{ \"acknowledged\": true }} to proceed, or call `chart` for the free,                  deterministic reads.

{ELICITATION_NOTE}",
                choices.len(),
                choices.len(),
                agents::active_source_label(),
            ),
            false,
        );
    }

    let mut session = agents::Session::new(
        agents::EngineChartSource::default(),
        grounded(&agents::demo_choices()[0]),
        interpreter(),
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
         pull_grounded_signals { ticker, acknowledged: true }.\n\nTop read:\n",
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
    if !acknowledged(args) {
        // The checkpoint, surfaced to the host: nothing external ran. The sentence comes from
        // `agents::grounding_consent` — the same function the desktop checkpoint uses — because a
        // second hand-written copy here is exactly how this text fell behind the roster last time.
        return (
            format!(
                "PENDING_APPROVAL — {} Nothing was fetched. Re-call with \
                 {{ \"acknowledged\": true }} to proceed, or keep the symbolic read.\n\n{}",
                agents::grounding_consent(&choice),
                ELICITATION_NOTE
            ),
            false,
        );
    }
    use agents::GroundedSource;
    let signals = grounded(&choice).fetch(&choice);
    let mut out = format!("GROUNDED ({}) for {}:\n", signals.source, ticker);
    for item in &signals.items {
        out.push_str(&format!("  - {item}\n"));
    }
    out.push_str("REMINDER: measured, not fate — not financial advice.");
    (out, false)
}

/// The grounded source: the real **multi-source** pull when `ZIQPU_LIVE` is set, else the
/// deterministic mock.
///
/// This used to be `EdgarSource` alone, which quietly made the MCP server a second-class surface:
/// a host driving Ziqpu got one source while the desktop app got five, and the difference was
/// invisible from the outside. Same [`CompositeSource`](agents::CompositeSource) now, so a grounded
/// briefing is the same briefing wherever the loop is driven from.
/// The sources for one choice — **live only when explicitly asked**, and picked by entity kind.
///
/// `for_entity` rather than `live_default`, so the roster matches what `grounding_consent` told the
/// human this call would spend. The default roster is SEC-shaped and would ask a vehicle or a
/// medicine questions only a public filer can answer.
fn grounded(choice: &agents::Choice) -> Box<dyn agents::GroundedSource> {
    if std::env::var("ZIQPU_LIVE").is_ok() {
        Box::new(agents::CompositeSource::for_entity(choice))
    } else {
        Box::new(agents::MockGroundedSource)
    }
}

/// The interpreter this server writes readings with — **live only when explicitly asked**.
///
/// The same opt-in discipline [`grounded`] already had, and the reason it needed it here too was
/// embarrassing: `build_interpreter` goes live whenever a key is present, so `cargo test -p mcp` was
/// making real billed model calls on every run, on the developer's own key, and had been for as long
/// as the flow test existed. Nobody noticed because it merely looked slow. A test suite that spends
/// money is a bug regardless of how small the amount is, and a default that reaches the network
/// unless told otherwise is the wrong default for a library the tests drive.
///
/// `ZIQPU_LIVE` gates both, so one variable means "this run may cost something".
fn interpreter() -> Box<dyn agents::Interpreter> {
    if std::env::var("ZIQPU_LIVE").is_ok() {
        agents::build_interpreter()
    } else {
        Box::new(agents::TemplateInterpreter)
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

        // 2. recommend WITHOUT acknowledgement → its own checkpoint, and nothing is spent.
        //    This tool calls the hosted model once per choice on the user's key, so on the MCP
        //    surface it is the expensive one — the gate used to be on `pull_grounded_signals`,
        //    whose sources are all keyless, and absent here.
        let pending_recs = call_tool("recommend", json!({ "profile": profile }));
        assert!(pending_recs.contains("PENDING_APPROVAL"), "{pending_recs}");
        assert!(
            pending_recs.contains("live model call"),
            "the pause must name the spend: {pending_recs}"
        );
        assert!(
            !pending_recs.contains("Ranked fit"),
            "nothing may be computed before the pause is answered: {pending_recs}"
        );
        assert!(
            pending_recs.contains("cannot verify"),
            "and it must not imply the server checked with a human: {pending_recs}"
        );

        // 3. acknowledged → the ranking runs
        let recs = call_tool(
            "recommend",
            json!({ "profile": profile, "acknowledged": true }),
        );
        assert!(recs.contains("Ranked fit"));
        assert!(recs.contains("CHECKPOINT"));

        // 4. the grounded checkpoint: no acknowledgement → nothing fetched
        let pending = call_tool("pull_grounded_signals", json!({ "ticker": "AAPL" }));
        assert!(pending.contains("PENDING_APPROVAL"));

        // 5. acknowledged → the (mock, in CI) grounded pull runs
        let grounded = call_tool(
            "pull_grounded_signals",
            json!({ "ticker": "AAPL", "acknowledged": true }),
        );
        assert!(grounded.contains("GROUNDED"));
        assert!(grounded.contains("not financial advice"));

        // 6. the deprecated spelling still works, so an existing host config does not break on
        //    an upgrade. It is accepted, documented as deprecated, and behaves identically.
        let legacy = call_tool(
            "pull_grounded_signals",
            json!({ "ticker": "AAPL", "approved": true }),
        );
        assert!(legacy.contains("GROUNDED"), "{legacy}");
    }

    /// What this server may honestly claim about approval.
    ///
    /// The audit's proposed fix — mint a single-use nonce, return it in PENDING_APPROVAL, require it
    /// back — does not work here, and shipping it would be worse than the gap: the calling model
    /// reads its own tool result, so it can read the nonce and echo it. That is the model shaking
    /// hands with itself. What IS true is that the host shows the tool description and arguments to
    /// a person before permitting the call, so the consent text goes there and the limitation is
    /// stated rather than papered over.
    #[test]
    fn the_server_never_claims_to_have_verified_a_human() {
        let catalogue = tools().to_string();
        let profile = call_tool(
            "make_profile",
            json!({ "date": "1990-05-15", "tz": "America/New_York", "lat": 40.71, "lon": -74.0 }),
        );
        for tool in ["recommend", "pull_grounded_signals"] {
            let pending = call_tool(tool, json!({ "profile": profile, "ticker": "AAPL" }));
            assert!(
                pending.contains("cannot verify"),
                "{tool} must say so: {pending}"
            );
        }
        // The costed tools declare that they reach outside and are not read-only, which is what a
        // host uses to decide when to prompt.
        assert!(catalogue.contains("\"openWorldHint\":true"), "{catalogue}");
        // And the schema no longer describes the flag as an approval the server checked.
        assert!(!catalogue.contains("Must be true to run the external pull"));
    }

    /// The MCP checkpoint and the desktop checkpoint must ask for the **same** consent.
    ///
    /// They didn't. This server hand-wrote its own "SEC EDGAR filings and Wikipedia" sentence, so
    /// when the roster grew to SEC financials, Wikidata, vPIC and openFDA, the desktop prompt was
    /// updated and this one silently wasn't — an MCP host was authorizing more than it was told.
    /// Both now render `agents::grounding_consent`, and this test fails if a second copy ever
    /// reappears.
    #[test]
    fn the_mcp_checkpoint_asks_for_the_same_consent_as_the_app() {
        let choice = agents::demo_choices()
            .into_iter()
            .find(|c| c.ticker == "AAPL")
            .expect("AAPL is seeded");
        let canonical = agents::grounding_consent(&choice);

        let pending = call_tool("pull_grounded_signals", json!({ "ticker": "AAPL" }));
        assert!(
            pending.contains(&canonical),
            "the MCP checkpoint must render the shared consent verbatim.\n  expected: {canonical}\n  \
             got: {pending}"
        );
        // And the thing that actually matters: it names every source the approval would spend.
        for source in ["SEC EDGAR", "SEC XBRL", "Wikidata", "Wikipedia"] {
            assert!(pending.contains(source), "{source} unnamed in: {pending}");
        }
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
