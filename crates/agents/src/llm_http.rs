//! A tiny shared HTTP helper for OpenAI-compatible chat completions, called over a `curl`
//! subprocess (the same no-new-dependency approach as the rest of the loop's live paths — no HTTP
//! crate enters the dependency tree). Used by the OpenAI-compat / OpenRouter interpreter, and it
//! mirrors the exact `curl` subprocess shape already used by `measure_llm.rs`.

use std::io::Write;
use std::process::{Command, Stdio};

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
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": user }
        ]
    })
    .to_string();

    // The key is passed to curl via a header arg. Fine for a local demo on the user's own machine;
    // a hosted deployment should use a proper client that keeps the key off argv.
    let mut child = Command::new("curl")
        .args([
            "-sS",
            &url,
            "-H",
            &format!("Authorization: Bearer {api_key}"),
            "-H",
            "content-type: application/json",
            "--data-binary",
            "@-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    child.stdin.take()?.write_all(body.as_bytes()).ok()?;
    let output = child.wait_with_output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let content = value["choices"][0]["message"]["content"].as_str()?;
    let content = content.trim();
    (!content.is_empty()).then(|| content.to_string())
}
