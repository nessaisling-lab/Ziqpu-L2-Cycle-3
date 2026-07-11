//! `ziqpu-model` — the local-model benchmark + (soon) fetch CLI. Increment 1 ships the two
//! read-only commands: `benchmark` (detect this machine → recommended tier + model) and `list`
//! (the whole tier→model table). Fetch/serve + the online layer-2 pick land in later increments.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("benchmark");
    match cmd {
        "benchmark" | "bench" => benchmark(),
        "list" => list(),
        "-h" | "--help" | "help" => usage(),
        other => {
            eprintln!("ziqpu-model: unknown command `{other}`.\n");
            usage();
            std::process::exit(2);
        }
    }
}

fn benchmark() {
    let spec = model::detect_spec();
    let tier = model::tier_for(&spec);
    let pick = model::recommend_for(&spec);
    let vram = spec
        .vram_gb
        .map(|v| format!("{v:.0} GB"))
        .unwrap_or_else(|| "unknown".to_string());

    println!("Ziqpu · local-model benchmark (layer 1: device capability)");
    println!("  RAM        {:.1} GB", spec.ram_gb);
    println!("  CPU cores  {}", spec.cores);
    println!("  GPU VRAM   {vram}");
    println!("  tier       {}", tier.label());
    if !model::meets_floor(&spec) {
        println!("  note       below the recommended 8 GB floor — the pick will run, but slowly");
    }
    println!(
        "  recommend  {} ({}, {}, ~{:.1} GB download)",
        pick.name, pick.params, pick.quant, pick.download_gb
    );
    println!("\n(layer 2 — the online best-for-this-agent pick — and `get`/`serve` land next.)");
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
    eprintln!("  benchmark   detect this machine and recommend a local model (default)");
    eprintln!("  list        show the tier -> model table");
}
