//! `ziqpu-model` — the local-model benchmark + (soon) fetch CLI. Commands: `benchmark` (detect this
//! machine → recommended local model, or "no local — use Raw/Live" below the floor), `list` (the
//! tier→model table), `resolve <term>` (current HF GGUF repos). `benchmark --force-local` overrides a
//! below-floor verdict with the tiny sub-floor model (slower, lower quality; guardrails still hold).

use model::Recommendation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("benchmark");
    let force_local = args.iter().any(|a| a == "--force-local");
    match cmd {
        "benchmark" | "bench" => benchmark(force_local),
        "list" => list(),
        "resolve" => resolve(args.get(2).map(String::as_str)),
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
    println!("\n(`get`/`serve` — ensure llama.cpp, pull, and wire settings.json — land next.)");
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

fn usage() {
    eprintln!("usage: ziqpu-model <command>");
    eprintln!("  benchmark [--force-local]   detect this machine and recommend a local model");
    eprintln!("  list                        show the tier -> model table");
    eprintln!(
        "  resolve <term>              list current Hugging Face GGUF repos for a search term"
    );
}
