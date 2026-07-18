//! Agentic tool-calling over the OpenAI-compatible `/chat/completions` protocol.
//!
//! This is the capability the **N3 origin-resolver** needs: to chart *anything*, a model must be able
//! to CALL tools — decode a VIN, look up a filing, read a reference — rather than guess an entity's
//! origin. It works against any OpenAI-shaped endpoint, which is the whole point: the same loop drives
//! a **local** `llama-server` started with `--jinja` (which turns on its tool-call grammar) and a
//! **hosted** model, so a seeker's own machine can resolve entities offline.
//!
//! This module is the **loop, not the tools**: request → the model asks to call one or more tools →
//! we execute them → feed the results back → repeat until the model returns a final answer (or a step
//! cap trips, so a model that keeps re-calling can never hang the UI). The concrete resolver tools
//! land with N3; this is the reusable engine.
//!
//! It is deliberately generic — no Ungasaga voice, no guardrail. Those belong to the reading
//! interpreter; the resolver composes this engine *beneath* them.

use serde_json::{json, Value};

/// A tool the model may call. `spec` is the OpenAI `{type:"function", function:{…}}` entry advertised
/// to the model (name + description + JSON-Schema parameters); `call` runs it with the model-supplied
/// arguments and returns a result string that is fed back into the conversation.
///
/// `Send + Sync` because the loop runs on a worker thread (a resolve must never block the UI) and a
/// tool may be shared across the turn.
pub trait Tool: Send + Sync {
    /// The function name the model calls (must match the `name` inside [`Tool::spec`]).
    fn name(&self) -> &str;
    /// The advertised `{type:"function", function:{name, description, parameters}}` object.
    fn spec(&self) -> Value;
    /// Execute with the already-parsed `args` (from the tool_call's `arguments` JSON). The returned
    /// string is handed back to the model verbatim as the tool result, so make it something a model
    /// can read (a short JSON blob or a plain sentence).
    fn call(&self, args: &Value) -> String;
}

/// The default step cap for [`run_tool_loop`] — generous enough for a multi-tool resolve, tight
/// enough that a model stuck re-calling a tool gives up in seconds rather than looping forever.
pub const DEFAULT_MAX_STEPS: usize = 6;

/// Run the tool-calling loop against an OpenAI-compatible endpoint and return the model's final text
/// answer, or `None` on a transport failure or if no final answer arrives within `max_steps` cycles.
///
/// Each cycle is one `/chat/completions` round-trip carrying the running conversation + the tool
/// specs. If the model returns `tool_calls`, every call is executed (an unknown tool name yields an
/// error result rather than aborting — the model can recover), the assistant turn and the tool
/// results are appended, and the loop continues. The first turn with no `tool_calls` is the answer.
pub fn run_tool_loop(
    base_url: &str,
    api_key: &str,
    model: &str,
    system: &str,
    user: &str,
    tools: &[&dyn Tool],
    max_steps: usize,
) -> Option<String> {
    let tool_specs: Vec<Value> = tools.iter().map(|t| t.spec()).collect();
    let mut messages = vec![
        json!({ "role": "system", "content": system }),
        json!({ "role": "user", "content": user }),
    ];

    for _ in 0..max_steps {
        let assistant =
            crate::llm_http::openai_tool_turn(base_url, api_key, model, &messages, &tool_specs)?;

        let calls = assistant
            .get("tool_calls")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        // No tool calls → this is the final answer.
        if calls.is_empty() {
            let content = assistant
                .get("content")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            return (!content.is_empty()).then_some(content);
        }

        // Record the assistant's tool-call turn BEFORE its tool results (the protocol requires the
        // assistant message with `tool_calls` to precede the matching `role:"tool"` messages).
        messages.push(assistant.clone());
        for call in &calls {
            let id = call.get("id").and_then(|i| i.as_str()).unwrap_or("");
            let func = call.get("function");
            let name = func
                .and_then(|f| f.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            // `arguments` is a JSON *string* on the wire; parse it, tolerating a malformed/empty one.
            let args_str = func
                .and_then(|f| f.get("arguments"))
                .and_then(|a| a.as_str())
                .unwrap_or("{}");
            let args: Value = serde_json::from_str(args_str).unwrap_or_else(|_| json!({}));

            let result = tools
                .iter()
                .find(|t| t.name() == name)
                .map(|t| t.call(&args))
                .unwrap_or_else(|| format!("error: no such tool `{name}`"));

            messages.push(json!({
                "role": "tool",
                "tool_call_id": id,
                "content": result,
            }));
        }
    }

    // Hit the step cap without a final answer — a model that keeps calling tools. Better `None` (the
    // caller degrades) than a hang.
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// A trivial tool with a call counter, so a test can prove the loop actually executed it.
    struct SecretTool {
        calls: Arc<AtomicUsize>,
    }
    impl Tool for SecretTool {
        fn name(&self) -> &str {
            "get_secret"
        }
        fn spec(&self) -> Value {
            json!({
                "type": "function",
                "function": {
                    "name": "get_secret",
                    "description": "Return the secret number.",
                    "parameters": { "type": "object", "properties": {} }
                }
            })
        }
        fn call(&self, _args: &Value) -> String {
            self.calls.fetch_add(1, Ordering::Relaxed);
            "42".to_string()
        }
    }

    /// An HTTP response with a JSON body.
    fn http(body: &str) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\
             Connection: close\r\n\r\n{}",
            body.len(),
            body
        )
    }

    /// A mock OpenAI-compatible server that answers each incoming connection with the next canned
    /// response (each `run_tool_loop` cycle opens a fresh connection). Captures every request body so
    /// a test can assert the loop fed the tool result back. Returns `(addr, request_bodies_handle)`.
    fn spawn_mock(responses: Vec<String>) -> (String, Arc<std::sync::Mutex<Vec<String>>>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let bodies = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let bodies2 = bodies.clone();
        std::thread::spawn(move || {
            for resp in responses {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                // Read the WHOLE request — a single read() can return just the headers (or a partial
                // body) for the larger later turns, so accumulate until Content-Length is satisfied.
                let mut data: Vec<u8> = Vec::new();
                let mut buf = [0u8; 4096];
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
                                    break; // full body in hand
                                }
                            }
                        }
                    }
                }
                if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
                    bodies2
                        .lock()
                        .unwrap()
                        .push(String::from_utf8_lossy(&data[pos + 4..]).to_string());
                }
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        (format!("http://{addr}"), bodies)
    }

    #[test]
    fn the_loop_executes_a_tool_and_feeds_the_result_back() {
        // Turn 1: the model asks to call get_secret. Turn 2: it answers using the tool result.
        let tool_call = http(
            r#"{"choices":[{"message":{"role":"assistant","content":null,
                "tool_calls":[{"id":"call_1","type":"function",
                "function":{"name":"get_secret","arguments":"{}"}}]}}]}"#,
        );
        let final_answer =
            http(r#"{"choices":[{"message":{"role":"assistant","content":"The secret is 42."}}]}"#);
        let (url, bodies) = spawn_mock(vec![tool_call, final_answer]);

        let calls = Arc::new(AtomicUsize::new(0));
        let tool = SecretTool {
            calls: calls.clone(),
        };
        let out = run_tool_loop(&url, "k", "m", "sys", "what is the secret?", &[&tool], 5);

        assert_eq!(out.as_deref(), Some("The secret is 42."));
        assert_eq!(
            calls.load(Ordering::Relaxed),
            1,
            "the tool ran exactly once"
        );
        // The SECOND request must carry the tool result (42) back to the model — the whole point of
        // the loop. It also proves the assistant tool-call turn was replayed before the result.
        let bodies = bodies.lock().unwrap();
        assert_eq!(bodies.len(), 2);
        assert!(bodies[1].contains("\"role\":\"tool\""), "{}", bodies[1]);
        assert!(bodies[1].contains("42"), "{}", bodies[1]);
        assert!(
            bodies[0].contains("\"tools\""),
            "the first turn must advertise the tools: {}",
            bodies[0]
        );
    }

    #[test]
    fn a_model_that_never_stops_calling_gives_up_at_the_cap() {
        // Both turns are tool calls → with max_steps=2 the loop makes 2 requests then returns None
        // instead of hanging. (No third connection is opened.)
        let tool_call = http(
            r#"{"choices":[{"message":{"role":"assistant","content":null,
                "tool_calls":[{"id":"c","type":"function",
                "function":{"name":"get_secret","arguments":"{}"}}]}}]}"#,
        );
        let (url, bodies) = spawn_mock(vec![tool_call.clone(), tool_call]);

        let tool = SecretTool {
            calls: Arc::new(AtomicUsize::new(0)),
        };
        let out = run_tool_loop(&url, "k", "m", "sys", "loop forever", &[&tool], 2);

        assert_eq!(out, None, "hitting the step cap yields None, not a hang");
        assert_eq!(
            bodies.lock().unwrap().len(),
            2,
            "exactly max_steps requests"
        );
    }

    /// LIVE integration — proves a REAL `llama-server --jinja` (not a mock) actually emits
    /// `tool_calls` in the shape this loop parses, executes the tool, and uses the result. Needs a
    /// served, tool-capable model; ignored by default (CI has no GPU). Run it after serving:
    ///
    /// ```text
    /// ziqpu-model serve                 # serves the machine's pick on :1234 WITH --jinja
    /// cargo test -p agents live_local_tool_call -- --ignored --nocapture
    /// ```
    ///
    /// Point it elsewhere with `ZIQPU_LLM_URL` (e.g. an LM Studio / other OpenAI-compat endpoint).
    struct AddTool;
    impl Tool for AddTool {
        fn name(&self) -> &str {
            "add"
        }
        fn spec(&self) -> Value {
            json!({
                "type": "function",
                "function": {
                    "name": "add",
                    "description": "Add two integers and return the sum.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "a": {"type": "integer"},
                            "b": {"type": "integer"}
                        },
                        "required": ["a", "b"]
                    }
                }
            })
        }
        fn call(&self, args: &Value) -> String {
            let a = args.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
            let b = args.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
            (a + b).to_string()
        }
    }

    #[test]
    #[ignore = "needs a served tool-capable llama-server (--jinja) on :1234"]
    fn live_local_tool_call() {
        let url = std::env::var("ZIQPU_LLM_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:1234/v1".to_string());
        let model =
            std::env::var("ZIQPU_LOCAL_MODEL").unwrap_or_else(|_| "local-model".to_string());
        let tool = AddTool;
        let out = run_tool_loop(
            &url,
            "",
            &model,
            "You are a calculator. Use the `add` tool for any addition; do not compute it yourself.",
            "What is 17 plus 25?",
            &[&tool],
            DEFAULT_MAX_STEPS,
        )
        .expect("the model should return a final answer");
        println!("live answer: {out}");
        assert!(
            out.contains("42"),
            "the model must have called add(17,25) and used the result 42, got: {out}"
        );
    }

    #[test]
    fn an_unknown_tool_name_is_reported_not_fatal() {
        // The model calls a tool we don't have; the loop feeds back an error result and lets the model
        // recover on the next turn (here, with a final answer).
        let bad_call = http(
            r#"{"choices":[{"message":{"role":"assistant","content":null,
                "tool_calls":[{"id":"x","type":"function",
                "function":{"name":"does_not_exist","arguments":"{}"}}]}}]}"#,
        );
        let recover =
            http(r#"{"choices":[{"message":{"role":"assistant","content":"Sorry, done."}}]}"#);
        let (url, bodies) = spawn_mock(vec![bad_call, recover]);

        let tool = SecretTool {
            calls: Arc::new(AtomicUsize::new(0)),
        };
        let out = run_tool_loop(&url, "k", "m", "sys", "call a bad tool", &[&tool], 5);

        assert_eq!(out.as_deref(), Some("Sorry, done."));
        let bodies = bodies.lock().unwrap();
        assert!(
            bodies[1].contains("no such tool"),
            "the error result is fed back: {}",
            bodies[1]
        );
    }
}
