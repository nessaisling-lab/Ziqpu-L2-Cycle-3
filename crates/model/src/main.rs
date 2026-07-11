//! `ziqpu-model` — the local-model benchmark + (soon) fetch CLI. Increment 1 ships the two
//! read-only commands: `benchmark` (detect this machine → recommended tier + model) and `list`
//! (the whole tier→model table). Fetch/serve + the online layer-2 pick land in later increments.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("benchmark");
    match cmd {
        "benchmark" | "bench" => benchmark(),
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

fn benchmark() {
    // Detect the GPU once; reuse it for both the spec (VRAM budget) and the display (its name).
    let gpu = model::detect_gpu();
    let spec = model::detect_spec_with(gpu.as_ref());
    let tier = model::tier_for(&spec);
    let pick = model::recommend_for(&spec);
    let gpu_line = match &gpu {
        Some(g) if g.unified => format!("{} · unified memory", g.name),
        Some(g) => format!("{} · {:.0} GB VRAM", g.name, g.vram_gb),
        None => "none detected (CPU path)".to_string(),
    };

    println!("Ziqpu · local-model benchmark (layer 1: device capability)");
    println!("  RAM        {:.1} GB", spec.ram_gb);
    println!("  CPU cores  {}", spec.cores);
    println!("  GPU        {gpu_line}");
    println!("  tier       {}", tier.label());
    if !model::meets_floor(&spec) {
        println!("  note       below the recommended 8 GB floor — the pick will run, but slowly");
    }
    println!(
        "  recommend  {} ({}, {}, ~{:.1} GB download)",
        pick.name, pick.params, pick.quant, pick.download_gb
    );

    // Layer 2 (online, best-effort): resolve the tier's model to a current HF GGUF repo. Degrades
    // quietly to the static pick above when offline.
    println!("\nlayer 2 · current GGUF on Hugging Face (online):");
    match model::resolve_current_repo(&pick) {
        Some(c) => {
            println!("  best repo  {} ({} downloads)", c.repo, c.downloads);
            println!("  fetch      llama-server -hf {}:{}", c.repo, pick.quant);
        }
        None => println!("  (offline or none found — the static pick above is the fallback)"),
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
}

fn usage() {
    eprintln!("usage: ziqpu-model <command>");
    eprintln!("  benchmark        detect this machine and recommend a local model (default)");
    eprintln!("  list             show the tier -> model table");
    eprintln!("  resolve <term>   list current Hugging Face GGUF repos for a search term");
}
