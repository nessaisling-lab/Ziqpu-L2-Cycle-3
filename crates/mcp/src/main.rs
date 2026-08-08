//! `ziqpu-mcp` — the MCP server over stdio. Reads newline-delimited JSON-RPC on stdin, writes
//! one JSON response per line on stdout. Point an MCP host (Claude Desktop, an IDE) at this binary.

use std::io::{self, BufRead, Write};

fn main() {
    // The key AND the saved preferences. Both live outside the environment — the key in the OS
    // keystore, the model and provider choice in settings.json — and the MCP server honoured
    // neither until they were loaded here. A key without its preferences meant the right
    // credential calling the wrong model. An exported variable still wins over both.
    agents::prefs::load_saved_configuration();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<serde_json::Value>(line) else {
            continue; // ignore malformed lines rather than crash the transport
        };
        if let Some(resp) = mcp::handle(&req) {
            if writeln!(stdout, "{resp}").is_err() || stdout.flush().is_err() {
                break;
            }
        }
    }
}
