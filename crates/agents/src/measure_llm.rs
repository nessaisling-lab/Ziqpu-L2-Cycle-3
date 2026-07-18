//! Hamun-ana as a real local model — via LM Studio (OpenAI-compatible) or Ollama.
//!
//! It only *sequences* the tools; the chart math stays exact and deterministic (§B of the loop
//! contract). The local model exposes a local HTTP API, called here via a `curl` subprocess (no new
//! crate). Any deviation, error, or missing server falls back to the deterministic sequence — so the
//! recorded tool order is always correct and CI never needs a model.
//!
//! Opt in with `ZIQPU_LOCAL_LLM=1`, then configure with these env vars (all optional):
//!
//! ```text
//! ZIQPU_LLM_PROVIDER  openai (default; LM Studio / Jan / llama.cpp-server) | ollama
//! ZIQPU_LLM_MODEL     model id      (default: gemma-4-e4b-it | qwen2.5:3b-instruct)
//! ZIQPU_LLM_URL       base URL      (default: http://localhost:1234/v1 | http://localhost:11434)
//! ```
//!
//! Legacy `ZIQPU_QWEN=1` (+ `ZIQPU_QWEN_MODEL`, `OLLAMA_HOST`) still selects the Ollama path.

use crate::measure::{expected_sequence, DeterministicMeasurer, Measurer};
use crate::types::ToolCall;
use std::io::Write;
use std::process::{Command, Stdio};

const HAMUN_ANA_SYSTEM: &str = "\
You are Hamun-ana, the measurer. To measure how a seeker fits a choice you must call three tools in \
this exact order: get_chart(you), get_chart(choice), get_synastry(you, choice). Reply with ONLY a \
JSON array of the tool names in order — [\"get_chart\",\"get_chart\",\"get_synastry\"] — and nothing else.";

/// Which local-server protocol to speak.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Provider {
    /// OpenAI-compatible `/v1/chat/completions` — LM Studio, Jan, llama.cpp-server.
    OpenAi,
    /// Ollama-native `/api/chat`.
    Ollama,
}

/// Local-model-backed measurer (Hamun-ana). Falls back to the deterministic sequence on any problem.
pub struct LocalMeasurer {
    provider: Provider,
    base: String,
    model: String,
    fallback: DeterministicMeasurer,
}

impl LocalMeasurer {
    /// Build only when opted in with `ZIQPU_LOCAL_LLM` (or legacy `ZIQPU_QWEN`). See the module
    /// docs for the env vars; sensible localhost defaults otherwise.
    pub fn from_env() -> Option<Self> {
        let new_optin = std::env::var("ZIQPU_LOCAL_LLM").is_ok();
        let legacy_optin = std::env::var("ZIQPU_QWEN").is_ok();
        if !new_optin && !legacy_optin {
            return None;
        }
        // Provider: an explicit setting wins; else the legacy `ZIQPU_QWEN` flag implies Ollama;
        // else default to OpenAI-compatible (the common local runtime — LM Studio et al.).
        let provider = match std::env::var("ZIQPU_LLM_PROVIDER").ok().as_deref() {
            Some("ollama") => Provider::Ollama,
            Some("openai") | Some("lmstudio") => Provider::OpenAi,
            _ if legacy_optin && !new_optin => Provider::Ollama,
            _ => Provider::OpenAi,
        };
        let base = std::env::var("ZIQPU_LLM_URL")
            .or_else(|_| std::env::var("OLLAMA_HOST"))
            .unwrap_or_else(|_| match provider {
                Provider::OpenAi => "http://localhost:1234/v1".to_string(),
                Provider::Ollama => "http://localhost:11434".to_string(),
            });
        let model = std::env::var("ZIQPU_LLM_MODEL")
            .or_else(|_| std::env::var("ZIQPU_QWEN_MODEL"))
            .unwrap_or_else(|_| match provider {
                Provider::OpenAi => "gemma-4-e4b-it".to_string(),
                Provider::Ollama => "qwen2.5:3b-instruct".to_string(),
            });
        Some(Self {
            provider,
            base,
            model,
            fallback: DeterministicMeasurer,
        })
    }

    /// A short human label for the demo banner, e.g. `gemma-4-e4b-it via LM Studio`.
    pub fn describe(&self) -> String {
        let runtime = match self.provider {
            Provider::OpenAi => "OpenAI-compatible (LM Studio)",
            Provider::Ollama => "Ollama",
        };
        format!("{} via {}", self.model, runtime)
    }

    /// Ask the model for the tool-name sequence. `None` on any transport/parse problem.
    fn ask(&self, ticker: &str) -> Option<Vec<String>> {
        let base = self.base.trim_end_matches('/');
        let user = format!("Measure the seeker against {ticker}.");
        // `max_tokens` is generous on purpose: reasoning-style local models (e.g. Gemma) spend
        // tokens thinking before they emit the final `content`; too small a cap truncates them
        // mid-thought and leaves `content` empty (→ fallback).
        let (url, body) = match self.provider {
            Provider::OpenAi => (
                format!("{base}/chat/completions"),
                serde_json::json!({
                    "model": self.model,
                    "stream": false,
                    "temperature": 0,
                    "max_tokens": 512,
                    "messages": [
                        { "role": "system", "content": HAMUN_ANA_SYSTEM },
                        { "role": "user", "content": user }
                    ]
                }),
            ),
            Provider::Ollama => (
                format!("{base}/api/chat"),
                serde_json::json!({
                    "model": self.model,
                    "stream": false,
                    "options": { "temperature": 0 },
                    "messages": [
                        { "role": "system", "content": HAMUN_ANA_SYSTEM },
                        { "role": "user", "content": user }
                    ]
                }),
            ),
        };
        let out = curl_post(&url, &body.to_string())?;
        let value: serde_json::Value = serde_json::from_slice(&out).ok()?;
        // The final answer lives in `content` for both dialects (a reasoning model's thinking goes
        // to a separate `reasoning_content` we deliberately ignore).
        let content = match self.provider {
            Provider::OpenAi => value["choices"][0]["message"]["content"].as_str()?,
            Provider::Ollama => value["message"]["content"].as_str()?,
        };
        // Pull the JSON array of tool names out of the reply.
        let start = content.find('[')?;
        let end = content.rfind(']')?;
        serde_json::from_str::<Vec<String>>(&content[start..=end]).ok()
    }
}

/// POST a JSON body to a local model via `curl` and return the raw response bytes.
///
/// `--max-time` is required even though this is a loopback call. The measurer runs once per choice
/// on the UI's event-loop thread (`state::measures_for` holds a `!Send` session in a `RefCell`), so
/// five serial unbounded POSTs would freeze the window rather than merely stalling a worker. An
/// accepting-but-silent listener on the configured port is the realistic trigger: LM Studio holds a
/// POST open while it JIT-loads a large quant, and a wedged server or an unrelated process squatting
/// :1234 does the same. Timing out is cheap — `None` degrades to the deterministic measurer below.
fn curl_post(url: &str, body: &str) -> Option<Vec<u8>> {
    let mut child = crate::no_window(Command::new("curl"))
        .args([
            "-sS",
            "--max-time",
            "20",
            url,
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
    Some(output.stdout)
}

impl Measurer for LocalMeasurer {
    fn sequence(&self, ticker: &str) -> Vec<ToolCall> {
        // Accept the model's plan only if it names the three tools in the correct order; otherwise
        // fall back. Either way the executed math (and the recorded order) is the exact contract.
        let named_correctly = self
            .ask(ticker)
            .map(|names| {
                names
                    .iter()
                    .map(String::as_str)
                    .eq(["get_chart", "get_chart", "get_synastry"])
            })
            .unwrap_or(false);
        if named_correctly {
            expected_sequence(ticker)
        } else {
            self.fallback.sequence(ticker)
        }
    }
}
