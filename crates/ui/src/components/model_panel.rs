//! ModelPanel — the in-app **local-model** benchmark + recommendation + search. Answers "how
//! powerful is this machine, and which local model should I run?" by calling the `model` crate
//! directly (no CLI, no second binary). Both the benchmark (which shells out to probe the GPU) and
//! the Hugging Face search are blocking, so they run off the event loop and stream their results
//! back through a coroutine — the same `!Send`-safe discipline the readings use.

use dioxus::prelude::*;
use futures_util::StreamExt;

use model::{
    agent_disqualified, detect_gpu, detect_spec_with, have_llama_server, llama_install_hint,
    llama_server_path, recommend_for, resolve_candidates, resolve_current_repo, Candidate,
    DeviceSpec, GpuInfo, Recommendation,
};

/// The off-thread benchmark result: machine spec, its recommendation, the detected GPU, and whether
/// a `llama-server` binary is installed. All owned + `Send`.
type BenchResult = (DeviceSpec, Recommendation, Option<GpuInfo>, bool);

/// The first free TCP port at or after `start`. Ziqpu's Local default is 1234, but LM Studio or
/// another OpenAI-compatible server frequently already holds it, so a fresh serve falls forward to
/// the next open port. Best-effort: it binds to test then releases, so there's a tiny race before
/// llama-server rebinds — acceptable for a local dev server.
fn free_local_port(start: u16) -> u16 {
    (start..start + 12)
        .find(|&p| std::net::TcpListener::bind(("127.0.0.1", p)).is_ok())
        .unwrap_or(start)
}

#[component]
pub fn ModelPanel() -> Element {
    let mut spec = use_signal(|| None::<DeviceSpec>);
    let mut rec = use_signal(|| None::<Recommendation>);
    let mut gpu = use_signal(|| None::<GpuInfo>);
    let mut have_server = use_signal(|| false);
    let mut running = use_signal(|| false);

    // Search state.
    let mut query = use_signal(String::new);
    let mut cands = use_signal(Vec::<Candidate>::new);
    let mut searching = use_signal(|| false);
    let mut searched = use_signal(|| false);

    // Off-thread benchmark result → set the signals.
    let bench = use_coroutine(move |mut rx: UnboundedReceiver<BenchResult>| async move {
        while let Some((s, r, g, srv)) = rx.next().await {
            // Prefill the search box with the pick's family — done HERE (an event handler), never in
            // the render body: writing a signal you also read during render triggers Dioxus's
            // "write during render" warning and can loop.
            if let Recommendation::Local(pick) = &r {
                if query.read().is_empty() {
                    query.set(pick.search_term.to_string());
                }
            }
            spec.set(Some(s));
            rec.set(Some(r));
            gpu.set(g);
            have_server.set(srv);
            running.set(false);
        }
    });

    // Off-thread Hugging Face search result → the candidate list.
    let finder = use_coroutine(
        move |mut rx: UnboundedReceiver<Vec<Candidate>>| async move {
            while let Some(list) = rx.next().await {
                cands.set(list);
                searching.set(false);
                searched.set(true);
            }
        },
    );

    // Serve state — the in-app "download & serve" spawns llama-server off-thread and reports status.
    let mut serving = use_signal(|| false);
    let mut serve_status = use_signal(|| None::<String>);
    let server_co = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        while let Some(msg) = rx.next().await {
            serve_status.set(Some(msg));
            serving.set(false);
        }
    });

    let run_bench = move |_| {
        running.set(true);
        let tx = bench.tx();
        std::thread::spawn(move || {
            let g = detect_gpu();
            let s = detect_spec_with(g.as_ref());
            let r = recommend_for(&s);
            let srv = have_llama_server();
            let _ = tx.unbounded_send((s, r, g, srv));
        });
    };

    // A `()`-taking action so both the Search button (a mouse event) and Enter (a key event) can
    // trigger it — it captures only Copy signals, so it is itself Copy and usable from both.
    let run_search = move |()| {
        let term = query.read().trim().to_string();
        if term.is_empty() {
            return;
        }
        searching.set(true);
        searched.set(false);
        let tx = finder.tx();
        std::thread::spawn(move || {
            let _ = tx.unbounded_send(resolve_candidates(&term));
        });
    };

    // Download + serve the recommended pick on :1234, off-thread. Resolves the current HF repo, then
    // spawns `llama-server` DETACHED (dropping the Child leaves it running) — the first run downloads
    // the weights. Reports a status line; never blocks the UI. `ModelPick` is Copy, so it moves in.
    let run_serve = move |()| {
        let pick = match &*rec.read() {
            Some(Recommendation::Local(p)) => Some(*p),
            _ => None,
        };
        let Some(pick) = pick else { return };
        serving.set(true);
        serve_status.set(Some("Resolving the model repo…".to_string()));
        let tx = server_co.tx();
        std::thread::spawn(move || {
            let msg = match llama_server_path() {
                None => format!("llama.cpp not found — install it first ({}).", llama_install_hint()),
                Some(bin) => match resolve_current_repo(&pick) {
                    None => "Couldn't resolve a current repo (offline?). Try the search below, or go online.".to_string(),
                    Some(c) => {
                        let hf = format!("{}:{}", c.repo, pick.quant);
                        // :1234 is Ziqpu's Local default, but LM Studio (or another server) often
                        // already holds it — fall forward to the next free port.
                        let port = free_local_port(1234);
                        match std::process::Command::new(&bin)
                            .args(["-hf", &hf, "--host", "127.0.0.1", "--port", &port.to_string()])
                            .spawn()
                        {
                            Err(e) => format!("Failed to start llama-server: {e}"),
                            Ok(mut child) => {
                                // Give it a moment; if it died immediately (busy port, missing quant),
                                // report that truthfully instead of a false "serving".
                                std::thread::sleep(std::time::Duration::from_millis(1800));
                                match child.try_wait() {
                                    Ok(Some(status)) => format!(
                                        "llama-server exited early ({status}). Check the terminal — usually the port is busy or that quant isn't in the repo."
                                    ),
                                    _ => {
                                        // Still up → point Ziqpu's Local mode at this port.
                                        std::env::set_var(
                                            "ZIQPU_LLM_URL",
                                            format!("http://127.0.0.1:{port}/v1"),
                                        );
                                        format!(
                                            "Serving {hf} on :{port}. Local mode now points here — first run downloads the model, so give it a minute, then switch the header toggle to Local."
                                        )
                                    }
                                }
                            }
                        }
                    }
                },
            };
            let _ = tx.unbounded_send(msg);
        });
    };

    // ---- precompute display strings (keeps the rsx! free of fiddly inline formatting) ----
    let rec_now = rec.read().clone();
    let s_opt = *spec.read();
    let specs_line = s_opt.map(|s| {
        let gpu_part = match gpu.read().as_ref() {
            Some(g) => {
                let uni = if g.unified { " unified" } else { "" };
                format!("{} ({:.0} GB{uni})", g.name, g.vram_gb)
            }
            None => "no discrete GPU".to_string(),
        };
        format!("RAM {:.1} GB · {} cores · {gpu_part}", s.ram_gb, s.cores)
    });

    let pick_line = match &rec_now {
        Some(Recommendation::Local(p)) => Some(format!(
            "Tier {} → {} · {} · {} · ~{:.0} GB download",
            p.tier.label(),
            p.name,
            p.params,
            p.quant,
            p.download_gb
        )),
        _ => None,
    };
    let nolocal_line = match &rec_now {
        Some(Recommendation::NoLocal { reason, .. }) => Some(format!(
            "No local model — {}. Use Raw (offline) or Live (an API key) instead.",
            reason.human()
        )),
        _ => None,
    };
    let is_local = matches!(rec_now, Some(Recommendation::Local(_)));
    let srv = *have_server.read();
    let install_hint = llama_install_hint().to_string();
    let benched = s_opt.is_some();
    let serving_now = *serving.read();
    let serve_msg = serve_status.read().clone();

    rsx! {
        div { class: "modelpanel",
            div { class: "modelpanel-head",
                span { class: "settings-label", "Local model — run readings on this machine" }
                button {
                    class: "btn btn--go modelpanel-btn",
                    r#type: "button",
                    onclick: run_bench,
                    if *running.read() { "Measuring…" } else { "Benchmark this machine" }
                }
            }

            if benched {
                if let Some(line) = specs_line {
                    div { class: "modelpanel-specs", "{line}" }
                }
                if let Some(line) = pick_line {
                    div { class: "modelpanel-pick", "{line}" }
                }
                if let Some(line) = nolocal_line {
                    div { class: "modelpanel-nolocal", "{line}" }
                }
                if is_local {
                    if srv {
                        button {
                            class: "btn btn--go modelpanel-btn",
                            r#type: "button",
                            disabled: serving_now,
                            onclick: move |_| {
                                let mut f = run_serve;
                                f(());
                            },
                            if serving_now { "Starting…" } else { "Download & serve on :1234" }
                        }
                        if let Some(msg) = serve_msg.clone() {
                            p { class: "settings-hint", "{msg}" }
                        } else {
                            p { class: "settings-hint",
                                "llama.cpp detected. This downloads the pick (first run) and serves it locally — or run `ziqpu-model serve` in a terminal."
                            }
                        }
                    } else {
                        p { class: "settings-hint modelpanel-warn", "{install_hint}" }
                    }
                }
            } else {
                p { class: "settings-hint",
                    "See this machine's tier and the recommended local model — nothing is downloaded."
                }
            }

            // ---- search the Hugging Face Hub for GGUF models ----
            div { class: "modelpanel-search",
                label { class: "settings-field",
                    span { class: "settings-label", "Search models (Hugging Face GGUF)" }
                    div { class: "settings-keyrow",
                        input {
                            class: "settings-input",
                            r#type: "text",
                            autocomplete: "off",
                            spellcheck: "false",
                            placeholder: "gpt-oss-20b, gemma-3-4b-it, qwen3.5-9b…",
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    let mut f = run_search;
                                    f(());
                                }
                            },
                        }
                        button {
                            class: "settings-reveal",
                            r#type: "button",
                            onclick: move |_| {
                                let mut f = run_search;
                                f(());
                            },
                            if *searching.read() { "…" } else { "search" }
                        }
                    }
                }
                if *searched.read() {
                    if cands.read().is_empty() {
                        p { class: "settings-hint", "No GGUF repos found (or offline). Try a shorter term." }
                    } else {
                        ul { class: "modelpanel-results",
                            {cands.read().iter().take(6).enumerate().map(|(i, c)| {
                                let repo = c.repo.clone();
                                let dl = c.downloads;
                                let bad = agent_disqualified(&repo);
                                let repo_cls = if bad { "modelpanel-repo modelpanel-repo--warn" } else { "modelpanel-repo" };
                                rsx! {
                                    li { key: "{i}",
                                        span { class: "{repo_cls}", "{repo}" }
                                        if bad {
                                            span { class: "modelpanel-uncensored", title: "Abliterated / uncensored — strips the model's safety guardrails. Off-brand for Ziqpu; the recommendation skips these.", "⚠ uncensored" }
                                        }
                                        span { class: "modelpanel-dl", "{dl} ↓" }
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}
