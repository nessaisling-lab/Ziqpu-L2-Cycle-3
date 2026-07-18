//! ModelPanel — the in-app **local-model** benchmark + recommendation + search. Answers "how
//! powerful is this machine, and which local model (and which quant) should I run?" by calling the
//! `model` crate directly (no CLI, no second binary). The benchmark (which probes the GPU and lists
//! the repo's quants online) and the Hugging Face search are blocking, so they run off the event loop
//! and stream results back through a coroutine — the same `!Send`-safe discipline the readings use.

use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::components::{WheatLoader, WheatPhase, WheatTier, WheatTierState};
use model::{
    agent_disqualified, detect_gpu, detect_spec_with, ensure_runtime, gpu_serve_args,
    have_llama_server, llama_install_hint, max_runnable, model_cached, plan_serve, probe_devices,
    recommend_for, resolve_candidates, resolve_llama_server, running_server_port, select_device,
    Candidate, DeviceSpec, GpuInfo, ModelPick, Recommendation, ServePlan,
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
    // The "#2 Max" pick — the biggest model this machine can hold, when that differs from the
    // Stable recommendation. `None` = no second option, the dropdown collapses to Stable only.
    Option<ModelPick>,
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

/// Where the PID of the `llama-server` **we** spawned is recorded, beside `settings.json` in the
/// user's data dir.
///
/// It has to survive the process, not just live in memory: we detach each server deliberately
/// (`Child::drop` never kills), so a server we started can outlive the app — and the next launch
/// still has to be able to stop it. An in-memory handle would forget it and let servers stack.
fn pid_path() -> Option<std::path::PathBuf> {
    Some(crate::profile::data_dir()?.join("llama-server.pid"))
}

/// Remember the server we just started, so the next serve — or the next launch — can stop exactly
/// that one. Best-effort.
fn record_our_server(pid: u32) {
    if let Some(path) = pid_path() {
        let _ = std::fs::write(path, pid.to_string());
    }
}

/// The PID of the last `llama-server` we started, if we started one.
fn recorded_pid() -> Option<u32> {
    std::fs::read_to_string(pid_path()?)
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Forget the recorded server (it's stopped, or it was never ours).
fn clear_recorded_pid() {
    if let Some(path) = pid_path() {
        let _ = std::fs::remove_file(path);
    }
}

/// The single **active local model** — what we last served, persisted beside the PID so a restart
/// can reconnect to it (or offer to re-serve it) instead of forgetting. One file, one model: this IS
/// the single-active record. Written **only on a successful serve**; cleared whenever we stop the
/// server. It carries the whole resolved plan so a re-serve needs no re-benchmark — crucially
/// including `fits_gpu`, so the anti-OOM CPU-side decision survives the restart too.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
struct ActiveLocal {
    repo: String,
    quant: String,
    size_gb: f64,
    fits_gpu: bool,
    port: u16,
}

impl ActiveLocal {
    fn from_plan(plan: &model::ServePlan, port: u16) -> Self {
        Self {
            repo: plan.repo.clone(),
            quant: plan.quant.clone(),
            size_gb: plan.size_gb,
            fits_gpu: plan.fits_gpu,
            port,
        }
    }
    /// Reconstruct the resolved plan for a re-serve — no benchmark, no re-fit.
    fn to_plan(&self) -> model::ServePlan {
        model::ServePlan {
            repo: self.repo.clone(),
            quant: self.quant.clone(),
            size_gb: self.size_gb,
            fits_gpu: self.fits_gpu,
        }
    }
    fn label(&self) -> String {
        format!("{}:{}", self.repo, self.quant)
    }
}

fn active_local_path() -> Option<std::path::PathBuf> {
    Some(crate::profile::data_dir()?.join("active_local.json"))
}

fn save_active_local(a: &ActiveLocal) {
    if let (Some(path), Ok(json)) = (active_local_path(), serde_json::to_string(a)) {
        let _ = std::fs::write(path, json);
    }
}

fn load_active_local() -> Option<ActiveLocal> {
    let s = std::fs::read_to_string(active_local_path()?).ok()?;
    serde_json::from_str(&s).ok()
}

/// Forget the active local model (its server was stopped, or we're serving a different one).
fn clear_active_local() {
    if let Some(path) = active_local_path() {
        let _ = std::fs::remove_file(path);
    }
}

/// Stop **the server we started** before starting another — SINGLE-ACTIVE serve.
///
/// Each "serve" click spawns a detached server; without this they STACK (observed: 4 copies of a
/// 14B, one on the GPU and three spilling to RAM, drove the commit charge to 99.6/100.7 GB and hung
/// the machine).
///
/// This used to be `taskkill /F /IM llama-server.exe` (and `pkill -x llama-server`) — kill *every*
/// llama-server on the machine. That fixed stacking by breaking other people's work: a developer
/// with their own llama-server running, or two Ziqpu windows open, would have it killed from under
/// them with no warning and no consent. Ziqpu may only stop processes it started, so it now tracks
/// its own child by PID.
///
/// **PID reuse is the hazard.** The OS recycles PIDs, so a recorded PID may name an unrelated
/// process by the time we act — and `/F` doesn't ask. Both branches therefore verify the image is
/// still `llama-server` before signalling, which is what makes "kill only ours" true rather than
/// merely intended.
///
/// A server the *user* started keeps running: [`free_local_port`] already steps to the next free
/// port, so the two coexist instead of fighting. Best-effort; ignores errors.
fn stop_our_server() {
    let Some(pid) = recorded_pid() else { return };
    #[cfg(windows)]
    {
        // taskkill applies /FI before terminating, so a recycled PID simply doesn't match and
        // nothing is killed — the guard and the kill are one atomic call.
        let _ = crate::no_window(std::process::Command::new("taskkill"))
            .args([
                "/F",
                "/PID",
                &pid.to_string(),
                "/FI",
                "IMAGENAME eq llama-server.exe",
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
    #[cfg(not(windows))]
    {
        // No equivalent atomic filter here, so check then signal. The race window (the PID dying
        // and being reused between the two calls) is vanishingly small and the cost of losing it is
        // one stray SIGTERM — versus killing every llama-server on the box, every time, by design.
        let still_ours = std::process::Command::new("ps")
            .args(["-p", &pid.to_string(), "-o", "comm="])
            .output()
            .ok()
            .is_some_and(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .trim()
                    .ends_with("llama-server")
            });
        if still_ours {
            // TERM, not KILL: llama-server releases VRAM on a clean exit, and the caller's pause
            // before loading the next model exists precisely to let that land.
            let _ = std::process::Command::new("kill")
                .args(["-TERM", &pid.to_string()])
                .status();
        }
    }
    clear_recorded_pid();
    // The model we were serving is no longer active — a new serve overwrites this on success, so
    // clearing here keeps the "single active model" record honest (one file, one model, or none).
    clear_active_local();
}

/// The phases of a local serve, streamed to the UI so a long, silent operation (an 11 GB first-run
/// download + a model load into VRAM) shows real progress instead of one static line.
#[derive(Clone, PartialEq)]
enum ServeProgress {
    /// Spawning `llama-server`; nothing to report yet.
    Preparing,
    /// First run on a machine with no llama.cpp at all: `ensure_runtime` is laying down the
    /// backend-correct build. The line is its live progress ("downloading b10066 — CUDA 13.3…").
    Installing(String),
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
///
/// In-process (`ureq`), not a `curl` subprocess. This is the poll loop's only success exit, and a
/// failed `curl` spawn reads exactly like "not ready" — so on a machine without curl a healthy
/// server could never be seen: `try_wait` stays `None` because the server is fine, the probe stays
/// false because curl isn't there, and the loop ran its full 30-minute cap before announcing a
/// timeout for a server that was serving the whole time. Windows 10+ and macOS ship curl; the
/// Linux tarball we publish is where it bites.
fn port_ready(port: u16) -> bool {
    agents::llm_http::probe_body(&format!("http://127.0.0.1:{port}/health"), 3)
        .is_some_and(|body| body.contains("\"ok\""))
}

/// Spawn `llama-server` for a resolved `plan` on a free port, stream progress via `send`, and — once
/// `/health` reports the model resident — point Ziqpu's Local mode at it and persist it as the single
/// active model.
///
/// Shared by the benchmark serve and the one-click re-serve of a remembered model, so both get the
/// same anti-iGPU device pin, the same anti-OOM CPU-side fallback (`plan.fits_gpu`), the same
/// single-active stop-before-start, the same detach-on-success, AND the same restart record. Runs on
/// the caller's worker thread (it blocks in the poll loop); `send` is a cheap `Send + Clone` progress
/// sink (a closure over the coroutine channel), so the one stderr reader thread can clone it.
fn serve_target<S>(bin: std::path::PathBuf, plan: model::ServePlan, send: S)
where
    S: Fn(ServeProgress) + Clone + Send + 'static,
{
    // Which GPU will this build actually serve on? Pin the discrete device so we never land on the
    // integrated GPU (the OOM saga). No pin when the fitted quant is bigger than the card's pool
    // (`fits_gpu == false`): forcing `-ngl 99` there allocates itself to death after a multi-GB
    // download — CPU-side, RAM-fitted, is the honest serve, and the runtime line says so.
    let device = if plan.fits_gpu {
        select_device(&probe_devices(&bin))
    } else {
        None
    };
    let runtime_line = match &device {
        Some(d) => format!("{} → {}", d.backend.label(), d.name),
        None if !plan.fits_gpu => "CPU (model larger than the GPU's memory — slower)".to_string(),
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
        // Cap the context so the KV cache fits alongside the weights (llama-server otherwise sizes it
        // to the model's full 32k+ trained window, several GB of KV).
        "-c".into(),
        model::SERVE_CTX_SIZE.to_string(),
        // Quieter load logs (llama.cpp defaults to verbosity 3) — keeps errors.
        "-lv".into(),
        "1".into(),
    ];
    // Full GPU offload + explicit device pin (`-ngl 99 -dev CUDA0`) — the anti-iGPU-trap flags.
    args.extend(gpu_serve_args(device.as_ref()));

    // SINGLE-ACTIVE: stop OUR prior server (and clear its record) first so serves don't stack (the
    // machine-hang bug), then give the OS a moment to release its VRAM before we load the new one.
    stop_our_server();
    std::thread::sleep(std::time::Duration::from_millis(700));

    // Spawn with stderr PIPED so we can stream the first-run download %. stdout is noise.
    let mut child = match crate::no_window(std::process::Command::new(&bin))
        .args(&args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        // Claim the PID immediately — the only handle that survives us (we detach on success), so the
        // next launch can stop a server we left running even if we're closed mid-load.
        Ok(c) => {
            record_our_server(c.id());
            c
        }
        Err(e) => {
            send(ServeProgress::Failed(format!(
                "Failed to start llama-server: {e}"
            )));
            return;
        }
    };

    // Reader thread: parse `Downloading … NN%` from stderr. llama-server rewrites the % on ONE line
    // with a carriage return, so split on BOTH \r and \n. `last_pct` (MAX = none yet) lets the poll
    // loop tell "downloading" from "loading" (a cached model prints no download line).
    use std::sync::atomic::{AtomicU8, Ordering};
    let last_pct = std::sync::Arc::new(AtomicU8::new(u8::MAX));
    if let Some(mut stderr) = child.stderr.take() {
        let send2 = send.clone();
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
                                send2(ServeProgress::Downloading(pct));
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

    // Poll loop: watch for early exit, announce Loading once the download is done (or right away for a
    // cached model), and flip to Serving — persisting the active-model record — when /health reports
    // the model resident.
    let started = std::time::Instant::now();
    let mut announced_loading = false;
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            // Dead → the recorded PID now names nothing (and the OS may recycle it). Drop the claim.
            clear_recorded_pid();
            send(ServeProgress::Failed(format!(
                "llama-server exited ({status}) — usually the port is busy or the download failed. Check the terminal."
            )));
            return;
        }
        if port_ready(port) {
            // Point Local mode here + remember this as the single active model (so a restart
            // reconnects or offers a re-serve). Dropping `child` now detaches the server.
            std::env::set_var("ZIQPU_LLM_URL", format!("http://127.0.0.1:{port}/v1"));
            save_active_local(&ActiveLocal::from_plan(&plan, port));
            send(ServeProgress::Serving(format!(
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
            send(ServeProgress::Loading);
        }
        if started.elapsed() > std::time::Duration::from_secs(1800) {
            send(ServeProgress::Failed(
                "Timed out waiting for the model — check the terminal.".into(),
            ));
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(600));
    }
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
    // The curated dropdown: `max_pick` = the "#2 Max" option when it differs from Stable;
    // `use_max` = which of the two the seeker chose (false = "#1 Stable", the default).
    let mut max_pick = use_signal(|| None::<ModelPick>);
    let mut use_max = use_signal(|| false);

    // Search state.
    let mut query = use_signal(String::new);
    let mut cands = use_signal(Vec::<Candidate>::new);
    let mut searching = use_signal(|| false);
    let mut searched = use_signal(|| false);

    // Off-thread benchmark result → set the signals.
    let bench = use_coroutine(move |mut rx: UnboundedReceiver<BenchResult>| async move {
        while let Some((s, r, g, srv, p, rt, is_cached, maxp)) = rx.next().await {
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
            // A fresh benchmark resets the choice to Stable — the machine may have changed.
            max_pick.set(maxp);
            use_max.set(false);
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

    // The single active local model, remembered across restarts (`active_local.json`). `remembered`
    // is what we last served; `remembered_live` is whether its server is still up on its port. Loaded
    // + probed once on mount, off-thread (the probe is a network call).
    let mut remembered = use_signal(|| None::<ActiveLocal>);
    let mut remembered_live = use_signal(|| false);
    let mount_probe = use_coroutine(
        move |mut rx: UnboundedReceiver<(Option<ActiveLocal>, bool)>| async move {
            while let Some((rec, live)) = rx.next().await {
                remembered.set(rec);
                remembered_live.set(live);
            }
        },
    );
    // Fire the load+probe exactly once (use_hook runs on the first render only), off-thread.
    use_hook(|| {
        let tx = mount_probe.tx();
        std::thread::spawn(move || {
            let rec = load_active_local();
            let live = rec.as_ref().is_some_and(|r| port_ready(r.port));
            let _ = tx.unbounded_send((rec, live));
        });
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
            // The "#2 Max" option — the biggest model this machine can hold, when that beats the
            // Stable pick. Runnable-only by construction (`runnable_models` is the source).
            let maxp = max_runnable(&s);
            let _ = tx.unbounded_send((s, r, g, srv, p, rt, is_cached, maxp));
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
        let spec_v = *spec.read();
        let gpu_v = gpu.read().clone();
        let stable = match &*rec.read() {
            Some(Recommendation::Local(p)) => Some(*p),
            _ => None,
        };
        let Some(stable) = stable else { return };
        // The dropdown's choice: "#2 Max" swaps in the biggest runnable model. The benchmark's
        // cached ServePlan was fitted for Stable, so a Max serve must re-plan from scratch.
        let chose_max = *use_max.read();
        let pick = match (chose_max, *max_pick.read()) {
            (true, Some(maxp)) => maxp,
            _ => stable,
        };
        let plan_v = if chose_max { None } else { plan.read().clone() };
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
            // Nothing anywhere? Install it ourselves: `ensure_runtime` fetches the right build for
            // this OS/arch/GPU into the app-managed root, streaming progress here. This runs ONLY on
            // the serve path — a below-floor machine never reaches it (no serve button), so "can't
            // run a model" never installs a runtime it can't use.
            let bin = match resolve_llama_server() {
                Some(bin) => bin,
                None => {
                    let txp = tx.clone();
                    match ensure_runtime(&mut |line| {
                        let _ = txp.unbounded_send(ServeProgress::Installing(line.to_string()));
                    }) {
                        Ok(bin) => bin,
                        Err(why) => {
                            let _ = tx.unbounded_send(ServeProgress::Failed(format!(
                                "Couldn't install llama.cpp: {why}. Manual fallback: {}.",
                                llama_install_hint()
                            )));
                            return;
                        }
                    }
                }
            };
            let resolved =
                plan_v.or_else(|| spec_v.and_then(|s| plan_serve(&pick, &s, gpu_v.as_ref())));
            let Some(plan) = resolved else {
                let _ = tx.unbounded_send(ServeProgress::Failed(
                    "Couldn't list the repo's quants (offline?). Try again online.".into(),
                ));
                return;
            };
            // Hand off to the shared serve machinery (device pin, anti-OOM CPU fallback, single-
            // active stop-before-start, progress streaming, and — on success — the active-model
            // record). Same code the one-click re-serve uses.
            serve_target(bin, plan, move |p| {
                let _ = tx.unbounded_send(p);
            });
        });
    };

    // Re-serve the REMEMBERED model — no benchmark, no re-pick. The record carries the whole resolved
    // plan (repo, quant, size, fits_gpu), so this reconstructs it and hands it straight to the shared
    // serve machinery, installing llama.cpp first if the runtime is gone (a moved profile / fresh box).
    let run_reserve = move |plan: ServePlan| {
        serving.set(true);
        serve_status.set(Some(ServeProgress::Preparing));
        let tx = server_co.tx();
        std::thread::spawn(move || {
            // Still up on some port? Reconnect instead of reloading.
            if let Some(port) = running_server_port() {
                std::env::set_var("ZIQPU_LLM_URL", format!("http://127.0.0.1:{port}/v1"));
                let _ = tx.unbounded_send(ServeProgress::Serving(format!(
                    "Connected — model already loaded on :{port}. Switch the header toggle to Local."
                )));
                return;
            }
            let bin = match resolve_llama_server() {
                Some(bin) => bin,
                None => {
                    let txp = tx.clone();
                    match ensure_runtime(&mut |line| {
                        let _ = txp.unbounded_send(ServeProgress::Installing(line.to_string()));
                    }) {
                        Ok(bin) => bin,
                        Err(why) => {
                            let _ = tx.unbounded_send(ServeProgress::Failed(format!(
                                "Couldn't install llama.cpp: {why}. Manual fallback: {}.",
                                llama_install_hint()
                            )));
                            return;
                        }
                    }
                }
            };
            serve_target(bin, plan, move |p| {
                let _ = tx.unbounded_send(p);
            });
        });
    };

    // Stop the active local model — free its VRAM/RAM and forget it (owner's "idle sleep" ask, manual
    // form). Stops only OUR server (never someone else's), clears the record + the reconnect env, and
    // updates the panel so the remembered chip flips to "re-serve".
    let run_stop = move |()| {
        // Take the Send sender on the UI thread — the Coroutine itself is !Send and must not cross
        // the thread boundary.
        let tx = mount_probe.tx();
        std::thread::spawn(move || {
            stop_our_server(); // stops our PID + clears active_local.json
            std::env::remove_var("ZIQPU_LLM_URL");
            let _ = tx.unbounded_send((None, false));
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
    // Point the below-floor audience at the road that actually exists in THIS build: the built-in
    // free tier when it's baked in (this is exactly who it exists for), a key only otherwise.
    let nolocal_line = match &rec_now {
        Some(Recommendation::NoLocal { reason, .. }) => {
            let live_road = if crate::settings::built_in_available() {
                "Live (built-in free tier — no key needed)"
            } else {
                "Live (an API key)"
            };
            Some(format!(
                "No local model — {}. Use Raw (offline) or {live_road} instead.",
                reason.human()
            ))
        }
        _ => None,
    };
    let is_local = matches!(rec_now, Some(Recommendation::Local(_)));
    let plan_known = plan.read().is_some();
    // The tier emblem: the machine's wheat. A runnable tier wears its band color; below the floor
    // is wild wheat — it grows, but you can't harvest it.
    let tier_state = match &rec_now {
        Some(Recommendation::Local(p)) => Some(WheatTierState::Tier(p.tier)),
        Some(Recommendation::NoLocal { .. }) => Some(WheatTierState::Wild),
        None => None,
    };
    // The curated two-option dropdown: "#1 Stable" (the tier-correct pick) and — when the machine
    // can hold more — "#2 Max" (the biggest runnable model; bigger answers, slower tokens).
    let chose_max = *use_max.read();
    let stable_option = match &rec_now {
        Some(Recommendation::Local(p)) => Some(format!(
            "#1 Stable — {} ({}, ~{:.0} GB)",
            p.name, p.params, p.download_gb
        )),
        _ => None,
    };
    let max_option = (*max_pick.read()).map(|m| {
        format!(
            "#2 Max — {} ({}, ~{:.0} GB, slower — may run CPU-side)",
            m.name, m.params, m.download_gb
        )
    });
    // Both labels or no dropdown — a machine whose Max IS its Stable gets no fake choice.
    let choice_labels = stable_option.zip(max_option);
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
            ServeProgress::Installing(line) => (
                format!("Installing llama.cpp — {line}"),
                Some(WheatPhase::Loading),
                "prep",
            ),
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
    // `serve_view` is rendered ONCE at the top of the panel (so a re-serve of a remembered model —
    // which runs before any benchmark — still shows progress); the is_local block only shows its
    // button hints when no serve is in flight.
    let serve_idle = serve_view.is_none();
    // The remembered single active local model: what we last served + whether its server is still up.
    let remembered_now = remembered.read().clone();
    let remembered_live_now = *remembered_live.read();

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

            // The remembered single active local model — persists across restarts. Live → a
            // "reconnected" chip with a Stop (free the VRAM); dead → a one-click Re-serve of the
            // exact same model, no re-benchmark. Hidden while a serve is already in flight.
            if serve_idle {
                if let Some(rem) = remembered_now.clone() {
                    if remembered_live_now {
                        div { class: "modelpanel-active modelpanel-active--live",
                            span { class: "modelpanel-active-dot" }
                            span { class: "modelpanel-active-text",
                                "Local model ready — {rem.label()} on :{rem.port}"
                            }
                            button {
                                class: "settings-reveal",
                                r#type: "button",
                                title: "Stop this local model and free its VRAM/RAM",
                                onclick: move |_| {
                                    let f = run_stop;
                                    f(());
                                },
                                "stop"
                            }
                        }
                    } else {
                        div { class: "modelpanel-active",
                            span { class: "modelpanel-active-text",
                                "Last local model: {rem.label()} — not running"
                            }
                            button {
                                class: "btn btn--go modelpanel-btn",
                                r#type: "button",
                                disabled: serving_now,
                                onclick: move |_| {
                                    let plan = rem.to_plan();
                                    let mut f = run_reserve;
                                    f(plan);
                                },
                                "Re-serve"
                            }
                        }
                    }
                }
            }

            // Serve progress (benchmark serve OR re-serve) — shown here at the top so it's visible in
            // both paths. `None` wheat phase = a failure (carnelian text); `kind` colors the label.
            if let Some((label, wheat, kind)) = serve_view.clone() {
                div { class: "modelpanel-serve modelpanel-serve--{kind}",
                    p { class: "modelpanel-serve-label", "{label}" }
                    if let Some(phase) = wheat {
                        WheatLoader { phase }
                    }
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
                // The machine's wheat: tier band color for a runnable machine, wild for below-floor.
                div { class: "modelpanel-tier-row",
                    if let Some(state) = tier_state {
                        WheatTier { state }
                    }
                    div { class: "modelpanel-tier-lines",
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
                    }
                }
                if is_local {
                    // The curated choice — ONLY models this machine can hold ever appear here.
                    if let Some((stable_label, max_label)) = choice_labels {
                        label { class: "settings-field modelpanel-choice",
                            span { class: "settings-label", "Model to run" }
                            select {
                                class: "settings-input",
                                value: if chose_max { "max" } else { "stable" },
                                onchange: move |e| use_max.set(e.value() == "max"),
                                option { value: "stable", "{stable_label}" }
                                option { value: "max", "{max_label}" }
                            }
                        }
                    }
                    if is_cached && !chose_max {
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
                        } else if !srv {
                            "Install & serve locally"
                        } else if is_cached && !chose_max {
                            "Serve locally"
                        } else if chose_max {
                            // The cache check ran against the Stable plan, so for Max we don't
                            // KNOW whether a download is coming — and must not assert one.
                            "Serve locally (downloads if needed)"
                        } else {
                            "Download & serve locally"
                        }
                    }
                    // Progress renders once at the top of the panel; here we only explain the button
                    // when no serve is in flight.
                    if serve_idle {
                        if srv {
                            p { class: "settings-hint",
                                "llama.cpp detected. This downloads the fitted quant (first run) and serves it — or run `ziqpu-model serve` in a terminal."
                            }
                        } else {
                            p { class: "settings-hint",
                                "First serve installs the right llama.cpp build for your GPU automatically (one-time, no admin rights) — or install it yourself: {install_hint}."
                            }
                        }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The persistence contract: the active-model record survives a JSON round-trip AND reconstructs
    /// the exact resolved plan — crucially `fits_gpu`, so a re-serve after a restart keeps the
    /// anti-OOM CPU-side decision instead of forcing a too-big model onto the card again.
    #[test]
    fn active_local_round_trips_and_preserves_the_plan() {
        let plan = model::ServePlan {
            repo: "unsloth/gpt-oss-20b-GGUF".into(),
            quant: "Q6_K".into(),
            size_gb: 11.3,
            fits_gpu: false,
        };
        let a = ActiveLocal::from_plan(&plan, 1237);

        let json = serde_json::to_string(&a).unwrap();
        let back: ActiveLocal = serde_json::from_str(&json).unwrap();
        assert_eq!(back, a, "the record must survive a JSON round-trip");

        let p2 = back.to_plan();
        assert_eq!(p2.repo, plan.repo);
        assert_eq!(p2.quant, plan.quant);
        assert_eq!(p2.size_gb, plan.size_gb);
        assert!(
            !p2.fits_gpu,
            "the CPU-side (anti-OOM) decision must survive the restart"
        );
        assert_eq!(back.port, 1237);
        assert_eq!(back.label(), "unsloth/gpt-oss-20b-GGUF:Q6_K");
    }
}
