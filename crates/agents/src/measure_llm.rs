//! Hamun-ana as a real local model — Qwen via Ollama.
//!
//! It only *sequences* the tools; the chart math stays exact and deterministic (§B of the loop
//! contract). Ollama exposes a local HTTP API, called here via a `curl` subprocess (no new crate).
//! Any deviation, error, or missing Ollama falls back to the deterministic sequence — so the
//! recorded tool order is always correct and CI never needs a model.
//!
//! Opt in with `ZIQPU_QWEN=1` (model via `ZIQPU_QWEN_MODEL`, host via `OLLAMA_HOST`).

use crate::measure::{expected_sequence, DeterministicMeasurer, Measurer};
use crate::types::ToolCall;
use std::io::Write;
use std::process::{Command, Stdio};

const HAMUN_ANA_SYSTEM: &str = "\
You are Hamun-ana, the measurer. To measure how a seeker fits a choice you must call three tools in \
this exact order: get_chart(you), get_chart(choice), get_synastry(you, choice). Reply with ONLY a \
JSON array of the tool names in order — [\"get_chart\",\"get_chart\",\"get_synastry\"] — and nothing else.";

/// Qwen-backed measurer. Falls back to the deterministic sequence on any problem.
pub struct OllamaMeasurer {
    model: String,
    host: String,
    fallback: DeterministicMeasurer,
}

impl OllamaMeasurer {
    /// Build only when opted in with `ZIQPU_QWEN`. Model defaults to `qwen2.5:3b-instruct`,
    /// host to `http://localhost:11434`.
    pub fn from_env() -> Option<Self> {
        if std::env::var("ZIQPU_QWEN").is_err() {
            return None;
        }
        let model =
            std::env::var("ZIQPU_QWEN_MODEL").unwrap_or_else(|_| "qwen2.5:3b-instruct".to_string());
        let host =
            std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
        Some(Self {
            model,
            host,
            fallback: DeterministicMeasurer,
        })
    }

    /// Ask Qwen for the tool-name sequence. `None` on any transport/parse problem.
    fn ask(&self, ticker: &str) -> Option<Vec<String>> {
        let body = serde_json::json!({
            "model": self.model,
            "stream": false,
            "options": { "temperature": 0 },
            "messages": [
                { "role": "system", "content": HAMUN_ANA_SYSTEM },
                { "role": "user", "content": format!("Measure the seeker against {ticker}.") }
            ]
        })
        .to_string();
        let url = format!("{}/api/chat", self.host.trim_end_matches('/'));
        let mut child = Command::new("curl")
            .args([
                "-sS",
                &url,
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
        let content = value["message"]["content"].as_str()?;
        // Pull the JSON array of tool names out of the reply.
        let start = content.find('[')?;
        let end = content.rfind(']')?;
        serde_json::from_str::<Vec<String>>(&content[start..=end]).ok()
    }
}

impl Measurer for OllamaMeasurer {
    fn sequence(&self, ticker: &str) -> Vec<ToolCall> {
        // Accept Qwen's plan only if it names the three tools in the correct order; otherwise fall
        // back. Either way the executed math (and the recorded order) is the exact contract.
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
