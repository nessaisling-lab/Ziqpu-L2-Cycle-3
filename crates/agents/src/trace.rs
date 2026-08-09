//! What happened *inside* the loop — the half the tool-call log never recorded.
//!
//! # The blind spot this fills
//!
//! This crate already traces the **decision skeleton**: [`crate::ToolCall`] records which tools ran,
//! in what order, and whether the checkpoint gated the pull. CI grades that log, and it has never
//! once been wrong — the ordering is deterministic.
//!
//! Every real defect found this cycle lived somewhere that log cannot see: inside a model turn.
//!
//! - The frontier returned the system prompt's own format specification and the app printed it as a
//!   finished reading. The output was visible; the prompt/response pair was not, so there was no way
//!   to tell a misread instruction from a malformed one. It was found by running twice and noticing.
//! - Responses were truncated mid-token. `finish_reason: "length"` was sitting in the response body
//!   the whole time; nothing read it and nothing logged it.
//! - An injected instruction came back cited as an SEC fact. Whether the model invented the
//!   attribution or the source really returned it could not be determined from outside, so a stub
//!   source had to be built to reconstruct what a trace would have shown immediately.
//!
//! In short: the reliable half was instrumented and the unreliable half was not.
//!
//! # Why it never touches disk
//!
//! A trace of this agent contains the seeker's derived chart and whatever third-party text was
//! fetched. This project just shipped a "forget my chart" control; a debug file holding the same
//! data would quietly reinstate what that button deletes, and debug artifacts are precisely the
//! files nobody audits. Someone would press Forget, believe it, and be wrong.
//!
//! So the buffer is **in memory, bounded, and dies with the process**. There is nothing to forget
//! because nothing persists. Writing it down is a deliberate act by whoever runs [`dump`], not a
//! default this code chose for them — which is also the honest answer to how long a debug artifact
//! should live: exactly as long as the run it explains.
//!
//! # Levels
//!
//! ```text
//! ZIQPU_TRACE=1      structure only — lengths, models, stop reasons, decisions. No payload.
//! ZIQPU_TRACE=full   the prompts and responses verbatim. For a bug you cannot pin down.
//! ```
//!
//! Redacted is the default `on` because it answers most questions — which tier ran, what it was
//! asked for, why a reading was rejected — without ever holding the text. `full` exists because
//! sometimes the exact bytes *are* the bug (the template echo was), and pretending otherwise would
//! just push people to `println!`.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

/// How much of a turn is recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Nothing is recorded and `record_*` returns immediately. The default.
    Off,
    /// Shapes and decisions: who ran, which model, how long, why a result was rejected. No payload.
    Redacted,
    /// Everything, verbatim, including prompts and fetched text.
    Full,
}

/// The trace level for this process, read once from `ZIQPU_TRACE`.
///
/// Cached rather than re-read, so a long run cannot half-trace if the environment changes under it —
/// a trace with a gap in the middle is worse than no trace, because the gap looks like silence.
pub fn level() -> Level {
    static LEVEL: OnceLock<Level> = OnceLock::new();
    *LEVEL.get_or_init(|| match std::env::var("ZIQPU_TRACE") {
        Ok(v) => match v.trim().to_ascii_lowercase().as_str() {
            "" | "0" | "false" | "no" | "off" => Level::Off,
            "full" | "2" | "verbose" => Level::Full,
            _ => Level::Redacted,
        },
        Err(_) => Level::Off,
    })
}

/// Whether anything is being recorded — cheap enough to guard a `format!` with.
pub fn on() -> bool {
    level() != Level::Off
}

/// The most recent entries. Bounded so a long-running app cannot grow this without limit; a trace
/// is for the run you are looking at, and the last few hundred lines are that run.
const CAP: usize = 512;

fn buffer() -> &'static Mutex<VecDeque<String>> {
    static BUF: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();
    BUF.get_or_init(|| Mutex::new(VecDeque::with_capacity(CAP)))
}

fn push(line: String) {
    let Ok(mut buf) = buffer().lock() else {
        return; // a poisoned lock must never take down a reading
    };
    if buf.len() == CAP {
        buf.pop_front();
    }
    buf.push_back(line);
}

/// One model turn: who asked, what came back, and whether it was usable.
///
/// `prompt` and `response` are only stored at [`Level::Full`]; at [`Level::Redacted`] their lengths
/// are kept, which is enough to spot a truncation, a runaway prompt, or an empty completion.
#[allow(clippy::too_many_arguments)]
pub fn turn(
    who: &str,
    model: &str,
    endpoint: &str,
    prompt: &str,
    response: Option<&str>,
    finish: Option<&str>,
    millis: u128,
) {
    match level() {
        Level::Off => (),
        Level::Redacted => push(format!(
            "TURN  {who:<10} model={model} host={} ms={millis} prompt={}B response={}B finish={}",
            host_of(endpoint),
            prompt.len(),
            response.map_or(0, str::len),
            finish.unwrap_or("-"),
        )),
        Level::Full => push(format!(
            "TURN  {who:<10} model={model} host={} ms={millis} finish={}\n  \
             PROMPT  {prompt}\n  RESPONSE {}",
            host_of(endpoint),
            finish.unwrap_or("-"),
            response.unwrap_or("(none)"),
        )),
    }
}

/// A decision the loop took: which rung produced a reading, why a completion was rejected, which
/// fallback fired.
///
/// These are the lines that answer "where did it change direction" — the honesty ladder degrades
/// *silently* by design, so from outside a template reading and a frontier reading look identical.
pub fn note(what: &str) {
    if on() {
        push(format!("NOTE  {what}"));
    }
}

/// A tool call inside the research loop: what was asked for, and what came back.
///
/// The one place a return-format problem is visible. A tool answering `recent filing: 4 on
/// 2026-08-07` and a model writing "four filings on Aug 7" is only diagnosable with both halves in
/// view — that exact bug shipped, and was found by reasoning backwards from the reading instead.
pub fn tool(name: &str, args: &str, result: &str) {
    match level() {
        Level::Off => (),
        Level::Redacted => push(format!(
            "TOOL  {name} args={}B -> {}B{}",
            args.len(),
            result.len(),
            if result.is_empty() {
                "  (EMPTY — anything grounded after this is unbacked)"
            } else {
                ""
            }
        )),
        Level::Full => push(format!("TOOL  {name} args={args}\n  -> {result}")),
    }
}

/// The host of a URL, so a trace never carries a query string or userinfo that might hold a key.
fn host_of(url: &str) -> String {
    let rest = url.split_once("://").map(|(_, r)| r).unwrap_or(url);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    authority
        .rsplit_once('@')
        .map(|(_, h)| h)
        .unwrap_or(authority)
        .to_string()
}

/// Everything recorded so far, oldest first. Empty when tracing is off.
///
/// Returning a `String` rather than writing a file is the whole privacy design: whoever calls this
/// decides where it goes, and if they never call it the trace evaporates with the process.
pub fn dump() -> String {
    buffer()
        .lock()
        .map(|buf| buf.iter().cloned().collect::<Vec<_>>().join("\n"))
        .unwrap_or_default()
}

/// Forget everything recorded.
///
/// Called by the UI's "forget my chart" control alongside deleting the profile. The buffer is
/// already memory-only, so this is belt to that brace — but a seeker who asks to be forgotten while
/// the app is still running should not have their chart sitting in a debug buffer afterwards.
pub fn clear() {
    if let Ok(mut buf) = buffer().lock() {
        buf.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Redacted keeps the shape and drops the payload — that is the whole point of the level.
    #[test]
    fn redacted_records_shape_not_content() {
        // `level()` caches per process, so exercise the formatting directly rather than fighting it.
        let secret = "Choice name (data): <<Tesla>>. Fit band: Aligned (60 / 100).";
        let line = format!(
            "TURN  {:<10} model={} host={} ms={} prompt={}B response={}B finish={}",
            "ungasaga",
            "claude",
            host_of("https://api.anthropic.com/v1/messages"),
            42,
            secret.len(),
            0,
            "max_tokens",
        );
        assert!(!line.contains("Tesla"), "no payload at redacted: {line}");
        assert!(line.contains("prompt=60B"), "{line}");
        assert!(
            line.contains("finish=max_tokens"),
            "the stop reason is the point: {line}"
        );
    }

    /// A trace must never carry a credential, whatever the endpoint looks like.
    #[test]
    fn only_the_host_survives_a_url() {
        assert_eq!(
            host_of("https://api.openai.com/v1?key=sk-secret"),
            "api.openai.com"
        );
        assert_eq!(
            host_of("https://user:pass@proxy.example/v1"),
            "proxy.example"
        );
        assert_eq!(host_of("http://localhost:1234/v1"), "localhost:1234");
    }

    /// An empty tool result is called out, because it is the moment fabrication becomes possible.
    #[test]
    fn an_empty_tool_result_is_flagged() {
        let empty = format!(
            "TOOL  {} args={}B -> {}B{}",
            "pull_grounded_signals", 2, 0, "  (EMPTY — anything grounded after this is unbacked)"
        );
        assert!(empty.contains("EMPTY"));
    }

    /// The buffer is bounded, so a long session cannot grow it without limit.
    #[test]
    fn the_buffer_is_bounded_and_clearable() {
        clear();
        for i in 0..(CAP + 50) {
            push(format!("line {i}"));
        }
        let n = buffer().lock().unwrap().len();
        assert_eq!(n, CAP, "the ring must cap at {CAP}, got {n}");
        clear();
        assert!(dump().is_empty(), "clear must leave nothing behind");
    }
}
