//! `ziqpu-mcp` — the MCP server over stdio. Reads newline-delimited JSON-RPC on stdin, writes
//! one JSON response per line on stdout. Point an MCP host (Claude Desktop, an IDE) at this binary.

use std::io::{self, BufRead, Write};

fn main() {
    // A key the seeker saved in Settings lives in the OS keystore, not the environment. Load it
    // before anything reads it, or the MCP server silently serves the deterministic template while
    // the key sits right there. An exported variable still wins.
    agents::vault::fill_env_from_vault();

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
