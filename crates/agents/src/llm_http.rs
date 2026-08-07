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

/// One OpenAI-compatible **tool-calling** turn: POST the running `messages` + the advertised `tools`,
/// and return `choices[0].message` verbatim (`{role, content, tool_calls?}`) so the caller — the
/// agentic loop in [`crate::tools`] — can both read the `tool_calls` and replay the turn into the
/// conversation. `tool_choice` is `"auto"` (the model decides). `None` on any transport / HTTP /
/// parse error. Used against a local `llama-server --jinja` or any hosted OpenAI-shaped endpoint.
pub(crate) fn openai_tool_turn(
    base_url: &str,
    api_key: &str,
    model: &str,
    messages: &[serde_json::Value],
    tools: &[serde_json::Value],
) -> Option<serde_json::Value> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let mut body = serde_json::json!({
        "model": model,
        "stream": false,
        "max_tokens": 1024,
        "messages": messages,
    });
    if !tools.is_empty() {
        body["tools"] = serde_json::json!(tools);
        body["tool_choice"] = serde_json::json!("auto");
    }
    let text = post_json(
        &url,
        &[
            ("Authorization", &format!("Bearer {api_key}")),
            ("content-type", "application/json"),
        ],
        &body.to_string(),
    )?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    value.get("choices")?.get(0)?.get("message").cloned()
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
        // Raised from 1536 after BOTH live runs of a grounded read came back truncated mid-token —
        // a grounded briefing carries the narrative, the `why:` line, a long GROUNDED line of real
        // signals, the reality beat and the REMINDER, and a reasoning model's excluded thinking can
        // still be charged against the cap. Detecting truncation without raising the cap would have
        // been a regression: every live grounded read would have degraded to the template instead.
        "max_tokens": 3072,
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
    // A response cut off at the token cap is not a reading, it is the first part of one. Observed
    // live: a grounded read ending "total assets: $148.52B (as of 2". The API says so plainly in
    // `finish_reason` and nothing was reading it, so half-sentences reached the seeker as finished
    // prose. Treat it as a failed call and let the honesty ladder degrade instead.
    if value["choices"][0]["finish_reason"].as_str() == Some("length") {
        return None;
    }
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
            // Drain the WHOLE request (headers + Content-Length body) before responding. On Windows,
            // closing a socket while the receive buffer still holds unread request bytes sends a RST,
            // which `ureq` reports as a transport error instead of the 429 — a real flake seen only on
            // windows CI. Fully draining first (then flushing the response) makes the close graceful.
            let mut data: Vec<u8> = Vec::new();
            let mut buf = [0u8; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        data.extend_from_slice(&buf[..n]);
                        if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
                            let head = String::from_utf8_lossy(&data[..pos]);
                            let cl = head
                                .lines()
                                .find_map(|l| {
                                    l.to_ascii_lowercase()
                                        .strip_prefix("content-length:")
                                        .and_then(|v| v.trim().parse::<usize>().ok())
                                })
                                .unwrap_or(0);
                            if data.len() >= pos + 4 + cl {
                                break; // full request in hand
                            }
                        }
                    }
                }
            }
            let body = r#"{"error":"monthly_budget_exhausted"}"#;
            let resp = format!(
                "HTTP/1.1 429 Too Many Requests\r\nContent-Type: application/json\r\n\
                 Content-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(resp.as_bytes());
            let _ = stream.flush();
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

/// Is this URL's host the local machine?
///
/// Written to resist the ways a host can *look* local without being it. The naive check —
/// "does the string contain 127.0.0.1 or localhost" — passes every one of these:
///
/// - `http://127.0.0.1.evil.com/v1` — a real domain that merely starts with the loopback address.
/// - `http://localhost.evil.com/v1` — same trick with the name.
/// - `http://127.0.0.1@evil.com/v1` — the loopback address is *userinfo*; the host is `evil.com`.
///   This one is the nastiest, because it reads as local to a person as well as to a substring test.
///
/// So the host is extracted properly — scheme off, path off, **userinfo off**, port off, IPv6
/// brackets off — and then matched exactly.
pub fn is_loopback_url(url: &str) -> bool {
    let host = url_host(url);

    if host == "localhost" || host == "::1" || host == "0:0:0:0:0:0:0:1" {
        return true;
    }
    // The whole 127.0.0.0/8 block is loopback, not just 127.0.0.1.
    let octets: Vec<&str> = host.split('.').collect();
    octets.len() == 4
        && octets
            .iter()
            .all(|o| !o.is_empty() && o.chars().all(|c| c.is_ascii_digit()))
        && octets[0] == "127"
        && octets.iter().all(|o| o.parse::<u8>().is_ok())
}

/// The endpoint the **local** model paths may use — or `None` when the configured one is refused.
///
/// This is the single gate for `ZIQPU_LLM_URL`, and it is single on purpose: four separate places
/// read that variable (the reading interpreter, the measurer, the research loop, the health probe),
/// and a rule enforced in some of them is not a rule. The consent string drifted exactly this way
/// once already.
///
/// **Why it needs a gate at all.** The field is labelled "Local model URL" and the reading it feeds
/// is badged `LOCAL`, but nothing checked that the address was local — so a typo, a copied config or
/// a tampered settings file would POST the seeker's birth chart to an arbitrary host while the UI
/// said it never left the machine. Birth data is PII by this project's own standard: `profile.json`
/// is `chmod 600` for precisely that reason.
///
/// A model served from another machine on your own network is a legitimate setup, so this refuses
/// rather than forbids: set `ZIQPU_ALLOW_REMOTE_MODEL=1` to allow a non-loopback endpoint knowingly.
/// The default is the least privilege the job needs — talk to a model on *this* machine.
pub fn local_endpoint() -> Option<String> {
    let url = std::env::var("ZIQPU_LLM_URL")
        .ok()
        .filter(|u| !u.trim().is_empty())
        .unwrap_or_else(|| "http://localhost:1234/v1".to_string());

    endpoint_allowed(&url).then_some(url)
}

/// Whether a "local" path may talk to `url` at all: it is on this machine, or the seeker has
/// knowingly allowed one that is not.
///
/// Split out of [`local_endpoint`] because the measurer resolves its own base URL (it also honours
/// `OLLAMA_HOST` and has per-provider defaults) and so cannot share the *resolution* — but it must
/// share the *rule*. Re-implementing the rule there is exactly how it drifts.
pub fn endpoint_allowed(url: &str) -> bool {
    is_loopback_url(url) || remote_model_allowed()
}

/// Whether the endpoint the *local* paths will actually use is **not** on this machine.
///
/// True only in the knowingly-allowed remote case: `ZIQPU_ALLOW_REMOTE_MODEL=1` plus a
/// `ZIQPU_LLM_URL` pointing off-box. It exists so the badge can stop saying LOCAL about a reading
/// that was written somewhere else — permitting the setup and describing it truthfully are two
/// different jobs, and the opt-out only bought the first one.
pub fn local_endpoint_is_offmachine() -> bool {
    local_endpoint().is_some_and(|url| !is_loopback_url(&url))
}

/// The lowercased host of `url` — scheme, path, **userinfo**, port and IPv6 brackets removed.
///
/// One parser, because every host decision in this crate has to agree about what the host *is*. The
/// interesting case is userinfo: in `http://127.0.0.1@evil.com/v1` the loopback address is a
/// username and the host is `evil.com`, which reads as local to a substring check and to a person.
fn url_host(url: &str) -> String {
    let rest = url.split_once("://").map(|(_, r)| r).unwrap_or(url);
    // Authority ends at the first '/', '?' or '#'.
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    // Anything before an '@' is userinfo, not the host. Take the LAST '@' — userinfo may contain one.
    let hostport = authority
        .rsplit_once('@')
        .map(|(_, h)| h)
        .unwrap_or(authority);

    // IPv6 literal: [::1]:1234
    let host = if let Some(end) = hostport.strip_prefix('[').and_then(|r| r.split_once(']')) {
        end.0
    } else {
        // Strip a :port, but only when what follows is numeric (an IPv6 host has colons too).
        match hostport.rsplit_once(':') {
            Some((h, p)) if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => h,
            _ => hostport,
        }
    };
    host.trim().to_ascii_lowercase()
}

/// The hosts a seeker's own provider key may be sent to.
///
/// Short and exact on purpose. These are the three services whose keys this app knows how to use;
/// anything else wanting one is either a mistake or someone else's endpoint.
const PROVIDER_KEY_HOSTS: &[&str] = &["openrouter.ai", "api.openai.com", "api.anthropic.com"];

/// Whether the **seeker's own provider key** may be attached to a request for `url`.
///
/// `OPENAI_BASE_URL` was honoured verbatim: whatever host it named received the key in an
/// `Authorization` header, over any scheme, with nothing checked and nothing shown in the UI. A key
/// is not like a chart — a chart leaked is a privacy harm, a key leaked is someone else spending the
/// seeker's money.
///
/// A host allowlist is the right rule *here* because the variable names a known provider. Loopback
/// is allowed: a local runtime is not a third party and ignores the bearer token anyway. Everything
/// else must be one of [`PROVIDER_KEY_HOSTS`], over HTTPS. As with the model endpoint this
/// **refuses rather than forbids** — `ZIQPU_ALLOW_CUSTOM_ENDPOINT=1` permits a gateway of the
/// seeker's own, knowingly.
pub fn key_destination_allowed(url: &str) -> bool {
    if custom_endpoint_allowed() {
        return true;
    }
    // A local runtime needs no TLS and is not a third party.
    if is_loopback_url(url) {
        return true;
    }
    if !url.starts_with("https://") {
        return false;
    }
    let host = url_host(url);
    PROVIDER_KEY_HOSTS.iter().any(|allowed| host == *allowed)
}

/// Whether the **built-in proxy token** may be sent to `url`.
///
/// Deliberately a weaker rule than [`key_destination_allowed`], and the difference is a fact about
/// the deployment rather than a concession. The proxy is not a third-party provider with a known
/// address — it is *our* endpoint, and whoever builds Ziqpu deploys their own Worker at their own
/// domain. There is no host to put on an allowlist without breaking every fork, so requiring one
/// would be security theatre that also breaks the product.
///
/// What is left is still worth enforcing, because it is the part that was actually missing: the
/// token grants spend on the operator's account, so it must never travel in clear text. HTTPS, or
/// loopback for local proxy development.
pub fn token_destination_allowed(url: &str) -> bool {
    custom_endpoint_allowed() || url.starts_with("https://") || is_loopback_url(url)
}

/// Whether the seeker has knowingly allowed a credential to go to an endpoint of their own choosing.
pub fn custom_endpoint_allowed() -> bool {
    std::env::var("ZIQPU_ALLOW_CUSTOM_ENDPOINT")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "no" || v == "off")
        })
        .unwrap_or(false)
}

/// Whether the seeker has knowingly allowed a non-loopback model endpoint.
pub fn remote_model_allowed() -> bool {
    std::env::var("ZIQPU_ALLOW_REMOTE_MODEL")
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            !(v.is_empty() || v == "0" || v == "false" || v == "no" || v == "off")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod credential_destination_tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    /// Serializes the tests that mutate process-global env vars. Cargo runs tests in threads of one
    /// process, so without this one test clears `ZIQPU_ALLOW_CUSTOM_ENDPOINT` while another has just
    /// set it — which is exactly how the first version of these two tests failed, intermittently and
    /// only when run together. `into_inner` shrugs off a lock poisoned by an earlier panic.
    static ENV_LOCK: Mutex<()> = Mutex::new(());
    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// A key is not a chart. A chart leaked is a privacy harm; a key leaked is someone else spending
    /// the seeker's money — so the destination is checked before the header is ever attached.
    #[test]
    fn a_provider_key_only_goes_to_a_provider() {
        let _env = env_guard();
        std::env::remove_var("ZIQPU_ALLOW_CUSTOM_ENDPOINT");

        for good in [
            "https://openrouter.ai/api/v1",
            "https://api.openai.com/v1",
            "https://api.anthropic.com/v1/messages",
            "http://localhost:1234/v1", // a local runtime is not a third party
            "http://127.0.0.1:8080/v1",
        ] {
            assert!(key_destination_allowed(good), "must allow {good}");
        }

        for bad in [
            "https://evil.com/v1",
            "http://openrouter.ai/api/v1", // plaintext — the key would be readable in transit
            "https://openrouter.ai.evil.com/v1", // merely starts with the provider name
            "https://openrouter.ai@evil.com/v1", // the provider is USERINFO; the host is evil.com
            "https://api.openai.com.evil.com/v1",
        ] {
            assert!(!key_destination_allowed(bad), "must refuse {bad}");
        }

        // Refuses rather than forbids: a seeker's own gateway is legitimate, knowingly.
        std::env::set_var("ZIQPU_ALLOW_CUSTOM_ENDPOINT", "1");
        assert!(key_destination_allowed("https://my-gateway.example/v1"));
        std::env::remove_var("ZIQPU_ALLOW_CUSTOM_ENDPOINT");
    }

    /// The proxy token gets the weaker rule, and the reason is a deployment fact: the proxy is our
    /// own endpoint and every build deploys its own domain, so there is no fixed host to allowlist.
    /// What remains — never in clear text — is the part that was actually missing.
    #[test]
    fn the_proxy_token_needs_tls_but_not_a_known_host() {
        let _env = env_guard();
        std::env::remove_var("ZIQPU_ALLOW_CUSTOM_ENDPOINT");
        assert!(token_destination_allowed(
            "https://p.example.workers.dev/v1/messages"
        ));
        assert!(token_destination_allowed(
            "http://localhost:8787/v1/messages"
        ));
        assert!(
            !token_destination_allowed("http://p.example.workers.dev/v1/messages"),
            "a spend-granting token must not travel in clear text"
        );
    }
}

#[cfg(test)]
mod endpoint_tests {
    use super::*;

    #[test]
    fn plain_loopback_forms_are_local() {
        for url in [
            "http://localhost:1234/v1",
            "http://127.0.0.1:1234/v1",
            "http://127.0.0.1/v1",
            "http://LocalHost:8080/v1",
            "http://127.5.9.3:1234/v1", // the whole 127/8 block is loopback
            "http://[::1]:1234/v1",
            "https://localhost/v1",
        ] {
            assert!(is_loopback_url(url), "should be loopback: {url}");
        }
    }

    /// The bypasses. Each of these defeats a "contains 127.0.0.1 or localhost" check, and the last
    /// one reads as local to a human too — the address is userinfo and the real host is elsewhere.
    #[test]
    fn hosts_that_only_look_local_are_not() {
        for url in [
            "http://127.0.0.1.evil.com/v1",
            "http://localhost.evil.com/v1",
            "http://127.0.0.1@evil.com/v1",
            "http://user:127.0.0.1@evil.com/v1",
            "http://evil.com/?next=http://127.0.0.1/v1",
            "http://10.0.0.5:1234/v1",
            "http://192.168.1.50:1234/v1", // a LAN host is still not this machine
            "https://api.example.com/v1",
            "http://1270.0.0.1/v1",
        ] {
            assert!(
                !is_loopback_url(url),
                "must NOT be treated as loopback: {url}"
            );
        }
    }
}
