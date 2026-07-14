//! ModelPanel — the in-app **local-model** benchmark + recommendation + search. Answers "how
//! powerful is this machine, and which local model (and which quant) should I run?" by calling the
//! `model` crate directly (no CLI, no second binary). The benchmark (which probes the GPU and lists
//! the repo's quants online) and the Hugging Face search are blocking, so they run off the event loop
//! and stream results back through a coroutine — the same `!Send`-safe discipline the readings use.

use dioxus::prelude::*;
use futures_util::StreamExt;

use model::{
    agent_disqualified, detect_gpu, detect_spec_with, have_llama_server, llama_install_hint,
    llama_server_path, plan_serve, recommend_for, resolve_candidates, Candidate, DeviceSpec,
    GpuInfo, Recommendation, ServePlan,
};

/// The off-thread benchmark result: machine spec, its recommendation, the detected GPU, whether a
/// `llama-server` binary is installed, and the **fitted serve plan** (repo + the largest quant this
/// machine can run — `None` offline). All owned + `Send`.
type BenchResult = (
    DeviceSpec,
    Recommendation,
    Option<GpuInfo>,
    bool,
    Option<ServePlan>,
);

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
    let mut plan = use_signal(|| None::<ServePlan>);
    let mut running = use_signal(|| false);

    // Search state.
    let mut query = use_signal(String::new);
    let mut cands = use_signal(Vec::<Candidate>::new);
    let mut searching = use_signal(|| false);
    let mut searched = use_signal(|| false);

    // Off-thread benchmark result → set the signals.
    let bench = use_coroutine(move |mut rx: UnboundedReceiver<BenchResult>| async move {
        while let Some((s, r, g, srv, p)) = rx.next().await {
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
            plan.set(p);
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
            // Fit the quant to this machine — lists the resolved repo's GGUFs online. None offline.
            let p = match &r {
                Recommendation::Local(pick) => plan_serve(pick, &s, g.as_ref()),
                _ => None,
            };
            let _ = tx.unbounded_send((s, r, g, srv, p));
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

    // Download + serve the fitted pick off-thread. Uses the benchmark's `ServePlan` (repo + the
    // largest quant this machine can run); re-plans if it's missing. Spawns `llama-server` DETACHED
    // (dropping the Child leaves it running) on the first free port from 1234, then points Ziqpu's
    // Local mode at it. Never blocks the UI.
    let run_serve = move |()| {
        let plan_v = plan.read().clone();
        let spec_v = *spec.read();
        let gpu_v = gpu.read().clone();
        let pick = match &*rec.read() {
            Some(Recommendation::Local(p)) => Some(*p),
            _ => None,
        };
        let Some(pick) = pick else { return };
        serving.set(true);
        serve_status.set(Some("Preparing to serve…".to_string()));
        let tx = server_co.tx();
        std::thread::spawn(move || {
            let Some(bin) = llama_server_path() else {
                let _ = tx.unbounded_send(format!(
                    "llama.cpp not found — install it first ({}).",
                    llama_install_hint()
                ));
                return;
            };
            let resolved =
                plan_v.or_else(|| spec_v.and_then(|s| plan_serve(&pick, &s, gpu_v.as_ref())));
            let Some(plan) = resolved else {
                let _ = tx.unbounded_send(
                    "Couldn't list the repo's quants (offline?). Try again online.".to_string(),
                );
                return;
            };
            let hf = format!("{}:{}", plan.repo, plan.quant);
            let port = free_local_port(1234);
            let msg = match std::process::Command::new(&bin)
                .args([
                    "-hf",
                    &hf,
                    "--host",
                    "127.0.0.1",
                    "--port",
                    &port.to_string(),
                    // Quieter load logs (llama.cpp defaults to verbosity 3) — keeps errors.
                    "-lv",
                    "1",
                ])
                .spawn()
            {
                Err(e) => format!("Failed to start llama-server: {e}"),
                Ok(mut child) => {
                    // Give it a moment; if it died immediately (busy port, download error), say so
                    // truthfully instead of a false "serving".
                    std::thread::sleep(std::time::Duration::from_millis(1800));
                    match child.try_wait() {
                        Ok(Some(status)) => format!(
                            "llama-server exited early ({status}). Check the terminal — usually the port is busy or the download failed."
                        ),
                        _ => {
                            std::env::set_var(
                                "ZIQPU_LLM_URL",
                                format!("http://127.0.0.1:{port}/v1"),
                            );
                            format!(
                                "Serving {hf} (~{:.0} GB) on :{port}. Local mode now points here — first run downloads the model, so give it a minute, then switch the header toggle to Local.",
                                plan.size_gb
                            )
                        }
                    }
                }
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
            "Tier {} → {} · {} params",
            p.tier.label(),
            p.name,
            p.params
        )),
        _ => None,
    };
    // The fitted quant (from the online plan): the largest the machine can run. `None` offline.
    let plan_line = plan.read().as_ref().map(|p| {
        format!(
            "→ {} (~{:.0} GB) — the best quant your machine can run · {}",
            p.quant, p.size_gb, p.repo
        )
    });
    let nolocal_line = match &rec_now {
        Some(Recommendation::NoLocal { reason, .. }) => Some(format!(
            "No local model — {}. Use Raw (offline) or Live (an API key) instead.",
            reason.human()
        )),
        _ => None,
    };
    let is_local = matches!(rec_now, Some(Recommendation::Local(_)));
    let plan_known = plan.read().is_some();
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
                if let Some(line) = plan_line {
                    div { class: "modelpanel-plan", "{line}" }
                }
                if is_local && !plan_known {
                    p { class: "settings-hint",
                        "The best quant for your hardware is chosen when you serve (needs a connection to list the repo)."
                    }
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
                            if serving_now { "Starting…" } else { "Download & serve locally" }
                        }
                        if let Some(msg) = serve_msg.clone() {
                            p { class: "settings-hint", "{msg}" }
                        } else {
                            p { class: "settings-hint",
                                "llama.cpp detected. This downloads the fitted quant (first run) and serves it — or run `ziqpu-model serve` in a terminal."
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
