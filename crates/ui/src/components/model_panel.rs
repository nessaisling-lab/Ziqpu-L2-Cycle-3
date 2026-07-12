//! ModelPanel — the in-app **local-model** benchmark + recommendation + search. Answers "how
//! powerful is this machine, and which local model should I run?" by calling the `model` crate
//! directly (no CLI, no second binary). Both the benchmark (which shells out to probe the GPU) and
//! the Hugging Face search are blocking, so they run off the event loop and stream their results
//! back through a coroutine — the same `!Send`-safe discipline the readings use.

use dioxus::prelude::*;
use futures_util::StreamExt;

use model::{
    detect_gpu, detect_spec_with, have_llama_server, llama_install_hint, recommend_for,
    resolve_candidates, Candidate, DeviceSpec, GpuInfo, Recommendation,
};

/// The off-thread benchmark result: machine spec, its recommendation, the detected GPU, and whether
/// a `llama-server` binary is installed. All owned + `Send`.
type BenchResult = (DeviceSpec, Recommendation, Option<GpuInfo>, bool);

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

    // ---- precompute display strings (keeps the rsx! free of fiddly inline formatting) ----
    let rec_now = rec.read().clone();

    // Prefill the search box with the recommended pick's family, so "Search" is one click.
    if let Some(Recommendation::Local(pick)) = &rec_now {
        if query.read().is_empty() {
            query.set(pick.search_term.to_string());
        }
    }

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
                        p { class: "settings-hint",
                            "llama.cpp is installed. Download + serve the pick on :1234 from a terminal:"
                        }
                        code { class: "modelpanel-cmd", "ziqpu-model serve" }
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
                                rsx! {
                                    li { key: "{i}",
                                        span { class: "modelpanel-repo", "{repo}" }
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
