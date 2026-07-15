//! ModelPanel — the in-app **local-model** benchmark + recommendation + search. Answers "how
//! powerful is this machine, and which local model (and which quant) should I run?" by calling the
//! `model` crate directly (no CLI, no second binary). The benchmark (which probes the GPU and lists
//! the repo's quants online) and the Hugging Face search are blocking, so they run off the event loop
//! and stream results back through a coroutine — the same `!Send`-safe discipline the readings use.

use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::components::{WheatLoader, WheatPhase};
use model::{
    agent_disqualified, detect_gpu, detect_spec_with, gpu_serve_args, have_llama_server,
    llama_install_hint, model_cached, plan_serve, probe_devices, recommend_for, resolve_candidates,
    resolve_llama_server, running_server_port, select_device, Candidate, DeviceSpec, GpuInfo,
    Recommendation, ServePlan,
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
    // The RuntimeHealth line + a warn flag: which backend + GPU the resolved llama-server will serve
    // on (e.g. "CUDA → NVIDIA GeForce RTX 5080 Laptop GPU"), and whether that path is risky (an
    // integrated GPU / CPU). None if llama.cpp isn't installed.
    Option<(String, bool)>,
    // Whether the fitted model is ALREADY in the Hugging Face cache — serving then only loads it, no
    // download. Drives the "downloaded ✓" hint + the "Serve" (vs "Download & serve") button label.
    bool,
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

/// Stop any running `llama-server` before starting a new one — SINGLE-ACTIVE serve. Each "serve"
/// click spawns a detached server; without this they STACK (observed: 4 copies of a 14B, one on the
/// GPU and three spilling to RAM, drove the commit charge to 99.6/100.7 GB and hung the machine). Also
/// clears a stale server left running after the app was closed. Best-effort; ignores errors. Note this
/// targets `llama-server` specifically, so LM Studio's own runtime (a different binary) is untouched.
fn stop_prior_servers() {
    #[cfg(windows)]
    let _ = std::process::Command::new("taskkill")
        .args(["/F", "/IM", "llama-server.exe"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    #[cfg(not(windows))]
    let _ = std::process::Command::new("pkill")
        .args(["-x", "llama-server"])
        .status();
}

/// The phases of a local serve, streamed to the UI so a long, silent operation (an 11 GB first-run
/// download + a model load into VRAM) shows real progress instead of one static line.
#[derive(Clone, PartialEq)]
enum ServeProgress {
    /// Spawning `llama-server`; nothing to report yet.
    Preparing,
    /// First-run model download, `0..=100` %. Parsed from `llama-server`'s stderr.
    Downloading(u8),
    /// Download done (or model cached); loading the weights into VRAM. Indeterminate.
    Loading,
    /// Live and serving — the success line (endpoint + runtime).
    Serving(String),
    /// Didn't come up — the reason (port busy, download failed, exited early).
    Failed(String),
}

impl ServeProgress {
    /// A terminal state clears the "starting…" spinner on the button.
    fn is_done(&self) -> bool {
        matches!(self, ServeProgress::Serving(_) | ServeProgress::Failed(_))
    }
}

/// Pull a download percentage out of a `llama-server` stderr line, e.g.
/// `Downloading gpt-oss-20b-Q6_K.gguf ─────╴ 73%` → `Some(73)`. `None` for any non-download line.
fn parse_download_pct(line: &str) -> Option<u8> {
    if !line.contains("Downloading") {
        return None;
    }
    let idx = line.rfind('%')?;
    let digits: String = line[..idx]
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    digits.parse::<u32>().ok().map(|n| n.min(100) as u8)
}

/// Is the just-spawned server ready to serve (model loaded)? Probes `/health` on the exact port —
/// `llama-server` answers `{"status":"ok"}` only once the model is resident. Keyless, 3 s cap.
fn port_ready(port: u16) -> bool {
    std::process::Command::new("curl")
        .args([
            "-sS",
            "--max-time",
            "3",
            &format!("http://127.0.0.1:{port}/health"),
        ])
        .output()
        .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("\"ok\""))
        .unwrap_or(false)
}

#[component]
pub fn ModelPanel() -> Element {
    let mut spec = use_signal(|| None::<DeviceSpec>);
    let mut rec = use_signal(|| None::<Recommendation>);
    let mut gpu = use_signal(|| None::<GpuInfo>);
    let mut have_server = use_signal(|| false);
    let mut plan = use_signal(|| None::<ServePlan>);
    let mut runtime = use_signal(|| None::<String>);
    let mut runtime_warn = use_signal(|| false);
    let mut cached = use_signal(|| false);
    let mut running = use_signal(|| false);

    // Search state.
    let mut query = use_signal(String::new);
    let mut cands = use_signal(Vec::<Candidate>::new);
    let mut searching = use_signal(|| false);
    let mut searched = use_signal(|| false);

    // Off-thread benchmark result → set the signals.
    let bench = use_coroutine(move |mut rx: UnboundedReceiver<BenchResult>| async move {
        while let Some((s, r, g, srv, p, rt, is_cached)) = rx.next().await {
            // Prefill the search box with the pick's family — done HERE (an event handler), never in
            // the render body: writing a signal you also read during render triggers Dioxus's
            // "write during render" warning and can loop.
            if let Recommendation::Local(pick) = &r {
                if query.read().is_empty() {
                    query.set(pick.search_term.to_string());
                }
            }
            // Warn only when the serving path is genuinely risky (integrated GPU / CPU) — a Vulkan
            // build on the discrete NVIDIA is fine, so it must NOT warn.
            runtime_warn.set(rt.as_ref().is_some_and(|(_, warn)| *warn));
            runtime.set(rt.map(|(line, _)| line));
            cached.set(is_cached);
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

    // Serve state — the in-app "download & serve" spawns llama-server off-thread and streams its
    // progress (download % → loading → serving) back here so the long silent steps show feedback.
    let mut serving = use_signal(|| false);
    let mut serve_status = use_signal(|| None::<ServeProgress>);
    let server_co = use_coroutine(move |mut rx: UnboundedReceiver<ServeProgress>| async move {
        while let Some(p) = rx.next().await {
            // Keep the button spinner up until a terminal state (Serving/Failed).
            if p.is_done() {
                serving.set(false);
            }
            serve_status.set(Some(p));
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
            // RuntimeHealth: which backend + GPU the resolved llama-server actually serves on. This is
            // the on-screen proof the iGPU-trap is fixed ("CUDA → RTX 5080" vs "Vulkan → AMD iGPU").
            // Set ZIQPU_DEBUG=1 for the verbose resolution trace (managed-runtime lookup + device list).
            let debug = std::env::var("ZIQPU_DEBUG").is_ok();
            if debug {
                if let Some(root) = model::managed_runtime_root() {
                    eprintln!(
                        "[ziqpu-runtime] managed root = {} (read_dir={:?})",
                        root.display(),
                        std::fs::read_dir(&root).map(|it| it
                            .flatten()
                            .map(|e| e.file_name().to_string_lossy().into_owned())
                            .collect::<Vec<_>>())
                    );
                }
            }
            let rt = resolve_llama_server().and_then(|bin| {
                let devs = probe_devices(&bin);
                if debug {
                    eprintln!(
                        "[ziqpu-runtime] llama-server = {} · {devs:?}",
                        bin.display()
                    );
                }
                select_device(&devs).map(|d| {
                    (
                        format!("{} → {}", d.backend.label(), d.name),
                        !d.is_healthy(),
                    )
                })
            });
            // Fit the quant to this machine — lists the resolved repo's GGUFs online. None offline.
            let p = match &r {
                Recommendation::Local(pick) => plan_serve(pick, &s, g.as_ref()),
                _ => None,
            };
            // Already in the local HF cache? Then serving only loads it — no download.
            let is_cached = p
                .as_ref()
                .map(|pl| model_cached(&pl.repo, &pl.quant))
                .unwrap_or(false);
            let _ = tx.unbounded_send((s, r, g, srv, p, rt, is_cached));
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
        serve_status.set(Some(ServeProgress::Preparing));
        let tx = server_co.tx();
        std::thread::spawn(move || {
            // RECONNECT, don't reload: if a model server is already loaded + serving, just point Local
            // mode at it. No re-download, no re-load — the whole "why load every time" fix. (Single-
            // active means the running server is the one we last served.)
            if let Some(port) = running_server_port() {
                std::env::set_var("ZIQPU_LLM_URL", format!("http://127.0.0.1:{port}/v1"));
                let _ = tx.unbounded_send(ServeProgress::Serving(format!(
                    "Connected — model already loaded on :{port}. Switch the header toggle to Local."
                )));
                return;
            }
            // Resolve the backend-correct llama-server (app-managed CUDA/Metal/ROCm build first, then
            // PATH/winget) — NOT the winget Vulkan build that binds a hybrid laptop's integrated GPU.
            let Some(bin) = resolve_llama_server() else {
                let _ = tx.unbounded_send(ServeProgress::Failed(format!(
                    "llama.cpp not found — install it first ({}).",
                    llama_install_hint()
                )));
                return;
            };
            let resolved =
                plan_v.or_else(|| spec_v.and_then(|s| plan_serve(&pick, &s, gpu_v.as_ref())));
            let Some(plan) = resolved else {
                let _ = tx.unbounded_send(ServeProgress::Failed(
                    "Couldn't list the repo's quants (offline?). Try again online.".into(),
                ));
                return;
            };
            // Which GPU will this build actually serve on? Pin the discrete device so we never land on
            // the integrated GPU (the OOM saga). Empty args on a CPU-only build/machine.
            let devices = probe_devices(&bin);
            let device = select_device(&devices);
            let runtime_line = match &device {
                Some(d) => format!("{} → {}", d.backend.label(), d.name),
                None => "CPU (no GPU visible to this build)".to_string(),
            };
            let hf = format!("{}:{}", plan.repo, plan.quant);
            let port = free_local_port(1234);
            let mut args: Vec<String> = vec![
                "-hf".into(),
                hf.clone(),
                "--host".into(),
                "127.0.0.1".into(),
                "--port".into(),
                port.to_string(),
                // Cap the context so the KV cache fits alongside the weights — llama-server otherwise
                // sizes it to the model's full trained context (32k+), several GB of KV. SERVE_CTX_SIZE.
                "-c".into(),
                model::SERVE_CTX_SIZE.to_string(),
                // Quieter load logs (llama.cpp defaults to verbosity 3) — keeps errors.
                "-lv".into(),
                "1".into(),
            ];
            // Full GPU offload + explicit device pin (`-ngl 99 -dev CUDA0`) — the anti-iGPU-trap flags.
            args.extend(gpu_serve_args(device.as_ref()));

            // SINGLE-ACTIVE: stop any prior server first so serves don't stack (the machine-hang bug),
            // then give the OS a moment to release its VRAM + committed memory before we load the new
            // one onto the same GPU.
            stop_prior_servers();
            std::thread::sleep(std::time::Duration::from_millis(700));

            // Spawn with stderr PIPED so we can stream the first-run download %. stdout is noise.
            let mut child = match std::process::Command::new(&bin)
                .args(&args)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.unbounded_send(ServeProgress::Failed(format!(
                        "Failed to start llama-server: {e}"
                    )));
                    return;
                }
            };

            // Reader thread: parse `Downloading … NN%` from stderr. llama-server rewrites the % on ONE
            // line with a carriage return (not newlines), so split on BOTH \r and \n or we'd only see
            // the number after the download finished. `last_pct` (MAX = none yet) lets the poll loop
            // below tell "downloading" from "loading" (a cached model prints no download line).
            use std::sync::atomic::{AtomicU8, Ordering};
            let last_pct = std::sync::Arc::new(AtomicU8::new(u8::MAX));
            if let Some(mut stderr) = child.stderr.take() {
                let tx2 = tx.clone();
                let lp = last_pct.clone();
                std::thread::spawn(move || {
                    use std::io::Read;
                    let mut buf = [0u8; 4096];
                    let mut acc: Vec<u8> = Vec::new();
                    let mut last_sent = u8::MAX;
                    while let Ok(n) = stderr.read(&mut buf) {
                        if n == 0 {
                            break; // EOF — server stopped logging (or exited)
                        }
                        for &b in &buf[..n] {
                            if b == b'\r' || b == b'\n' {
                                let line = String::from_utf8_lossy(&acc);
                                if let Some(pct) = parse_download_pct(&line) {
                                    lp.store(pct, Ordering::Relaxed);
                                    if pct != last_sent {
                                        last_sent = pct;
                                        let _ = tx2.unbounded_send(ServeProgress::Downloading(pct));
                                    }
                                }
                                acc.clear();
                            } else {
                                acc.push(b);
                            }
                        }
                    }
                });
            }

            // Poll loop: watch for early exit, announce Loading once the download is done (or right
            // away for a cached model), and flip to Serving when /health reports the model resident.
            let started = std::time::Instant::now();
            let mut announced_loading = false;
            loop {
                if let Ok(Some(status)) = child.try_wait() {
                    let _ = tx.unbounded_send(ServeProgress::Failed(format!(
                        "llama-server exited ({status}) — usually the port is busy or the download failed. Check the terminal."
                    )));
                    return;
                }
                if port_ready(port) {
                    // Point Local mode here and report success. Dropping `child` now detaches the
                    // server (Child::drop never kills the process) so it keeps serving.
                    std::env::set_var("ZIQPU_LLM_URL", format!("http://127.0.0.1:{port}/v1"));
                    let _ = tx.unbounded_send(ServeProgress::Serving(format!(
                        "Serving {hf} (~{:.0} GB) on :{port} · {runtime_line}. Switch the header toggle to Local.",
                        plan.size_gb
                    )));
                    return;
                }
                let pct = last_pct.load(Ordering::Relaxed);
                let downloading = pct != u8::MAX && pct < 100;
                if !downloading
                    && !announced_loading
                    && (pct == 100 || started.elapsed() > std::time::Duration::from_secs(3))
                {
                    announced_loading = true;
                    let _ = tx.unbounded_send(ServeProgress::Loading);
                }
                if started.elapsed() > std::time::Duration::from_secs(1800) {
                    let _ = tx.unbounded_send(ServeProgress::Failed(
                        "Timed out waiting for the model — check the terminal.".into(),
                    ));
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(600));
            }
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
    // RuntimeHealth: the backend + GPU the serve will land on. A Vulkan runtime on a discrete-GPU
    // machine is the iGPU-trap warning; CUDA/Metal/ROCm is the healthy path.
    let runtime_line = runtime.read().clone();
    let runtime_bad = *runtime_warn.read();
    let srv = *have_server.read();
    let install_hint = llama_install_hint().to_string();
    let benched = s_opt.is_some();
    let is_cached = *cached.read();
    let serving_now = *serving.read();
    // Serve progress → (label, wheat phase, kind). The wheat loader draws the phase; `None` phase =
    // no wheat (a failure, shown as carnelian text). `kind` colors the label.
    let serve_view: Option<(String, Option<WheatPhase>, &'static str)> =
        serve_status.read().as_ref().map(|p| match p {
            ServeProgress::Preparing => {
                ("Preparing…".to_string(), Some(WheatPhase::Loading), "prep")
            }
            ServeProgress::Downloading(pct) => (
                format!("Downloading model… {pct}%"),
                Some(WheatPhase::Download(*pct)),
                "dl",
            ),
            ServeProgress::Loading => (
                "Loading model into VRAM…".to_string(),
                Some(WheatPhase::Loading),
                "load",
            ),
            ServeProgress::Serving(line) => (line.clone(), Some(WheatPhase::Done), "ok"),
            ServeProgress::Failed(reason) => (reason.clone(), None, "fail"),
        });

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

            // While the benchmark runs (GPU probe + online quant lookup), show it's working.
            if *running.read() {
                WheatLoader { phase: WheatPhase::Loading }
            }

            if benched {
                if let Some(line) = specs_line {
                    div { class: "modelpanel-specs", "{line}" }
                }
                if let Some(line) = runtime_line {
                    div {
                        class: if runtime_bad { "modelpanel-runtime modelpanel-runtime--warn" } else { "modelpanel-runtime modelpanel-runtime--ok" },
                        span { class: "modelpanel-runtime-dot" }
                        "runtime · {line}"
                        if runtime_bad {
                            span { class: "modelpanel-runtime-note",
                                " — ⚠ integrated GPU; a CUDA/Metal build serves on your discrete card"
                            }
                        }
                    }
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
                        if is_cached {
                            div { class: "modelpanel-cached",
                                span { class: "modelpanel-cached-dot" }
                                "downloaded — serving just loads it, no re-download"
                            }
                        }
                        button {
                            class: "btn btn--go modelpanel-btn",
                            r#type: "button",
                            disabled: serving_now,
                            onclick: move |_| {
                                let mut f = run_serve;
                                f(());
                            },
                            if serving_now {
                                "Starting…"
                            } else if is_cached {
                                "Serve locally"
                            } else {
                                "Download & serve locally"
                            }
                        }
                        if let Some((label, wheat, kind)) = serve_view {
                            div { class: "modelpanel-serve modelpanel-serve--{kind}",
                                p { class: "modelpanel-serve-label", "{label}" }
                                if let Some(phase) = wheat {
                                    WheatLoader { phase }
                                }
                            }
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
