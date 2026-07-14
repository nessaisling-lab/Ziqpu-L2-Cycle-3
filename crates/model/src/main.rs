//! `ziqpu-model` — the local-model benchmark + fetch CLI. Commands: `benchmark` (detect this machine
//! → recommended local model, or "no local — use Raw/Live" below the floor), `list` (the tier→model
//! table), `resolve <term>` (current HF GGUF repos), `get` (resolve + check llama.cpp + print next
//! steps), `serve` (download + serve the model via llama-server on :1234, the app's Local default).
//! `--force-local` overrides a below-floor verdict with the tiny sub-floor model (guardrails hold).

use model::Recommendation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("benchmark");
    let force_local = args.iter().any(|a| a == "--force-local");
    match cmd {
        "benchmark" | "bench" => benchmark(force_local),
        "list" => list(),
        "resolve" => resolve(args.get(2).map(String::as_str)),
        "get" => get(force_local),
        "serve" => serve(force_local, port_arg(&args)),
        "-h" | "--help" | "help" => usage(),
        other => {
            eprintln!("ziqpu-model: unknown command `{other}`.\n");
            usage();
            std::process::exit(2);
        }
    }
}

fn benchmark(force_local: bool) {
    // Detect the GPU once; reuse it for both the spec (VRAM budget) and the display (its name).
    let gpu = model::detect_gpu();
    let spec = model::detect_spec_with(gpu.as_ref());
    let gpu_line = match &gpu {
        Some(g) if g.unified => format!("{} · unified memory", g.name),
        Some(g) => format!("{} · {:.0} GB VRAM", g.name, g.vram_gb),
        None => "none detected (CPU path)".to_string(),
    };

    println!("Ziqpu · local-model benchmark (layer 1: device capability)");
    println!("  RAM        {:.1} GB", spec.ram_gb);
    println!("  CPU cores  {}", spec.cores);
    println!("  GPU        {gpu_line}");

    // The pick to fetch for — a Local recommendation, or a forced sub-floor model, or nothing.
    let pick = match model::recommend_for(&spec) {
        Recommendation::Local(pick) => {
            println!("  tier       {}", pick.tier.label());
            println!(
                "  recommend  {} ({}, {}, ~{:.1} GB download)",
                pick.name, pick.params, pick.quant, pick.download_gb
            );
            Some(pick)
        }
        Recommendation::NoLocal { reason, fallback } => {
            println!("  tier       — (below the local-model floor)");
            println!("  no local   {} · use {}", reason.human(), fallback.label());
            if force_local {
                let sub = model::SUBFLOOR_PICK;
                println!(
                    "  forced     {} ({}, {}, ~{:.1} GB) — slower + lower quality; guardrails still enforced",
                    sub.name, sub.params, sub.quant, sub.download_gb
                );
                Some(sub)
            } else {
                println!(
                    "  (re-run with --force-local to grab the tiny {} anyway)",
                    model::SUBFLOOR_PICK.name
                );
                None
            }
        }
    };

    // Layer 2 (online, best-effort): only when there is a model to fetch for.
    match pick {
        Some(pick) => {
            println!("\nlayer 2 · current GGUF on Hugging Face (online):");
            match model::resolve_current_repo(&pick) {
                Some(c) => {
                    println!("  best repo  {} ({} downloads)", c.repo, c.downloads);
                    println!("  fetch      llama-server -hf {}:{}", c.repo, pick.quant);
                }
                None => {
                    println!("  (offline or none found — the static pick above is the fallback)")
                }
            }
        }
        None => {
            println!(
                "\n(no local model — the app uses Raw (offline) or Live (API); nothing to fetch.)"
            )
        }
    }
    println!(
        "\nNext: `ziqpu-model get` checks llama.cpp + your pick; `ziqpu-model serve` runs it on :1234."
    );
}

fn resolve(term: Option<&str>) {
    let Some(term) = term else {
        eprintln!("usage: ziqpu-model resolve <search-term>   (e.g. gpt-oss-20b)");
        std::process::exit(2);
    };
    let cands = model::resolve_candidates(term);
    if cands.is_empty() {
        println!("no GGUF repos found for `{term}` (offline, or no match).");
        return;
    }
    println!("Hugging Face GGUF repos for `{term}` (most-downloaded first):");
    for c in cands.iter().take(10) {
        println!(
            "  {:>10} dl · {:>5} likes · {}",
            c.downloads, c.likes, c.repo
        );
    }
}

fn list() {
    println!("Ziqpu · local-model tiers (Desktop class)");
    for m in model::all_models() {
        println!(
            "  {:<7} {:<24} {:<5} {:<7} ~{:.1} GB   (min {:.0} GB RAM)",
            m.tier.label(),
            m.name,
            m.params,
            m.quant,
            m.download_gb,
            m.min_ram_gb
        );
    }
    println!("  (below 8 GB RAM / 4 cores, or no AVX2 → no local model; use Raw or Live)");
}

/// The port for `serve` (`--port N`), defaulting to 1234 — the app's Local-mode default, so a served
/// model works with zero config.
fn port_arg(args: &[String]) -> u16 {
    args.iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|p| p.parse().ok())
        .unwrap_or(1234)
}

/// Detect this machine + resolve the recommended pick → `(pick, spec, gpu)`, honoring `--force-local`
/// below the floor (else prints the Raw/Live guidance and returns None). No network — the caller
/// resolves the concrete repo/quant. `spec`/`gpu` are returned so the caller can fit the quant to the
/// machine's memory budget (`plan_serve`) rather than serving the static pick's quant.
fn resolve_pick(
    force_local: bool,
) -> Option<(model::ModelPick, model::DeviceSpec, Option<model::GpuInfo>)> {
    let gpu = model::detect_gpu();
    let spec = model::detect_spec_with(gpu.as_ref());
    let pick = match model::recommend_for(&spec) {
        Recommendation::Local(p) => p,
        Recommendation::NoLocal { .. } if force_local => model::SUBFLOOR_PICK,
        Recommendation::NoLocal { reason, fallback } => {
            eprintln!(
                "no local model for this machine — {} · use {}",
                reason.human(),
                fallback.label()
            );
            eprintln!(
                "(re-run with --force-local to serve the tiny {} anyway)",
                model::SUBFLOOR_PICK.name
            );
            return None;
        }
    };
    Some((pick, spec, gpu))
}

/// Resolve the model this machine should serve → `(hf_repo, quant, human_label)` at the pick's
/// **static** quant (what `get` reports as "next steps"; `serve` fits the quant per machine instead).
/// Needs the network to resolve the current HF repo.
fn resolve_target(force_local: bool) -> Option<(String, String, String)> {
    let (pick, _spec, _gpu) = resolve_pick(force_local)?;
    let Some(cand) = model::resolve_current_repo(&pick) else {
        eprintln!(
            "couldn't resolve a current Hugging Face repo for {} (offline?)",
            pick.name
        );
        eprintln!(
            "try `ziqpu-model resolve {}` when online.",
            pick.search_term
        );
        return None;
    };
    Some((
        cand.repo,
        pick.quant.to_string(),
        format!("{} ({})", pick.name, pick.quant),
    ))
}

/// `get` — resolve the model + report whether llama.cpp is installed, and the exact next command.
fn get(force_local: bool) {
    let Some((repo, quant, label)) = resolve_target(force_local) else {
        std::process::exit(1);
    };
    println!("Model for this machine: {label}");
    println!("  HF repo   {repo}:{quant}");
    let flag = if force_local { " --force-local" } else { "" };
    if model::have_llama_server() {
        println!("  llama.cpp found ✓");
        println!("\nRun this to download + serve (leave it running):");
        println!("  ziqpu-model serve{flag}");
    } else {
        println!("  llama.cpp NOT found — install it first:");
        println!("    {}", model::llama_install_hint());
        println!("  then: ziqpu-model serve{flag}");
    }
}

/// `serve` — download (first run) + serve the model via `llama-server` on `--port` (default 1234, the
/// app's Local default). Blocks while serving; the seeker leaves it running and Ziqpu's Local mode
/// talks to it. No settings change needed at the default port.
fn serve(force_local: bool, port: u16) {
    let Some((pick, spec, gpu)) = resolve_pick(force_local) else {
        std::process::exit(1);
    };
    let Some(bin) = model::llama_server_path() else {
        eprintln!("llama.cpp not found. Install it, then re-run `ziqpu-model serve`:");
        eprintln!("  {}", model::llama_install_hint());
        std::process::exit(1);
    };
    // Fit the quant to this machine's memory budget (the largest-quality GGUF that fits, NOT the
    // static pick's quant — that hardcoded a quant a smaller card can't hold). Offline → static pick.
    let (repo, quant, size_gb) = match model::plan_serve(&pick, &spec, gpu.as_ref()) {
        Some(p) => (p.repo, p.quant, Some(p.size_gb)),
        None => {
            let Some(cand) = model::resolve_current_repo(&pick) else {
                eprintln!(
                    "couldn't resolve a current Hugging Face repo for {} (offline?)",
                    pick.name
                );
                eprintln!(
                    "try `ziqpu-model resolve {}` when online.",
                    pick.search_term
                );
                std::process::exit(1);
            };
            (cand.repo, pick.quant.to_string(), None)
        }
    };
    let hf = format!("{repo}:{quant}");
    println!("Ziqpu · serving {} ({quant})", pick.name);
    println!("  endpoint  http://127.0.0.1:{port}/v1   (Ziqpu Local mode's default is :1234)");
    match size_gb {
        Some(gb) => {
            println!("  model     {hf}  (~{gb:.1} GB — the best quant this machine can run)")
        }
        None => println!("  model     {hf}"),
    }
    println!(
        "  first run downloads the model from Hugging Face, then serves. Leave this running.\n"
    );
    let status = std::process::Command::new(&bin)
        .args([
            "-hf",
            &hf,
            "--host",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            // Cap the context (default 0 = the model's full 32k+ trained window, whose KV cache is
            // several GB) so weights + KV fit in VRAM — the real fix for the 16 GB card's OOM.
            "-c",
            &model::SERVE_CTX_SIZE.to_string(),
            "-lv",
            "1",
        ])
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => std::process::exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("failed to run llama-server: {e}");
            std::process::exit(1);
        }
    }
}

fn usage() {
    eprintln!("usage: ziqpu-model <command>");
    eprintln!("  benchmark [--force-local]   detect this machine and recommend a local model");
    eprintln!("  list                        show the tier -> model table");
    eprintln!(
        "  resolve <term>              list current Hugging Face GGUF repos for a search term"
    );
    eprintln!(
        "  get [--force-local]         resolve the model + check llama.cpp; print next steps"
    );
    eprintln!("  serve [--force-local] [--port N]  download + serve the model (default :1234)");
}
