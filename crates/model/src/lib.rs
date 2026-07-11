//! Ziqpu local-model **benchmark + pick** — layer 1 of the two-layer selection (PRD nightfall N2,
//! feature 3). Given the machine's specs, choose the strongest local model that runs *pleasantly*
//! on it, using a committed port of `Ziqpu_Local_LLM_Hierarchy.md` (Desktop class).
//!
//! The tier logic here is **pure and dependency-free** — it takes a [`DeviceSpec`] and returns a
//! [`Tier`] / [`ModelPick`], so it is fully unit-tested against fixed specs. Only [`detect_spec`]
//! reads the real machine (via `sysinfo`); it is thin I/O, deliberately kept out of the tested core.
//!
//! Layer 2 (the online "best current GGUF for this agent on this silicon" check) and the fetch/serve
//! step build on top of this — this module answers only "what size fits, and what's the default pick".

/// The five capability tiers, weakest→strongest. The *same* tier name maps to different models
/// across device classes in the hierarchy; this crate implements the **Desktop** class (the desktop
/// agent's home), which also covers laptops running llama.cpp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    Low,
    Weak,
    Medium,
    Strong,
    Ultra,
}

impl Tier {
    pub fn label(self) -> &'static str {
        match self {
            Tier::Low => "Low",
            Tier::Weak => "Weak",
            Tier::Medium => "Medium",
            Tier::Strong => "Strong",
            Tier::Ultra => "Ultra",
        }
    }
}

/// A machine's measured capability. `disk_free_gb` / `vram_gb` are `Option` because they are not
/// always detectable — an unknown value is treated as "don't gate on it" for disk, and "no confirmed
/// GPU" for VRAM (which caps the pick at a CPU-runnable tier). RAM and cores are always known.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceSpec {
    pub ram_gb: f64,
    pub cores: u32,
    pub disk_free_gb: Option<f64>,
    pub vram_gb: Option<f64>,
}

/// One tier's hardware gate — a committed row of the hierarchy's Desktop threshold table.
struct Threshold {
    tier: Tier,
    min_ram_gb: f64,
    min_cores: u32,
    min_disk_gb: f64,
    min_vram_gb: f64,
    /// `true` only for tiers where a big GPU is the *only* path to usable speed (Strong/Ultra). For
    /// the lower tiers a CPU/hybrid path is fine, so VRAM is not gated.
    gpu_required: bool,
}

/// Reported RAM runs a few percent under nameplate (firmware / OS reserve): a 32 GB machine reads
/// ~31.3 GB, a 16 GB one ~15.7. Comparing against a hard nameplate minimum would drop such a machine
/// a whole tier, so RAM gates are met at `min * RAM_TOLERANCE`. VRAM and disk are reported accurately
/// and are not tolerated.
const RAM_TOLERANCE: f64 = 0.94;

/// Desktop-class thresholds, ported verbatim from `Ziqpu_Local_LLM_Hierarchy.md` (§Class 3). Ordered
/// strongest→weakest so [`tier_for`] can return the first (highest) tier a machine satisfies.
const DESKTOP_THRESHOLDS: [Threshold; 5] = [
    Threshold {
        tier: Tier::Ultra,
        min_ram_gb: 64.0,
        min_cores: 12,
        min_disk_gb: 130.0,
        min_vram_gb: 24.0,
        gpu_required: true,
    },
    Threshold {
        tier: Tier::Strong,
        min_ram_gb: 32.0,
        min_cores: 8,
        min_disk_gb: 50.0,
        min_vram_gb: 24.0,
        gpu_required: true,
    },
    Threshold {
        tier: Tier::Medium,
        min_ram_gb: 16.0,
        min_cores: 6,
        min_disk_gb: 30.0,
        min_vram_gb: 16.0,
        gpu_required: false,
    },
    Threshold {
        tier: Tier::Weak,
        min_ram_gb: 16.0,
        min_cores: 4,
        min_disk_gb: 16.0,
        min_vram_gb: 8.0,
        gpu_required: false,
    },
    Threshold {
        tier: Tier::Low,
        min_ram_gb: 8.0,
        min_cores: 4,
        min_disk_gb: 8.0,
        min_vram_gb: 0.0,
        gpu_required: false,
    },
];

impl Threshold {
    /// Does `spec` clear this tier's gate? RAM and cores are hard gates; disk is gated only when
    /// known (an unknown disk never blocks a pick); VRAM is a hard gate only for `gpu_required`
    /// tiers, and an unknown VRAM there means "no confirmed GPU" → the tier is refused.
    fn satisfied_by(&self, spec: &DeviceSpec) -> bool {
        let ram_ok = spec.ram_gb >= self.min_ram_gb * RAM_TOLERANCE;
        let cores_ok = spec.cores >= self.min_cores;
        let disk_ok = spec
            .disk_free_gb
            .map(|d| d >= self.min_disk_gb)
            .unwrap_or(true);
        let vram_ok = if self.gpu_required {
            spec.vram_gb.map(|v| v >= self.min_vram_gb).unwrap_or(false)
        } else {
            true
        };
        ram_ok && cores_ok && disk_ok && vram_ok
    }
}

/// The highest tier the machine can run pleasantly. If the machine is below even the Low floor
/// (< 8 GB), we still return [`Tier::Low`] — the smallest model is the honest best-effort — rather
/// than refuse outright. Callers can compare `spec.ram_gb` against the Low minimum to warn.
pub fn tier_for(spec: &DeviceSpec) -> Tier {
    DESKTOP_THRESHOLDS
        .iter()
        .find(|t| t.satisfied_by(spec))
        .map(|t| t.tier)
        .unwrap_or(Tier::Low)
}

/// Whether the machine clears the Low floor (the weakest tier's hard gate). `false` means the pick
/// is a below-spec best effort and readings may be slow.
pub fn meets_floor(spec: &DeviceSpec) -> bool {
    DESKTOP_THRESHOLDS
        .iter()
        .find(|t| t.tier == Tier::Low)
        .map(|t| t.satisfied_by(spec))
        .unwrap_or(false)
}

/// The default model for a tier — a static pick from the hierarchy's Desktop table. Layer 2 may
/// refine the concrete GGUF repo online; this is the offline-safe fallback and the human-readable
/// name shown in the benchmark. (`params`/`quant`/`download_gb` mirror the hierarchy verbatim.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelPick {
    pub tier: Tier,
    pub name: &'static str,
    pub params: &'static str,
    pub quant: &'static str,
    pub download_gb: f64,
    pub min_ram_gb: f64,
    /// A Hugging Face search term for layer 2 to resolve this pick to a *current* GGUF repo. Kept
    /// hyphenated/space-free so it needs no URL encoding, and deliberately a little fuzzy — layer 2
    /// finds whatever GGUF repos actually exist for this model family today (releases drift).
    pub search_term: &'static str,
}

/// The Desktop-class model per tier (hierarchy §Class 3 recommendations).
const DESKTOP_MODELS: [ModelPick; 5] = [
    ModelPick {
        tier: Tier::Low,
        name: "Llama 3.2 3B Instruct",
        params: "3B",
        quant: "Q4_K_M",
        download_gb: 2.0,
        min_ram_gb: 8.0,
        search_term: "Llama-3.2-3B-Instruct",
    },
    ModelPick {
        tier: Tier::Weak,
        name: "Qwen3.5 9B Instruct",
        params: "9B",
        quant: "Q4_K_M",
        download_gb: 6.6,
        min_ram_gb: 16.0,
        search_term: "Qwen3.5-9B-Instruct",
    },
    ModelPick {
        tier: Tier::Medium,
        name: "GPT-OSS 20B",
        params: "20B",
        quant: "MXFP4",
        download_gb: 14.0,
        min_ram_gb: 16.0,
        search_term: "gpt-oss-20b",
    },
    ModelPick {
        tier: Tier::Strong,
        name: "Qwen3.5 35B-A3B",
        params: "35B",
        quant: "Q4_K_M",
        download_gb: 24.0,
        min_ram_gb: 32.0,
        search_term: "Qwen3.5-35B",
    },
    ModelPick {
        tier: Tier::Ultra,
        name: "GPT-OSS 120B",
        params: "120B",
        quant: "MXFP4",
        download_gb: 65.0,
        min_ram_gb: 64.0,
        search_term: "gpt-oss-120b",
    },
];

/// The whole tier→model table, weakest→strongest — for a `list` view and for callers offering the
/// "download a different model" affordance.
pub fn all_models() -> &'static [ModelPick] {
    &DESKTOP_MODELS
}

/// The default model pick for a machine — its detected tier's model.
pub fn recommend_for(spec: &DeviceSpec) -> ModelPick {
    let tier = tier_for(spec);
    DESKTOP_MODELS
        .iter()
        .find(|m| m.tier == tier)
        .copied()
        .expect("every tier has a model")
}

// ---- Layer 2: resolve the tier's pick to a *current* Hugging Face GGUF repo (online) ----

/// A candidate GGUF repo found on the Hugging Face Hub — its repo id and popularity signals. Layer 2
/// exists because the static hierarchy drifts: this finds what actually exists on the Hub today.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub repo: String,
    pub downloads: u64,
    pub likes: u64,
}

/// The Hub API query for GGUF repos matching `term`, most-downloaded first. `filter=gguf` limits to
/// llama.cpp-runnable repos; the term is space-free (see [`ModelPick::search_term`]) so it needs no
/// real encoding.
fn hf_api_url(term: &str) -> String {
    format!(
        "https://huggingface.co/api/models?search={}&filter=gguf&sort=downloads&direction=-1&limit=20",
        term.replace(' ', "%20")
    )
}

/// Parse a Hugging Face `/api/models` response into ranked [`Candidate`]s (most-downloaded first).
/// Pure; unit-tested against a fixture. Malformed/empty JSON yields an empty list (offline-safe —
/// never panics).
pub fn parse_hf_models(json: &str) -> Vec<Candidate> {
    let arr: Vec<serde_json::Value> = serde_json::from_str(json).unwrap_or_default();
    let mut cands: Vec<Candidate> = arr
        .iter()
        .filter_map(|v| {
            let repo = v.get("id").and_then(|x| x.as_str())?.to_string();
            Some(Candidate {
                repo,
                downloads: v.get("downloads").and_then(|x| x.as_u64()).unwrap_or(0),
                likes: v.get("likes").and_then(|x| x.as_u64()).unwrap_or(0),
            })
        })
        .collect();
    cands.sort_by(|a, b| b.downloads.cmp(&a.downloads).then(b.likes.cmp(&a.likes)));
    cands
}

/// Query the Hub for current GGUF repos matching `term` — **thin I/O, not unit-tested** (the parser
/// is). HTTP is a `curl` subprocess (the repo's cross-platform, no-HTTP-crate convention), capped at
/// 8 s so an offline machine degrades quietly to an empty list. Callers fall back to the static pick.
pub fn resolve_candidates(term: &str) -> Vec<Candidate> {
    use std::process::Command;
    let url = hf_api_url(term);
    let Ok(out) = Command::new("curl")
        .args(["-sS", "--max-time", "8", &url])
        .output()
    else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    parse_hf_models(&String::from_utf8_lossy(&out.stdout))
}

/// Repo markers that disqualify a base model for a **guarded** agent. Ungasaga must refuse advice
/// and stay within its guardrails; an "abliterated" / "uncensored" / jailbroken quant is trained to
/// do the opposite, so it's the wrong foundation regardless of its download count. Matched
/// case-insensitively against the repo id. This is the "best for *our* agent" half of layer 2.
const AGENT_DISQUALIFIERS: [&str; 9] = [
    "abliterat",
    "uncensored",
    "derestrict",
    "heretic",
    "jailbreak",
    "unhinged",
    "nsfw",
    "erotic",
    "roleplay",
];

/// Whether a repo id carries a guardrail-hostile marker (see [`AGENT_DISQUALIFIERS`]).
fn agent_disqualified(repo: &str) -> bool {
    let lower = repo.to_ascii_lowercase();
    AGENT_DISQUALIFIERS.iter().any(|m| lower.contains(m))
}

/// Choose the best **agent-appropriate** candidate from a download-ranked list: the most-downloaded
/// repo that isn't a guardrail-hostile variant. Pure + unit-tested. If *every* candidate is
/// disqualified (pathological), falls back to the top raw candidate rather than returning nothing.
pub fn pick_for_agent(cands: &[Candidate]) -> Option<Candidate> {
    cands
        .iter()
        .find(|c| !agent_disqualified(&c.repo))
        .or_else(|| cands.first())
        .cloned()
}

/// Layer 2's answer for one machine: the current best **agent-appropriate** GGUF repo for the tier's
/// model, or `None` offline (the caller then uses the static [`ModelPick`]). "Best" = the
/// most-downloaded match that respects the agent's guardrails.
pub fn resolve_current_repo(pick: &ModelPick) -> Option<Candidate> {
    pick_for_agent(&resolve_candidates(pick.search_term))
}

/// A detected GPU: name + video memory. `unified` marks Apple-Silicon-style shared memory, where the
/// GPU draws from system RAM rather than a dedicated pool — the caller then uses RAM as the VRAM
/// budget instead of a (nonexistent) discrete figure.
#[derive(Debug, Clone, PartialEq)]
pub struct GpuInfo {
    pub name: String,
    pub vram_gb: f64,
    pub unified: bool,
}

/// Parse `nvidia-smi --query-gpu=name,memory.total --format=csv,noheader,nounits` — one GPU per line
/// as `name, <MiB>`. Returns the highest-VRAM GPU (the discrete one on a hybrid laptop). Pure;
/// unit-tested. A GPU name has spaces but no commas, so we split on the **last** comma.
fn parse_nvidia_smi(out: &str) -> Option<GpuInfo> {
    out.lines()
        .filter_map(|line| {
            let (name, mem) = line.rsplit_once(',')?;
            let mib: f64 = mem.trim().parse().ok()?;
            Some(GpuInfo {
                name: name.trim().to_string(),
                vram_gb: mib / 1024.0,
                unified: false,
            })
        })
        .max_by(|a, b| a.vram_gb.total_cmp(&b.vram_gb))
}

/// Parse `system_profiler SPDisplaysDataType` (macOS). Apple Silicon (`Chipset Model: Apple M…`)
/// reports no discrete VRAM — it shares system memory, so we flag `unified` and leave the figure for
/// the caller to fill from RAM. Intel / discrete Macs carry a `VRAM (Total): N GB|MB` line. Pure;
/// unit-tested; best-effort (the Mac partner validates on real hardware).
fn parse_macos_gpu(out: &str) -> Option<GpuInfo> {
    let chipset = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("Chipset Model:"))
        .map(str::trim);
    let name = chipset.unwrap_or("GPU").to_string();
    if chipset.map(|c| c.starts_with("Apple M")).unwrap_or(false) {
        return Some(GpuInfo {
            name,
            vram_gb: 0.0,
            unified: true,
        });
    }
    // Discrete / Intel: read "VRAM (Total): N GB" (or MB).
    let vram_gb = out.lines().find_map(|l| {
        let v = l.trim().strip_prefix("VRAM (Total):")?.trim();
        let (num, unit) = v.split_once(' ')?;
        let n: f64 = num.trim().parse().ok()?;
        Some(if unit.trim().eq_ignore_ascii_case("MB") {
            n / 1024.0
        } else {
            n
        })
    })?;
    Some(GpuInfo {
        name,
        vram_gb,
        unified: false,
    })
}

/// Parse the Windows video-adapter registry dump — one `DriverDesc|qwMemorySize` line per adapter,
/// VRAM in bytes. This is the **no-admin** path for when `nvidia-smi` is locked down (common on
/// hybrid laptops). Returns the highest-VRAM adapter (the discrete GPU over the iGPU); lines with no
/// size are skipped. Pure; unit-tested. `qwMemorySize` is the true 64-bit VRAM, unlike WMI's
/// `AdapterRAM`, which is a 32-bit field that caps at ~4 GB.
fn parse_windows_registry_gpu(out: &str) -> Option<GpuInfo> {
    out.lines()
        .filter_map(|line| {
            let (name, bytes) = line.rsplit_once('|')?;
            let b: f64 = bytes.trim().parse().ok()?;
            if b <= 0.0 {
                return None;
            }
            Some(GpuInfo {
                name: name.trim().to_string(),
                vram_gb: b / (1024.0 * 1024.0 * 1024.0),
                unified: false,
            })
        })
        .max_by(|a, b| a.vram_gb.total_cmp(&b.vram_gb))
}

/// Detect the machine's GPU — **thin I/O, not unit-tested** (the parsers above are). Order: try
/// `nvidia-smi` (most accurate, but locked to admin on some laptops); on Windows fall back to the
/// driver registry (no admin); on macOS fall back to `system_profiler` (Apple Silicon / discrete).
/// Returns `None` when no probe succeeds — the tier logic then treats VRAM as unknown and stays on a
/// CPU-runnable pick.
pub fn detect_gpu() -> Option<GpuInfo> {
    use std::process::Command;
    if let Ok(out) = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
    {
        if out.status.success() {
            if let Some(g) = parse_nvidia_smi(&String::from_utf8_lossy(&out.stdout)) {
                return Some(g);
            }
        }
    }
    if cfg!(target_os = "windows") {
        // The video-adapter class key; `qwMemorySize` is the real VRAM in bytes.
        let script = r#"Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*' -ErrorAction SilentlyContinue | ForEach-Object { "$($_.DriverDesc)|$($_.'HardwareInformation.qwMemorySize')" }"#;
        if let Ok(out) = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .output()
        {
            // PowerShell can exit non-zero on a non-terminating property error while still emitting
            // valid lines, so parse stdout regardless of the exit code.
            if let Some(g) = parse_windows_registry_gpu(&String::from_utf8_lossy(&out.stdout)) {
                return Some(g);
            }
        }
    }
    if cfg!(target_os = "macos") {
        if let Ok(out) = Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .output()
        {
            if out.status.success() {
                if let Some(g) = parse_macos_gpu(&String::from_utf8_lossy(&out.stdout)) {
                    return Some(g);
                }
            }
        }
    }
    None
}

/// Detect the real machine's capability. **Thin I/O, not unit-tested** — RAM via `sysinfo`, cores via
/// the std runtime hint, GPU via [`detect_gpu`]. Disk-free stays `None` for now (never blocks a pick).
pub fn detect_spec() -> DeviceSpec {
    detect_spec_with(detect_gpu().as_ref())
}

/// Build a [`DeviceSpec`] from an already-detected GPU, so a caller that also wants the GPU's *name*
/// can detect once and reuse it (the CLI does). A unified-memory GPU (Apple Silicon) uses RAM as its
/// VRAM budget; a discrete GPU uses its own VRAM; no GPU leaves VRAM `None` (caps at a CPU tier).
pub fn detect_spec_with(gpu: Option<&GpuInfo>) -> DeviceSpec {
    use sysinfo::System;
    let mut sys = System::new();
    sys.refresh_memory();
    // sysinfo 0.30+ reports memory in bytes.
    let ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let cores = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    let vram_gb = gpu.map(|g| if g.unified { ram_gb } else { g.vram_gb });
    DeviceSpec {
        ram_gb,
        cores,
        disk_free_gb: None,
        vram_gb,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(ram: f64, cores: u32, disk: Option<f64>, vram: Option<f64>) -> DeviceSpec {
        DeviceSpec {
            ram_gb: ram,
            cores,
            disk_free_gb: disk,
            vram_gb: vram,
        }
    }

    #[test]
    fn a_basic_office_box_lands_on_low() {
        // 8 GB / 4 cores, no GPU → Low (Llama 3.2 3B).
        let s = spec(8.0, 4, Some(50.0), None);
        assert_eq!(tier_for(&s), Tier::Low);
        assert_eq!(recommend_for(&s).name, "Llama 3.2 3B Instruct");
        assert!(meets_floor(&s));
    }

    #[test]
    fn sixteen_gigs_no_gpu_reaches_medium_not_strong() {
        // 16 GB / 6 cores, no confirmed GPU → Medium (CPU-runnable GPT-OSS 20B), never Strong.
        let s = spec(16.0, 6, Some(60.0), None);
        assert_eq!(tier_for(&s), Tier::Medium);
    }

    #[test]
    fn strong_requires_a_confirmed_gpu() {
        // 32 GB / 8 cores but VRAM unknown → capped at Medium (Strong is gpu_required).
        assert_eq!(tier_for(&spec(32.0, 8, Some(80.0), None)), Tier::Medium);
        // Same box with a 24 GB GPU → Strong.
        assert_eq!(
            tier_for(&spec(32.0, 8, Some(80.0), Some(24.0))),
            Tier::Strong
        );
    }

    #[test]
    fn an_enthusiast_build_reaches_ultra() {
        let s = spec(128.0, 16, Some(400.0), Some(32.0));
        assert_eq!(tier_for(&s), Tier::Ultra);
        assert_eq!(recommend_for(&s).name, "GPT-OSS 120B");
    }

    #[test]
    fn below_floor_still_returns_low_but_flags_it() {
        // A 4 GB / 2-core machine is below even Low; we still recommend the smallest model, honestly
        // flagged as below the floor.
        let s = spec(4.0, 2, Some(20.0), None);
        assert_eq!(tier_for(&s), Tier::Low);
        assert!(!meets_floor(&s));
    }

    #[test]
    fn an_unknown_disk_never_blocks_a_pick() {
        // Disk None must not gate — a 32 GB GPU box with unknown disk still reaches Strong.
        assert_eq!(tier_for(&spec(32.0, 8, None, Some(24.0))), Tier::Strong);
    }

    #[test]
    fn more_ram_never_lowers_the_tier() {
        // Monotonicity: sweeping RAM up (GPU fixed generous) never demotes the tier.
        let mut last = Tier::Low;
        for ram in [8.0, 12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0, 128.0] {
            let t = tier_for(&spec(ram, 16, Some(400.0), Some(32.0)));
            assert!(t >= last, "tier dropped at {ram} GB: {last:?} -> {t:?}");
            last = t;
        }
    }

    #[test]
    fn every_tier_has_exactly_one_model() {
        for tier in [
            Tier::Low,
            Tier::Weak,
            Tier::Medium,
            Tier::Strong,
            Tier::Ultra,
        ] {
            let n = all_models().iter().filter(|m| m.tier == tier).count();
            assert_eq!(n, 1, "tier {tier:?} should have exactly one model");
        }
    }

    #[test]
    fn reported_ram_just_under_nameplate_keeps_its_tier() {
        // A 32 GB box reads ~31.3 GB and a 16 GB one ~15.7: the tolerance must not drop a rung.
        assert_eq!(
            tier_for(&spec(31.3, 8, Some(80.0), Some(24.0))),
            Tier::Strong
        );
        assert_eq!(
            tier_for(&spec(15.7, 6, Some(60.0), Some(16.0))),
            Tier::Medium
        );
        // But a genuinely-smaller machine still falls: 12 GB is not a tolerated 16.
        assert_ne!(
            tier_for(&spec(12.0, 6, Some(60.0), Some(16.0))),
            Tier::Medium
        );
    }

    #[test]
    fn nvidia_smi_parses_name_and_vram_and_picks_the_biggest() {
        let one = parse_nvidia_smi("NVIDIA GeForce RTX 5080 Laptop GPU, 16384").unwrap();
        assert_eq!(one.name, "NVIDIA GeForce RTX 5080 Laptop GPU");
        assert_eq!(one.vram_gb, 16.0);
        assert!(!one.unified);
        // Two GPUs → the higher-VRAM one wins.
        let two = parse_nvidia_smi("NVIDIA A, 8192\nNVIDIA B, 24576").unwrap();
        assert_eq!(two.vram_gb, 24.0);
        assert!(parse_nvidia_smi("garbage without a comma").is_none());
    }

    #[test]
    fn macos_gpu_parses_apple_silicon_and_discrete() {
        let apple = parse_macos_gpu("      Chipset Model: Apple M3 Max\n      Type: GPU").unwrap();
        assert_eq!(apple.name, "Apple M3 Max");
        assert!(apple.unified, "Apple Silicon shares system memory");
        let discrete =
            parse_macos_gpu("Chipset Model: AMD Radeon Pro 5500M\nVRAM (Total): 8 GB").unwrap();
        assert_eq!(discrete.vram_gb, 8.0);
        assert!(!discrete.unified);
        // MB units convert to GB.
        let mb = parse_macos_gpu("Chipset Model: Intel Iris\nVRAM (Total): 1536 MB").unwrap();
        assert_eq!(mb.vram_gb, 1.5);
    }

    #[test]
    fn windows_registry_picks_the_discrete_gpu_over_the_igpu() {
        // A real hybrid-laptop dump: AMD iGPU (0.5 GB) + NVIDIA discrete (~16 GB) → the NVIDIA wins.
        let out =
            "AMD Radeon(TM) 890M Graphics|536870912\nNVIDIA GeForce RTX 5080 Laptop GPU|17094934528";
        let g = parse_windows_registry_gpu(out).unwrap();
        assert_eq!(g.name, "NVIDIA GeForce RTX 5080 Laptop GPU");
        assert!((g.vram_gb - 15.92).abs() < 0.05, "vram was {}", g.vram_gb);
        assert!(!g.unified);
        // Adapters with no size (basic display drivers) are skipped.
        assert!(parse_windows_registry_gpu("Microsoft Basic Render Driver|").is_none());
    }

    #[test]
    fn hf_models_parse_and_rank_by_downloads() {
        let json = r#"[
            {"id":"bartowski/gpt-oss-20b-GGUF","downloads":45000,"likes":120},
            {"id":"ggml-org/gpt-oss-20b-GGUF","downloads":90000,"likes":300},
            {"id":"someone/unrelated","downloads":10,"likes":1}
        ]"#;
        let c = parse_hf_models(json);
        assert_eq!(c.len(), 3);
        assert_eq!(
            c[0].repo, "ggml-org/gpt-oss-20b-GGUF",
            "most-downloaded ranks first"
        );
        assert_eq!(c[0].downloads, 90000);
        // Malformed / empty JSON → empty, never panics (the offline-safe contract).
        assert!(parse_hf_models("not json").is_empty());
        assert!(parse_hf_models("").is_empty());
    }

    #[test]
    fn hf_api_url_targets_gguf_sorted_by_downloads() {
        let u = hf_api_url("gpt-oss-20b");
        assert!(u.contains("/api/models"), "{u}");
        assert!(u.contains("search=gpt-oss-20b"), "{u}");
        assert!(u.contains("filter=gguf"), "{u}");
        assert!(u.contains("sort=downloads"), "{u}");
    }

    #[test]
    fn agent_pick_skips_guardrail_hostile_variants() {
        fn cand(repo: &str, dl: u64) -> Candidate {
            Candidate {
                repo: repo.to_string(),
                downloads: dl,
                likes: 0,
            }
        }
        // Even when an abliterated/uncensored variant out-downloads the clean quant, the agent pick
        // skips it — a guarded agent needs a guardrail-respecting base.
        let ranked = vec![
            cand("someone/gpt-oss-20b-abliterated-GGUF", 900_000),
            cand("cruel/gpt-oss-20b-Uncensored-GGUF", 500_000),
            cand("unsloth/gpt-oss-20b-GGUF", 480_000),
            cand("bartowski/gpt-oss-20b-GGUF", 8_000),
        ];
        let pick = pick_for_agent(&ranked).unwrap();
        assert_eq!(pick.repo, "unsloth/gpt-oss-20b-GGUF");
        // Individual markers are caught case-insensitively.
        assert!(agent_disqualified("x/Model-ABLITERATED"));
        assert!(agent_disqualified("x/model-heretic-neo"));
        assert!(!agent_disqualified("unsloth/gpt-oss-20b-GGUF"));
        // Pathological all-disqualified list still yields the top raw candidate, not nothing.
        let all_bad = vec![cand("a/uncensored", 5), cand("b/abliterated", 3)];
        assert_eq!(pick_for_agent(&all_bad).unwrap().repo, "a/uncensored");
        assert!(pick_for_agent(&[]).is_none());
    }
}
