//! `ziqpu-mcp` — the MCP server over stdio. Reads newline-delimited JSON-RPC on stdin, writes
//! one JSON response per line on stdout. Point an MCP host (Claude Desktop, an IDE) at this binary.
//!
//! # Bidirectional, so the consent gate can reach a person
//!
//! The loop used to be strictly request-in / response-out, which is why `mcp::handle` could stay
//! pure and unit-testable — and also why the consent gate could only ever trust a flag the calling
//! model set for itself. Elicitation reverses the direction: the **server** issues a request and the
//! **host** answers it, interleaved with the host's own traffic.
//!
//! [`StdioElicitor`] is where that reversal lives, so `mcp::handle` keeps its purity. The library
//! takes an `Elicitor`; this binary supplies one backed by real stdio; the tests supply a fake.

use std::cell::RefCell;
use std::io::{self, BufRead, Write};

use serde_json::{json, Value};

fn main() {
    // The key AND the saved preferences. Both live outside the environment — the key in the OS
    // keystore, the model and provider choice in settings.json — and the MCP server honoured
    // neither until they were loaded here. A key without its preferences meant the right
    // credential calling the wrong model. An exported variable still wins over both.
    agents::prefs::load_saved_configuration();

    let stdin = io::stdin();
    let transport = Transport {
        lines: RefCell::new(stdin.lock().lines()),
        stdout: RefCell::new(io::stdout()),
        next_id: RefCell::new(0),
    };

    while let Some(req) = transport.next_message() {
        if let Some(resp) = mcp::handle_with(&req, &transport) {
            if !transport.write(&resp) {
                break;
            }
        }
    }
}

/// Owns stdin and stdout so a server-initiated request can be written and its reply awaited.
struct Transport {
    lines: RefCell<io::Lines<io::StdinLock<'static>>>,
    stdout: RefCell<io::Stdout>,
    next_id: RefCell<u64>,
}

impl Transport {
    /// The next well-formed JSON message from the host, or `None` at end of input.
    ///
    /// Malformed lines are skipped rather than fatal — one bad line from a host should not take the
    /// session down.
    fn next_message(&self) -> Option<Value> {
        loop {
            let line = self.lines.borrow_mut().next()?.ok()?;
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                return Some(v);
            }
        }
    }

    fn write(&self, msg: &Value) -> bool {
        let mut out = self.stdout.borrow_mut();
        writeln!(out, "{msg}").is_ok() && out.flush().is_ok()
    }
}

impl mcp::Elicitor for Transport {
    /// Ask the host to put `message` to a human, and wait for the answer.
    ///
    /// # The interleaving, which is the whole difficulty
    ///
    /// After writing the request, anything may arrive next: our answer, or another host request that
    /// still deserves a reply. Blocking on "the next line is my response" would deadlock a host that
    /// pipelines. So this reads in a loop and dispatches:
    ///
    /// * our response (matched by id) → return the decision;
    /// * any other **request** → answer it with the PURE handler and keep waiting.
    ///
    /// Answering interleaved traffic with the pure handler is deliberate. A nested elicitation would
    /// stack a second approval dialog on top of the first, and a person facing two prompts cannot
    /// tell which one they are answering — so a gated tool called *while* a gate is open falls back
    /// to its flag path rather than asking again.
    fn ask(&self, message: &str) -> mcp::Elicited {
        if !mcp::host_supports_elicitation() {
            return mcp::Elicited::Unavailable;
        }

        // A namespace the host's own numeric ids cannot collide with.
        let id = {
            let mut n = self.next_id.borrow_mut();
            *n += 1;
            format!("ziqpu-elicit-{n}")
        };

        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "elicitation/create",
            "params": {
                "message": message,
                "requestedSchema": {
                    "type": "object",
                    "properties": {
                        "proceed": {
                            "type": "boolean",
                            "title": "Proceed?",
                            "description": "Yes runs the call described above. No cancels it."
                        }
                    },
                    "required": ["proceed"]
                }
            }
        });
        if !self.write(&request) {
            return mcp::Elicited::Unavailable;
        }

        loop {
            // End of input while waiting: the host is gone. Not a yes.
            let Some(msg) = self.next_message() else {
                return mcp::Elicited::Unavailable;
            };

            let is_ours = msg.get("id").and_then(|v| v.as_str()) == Some(id.as_str());
            if is_ours {
                if msg.get("error").is_some() {
                    return mcp::Elicited::Unavailable;
                }
                return decision_from(msg.get("result"));
            }

            // Someone else's traffic. Answer it and keep waiting.
            if let Some(resp) = mcp::handle(&msg) {
                if !self.write(&resp) {
                    return mcp::Elicited::Unavailable;
                }
            }
        }
    }
}

/// Read the host's elicitation result into a decision.
///
/// Two things must BOTH hold for a yes: the action is `accept`, and the returned content actually
/// says `proceed: true`. A host that reports "accept" while the person unticked the box has not
/// given consent, and taking `action` alone would turn a dialog dismissal into a purchase.
fn decision_from(result: Option<&Value>) -> mcp::Elicited {
    let Some(result) = result else {
        return mcp::Elicited::Unavailable;
    };
    match result.get("action").and_then(|a| a.as_str()) {
        Some("accept") => {
            let proceed = result
                .get("content")
                .and_then(|c| c.get("proceed"))
                .and_then(|p| p.as_bool())
                .unwrap_or(false);
            if proceed {
                mcp::Elicited::Accepted
            } else {
                mcp::Elicited::Declined
            }
        }
        Some("decline") => mcp::Elicited::Declined,
        Some("cancel") => mcp::Elicited::Cancelled,
        // An action this server does not recognise is not consent.
        _ => mcp::Elicited::Cancelled,
    }
}

#[cfg(test)]
mod tests {
    use super::decision_from;
    use mcp::Elicited;
    use serde_json::json;

    /// The host's answer is read strictly: `accept` alone is not enough.
    #[test]
    fn only_an_explicit_yes_is_accepted() {
        let accept = json!({ "action": "accept", "content": { "proceed": true } });
        assert_eq!(decision_from(Some(&accept)), Elicited::Accepted);

        // "accept" with the box unticked is a NO. Reading `action` alone would turn a dismissed
        // dialog into a billed call.
        let unticked = json!({ "action": "accept", "content": { "proceed": false } });
        assert_eq!(decision_from(Some(&unticked)), Elicited::Declined);

        let missing = json!({ "action": "accept", "content": {} });
        assert_eq!(decision_from(Some(&missing)), Elicited::Declined);

        assert_eq!(
            decision_from(Some(&json!({ "action": "decline" }))),
            Elicited::Declined
        );
        assert_eq!(
            decision_from(Some(&json!({ "action": "cancel" }))),
            Elicited::Cancelled
        );
        // Anything unrecognised, or nothing at all, is never a yes.
        assert_eq!(
            decision_from(Some(&json!({ "action": "whatever" }))),
            Elicited::Cancelled
        );
        assert_eq!(decision_from(Some(&json!({}))), Elicited::Cancelled);
        assert_eq!(decision_from(None), Elicited::Unavailable);
    }
}
