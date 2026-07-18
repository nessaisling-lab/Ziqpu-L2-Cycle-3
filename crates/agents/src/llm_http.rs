//! A tiny shared HTTPS helper for the live interpreter paths (OpenAI-compatible / OpenRouter here,
//! Anthropic in `interpret_llm.rs`). Uses `ureq` (pure-Rust rustls TLS, bundled roots) so the request
//! runs IN-PROCESS: no `curl.exe` dependency, and — the security point — the API key is sent as an
//! in-process header and never touches a process command line (the old curl `-H` argv exposure).

use std::time::Duration;

/// The outcome of a POST when the caller must distinguish an HTTP error (with its status + body)
/// from a transport failure — e.g. to recognize the key proxy's "over budget" 429 vs an offline box.
/// [`post_json`] (the common case) discards this detail; the built-in-tier path in `interpret_llm`
/// reads the full outcome to keep [`crate::tier`] honest.
pub(crate) enum PostOutcome {
    /// A 2xx response body.
    Ok(String),
    /// A non-2xx response: the HTTP status and the body (the body carries the proxy's error string).
    Status(u16, String),
    /// Transport / read failure — nothing came back (offline, DNS, timeout, unreadable body).
    Transport,
}

/// One JSON POST over HTTPS, returning the full [`PostOutcome`]. Sets each `(name, value)` header
/// (this is where the secret Authorization header rides — in memory, never on argv) and sends `body`.
/// A 60s timeout guards a hung provider. `ureq::Error::Status` still carries the response, so a
/// non-2xx body is preserved (that is how the proxy's `monthly_budget_exhausted` reaches the caller).
pub(crate) fn post_json_outcome(url: &str, headers: &[(&str, &str)], body: &str) -> PostOutcome {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(60))
        .build();
    let mut req = agent.post(url);
    for (name, value) in headers {
        req = req.set(name, value);
    }
    match req.send_string(body) {
        Ok(r) => match r.into_string() {
            Ok(s) => PostOutcome::Ok(s),
            Err(_) => PostOutcome::Transport,
        },
        Err(ureq::Error::Status(code, r)) => {
            // `Error::Status` carries the response — the proxy's error JSON rides in this body.
            PostOutcome::Status(code, r.into_string().unwrap_or_default())
        }
        Err(_) => PostOutcome::Transport,
    }
}

/// One JSON POST over HTTPS. Returns the response body as a string, or `None` on any transport /
/// non-2xx / read error — matching the loop's "on any error, fall back to the template" contract.
/// A thin wrapper over [`post_json_outcome`] for callers that don't need the status.
pub(crate) fn post_json(url: &str, headers: &[(&str, &str)], body: &str) -> Option<String> {
    match post_json_outcome(url, headers, body) {
        PostOutcome::Ok(s) => Some(s),
        _ => None,
    }
}

/// One JSON GET over HTTPS. Same discipline as [`post_json`] — any key rides an in-process header,
/// never a command line. Returns the response body as a string, or `None` on any transport /
/// non-2xx / read error. Used for live provider model-catalog discovery (see [`crate::models`]);
/// a 20s cap keeps a slow catalog from blocking a UI worker thread for long.
pub(crate) fn get_json(url: &str, headers: &[(&str, &str)]) -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .build();
    let mut req = agent.get(url);
    for (name, value) in headers {
        req = req.set(name, value);
    }
    req.call().ok()?.into_string().ok()
}

/// GET a **local, keyless** endpoint and return its body, or `None` if nothing answered.
///
/// In-process, and that is the point. The local-stack health probes used to shell out to `curl`, and
/// a missing `curl` is indistinguishable from "not ready" — `Command::output()` returns `Err`, which
/// collapsed to "not ready" forever. On a minimal Linux box (an artifact we publish) that turned a
/// perfectly healthy `llama-server` into a thirty-minute spinner ending in "Timed out waiting for
/// the model", because the caller's only other loop exit was the child dying. Windows 10+ and macOS
/// ship curl, which is exactly why it hid on the maintainer's machine.
///
/// **Why the body and not a bool:** a non-2xx response is meaningful here. `llama-server` answers
/// `/health` with **503 while the model loads**, and `curl` (without `-f`) exits 0 on that — so the
/// callers this replaces read a 503 as *loading*, not *down*. `ureq` reports non-2xx as `Err`, so
/// collapsing errors would silently turn "loading" into "down" and destroy the warm-up wait that
/// stops the first cards falling back to the template. `Error::Status` still carries the response,
/// so we keep it. `None` means genuinely nothing answered: no listener, or a timeout.
///
/// `timeout_secs` bounds the whole call — these run in polling loops, so a hung listener must not
/// stall the poll.
pub fn probe_body(url: &str, timeout_secs: u64) -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(timeout_secs))
        .build();
    match agent.get(url).call() {
        Ok(r) => r.into_string().ok(),
        Err(ureq::Error::Status(_, r)) => r.into_string().ok(),
        Err(_) => None,
    }
}

/// One OpenAI-compatible `/chat/completions` round-trip. POSTs to `{base_url}/chat/completions`
/// with a Bearer token and a system + user message (`stream:false`, no temperature), then parses
/// `choices[0].message.content`. Returns `None` on any transport, HTTP, or parse error.
pub(crate) fn openai_chat(
    base_url: &str,
    api_key: &str,
    model: &str,
    system: &str,
    user: &str,
) -> Option<String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "stream": false,
        // Headroom for the richer, narrative reading (the system prompt asks for a flowing passage).
        "max_tokens": 1536,
        // Reasoning models (nemotron-super, DeepSeek-R1, Qwen3-thinking) emit a long chain-of-thought.
        // Ask OpenRouter to keep it OUT of the response (harmless field for plain models / llama.cpp);
        // `strip_reasoning` below is the belt-and-suspenders for providers that ignore this and dump
        // the reasoning into `content` anyway (which is exactly what we saw leak into a reading).
        "reasoning": { "exclude": true },
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": user }
        ]
    })
    .to_string();

    // The Bearer key rides an in-process header (post_json), never a command line.
    let text = post_json(
        &url,
        &[
            ("Authorization", &format!("Bearer {api_key}")),
            ("content-type", "application/json"),
        ],
        &body,
    )?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    let content = value["choices"][0]["message"]["content"].as_str()?;
    let content = strip_reasoning(content);
    (!content.is_empty()).then_some(content)
}

/// Strip a reasoning model's chain-of-thought so only the reading survives. Reasoning models emit a
/// long "think" phase before the answer; well-behaved ones route it to a separate field (handled by
/// the API + our `reasoning.exclude`), but some dump it straight into `content` (observed live:
/// pages of "We must write 4-7 sentences. Let's craft… Check constraints…" ahead of the reading).
/// Defense in depth: drop tagged `<think>`/`<reasoning>` blocks, then — since every Ungasaga reading
/// begins with a `FIT:` line — keep only the text from the LAST `FIT:` onward. A clean answer has
/// exactly one `FIT:` at the start, so this is a no-op there; a reason-then-answer response keeps just
/// the final reading. Returns trimmed text.
fn strip_reasoning(s: &str) -> String {
    let mut out = s.to_string();
    for (open, close) in [
        ("<think>", "</think>"),
        ("<thinking>", "</thinking>"),
        ("<reasoning>", "</reasoning>"),
    ] {
        out = strip_tag_block(&out, open, close);
    }
    if let Some(idx) = out.rfind("FIT:") {
        out = out[idx..].to_string();
    }
    out.trim().to_string()
}

/// Remove every `open`…`close` block (case-insensitive tags), plus a trailing UNCLOSED `open` to the
/// end (a truncated think block). Byte-safe: `to_ascii_lowercase` preserves byte length so tag indices
/// line up with the original, and content is copied whole UTF-8 chars at a time.
fn strip_tag_block(s: &str, open: &str, close: &str) -> String {
    let lower = s.to_ascii_lowercase();
    let open_l = open.to_ascii_lowercase();
    let close_l = close.to_ascii_lowercase();
    let mut out = String::new();
    let mut i = 0;
    while i < s.len() {
        if lower[i..].starts_with(&open_l) {
            match lower[i + open_l.len()..].find(&close_l) {
                Some(rel) => i += open_l.len() + rel + close_l.len(),
                None => break, // unclosed → drop the rest
            }
        } else {
            let ch = s[i..].chars().next().unwrap();
            out.push_str(&s[i..i + ch.len_utf8()]);
            i += ch.len_utf8();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{post_json_outcome, strip_reasoning, strip_tag_block, PostOutcome};

    /// The load-bearing behavior for the built-in-tier honesty fix: a non-2xx response must surface
    /// its **status + body**, not collapse to "nothing came back" — that body is how the proxy's
    /// `monthly_budget_exhausted` reaches `tier::classify`. Verified against a real one-shot socket
    /// (no env vars, no global latch → no cross-test races), because the old `.ok()?` silently
    /// discarded exactly this.
    #[test]
    fn post_json_outcome_surfaces_status_and_body_on_non_2xx() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf); // drain the request line + headers + body
            let body = r#"{"error":"monthly_budget_exhausted"}"#;
            let resp = format!(
                "HTTP/1.1 429 Too Many Requests\r\nContent-Type: application/json\r\n\
                 Content-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(resp.as_bytes());
        });

        let url = format!("http://{addr}/v1/messages");
        match post_json_outcome(&url, &[("content-type", "application/json")], "{}") {
            PostOutcome::Status(code, body) => {
                assert_eq!(code, 429);
                assert!(
                    body.contains("monthly_budget_exhausted"),
                    "the 429 body must survive, got {body:?}"
                );
                // And the full chain classifies it — the reason the banner ever appears.
                assert_eq!(
                    crate::tier::classify(code, &body),
                    crate::tier::BuiltInTier::OverBudget
                );
            }
            _ => panic!("expected a Status outcome carrying the 429 body, not a collapse to None"),
        }
        server.join().unwrap();
    }

    #[test]
    fn strip_reasoning_keeps_only_the_final_reading() {
        // A reason-then-answer dump (nemotron-style): reasoning, a draft, then the real FIT: block.
        let raw = "We must write 4-7 sentences. Let's craft. Must output as shape: FIT: <band>. \
                   Now let's write: FIT: Mixed (56 / 100) — Home Depot, Inc. A real, honest split. \
                   REMINDER: measured, not fate — not financial advice.";
        let out = strip_reasoning(raw);
        assert!(out.starts_with("FIT: Mixed (56 / 100) — Home Depot"));
        assert!(!out.contains("We must write"));
        assert!(out.contains("REMINDER: measured, not fate"));
    }

    #[test]
    fn strip_reasoning_removes_think_tags_and_passes_clean_text() {
        let tagged =
            "<think>ok let me reason about this a while</think>FIT: Aligned (72 / 100) — X. Good.";
        assert!(strip_reasoning(tagged).starts_with("FIT: Aligned"));
        // A clean single-FIT reading is unchanged (aside from trim).
        let clean = "FIT: Aligned (72 / 100) — X. It flows. REMINDER: measured, not fate.";
        assert_eq!(strip_reasoning(clean), clean);
        // A reading with NO FIT: (defensive) still drops think tags and keeps the prose.
        assert_eq!(
            strip_tag_block("<think>x</think>hello", "<think>", "</think>"),
            "hello"
        );
    }
}
