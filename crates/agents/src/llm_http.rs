//! A tiny shared HTTPS helper for the live interpreter paths (OpenAI-compatible / OpenRouter here,
//! Anthropic in `interpret_llm.rs`). Uses `ureq` (pure-Rust rustls TLS, bundled roots) so the request
//! runs IN-PROCESS: no `curl.exe` dependency, and — the security point — the API key is sent as an
//! in-process header and never touches a process command line (the old curl `-H` argv exposure).

use std::time::Duration;

/// One JSON POST over HTTPS. Sets each `(name, value)` header (this is where the secret Authorization
/// header rides — in memory, never on argv) and sends `body`. Returns the response body as a string,
/// or `None` on any transport / non-2xx / read error. A 60s timeout guards a hung provider.
pub(crate) fn post_json(url: &str, headers: &[(&str, &str)], body: &str) -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(60))
        .build();
    let mut req = agent.post(url);
    for (name, value) in headers {
        req = req.set(name, value);
    }
    // `send_string` errors on transport failure AND on a non-2xx status; `.ok()?` collapses both to
    // `None`, matching the loop's "on any error, fall back to the template" contract.
    req.send_string(body).ok()?.into_string().ok()
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
    use super::{strip_reasoning, strip_tag_block};

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
