//! Ziqpu local-model **benchmark + pick** — layer 1 of the two-layer selection (PRD nightfall N2,
//! feature 3). Given the machine's specs, either recommend the strongest local model that runs
//! *pleasantly* on it, or — below the minimum-viable floor — return [`Recommendation::NoLocal`], which
//! routes the app to Raw (offline engine template) or Live (hosted API). Based on a committed port of
//! `Ziqpu_Local_LLM_Hierarchy.md` (Desktop class; laptops run the same table — llama.cpp is OS-agnostic).
//!
//! The **capability envelope** is explicit. The *floor* ([`tier_for`] → `None`) is the true minimum to
//! run any useful local model (≥ 8 GB RAM, ≥ 4 cores, AVX2 on x86); below it there is no local model.
//! The *ceiling* ([`Tier::Ultra`]) is workstation-class (RTX 5090 32 GB + 128 GB+ RAM). A machine that
//! *gates* a tier but can't *hold* that tier's model in RAM drops to the largest that fits (see
//! [`recommend_for`]) — so a big unified-memory laptop is never capped, and a 64 GB Mac that clears
//! Ultra's gates but can't fit the 65 GB 120B correctly lands on Strong. Mobile (~16 GB unified) is a
//! future N4 class, out of this Desktop crate.
//!
//! The tier logic is **pure and dependency-free** — it takes a [`DeviceSpec`] and returns a
//! [`Recommendation`], fully unit-tested against fixed specs (using *reported*, not nominal, values —
//! RAM/VRAM read a few percent under nameplate, so both gates carry a tolerance). Only [`detect_spec`]
//! / [`detect_gpu`] read the real machine; that thin I/O is kept out of the tested core. Layer 2 (the
//! online best-GGUF check) and the fetch/serve step build on top.

/// Spawn a subprocess without flashing a console window on Windows (CREATE_NO_WINDOW). No-op
/// elsewhere. Wrap every `Command::new(...)` this crate spawns from the GUI so a windowless release
/// build stays windowless (the CLI binary `model/src/main.rs` intentionally does NOT use it). Two
/// cfg'd defs keep it warning-clean on non-Windows.
#[cfg(windows)]
pub(crate) fn no_window(mut cmd: std::process::Command) -> std::process::Command {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    cmd
}
#[cfg(not(windows))]
pub(crate) fn no_window(cmd: std::process::Command) -> std::process::Command {
    cmd
}

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
    /// Whether the CPU has AVX2. `Some(false)` = an x86 CPU without AVX2 → llama.cpp runs unbearably
    /// slowly → **below the floor**. `None` = non-x86 (Apple Silicon / ARM don't need AVX2) or
    /// undetected → never gates. Only `Some(false)` blocks.
    pub avx2: Option<bool>,
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

/// VRAM is reported slightly under nameplate too (a 24 GB card reads ~23.99 GB, a 32 GB one ~31.75),
/// so the VRAM gate gets the same tolerance — otherwise the Ultra gate would reject the very RTX 5090
/// (32 GB) it targets, and Strong would reject a 24 GB 4090.
const VRAM_TOLERANCE: f64 = 0.94;

/// The fraction of RAM a model may occupy and still leave room for the OS + KV cache. A model whose
/// download size exceeds `ram_gb * FIT_FRACTION` can't be held in memory (weights live in RAM even on
/// the GPU-offload hybrid path), so the pick drops to the largest model that does fit.
const FIT_FRACTION: f64 = 0.8;

/// Desktop-class thresholds, ported verbatim from `Ziqpu_Local_LLM_Hierarchy.md` (§Class 3). Ordered
/// strongest→weakest so [`tier_for`] can return the first (highest) tier a machine satisfies.
const DESKTOP_THRESHOLDS: [Threshold; 5] = [
    // Ultra = the workstation ceiling: RTX 5090 (32 GB GDDR7) + 128-256 GB DDR5. Not a routine rung.
    Threshold {
        tier: Tier::Ultra,
        min_ram_gb: 64.0,
        min_cores: 12,
        min_disk_gb: 130.0,
        min_vram_gb: 32.0,
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
            spec.vram_gb
                .map(|v| v >= self.min_vram_gb * VRAM_TOLERANCE)
                .unwrap_or(false)
        } else {
            true
        };
        ram_ok && cores_ok && disk_ok && vram_ok
    }
}

/// The highest tier whose gates the machine clears, or **`None` when it is below the floor** — no
/// tier's minimum is met, or the CPU is x86-without-AVX2 (llama.cpp can't run at a tolerable speed).
/// A `None` means "no local model": the caller routes to Raw (offline template) or Live (hosted API).
pub fn tier_for(spec: &DeviceSpec) -> Option<Tier> {
    // An x86 CPU without AVX2 is below the floor regardless of RAM. This gate lives *here*, in the
    // tier decision (not only in the reason-reporter), so exclusion and reporting agree. Apple
    // Silicon / ARM report `avx2 == None` and are never blocked.
    if spec.avx2 == Some(false) {
        return None;
    }
    DESKTOP_THRESHOLDS
        .iter()
        .find(|t| t.satisfied_by(spec))
        .map(|t| t.tier)
}

/// Whether the machine can run *any* local model (clears the floor). `false` → no local; use Raw/Live.
pub fn meets_floor(spec: &DeviceSpec) -> bool {
    tier_for(spec).is_some()
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
        name: "Gemma 3 4B Instruct",
        params: "4B",
        quant: "Q4_K_M",
        download_gb: 3.0,
        min_ram_gb: 8.0,
        search_term: "gemma-3-4b-it",
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
    // Medium = a 16 GB dedicated-VRAM card (e.g. RTX 5080 Laptop). A DENSE 14B at Q4_K_M (~9 GB) fits
    // with real headroom for the KV cache + compute (a 20B's ~12 GB quants left almost none and OOM'd),
    // and Qwen3-14B has a strong native tool-calling template — the load-bearing capability for the
    // reading agent. Gemma 3 4B (the Low pick) is the safe fallback if a machine struggles with 14B.
    ModelPick {
        tier: Tier::Medium,
        name: "Qwen3 14B",
        params: "14B",
        quant: "Q4_K_M",
        download_gb: 9.0,
        min_ram_gb: 16.0,
        search_term: "Qwen3-14B",
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
        quant: "Q4_K_M",
        download_gb: 65.0,
        min_ram_gb: 64.0,
        search_term: "gpt-oss-120b",
    },
];

/// The sub-floor fallback — a tiny 3B for a below-floor machine whose user *forces* local anyway
/// (`--force-local`). Smaller and rougher than the Low pick; **never returned by [`recommend_for`]**
/// (a below-floor machine gets [`Recommendation::NoLocal`] by default). Ziqpu's no-advice guardrail
/// is enforced in code regardless of model, so this is safe — just slower and lower quality.
pub const SUBFLOOR_PICK: ModelPick = ModelPick {
    tier: Tier::Low,
    name: "Llama 3.2 3B Instruct",
    params: "3B",
    quant: "Q4_K_M",
    download_gb: 2.0,
    min_ram_gb: 4.0,
    search_term: "Llama-3.2-3B-Instruct",
};

/// The whole tier→model table, weakest→strongest — for a `list` view and for callers offering the
/// "download a different model" affordance.
pub fn all_models() -> &'static [ModelPick] {
    &DESKTOP_MODELS
}

/// Which non-local path a below-floor machine should use instead. Mirrors the app's
/// `ReadMode { Raw, Local, Live }`: [`Recommendation::NoLocal`] means "skip the Local rung".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fallback {
    /// The always-available offline deterministic engine template (no model, no network).
    Raw,
    /// A hosted API (OpenRouter) — best prose, needs a key + network.
    Live,
    /// Either works; the app's Raw/Live toggle lets the seeker choose. The default for a below-floor
    /// machine — the crate can't detect connectivity to prefer one.
    RawOrLive,
}

impl Fallback {
    pub fn label(self) -> &'static str {
        match self {
            Fallback::Raw => "Raw (offline engine template)",
            Fallback::Live => "Live (hosted API)",
            Fallback::RawOrLive => "Raw (offline) or Live (hosted API)",
        }
    }
}

/// Why a machine gets no local recommendation — the floor gate it fails first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoLocalReason {
    /// Below the 8 GB RAM floor (after the tolerance).
    BelowRam { have_gb: f64, need_gb: f64 },
    /// Fewer than the 4-core floor.
    TooFewCores { have: u32, need: u32 },
    /// An x86 CPU without AVX2 — llama.cpp can't run at a tolerable speed.
    NoAvx2,
}

impl NoLocalReason {
    /// A one-line human explanation for the CLI.
    pub fn human(self) -> String {
        match self {
            NoLocalReason::BelowRam { have_gb, need_gb } => {
                format!("{have_gb:.1} GB RAM is below the {need_gb:.0} GB floor")
            }
            NoLocalReason::TooFewCores { have, need } => {
                format!("{have} CPU cores is below the {need}-core floor")
            }
            NoLocalReason::NoAvx2 => "the CPU lacks AVX2 (required by llama.cpp)".to_string(),
        }
    }
}

/// The benchmark's answer for a machine: a concrete local pick, or — below the minimum viable floor —
/// no local model, with the reason and the Raw/Live alternative.
#[derive(Debug, Clone, PartialEq)]
pub enum Recommendation {
    Local(ModelPick),
    NoLocal {
        reason: NoLocalReason,
        fallback: Fallback,
    },
}

/// Whether `model` can be held in memory on `spec` — its download size plus OS/KV headroom must fit
/// within `ram_gb * FIT_FRACTION`. This is what makes a machine that *clears a tier's gates* but
/// *can't hold that tier's model* (e.g. a 64 GB unified laptop clearing Ultra but not fitting the
/// ~65 GB 120B) drop to the largest model it can actually run — weights live in RAM even on the
/// GPU-offload hybrid path, so RAM (not VRAM) is the binding fit constraint.
fn model_fits(spec: &DeviceSpec, model: &ModelPick) -> bool {
    model.download_gb <= spec.ram_gb * FIT_FRACTION
}

/// The floor gate a below-floor machine fails first — in the same order [`tier_for`] gates: AVX2,
/// then the Low row's RAM, then cores.
fn first_failed_floor_gate(spec: &DeviceSpec) -> NoLocalReason {
    const FLOOR_RAM: f64 = 8.0;
    const FLOOR_CORES: u32 = 4;
    if spec.avx2 == Some(false) {
        return NoLocalReason::NoAvx2;
    }
    if spec.ram_gb < FLOOR_RAM * RAM_TOLERANCE {
        return NoLocalReason::BelowRam {
            have_gb: spec.ram_gb,
            need_gb: FLOOR_RAM,
        };
    }
    NoLocalReason::TooFewCores {
        have: spec.cores,
        need: FLOOR_CORES,
    }
}

/// The benchmark's recommendation for a machine. Below the floor → [`Recommendation::NoLocal`] (use
/// Raw/Live). Otherwise the **largest model that actually fits**: from the machine's gated tier down,
/// the highest whose model clears [`model_fits`] — so a machine that gates a tier it can't hold the
/// model for gets the biggest model it *can* run, not one that would swap to disk.
pub fn recommend_for(spec: &DeviceSpec) -> Recommendation {
    let Some(gated) = tier_for(spec) else {
        return Recommendation::NoLocal {
            reason: first_failed_floor_gate(spec),
            fallback: Fallback::RawOrLive,
        };
    };
    // DESKTOP_MODELS is Low→Ultra; walk it Ultra→Low and take the first at-or-below the gated tier
    // whose model fits. The floor guarantees the Low model (3 GB) fits 8 GB, so a pick always exists.
    let pick = DESKTOP_MODELS
        .iter()
        .rev()
        .filter(|m| m.tier <= gated)
        .find(|m| model_fits(spec, m))
        .copied()
        .unwrap_or(DESKTOP_MODELS[0]);
    Recommendation::Local(pick)
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

/// A descriptive User-Agent for the Hugging Face API call — good API citizenship, and it keeps the
/// request out of the rate-limit ambiguity that unspecified anonymous clients can hit.
const HF_USER_AGENT: &str =
    "ziqpu-model/0.1 (+https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3)";

/// A system tool's spawn target, pinned to an absolute path on Windows and left as the Unix name
/// elsewhere (CWE-427). `win_rel` is the path under `%SystemRoot%\System32`; `unix` is the Unix
/// command (bare name or absolute path).
///
/// **The vector is the application directory, not the CWD.** An earlier version of this comment
/// blamed `CreateProcess` searching the current directory — that is true of raw `CreateProcess` but
/// *not* of Rust's `std::process::Command`, whose `resolve_exe` deliberately excludes the CWD.
/// Measured on Windows 11 / rustc 1.96: a planted `curl.exe` in the CWD is **not** picked up, while
/// one sitting **beside our own exe** is, because Rust's order is child-PATH → application
/// directory → System32 → Windows → PATH. Pinning the System32 path defeats it (also measured).
///
/// That distinction is load-bearing for us: we ship Windows as a plain zip, so users extract and run
/// from wherever they downloaded it — typically Downloads, which is exactly where attacker-supplied
/// files land. The app directory is therefore a live planting surface, not a theoretical one.
/// (Impact is code execution at the user's own privileges, not elevation.)
fn system_cmd(win_rel: &str, unix: &str) -> String {
    #[cfg(windows)]
    {
        let _ = unix;
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| String::from(r"C:\Windows"));
        format!(r"{root}\System32\{win_rel}")
    }
    #[cfg(not(windows))]
    {
        let _ = win_rel;
        unix.to_string()
    }
}

/// Run a small system probe with a hard timeout, returning its stdout on exit-within-timeout. `None`
/// only on spawn error or timeout — a wedged `nvidia-smi` (a real failure mode when the driver is
/// stuck) is killed rather than hanging the benchmark. Note: stdout is returned regardless of exit
/// *code* — the Windows registry query exits non-zero on a benign non-terminating error while still
/// emitting valid lines, and each caller's parser rejects garbage/empty output on its own. Safe only
/// for small outputs (well under the OS pipe buffer, so the wait-then-read can't deadlock) — exactly
/// the GPU probes; the larger `curl` fetch keeps its own `--max-time`/`--max-filesize` instead.
fn run_capped(cmd: &str, args: &[&str], secs: u64) -> Option<String> {
    use std::io::Read;
    use std::process::{Command, Stdio};
    use std::time::Duration;
    use wait_timeout::ChildExt;
    let mut child = no_window(Command::new(cmd))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    match child.wait_timeout(Duration::from_secs(secs)) {
        Ok(Some(_status)) => {
            let mut buf = String::new();
            child.stdout.take()?.read_to_string(&mut buf).ok()?;
            Some(buf)
        }
        _ => {
            let _ = child.kill();
            let _ = child.wait();
            None
        }
    }
}

/// Query the Hub for current GGUF repos matching `term` — **thin I/O, not unit-tested** (the parser
/// is). HTTP is a `curl` subprocess (the repo's cross-platform, no-HTTP-crate convention), pinned to
/// System32 on Windows (SEC-001), bounded by both `--max-time` and `--max-filesize` (SEC-004), and
/// sent with a descriptive User-Agent. Degrades quietly to an empty list; callers fall back to the
/// static pick. `.output()` reads stdout concurrently, so a large body can't deadlock the call.
pub fn resolve_candidates(term: &str) -> Vec<Candidate> {
    use std::process::Command;
    let url = hf_api_url(term);
    let Ok(out) = no_window(Command::new(system_cmd("curl.exe", "curl")))
        .args([
            "-sS",
            "--max-time",
            "8",
            "--max-filesize",
            "5000000",
            "-A",
            HF_USER_AGENT,
            url.as_str(),
        ])
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

/// Whether a repo id carries a guardrail-hostile marker (see [`AGENT_DISQUALIFIERS`]) — an
/// abliterated / uncensored / jailbroken / NSFW variant. Public so the UI can flag such repos in the
/// model search (the recommendation already skips them via [`pick_for_agent`]).
pub fn agent_disqualified(repo: &str) -> bool {
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

// ---- Layer 2b: pick the QUANT — list a repo's GGUFs, choose the largest that fits the machine ----

/// One GGUF quantization available in a repo — its quant tag (e.g. `Q4_K_M`) and total size in GB
/// (summed across shards). What llama.cpp downloads for `-hf <repo>:<quant>`.
#[derive(Debug, Clone, PartialEq)]
pub struct GgufOption {
    pub quant: String,
    pub size_gb: f64,
}

/// Does a hyphen-separated filename segment look like a quant tag? A quant starts with `Q`/`F`/`BF`/
/// `IQ` and the next char is a digit — so a model name like `qwen2.5` (`Q` then a letter) is NOT a
/// quant. Checked uppercased.
fn looks_like_quant(seg: &str) -> bool {
    let up = seg.to_ascii_uppercase();
    for pre in ["IQ", "BF", "Q", "F"] {
        if let Some(rest) = up.strip_prefix(pre) {
            return rest.chars().next().is_some_and(|c| c.is_ascii_digit());
        }
    }
    false
}

/// The quant tag from a GGUF filename — the last quant-looking segment, so shard suffixes
/// (`…-00001-of-00002`) and the model name are skipped. Splits on BOTH `-` and `.` (repos differ:
/// `gpt-oss-20b-Q4_K_M.gguf` uses dashes, `Qwen3-14B.Q4_K_M.gguf` uses a dot) — but NOT `_`, since a
/// quant tag itself contains underscores (`Q4_K_M`). `None` when the name isn't `.gguf` or has no
/// quant segment. E.g. both examples above → `Q4_K_M`; `m-UD-Q4_K_XL-00001-of-00002.gguf` → `Q4_K_XL`.
fn quant_from_filename(name: &str) -> Option<String> {
    if !name.to_ascii_lowercase().ends_with(".gguf") {
        return None;
    }
    let stem = &name[..name.len() - 5];
    stem.split(['-', '.'])
        .rev()
        .find(|s| looks_like_quant(s))
        .map(|s| s.to_string())
}

/// Parse a Hugging Face `/tree` response into the repo's GGUF quants, summing shard sizes per quant.
/// Pure; unit-tested. Full-precision (`F16`/`F32`/`BF16`) files are kept here — [`best_gguf_for`]
/// filters them out of a recommendation. Malformed/empty JSON → empty (offline-safe, never panics).
pub fn parse_repo_tree(json: &str) -> Vec<GgufOption> {
    let arr: Vec<serde_json::Value> = serde_json::from_str(json).unwrap_or_default();
    let mut by_quant: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for v in &arr {
        let Some(path) = v.get("path").and_then(|x| x.as_str()) else {
            continue;
        };
        let Some(quant) = quant_from_filename(path) else {
            continue;
        };
        let bytes = v
            .get("lfs")
            .and_then(|l| l.get("size"))
            .and_then(|s| s.as_u64())
            .or_else(|| v.get("size").and_then(|s| s.as_u64()))
            .unwrap_or(0);
        *by_quant.entry(quant).or_insert(0) += bytes;
    }
    by_quant
        .into_iter()
        .map(|(quant, bytes)| GgufOption {
            quant,
            size_gb: bytes as f64 / 1_073_741_824.0,
        })
        .collect()
}

/// Fetch a repo's GGUF quants from the Hub (`/api/models/<repo>/tree/main`). Thin I/O (the parser is
/// tested); same `curl` discipline as [`resolve_candidates`] (System32-pinned, time/size-bounded,
/// User-Agent). Empty on any failure (offline-safe).
pub fn list_repo_ggufs(repo: &str) -> Vec<GgufOption> {
    use std::process::Command;
    let url = format!("https://huggingface.co/api/models/{repo}/tree/main?recursive=1");
    let Ok(out) = no_window(Command::new(system_cmd("curl.exe", "curl")))
        .args([
            "-sS",
            "--max-time",
            "8",
            "--max-filesize",
            "5000000",
            "-A",
            HF_USER_AGENT,
            url.as_str(),
        ])
        .output()
    else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    parse_repo_tree(&String::from_utf8_lossy(&out.stdout))
}

/// Context window (tokens) the served model is capped to. `llama-server`'s default is 0 = "use the
/// model's full trained context" — for GPT-OSS-20B that's 32k+, whose KV cache is several GB and was
/// the real cause of the 16 GB card's OOM (weights fit; weights + full-context KV did not). A reading
/// needs far less, so we cap it: the KV cache shrinks to a fraction of a GB and the weight budget below
/// can be generous again. Passed as `-c` by every serve path (CLI + panel).
pub const SERVE_CTX_SIZE: u32 = 8192;

/// Fraction of VRAM the model **weights** may occupy — the rest (~a fifth) covers the (now capped, see
/// [`SERVE_CTX_SIZE`]) KV cache, compute buffers, and driver overhead, ~2–3 GB for a 20B MoE at an 8k
/// context. With the context capped this can be generous; the earlier 0.68 (set before the context cap)
/// overcorrected and dropped a 16 GB card to a low-quality Q3 fallback on a model it runs comfortably.
const VRAM_USABLE_FRACTION: f64 = 0.80;

/// How much memory the model **weights** may occupy on this machine — the budget the quant is fitted
/// to. A discrete GPU gets `vram * VRAM_USABLE_FRACTION` (the rest is runtime headroom) so the pick
/// runs fully on the GPU without OOM; a CPU / unified-memory machine gets `ram_gb * FIT_FRACTION`.
pub fn device_model_budget_gb(spec: &DeviceSpec, gpu: Option<&GpuInfo>) -> f64 {
    match gpu {
        Some(g) if !g.unified && g.vram_gb >= 1.0 => g.vram_gb * VRAM_USABLE_FRACTION,
        _ => spec.ram_gb * FIT_FRACTION,
    }
}

/// A coarse **quality** rank for a GGUF quant — higher is better. Needed because for a compact model
/// (GPT-OSS is MoE/MXFP4-native) the quant file sizes barely differ, so size alone is a poor quality
/// signal; the pick should be the highest-quality quant that fits, not merely the largest file.
pub fn quant_rank(quant: &str) -> u32 {
    let q = quant.to_ascii_uppercase();
    let base = if q.starts_with("F32") {
        100
    } else if q.starts_with("BF16") || q.starts_with("F16") {
        95
    } else if q.starts_with("Q8") {
        80
    } else if q.starts_with("Q6") {
        60
    } else if q.starts_with("Q5") {
        50
    } else if q.starts_with("Q4") || q.starts_with("IQ4") {
        40
    } else if q.starts_with("Q3") || q.starts_with("IQ3") {
        30
    } else if q.starts_with("Q2") || q.starts_with("IQ2") {
        20
    } else {
        10
    };
    // K-quant size modifier: S < (none) < M < L < XL.
    let modi = if q.ends_with("_XL") {
        4
    } else if q.ends_with("_L") {
        3
    } else if q.ends_with("_M") {
        2
    } else if q.ends_with("_S") {
        1
    } else {
        0
    };
    base + modi
}

/// Pick the **highest-quality quant that fits** `budget_gb` — the best the machine can actually run.
/// Ranks by [`quant_rank`] (not file size, which is near-constant for a compact model), tie-breaking
/// toward the smaller file (safer headroom). Full-precision (`F16`/`F32`/`BF16`) is excluded (overkill
/// for a guarded reader) unless nothing else exists. Nothing fits → the smallest file. Pure; tested.
pub fn best_gguf_for(files: &[GgufOption], budget_gb: f64) -> Option<GgufOption> {
    let is_quant = |q: &str| {
        let up = q.to_ascii_uppercase();
        up.starts_with('Q') || up.starts_with("IQ")
    };
    let quants: Vec<&GgufOption> = files.iter().filter(|f| is_quant(&f.quant)).collect();
    let pool: Vec<&GgufOption> = if quants.is_empty() {
        files.iter().collect()
    } else {
        quants
    };
    pool.iter()
        .filter(|f| f.size_gb <= budget_gb)
        .max_by(|a, b| {
            quant_rank(&a.quant)
                .cmp(&quant_rank(&b.quant))
                // Same rank → prefer the smaller file (more runtime headroom).
                .then_with(|| b.size_gb.total_cmp(&a.size_gb))
        })
        .or_else(|| pool.iter().min_by(|a, b| a.size_gb.total_cmp(&b.size_gb)))
        .map(|f| (*f).clone())
}

/// The full serve plan for a machine: the current repo for the tier's model + the largest quant it can
/// run, fitted to [`device_model_budget_gb`]. `None` offline (no repo / no files) — the caller then
/// falls back to the static [`ModelPick`] quant.
#[derive(Debug, Clone, PartialEq)]
pub struct ServePlan {
    pub repo: String,
    pub quant: String,
    pub size_gb: f64,
}

/// Resolve the recommended pick to a concrete `repo:quant` fitted to this machine's memory budget.
pub fn plan_serve(pick: &ModelPick, spec: &DeviceSpec, gpu: Option<&GpuInfo>) -> Option<ServePlan> {
    let repo = resolve_current_repo(pick)?.repo;
    let best = best_gguf_for(&list_repo_ggufs(&repo), device_model_budget_gb(spec, gpu))?;
    Some(ServePlan {
        repo,
        quant: best.quant,
        size_gb: best.size_gb,
    })
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
    // nvidia-smi: absolute on Windows (System32) to defeat CWD binary-planting; bare on Unix. Under a
    // hard timeout — a wedged driver can hang nvidia-smi indefinitely (SEC-003).
    if let Some(out) = run_capped(
        &system_cmd("nvidia-smi.exe", "nvidia-smi"),
        &[
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ],
        5,
    ) {
        if let Some(g) = parse_nvidia_smi(&out) {
            return Some(g);
        }
    }
    if cfg!(target_os = "windows") {
        // The video-adapter class key; `qwMemorySize` is the real VRAM in bytes. powershell.exe is
        // pinned to its System32 location (SEC-001) and run under a timeout (SEC-003). The query
        // exits non-zero on a benign non-terminating error while still emitting valid lines —
        // run_capped returns stdout regardless of exit code, and the parser rejects garbage.
        let script = r#"Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\*' -ErrorAction SilentlyContinue | ForEach-Object { "$($_.DriverDesc)|$($_.'HardwareInformation.qwMemorySize')" }"#;
        if let Some(out) = run_capped(
            &system_cmd(r"WindowsPowerShell\v1.0\powershell.exe", "powershell"),
            &["-NoProfile", "-NonInteractive", "-Command", script],
            6,
        ) {
            if let Some(g) = parse_windows_registry_gpu(&out) {
                return Some(g);
            }
        }
    }
    if cfg!(target_os = "macos") {
        // system_profiler by absolute path (this branch is macOS-only), under a timeout.
        if let Some(out) = run_capped("/usr/sbin/system_profiler", &["SPDisplaysDataType"], 6) {
            if let Some(g) = parse_macos_gpu(&out) {
                return Some(g);
            }
        }
    }
    None
}

/// The fraction of unified (Apple-Silicon) system memory usable as a VRAM budget for tier gating —
/// the OS and other apps need the rest, so the model isn't handed the entire pool.
const UNIFIED_VRAM_FRACTION: f64 = 0.7;

/// The VRAM budget used for tier gating, given a detected GPU and the machine's RAM. A discrete GPU
/// contributes its own VRAM; a unified-memory GPU (Apple Silicon) contributes a fraction of system
/// RAM (leaving OS headroom rather than the whole pool); no GPU yields `None` (unknown → the pick
/// stays on a CPU-runnable tier). Pure + unit-tested.
pub fn vram_budget(gpu: Option<&GpuInfo>, ram_gb: f64) -> Option<f64> {
    gpu.map(|g| {
        if g.unified {
            ram_gb * UNIFIED_VRAM_FRACTION
        } else {
            g.vram_gb
        }
    })
}

/// Detect the real machine's capability. **Thin I/O, not unit-tested** — RAM via `sysinfo`, cores via
/// the std runtime hint, GPU via [`detect_gpu`]. Disk-free stays `None` for now (never blocks a pick).
pub fn detect_spec() -> DeviceSpec {
    detect_spec_with(detect_gpu().as_ref())
}

/// Build a [`DeviceSpec`] from an already-detected GPU, so a caller that also wants the GPU's *name*
/// can detect once and reuse it (the CLI does). The VRAM budget comes from [`vram_budget`] (discrete
/// VRAM, or a fraction of RAM for unified memory); no GPU leaves VRAM `None` (caps at a CPU tier).
pub fn detect_spec_with(gpu: Option<&GpuInfo>) -> DeviceSpec {
    use sysinfo::System;
    let mut sys = System::new();
    sys.refresh_memory();
    // sysinfo 0.30+ reports memory in bytes.
    let ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let cores = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    let vram_gb = vram_budget(gpu, ram_gb);
    DeviceSpec {
        ram_gb,
        cores,
        disk_free_gb: None,
        vram_gb,
        avx2: detect_avx2(),
    }
}

/// Detect AVX2 on x86 (the running CPU); `None` on non-x86 (ARM / Apple Silicon don't need it and
/// are never gated on it). No new dependency — `is_x86_feature_detected!` is in `std`.
fn detect_avx2() -> Option<bool> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        Some(std::arch::is_x86_feature_detected!("avx2"))
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        None
    }
}

/// The llama.cpp install command for this OS — shown by `get`/`serve` when `llama-server` is missing.
pub fn llama_install_hint() -> &'static str {
    if cfg!(target_os = "windows") {
        "winget install llama.cpp"
    } else if cfg!(target_os = "macos") {
        "brew install llama.cpp"
    } else {
        "see github.com/ggml-org/llama.cpp releases, or your package manager"
    }
}

/// Locate a runnable `llama-server` binary: first on `PATH`, then — on Windows — the winget install
/// location (`%LOCALAPPDATA%\Microsoft\WinGet\Packages\ggml.llamacpp_*\llama-server.exe`), which is
/// NOT added to `PATH` until the shell restarts. Returns the path to spawn (`"llama-server"` itself
/// when it's on `PATH`). `None` when llama.cpp isn't installed anywhere we look.
pub fn llama_server_path() -> Option<std::path::PathBuf> {
    let bin = if cfg!(windows) {
        "llama-server.exe"
    } else {
        "llama-server"
    };
    // 1. On PATH? (a spawn that succeeds — the exit code doesn't matter).
    let on_path = no_window(std::process::Command::new(bin))
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();
    if on_path {
        return Some(std::path::PathBuf::from(bin));
    }
    // 2. Windows winget install location (off-PATH until a shell restart).
    #[cfg(windows)]
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let pkgs = std::path::Path::new(&local)
            .join("Microsoft")
            .join("WinGet")
            .join("Packages");
        if let Ok(entries) = std::fs::read_dir(&pkgs) {
            for entry in entries.flatten() {
                if entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("ggml.llamacpp")
                {
                    let cand = entry.path().join("llama-server.exe");
                    if cand.exists() {
                        return Some(cand);
                    }
                }
            }
        }
    }
    None
}

/// Whether a runnable `llama-server` exists (PATH or the winget location) — see [`llama_server_path`].
/// Used by `get`/`serve` and the UI to decide between "run it" and "here's how to install llama.cpp".
pub fn have_llama_server() -> bool {
    resolve_llama_server().is_some()
}

/// The user's home directory (`%USERPROFILE%` / `$HOME`) — where the Hugging Face cache lives.
fn home_dir() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    let key = "USERPROFILE";
    #[cfg(not(windows))]
    let key = "HOME";
    std::env::var(key).ok().map(std::path::PathBuf::from)
}

/// Is the GGUF for `repo`:`quant` ALREADY in the local Hugging Face cache? When it is, a serve only
/// LOADS the model — the (large, one-time) download is skipped — so the UI can say "downloaded ✓" and
/// label the button "Serve" instead of "Download & serve". Matches a `.gguf` under the repo's snapshot
/// whose name contains the quant tag (case-insensitive).
pub fn model_cached(repo: &str, quant: &str) -> bool {
    let Some(home) = home_dir() else {
        return false;
    };
    let snaps = home
        .join(".cache")
        .join("huggingface")
        .join("hub")
        .join(format!("models--{}", repo.replace('/', "--")))
        .join("snapshots");
    let ql = quant.to_ascii_lowercase();
    let Ok(entries) = std::fs::read_dir(&snaps) else {
        return false;
    };
    for snap in entries.flatten() {
        if let Ok(files) = std::fs::read_dir(snap.path()) {
            for f in files.flatten() {
                let name = f.file_name().to_string_lossy().to_ascii_lowercase();
                if name.ends_with(".gguf") && name.contains(&ql) {
                    return true;
                }
            }
        }
    }
    false
}

/// A local port [1234, 1245] with a **healthy** `llama-server` (`/health` → `{"status":"ok"}`), or
/// `None`. Lets the app RECONNECT to an already-loaded model instead of reloading it, and auto-detect
/// a server still running from a prior session on startup. Only `llama-server` answers `/health` with
/// `"ok"` — LM Studio's server does not — so this never false-matches LM Studio on :1234.
/// Runs at startup, before the window opens, so it probes up to 12 ports on every launch — which is
/// why the spawn target is pinned via [`system_cmd`] like the file's other `curl` callers rather
/// than left bare. (Known gap: on a machine with no `curl` at all this simply finds nothing and the
/// app declines to reconnect — a soft, honest degrade, unlike the health probes in `agents`/`ui`
/// which are now in-process.)
pub fn running_server_port() -> Option<u16> {
    let curl = system_cmd("curl.exe", "curl");
    (1234u16..=1245).find(|&p| {
        no_window(std::process::Command::new(&curl))
            .args([
                "-sS",
                "--max-time",
                "2",
                &format!("http://127.0.0.1:{p}/health"),
            ])
            .output()
            .map(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).contains("\"ok\""))
            .unwrap_or(false)
    })
}

// ─── GPU runtime backend + device selection ──────────────────────────────────────────────────────
//
// The runtime bug this solves: the winget `ggml.llamacpp` is a Vulkan-only build, and on a hybrid
// laptop (AMD iGPU + NVIDIA dGPU) its Vulkan ICD enumerates ONLY the AMD iGPU (`Vulkan0`) — so a big
// model loads onto shared system RAM and OOMs, never touching the discrete RTX 5080. The fix is to
// run a backend build that actually targets the discrete GPU (CUDA on NVIDIA, Metal on Apple, ROCm on
// AMD) and to PIN the served device explicitly with `-dev`, never trusting auto-pick on a hybrid.

/// The compute backend a `llama-server` build talks to the GPU through. Which one is available is a
/// property of the *binary* (chosen at compile time), surfaced at runtime by `--list-devices`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Cuda,
    Metal,
    Rocm,
    Sycl,
    Vulkan,
    Cpu,
}

impl Backend {
    /// Classify a `--list-devices` token prefix (`CUDA0` → Cuda, `Vulkan0` → Vulkan, …).
    fn from_token(token: &str) -> Backend {
        let t = token.to_ascii_uppercase();
        if t.starts_with("CUDA") {
            Backend::Cuda
        } else if t.starts_with("METAL") {
            Backend::Metal
        } else if t.starts_with("ROCM") || t.starts_with("HIP") {
            Backend::Rocm
        } else if t.starts_with("SYCL") {
            Backend::Sycl
        } else if t.starts_with("VULKAN") {
            Backend::Vulkan
        } else {
            Backend::Cpu
        }
    }

    /// A short human label for the RuntimeHealth line.
    pub fn label(&self) -> &'static str {
        match self {
            Backend::Cuda => "CUDA",
            Backend::Metal => "Metal",
            Backend::Rocm => "ROCm",
            Backend::Sycl => "SYCL",
            Backend::Vulkan => "Vulkan",
            Backend::Cpu => "CPU",
        }
    }

    /// Whether this backend targets a discrete/dedicated GPU well. Vulkan is deprioritized because on
    /// a hybrid machine it tends to bind the integrated GPU (the whole bug); CPU is not a GPU at all.
    fn is_preferred_gpu(&self) -> bool {
        matches!(
            self,
            Backend::Cuda | Backend::Metal | Backend::Rocm | Backend::Sycl
        )
    }
}

/// One compute device as reported by `llama-server --list-devices`, e.g.
/// `CUDA0: NVIDIA GeForce RTX 5080 Laptop GPU (16302 MiB, 15067 MiB free)`.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    /// The `-dev` token, e.g. `CUDA0` / `Vulkan0` / `Metal0`.
    pub token: String,
    pub backend: Backend,
    pub name: String,
    pub total_mib: u64,
    pub free_mib: u64,
}

impl Device {
    pub fn vram_gb(&self) -> f64 {
        self.total_mib as f64 / 1024.0
    }

    /// Whether this looks like a discrete GPU (by name) — a dedicated card, not an integrated one
    /// sharing system RAM. Used to keep the RuntimeHealth warning honest: Vulkan on a discrete NVIDIA
    /// is fine (green); Vulkan on an integrated GPU is the OOM-risk case (carnelian).
    pub fn is_discrete(&self) -> bool {
        name_discreteness_rank(&self.name) == 2
    }

    /// A healthy serving path: a preferred backend (CUDA/Metal/ROCm/SYCL) OR at least a discrete GPU.
    /// Integrated-GPU or CPU serving of a multi-GB model is the risky case the UI warns about.
    pub fn is_healthy(&self) -> bool {
        self.backend.is_preferred_gpu() || self.is_discrete()
    }
}

/// Parse the `--list-devices` block into [`Device`]s. Pure + unit-tested. Tolerant: a line without the
/// `(… MiB, … MiB free)` tail still yields a device (memory 0) so a format drift never drops a GPU.
/// The `Available devices:` header and any stray `E`/log lines are ignored (a device line starts with
/// `<TOKEN>:` where TOKEN is alphanumeric).
pub fn parse_list_devices(output: &str) -> Vec<Device> {
    let mut out = Vec::new();
    for raw in output.lines() {
        let line = raw.trim();
        // A device line is `TOKEN: name (....)`. Split once on ':'; the token must be a bare
        // alnum run (CUDA0, Vulkan1) — this rejects timestamped log lines like `0.02.5 E ...`.
        let Some((token, rest)) = line.split_once(':') else {
            continue;
        };
        let token = token.trim();
        if token.is_empty()
            || !token.chars().all(|c| c.is_ascii_alphanumeric())
            || !token.chars().any(|c| c.is_ascii_alphabetic())
            || !token.chars().any(|c| c.is_ascii_digit())
        {
            continue;
        }
        let rest = rest.trim();
        // Split the trailing "(total MiB, free MiB free)" if present.
        let (name, total_mib, free_mib) = match rest.rsplit_once('(') {
            Some((name, mem)) => {
                let nums: Vec<u64> = mem
                    .split(|c: char| !c.is_ascii_digit())
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.parse().ok())
                    .collect();
                (
                    name.trim(),
                    nums.first().copied().unwrap_or(0),
                    nums.get(1).copied().unwrap_or(0),
                )
            }
            None => (rest, 0, 0),
        };
        out.push(Device {
            token: token.to_string(),
            backend: Backend::from_token(token),
            name: name.to_string(),
            total_mib,
            free_mib,
        });
    }
    out
}

/// A coarse discrete-vs-integrated rank from a GPU's name: 2 = clearly discrete, 0 = clearly an
/// integrated GPU (shares system RAM), 1 = unknown. Needed because a hybrid laptop's Vulkan build
/// enumerates BOTH the iGPU and the dGPU as `Vulkan0`/`Vulkan1`, and the iGPU often reports MORE
/// "VRAM" (a slice of system RAM) than the discrete card's dedicated pool — so picking by memory alone
/// lands on the iGPU (the OOM trap). Name is the reliable signal.
fn name_discreteness_rank(name: &str) -> u8 {
    let n = name.to_ascii_uppercase();
    // Discrete markers first (an AMD "Radeon RX"/"Radeon Pro" is discrete despite the "Radeon").
    const DISCRETE: [&str; 8] = [
        "NVIDIA",
        "GEFORCE",
        "RTX",
        "GTX",
        "QUADRO",
        "RADEON RX",
        "RADEON PRO",
        "ARC ",
    ];
    if DISCRETE.iter().any(|m| n.contains(m)) {
        return 2;
    }
    // Integrated markers: AMD/Intel iGPUs are "… Graphics"; Apple/Intel iGPU families.
    const INTEGRATED: [&str; 6] = ["GRAPHICS", "UHD", "IRIS", "VEGA", "890M", "RADEON(TM)"];
    if INTEGRATED.iter().any(|m| n.contains(m)) {
        return 0;
    }
    1
}

/// Choose the device to serve on: a preferred-GPU backend (CUDA/Metal/ROCm/SYCL) over Vulkan over CPU;
/// then a DISCRETE GPU over an integrated one (by name — the iGPU can falsely report more shared RAM);
/// then the highest total VRAM. `None` = no device line at all (CPU-only build/machine). Pure +
/// unit-tested. This is the anti-iGPU-trap rule — on a hybrid it must land on the dedicated card.
pub fn select_device(devices: &[Device]) -> Option<Device> {
    devices
        .iter()
        .max_by(|a, b| {
            a.backend
                .is_preferred_gpu()
                .cmp(&b.backend.is_preferred_gpu())
                .then(name_discreteness_rank(&a.name).cmp(&name_discreteness_rank(&b.name)))
                .then(a.total_mib.cmp(&b.total_mib))
        })
        .cloned()
}

/// The app-managed runtime root — where [`ensure_runtime`] lays down the per-vendor llama.cpp build
/// it fetches, e.g. `%LOCALAPPDATA%\Ziqpu\runtime\llama-b10064-bin-win-cuda-13.3-x64\`. Kept
/// separate from the winget location so the managed (correct-backend) build always wins.
pub fn managed_runtime_root() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("LOCALAPPDATA")
            .ok()
            .map(|l| std::path::Path::new(&l).join("Ziqpu").join("runtime"))
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME").ok().map(|h| {
            std::path::Path::new(&h)
                .join("Library")
                .join("Application Support")
                .join("Ziqpu")
                .join("runtime")
        })
    }
    #[cfg(all(not(windows), not(target_os = "macos")))]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| std::path::Path::new(&h).join(".local/share/ziqpu/runtime"))
    }
}

/// The platform name of the `llama-server` binary.
fn llama_server_bin_name() -> &'static str {
    if cfg!(windows) {
        "llama-server.exe"
    } else {
        "llama-server"
    }
}

/// Find `bin` under `dir`, descending at most `depth` directory levels. Bounded because the
/// runtime archives differ in shape — the Windows zips are flat, the macOS/Linux tarballs nest a
/// `llama-bNNNNN/` directory — and a depth-capped walk handles both without ever crawling a
/// user's whole disk if the root is misconfigured.
fn find_bin_under(dir: &std::path::Path, bin: &str, depth: u8) -> Option<std::path::PathBuf> {
    let direct = dir.join(bin);
    if direct.is_file() {
        return Some(direct);
    }
    if depth == 0 {
        return None;
    }
    let entries = std::fs::read_dir(dir).ok()?;
    entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .find_map(|e| find_bin_under(&e.path(), bin, depth - 1))
}

/// A `llama-server` inside the managed runtime root, if one has been laid down. This is the
/// backend-correct build for the machine, so it takes priority over PATH/winget in
/// [`resolve_llama_server`].
fn managed_llama_server() -> Option<std::path::PathBuf> {
    find_bin_under(&managed_runtime_root()?, llama_server_bin_name(), 3)
}

/// Resolve the `llama-server` to run: the app-managed (backend-correct) build first, then PATH, then
/// the winget location. Supersedes bare [`llama_server_path`] in the serve paths so a machine with the
/// managed CUDA build serves on the dGPU instead of the winget Vulkan build's iGPU.
pub fn resolve_llama_server() -> Option<std::path::PathBuf> {
    managed_llama_server().or_else(llama_server_path)
}

/// Probe a specific `llama-server` binary for its visible devices (`--list-devices`, 8 s cap). Empty
/// on a CPU-only build/machine or if the probe fails. Used to build the RuntimeHealth line and to pick
/// the `-dev` token.
pub fn probe_devices(bin: &std::path::Path) -> Vec<Device> {
    run_capped(&bin.to_string_lossy(), &["--list-devices"], 8)
        .map(|o| parse_list_devices(&o))
        .unwrap_or_default()
}

/// The GPU serve flags for a chosen device: full offload + an explicit device pin. Empty for CPU/none
/// (llama-server runs on CPU). Metal needs no `-dev` (one implicit device) but `-ngl 99` is harmless.
pub fn gpu_serve_args(device: Option<&Device>) -> Vec<String> {
    match device {
        Some(d) if d.backend == Backend::Metal => vec!["-ngl".into(), "99".into()],
        Some(d) if d.backend != Backend::Cpu => {
            vec!["-ngl".into(), "99".into(), "-dev".into(), d.token.clone()]
        }
        _ => Vec::new(),
    }
}

// ─── ensure_runtime: fetch the backend-correct llama.cpp build for this machine ──────────────────
//
// The distribution bug this solves: "local models need llama.cpp installed yourself" is a dead end
// for a stranger evaluating the app — and the obvious self-serve fix (winget) installs a
// Vulkan-only build that binds a hybrid laptop's iGPU (the OOM saga above). So the app lays down
// the right build itself: pick the official ggml-org/llama.cpp release asset for this OS/arch/GPU,
// download it into the app-managed runtime root, and let `resolve_llama_server` prefer it forever
// after. One-time, per-user, no admin rights.

/// Which vendor's silicon the detected GPU is — decides which llama.cpp build to fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    None,
}

/// Classify a detected GPU by name. Pure; unit-tested. `None` (no GPU detected) → CPU build.
pub fn gpu_vendor(gpu: Option<&GpuInfo>) -> GpuVendor {
    let Some(g) = gpu else {
        return GpuVendor::None;
    };
    if g.unified {
        return GpuVendor::Apple;
    }
    let n = g.name.to_ascii_uppercase();
    if ["NVIDIA", "GEFORCE", "RTX", "GTX", "QUADRO", "TESLA"]
        .iter()
        .any(|m| n.contains(m))
    {
        // Checked FIRST: "RTX" alone must not fall through to another vendor's arm.
        GpuVendor::Nvidia
    } else if n.contains("AMD") || n.contains("RADEON") {
        GpuVendor::Amd
    } else if n.contains("INTEL") || n.contains("ARC") || n.contains("IRIS") || n.contains("UHD") {
        GpuVendor::Intel
    } else if n.starts_with("APPLE") {
        GpuVendor::Apple
    } else {
        GpuVendor::None
    }
}

/// Does this NVIDIA card need a CUDA **13** build? CUDA 13 dropped everything older than Turing
/// (sm_75) but is the only line that knows Blackwell (RTX 50-series, RTX PRO); CUDA 12 knows
/// Pascal→Ada but NOT Blackwell. So: an RTX model number ≥ 5000, or the "RTX PRO" workstation
/// line, needs 13 — everything else is safest on 12. Pure; unit-tested. (Edge checked: the
/// "RTX 500/1000 Ada" laptop workstation cards parse as < 5000 → 12, correct for Ada; the
/// "RTX 5000 Ada Generation" parses as ≥ 5000 → 13, also fine — CUDA 13 kept Ada.)
pub fn nvidia_wants_cuda13(name: &str) -> bool {
    let n = name.to_ascii_uppercase();
    if n.contains("RTX PRO") {
        return true;
    }
    if let Some(rest) = n.split("RTX").nth(1) {
        let digits: String = rest
            .trim_start()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if let Ok(model) = digits.parse::<u32>() {
            return model >= 5000;
        }
    }
    false
}

/// The compile target, made explicit so [`pick_runtime_asset_for`] is a pure function testable for
/// every platform from any host (the CI matrix runs all three OSes; each must see the others'
/// tables pass, not just its own).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetOs {
    Windows,
    Mac,
    Linux,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X64,
    Arm64,
}

/// This build's platform.
pub fn current_platform() -> (TargetOs, TargetArch) {
    let os = if cfg!(target_os = "windows") {
        TargetOs::Windows
    } else if cfg!(target_os = "macos") {
        TargetOs::Mac
    } else {
        TargetOs::Linux
    };
    let arch = if cfg!(target_arch = "aarch64") {
        TargetArch::Arm64
    } else {
        TargetArch::X64
    };
    (os, arch)
}

/// The runtime download chosen for a machine: the release asset to fetch, an optional companion
/// (the CUDA builds need the separate `cudart-…` DLL bundle unzipped beside `llama-server.exe`),
/// and a short label for progress lines.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimePick {
    pub asset: String,
    pub companion: Option<String>,
    pub label: String,
}

/// The best CUDA asset of a given major line (`"12"` / `"13"`) — highest minor wins, compared
/// numerically so a hypothetical `12.10` beats `12.4`. Returns `(asset_name, "12.4")`.
fn best_cuda_asset(names: &[&str], major: &str) -> Option<(String, String)> {
    names
        .iter()
        .filter(|n| !n.starts_with("cudart-"))
        .filter_map(|n| {
            let ver = n.split("-bin-win-cuda-").nth(1)?.strip_suffix("-x64.zip")?;
            let (maj, min) = ver.split_once('.')?;
            if maj != major {
                return None;
            }
            Some((min.parse::<u32>().ok()?, n.to_string(), ver.to_string()))
        })
        .max_by_key(|(min, _, _)| *min)
        .map(|(_, name, ver)| (name, ver))
}

/// Pick the release asset for a platform + GPU. Pure; unit-tested against the real asset list of a
/// real release. Returns `None` only when the release carries nothing usable for the platform.
///
/// The vendor table, and why it deviates from the obvious per-vendor maximum:
/// - **NVIDIA → CUDA** (12 or 13 by card generation — see [`nvidia_wants_cuda13`]) + the matching
///   `cudart` companion. CUDA is the entire point of the managed runtime: it targets the discrete
///   card directly and cannot land on the iGPU. Falls back to Vulkan if the release dropped CUDA.
/// - **AMD / Intel → Vulkan**, deliberately NOT the per-vendor HIP/SYCL builds: HIP zips bake a
///   fixed gfx-architecture list (a card outside it hard-fails) and SYCL needs the oneAPI runtime
///   installed system-wide — both are remote failure modes we cannot check from here, on machines
///   we will never see. Vulkan runs on every AMD/Intel driver stack, and the `-dev` pin +
///   discreteness ranking (see `select_device`) already defuse the hybrid-iGPU trap that made
///   Vulkan scary in the first place.
/// - **Apple → the macOS tarball** (Metal is in the arm64 build); Intel Macs get the x64 CPU build.
/// - **No GPU → CPU build**, never Vulkan: a Vulkan build without a Vulkan driver fails to start.
pub fn pick_runtime_asset_for(
    os: TargetOs,
    arch: TargetArch,
    names: &[&str],
    gpu: Option<&GpuInfo>,
) -> Option<RuntimePick> {
    let vendor = gpu_vendor(gpu);
    let by_suffix = |suffix: &str| -> Option<String> {
        names
            .iter()
            .find(|n| n.ends_with(suffix))
            .map(|n| n.to_string())
    };
    let plain = |asset: Option<String>, label: &str| -> Option<RuntimePick> {
        asset.map(|a| RuntimePick {
            asset: a,
            companion: None,
            label: label.to_string(),
        })
    };
    match (os, arch) {
        (TargetOs::Windows, TargetArch::X64) => match vendor {
            GpuVendor::Nvidia => {
                let major = if gpu.map(|g| nvidia_wants_cuda13(&g.name)).unwrap_or(false) {
                    "13"
                } else {
                    "12"
                };
                if let Some((asset, ver)) = best_cuda_asset(names, major) {
                    // The CUDA zips do NOT bundle the CUDA runtime DLLs — the release carries them
                    // as a separate version-matched `cudart-…` zip that must land beside the exe.
                    let companion = names
                        .iter()
                        .find(|n| n.starts_with("cudart-") && n.contains(&format!("cuda-{ver}-")))
                        .map(|n| n.to_string());
                    return Some(RuntimePick {
                        asset,
                        companion,
                        label: format!("CUDA {ver} (NVIDIA)"),
                    });
                }
                plain(by_suffix("-bin-win-vulkan-x64.zip"), "Vulkan (NVIDIA)")
                    .or_else(|| plain(by_suffix("-bin-win-cpu-x64.zip"), "CPU"))
            }
            GpuVendor::Amd => plain(by_suffix("-bin-win-vulkan-x64.zip"), "Vulkan (AMD)")
                .or_else(|| plain(by_suffix("-bin-win-cpu-x64.zip"), "CPU")),
            GpuVendor::Intel => plain(by_suffix("-bin-win-vulkan-x64.zip"), "Vulkan (Intel)")
                .or_else(|| plain(by_suffix("-bin-win-cpu-x64.zip"), "CPU")),
            GpuVendor::Apple | GpuVendor::None => plain(by_suffix("-bin-win-cpu-x64.zip"), "CPU"),
        },
        (TargetOs::Windows, TargetArch::Arm64) => {
            plain(by_suffix("-bin-win-cpu-arm64.zip"), "CPU (ARM64)")
        }
        (TargetOs::Mac, TargetArch::Arm64) => plain(
            by_suffix("-bin-macos-arm64.tar.gz"),
            "Metal (Apple Silicon)",
        ),
        (TargetOs::Mac, TargetArch::X64) => {
            plain(by_suffix("-bin-macos-x64.tar.gz"), "CPU (Intel Mac)")
        }
        (TargetOs::Linux, TargetArch::X64) => match vendor {
            GpuVendor::None => plain(by_suffix("-bin-ubuntu-x64.tar.gz"), "CPU"),
            _ => plain(by_suffix("-bin-ubuntu-vulkan-x64.tar.gz"), "Vulkan (GPU)")
                .or_else(|| plain(by_suffix("-bin-ubuntu-x64.tar.gz"), "CPU")),
        },
        (TargetOs::Linux, TargetArch::Arm64) => match vendor {
            GpuVendor::None => plain(by_suffix("-bin-ubuntu-arm64.tar.gz"), "CPU (ARM64)"),
            _ => plain(
                by_suffix("-bin-ubuntu-vulkan-arm64.tar.gz"),
                "Vulkan (GPU, ARM64)",
            )
            .or_else(|| plain(by_suffix("-bin-ubuntu-arm64.tar.gz"), "CPU (ARM64)")),
        },
    }
}

/// [`pick_runtime_asset_for`] on the running platform.
pub fn pick_runtime_asset(names: &[&str], gpu: Option<&GpuInfo>) -> Option<RuntimePick> {
    let (os, arch) = current_platform();
    pick_runtime_asset_for(os, arch, names, gpu)
}

/// One downloadable asset of a llama.cpp release.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeAsset {
    pub name: String,
    pub url: String,
    pub size: u64,
}

/// A llama.cpp release: its tag + assets.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRelease {
    pub tag: String,
    pub assets: Vec<RuntimeAsset>,
}

/// Parse the GitHub `releases/latest` response. Pure; unit-tested against a fixture.
pub fn parse_runtime_release(json: &str) -> Option<RuntimeRelease> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let tag = v.get("tag_name")?.as_str()?.to_string();
    let assets = v
        .get("assets")?
        .as_array()?
        .iter()
        .filter_map(|a| {
            Some(RuntimeAsset {
                name: a.get("name")?.as_str()?.to_string(),
                url: a.get("browser_download_url")?.as_str()?.to_string(),
                size: a.get("size").and_then(|s| s.as_u64()).unwrap_or(0),
            })
        })
        .collect();
    Some(RuntimeRelease { tag, assets })
}

/// GET the latest ggml-org/llama.cpp release from the GitHub API — **thin I/O, not unit-tested**
/// (the parser is). Same curl discipline as the HF calls: System32-pinned on Windows, bounded by
/// `--max-time`/`--max-filesize`, descriptive UA. `None` = offline / rate-limited / drifted.
pub fn fetch_runtime_release() -> Option<RuntimeRelease> {
    let out = no_window(std::process::Command::new(system_cmd("curl.exe", "curl")))
        .args([
            "-sS",
            "-L",
            "--max-time",
            "20",
            "--max-filesize",
            "3000000",
            "-A",
            HF_USER_AGENT,
            "https://api.github.com/repos/ggml-org/llama.cpp/releases/latest",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    parse_runtime_release(&String::from_utf8_lossy(&out.stdout))
}

/// Download `url` to `dest` with curl (`-L --fail`, generous cap for the multi-hundred-MB CUDA
/// bundles). Returns a human-readable reason on failure.
fn download_to(url: &str, dest: &std::path::Path) -> Result<(), String> {
    let status = no_window(std::process::Command::new(system_cmd("curl.exe", "curl")))
        .args([
            "-sS",
            "-L",
            "--fail",
            "--max-time",
            "3600",
            "--max-filesize",
            "800000000",
            "-A",
            HF_USER_AGENT,
            "-o",
            &dest.to_string_lossy(),
            url,
        ])
        .status()
        .map_err(|e| format!("couldn't run curl ({e})"))?;
    if !status.success() {
        return Err(format!("download failed ({status})"));
    }
    Ok(())
}

/// Extract `archive` into `dir` with the system `tar` — which reads BOTH formats the release ships
/// (`.tar.gz` natively everywhere; Windows' System32 `tar.exe` is bsdtar, which also reads `.zip`).
/// Zero new dependencies, same pinned-path discipline as every other subprocess here.
fn extract_archive(archive: &std::path::Path, dir: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("couldn't create {} ({e})", dir.display()))?;
    let status = no_window(std::process::Command::new(system_cmd("tar.exe", "tar")))
        .args([
            "-xf",
            &archive.to_string_lossy(),
            "-C",
            &dir.to_string_lossy(),
        ])
        .status()
        .map_err(|e| format!("couldn't run tar ({e})"))?;
    if !status.success() {
        return Err(format!("extraction failed ({status})"));
    }
    Ok(())
}

/// Make sure a backend-correct `llama-server` exists in the app-managed runtime root, downloading
/// and laying one down if needed. Returns the path to the binary. `progress` receives short
/// human-readable lines as the steps run (the CLI prints them; the UI feeds its loader).
///
/// Steps: already laid down → done. Else: latest release → pick the asset for this OS/arch/GPU →
/// download → extract into `runtime/<asset-stem>/` → (CUDA) unzip the cudart DLLs beside the exe →
/// self-check with `--version`. Every failure is a `Err(reason)` the caller can show; nothing here
/// panics, and a failure leaves `resolve_llama_server`'s other sources (PATH, winget) untouched.
pub fn ensure_runtime(progress: &mut dyn FnMut(&str)) -> Result<std::path::PathBuf, String> {
    if let Some(bin) = managed_llama_server() {
        progress("llama.cpp runtime already installed");
        return Ok(bin);
    }
    let root = managed_runtime_root().ok_or("couldn't locate a home directory")?;
    std::fs::create_dir_all(&root)
        .map_err(|e| format!("couldn't create {} ({e})", root.display()))?;

    progress("checking the latest llama.cpp release…");
    let release =
        fetch_runtime_release().ok_or("couldn't reach the llama.cpp releases (offline?)")?;
    let names: Vec<&str> = release.assets.iter().map(|a| a.name.as_str()).collect();
    let gpu = detect_gpu();
    let pick = pick_runtime_asset(&names, gpu.as_ref())
        .ok_or_else(|| format!("release {} has no build for this machine", release.tag))?;
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == pick.asset)
        .ok_or("picked asset vanished from the release")?;

    // Lay the build down under a dir named for the asset, so what is installed is legible on disk.
    let stem = pick
        .asset
        .trim_end_matches(".zip")
        .trim_end_matches(".tar.gz");
    let dest_dir = root.join(stem);
    let archive = root.join(&pick.asset);
    progress(&format!(
        "downloading {} — {} ({:.0} MB)…",
        release.tag,
        pick.label,
        asset.size as f64 / 1_048_576.0
    ));
    download_to(&asset.url, &archive)?;
    progress("unpacking…");
    let extracted = extract_archive(&archive, &dest_dir);
    let _ = std::fs::remove_file(&archive); // best-effort cleanup either way
    extracted?;

    let bin = find_bin_under(&dest_dir, llama_server_bin_name(), 3)
        .ok_or("the archive did not contain llama-server")?;
    // llama-server.exe's own directory is where Windows resolves its DLLs from — the cudart
    // companion must land exactly there, wherever the main archive put the exe.
    if let Some(companion) = &pick.companion {
        let exe_dir = bin.parent().ok_or("no parent dir for llama-server")?;
        let casset = release
            .assets
            .iter()
            .find(|a| &a.name == companion)
            .ok_or("cudart companion vanished from the release")?;
        let carchive = root.join(companion);
        progress(&format!(
            "downloading CUDA runtime DLLs ({:.0} MB)…",
            casset.size as f64 / 1_048_576.0
        ));
        download_to(&casset.url, &carchive)?;
        let cextracted = extract_archive(&carchive, exe_dir);
        let _ = std::fs::remove_file(&carchive);
        cextracted?;
    }

    // Self-check: a build with a missing DLL or wrong arch dies right here, as a readable error,
    // instead of later inside a serve.
    progress("verifying…");
    let ok = no_window(std::process::Command::new(&bin))
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        return Err(format!(
            "the runtime was laid down at {} but failed its self-check",
            bin.display()
        ));
    }
    progress("llama.cpp runtime ready ✓");
    Ok(bin)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Common test machine — AVX2 present (the usual x86 case). Use `spec_avx2` for the no-AVX2 case.
    fn spec(ram: f64, cores: u32, disk: Option<f64>, vram: Option<f64>) -> DeviceSpec {
        DeviceSpec {
            ram_gb: ram,
            cores,
            disk_free_gb: disk,
            vram_gb: vram,
            avx2: Some(true),
        }
    }

    fn spec_avx2(ram: f64, cores: u32, vram: Option<f64>, avx2: Option<bool>) -> DeviceSpec {
        DeviceSpec {
            ram_gb: ram,
            cores,
            disk_free_gb: None,
            vram_gb: vram,
            avx2,
        }
    }

    #[test]
    fn a_basic_office_box_is_the_floor_with_a_guarded_pick() {
        // 8 GB / 4 cores, no GPU → Low tier; the Low pick is the guardable 4B, and it fits 8 GB.
        let s = spec(8.0, 4, Some(50.0), None);
        assert_eq!(tier_for(&s), Some(Tier::Low));
        assert!(meets_floor(&s));
        match recommend_for(&s) {
            Recommendation::Local(m) => {
                assert_eq!(m.name, "Gemma 3 4B Instruct");
                assert!(model_fits(&s, &m), "the 4B fits the 8 GB floor");
            }
            other => panic!("expected Local, got {other:?}"),
        }
    }

    #[test]
    fn sixteen_gigs_no_gpu_reaches_medium_not_strong() {
        let s = spec(16.0, 6, Some(60.0), None);
        assert_eq!(tier_for(&s), Some(Tier::Medium));
    }

    #[test]
    fn strong_requires_a_confirmed_gpu() {
        assert_eq!(
            tier_for(&spec(32.0, 8, Some(80.0), None)),
            Some(Tier::Medium)
        );
        assert_eq!(
            tier_for(&spec(32.0, 8, Some(80.0), Some(24.0))),
            Some(Tier::Strong)
        );
    }

    #[test]
    fn a_24gb_gpu_is_strong_and_ultra_needs_a_32gb_card() {
        // Same big-RAM box: a 24 GB card is Strong; a 32 GB card (RTX 5090) is Ultra. This is the
        // corrected ceiling — 24 GB is no longer Ultra.
        let s24 = spec(128.0, 16, Some(400.0), Some(24.0));
        let s32 = spec(128.0, 16, Some(400.0), Some(32.0));
        assert_eq!(tier_for(&s24), Some(Tier::Strong));
        assert_eq!(tier_for(&s32), Some(Tier::Ultra));
        // And the workstation actually fits the 120B (65 GB <= 128 * 0.8).
        assert!(
            matches!(recommend_for(&s32), Recommendation::Local(m) if m.name == "GPT-OSS 120B")
        );
    }

    #[test]
    fn below_floor_returns_no_local_not_a_slow_model() {
        // 4 GB / 2 cores is below the floor → no local model; route to Raw/Live (the reframe).
        let s = spec(4.0, 2, Some(20.0), None);
        assert_eq!(tier_for(&s), None);
        assert!(!meets_floor(&s));
        match recommend_for(&s) {
            Recommendation::NoLocal { reason, fallback } => {
                assert!(matches!(reason, NoLocalReason::BelowRam { .. }));
                assert_eq!(fallback, Fallback::RawOrLive);
            }
            other => panic!("expected NoLocal, got {other:?}"),
        }
    }

    #[test]
    fn no_avx2_is_below_floor_regardless_of_ram() {
        // A capable-on-paper 16 GB / 8-core x86 box without AVX2 still can't run llama.cpp → NoLocal.
        let s = spec_avx2(16.0, 8, Some(16.0), Some(false));
        assert_eq!(tier_for(&s), None);
        assert!(matches!(
            recommend_for(&s),
            Recommendation::NoLocal {
                reason: NoLocalReason::NoAvx2,
                ..
            }
        ));
        // avx2 == None (Apple Silicon / ARM) never blocks.
        assert!(tier_for(&spec_avx2(16.0, 8, Some(16.0), None)).is_some());
    }

    #[test]
    fn an_unknown_disk_never_blocks_a_pick() {
        assert_eq!(
            tier_for(&spec(32.0, 8, None, Some(24.0))),
            Some(Tier::Strong)
        );
    }

    #[test]
    fn more_ram_never_lowers_the_tier() {
        // Monotonicity: sweeping RAM up (GPU fixed generous) never demotes the tier.
        let mut last: Option<Tier> = None;
        for ram in [8.0, 12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0, 128.0] {
            let t = tier_for(&spec(ram, 16, Some(400.0), Some(32.0)));
            assert!(t >= last, "tier dropped at {ram} GB: {last:?} -> {t:?}");
            last = t;
        }
    }

    #[test]
    fn unified_laptop_fits_the_pick_to_ram_not_just_the_tier() {
        // 128 GB unified Mac: budget 0.7*128 = 89.6 (>= 32 Ultra gate); the 120B (65 GB) fits
        // 128*0.8 = 102.4 → Ultra pick. Reframe #3: a big laptop is not capped.
        let big = spec(128.0, 12, Some(400.0), Some(128.0 * UNIFIED_VRAM_FRACTION));
        assert_eq!(tier_for(&big), Some(Tier::Ultra));
        assert!(
            matches!(recommend_for(&big), Recommendation::Local(m) if m.name == "GPT-OSS 120B")
        );
        // 64 GB unified Mac: budget 44.8 gates Ultra, BUT the 120B (65 GB) does NOT fit 64*0.8 = 51.2
        // → fit-the-pick drops to the largest that fits, Strong's 35B (24 GB <= 51.2).
        let mid = spec(64.0, 12, Some(200.0), Some(64.0 * UNIFIED_VRAM_FRACTION));
        assert_eq!(tier_for(&mid), Some(Tier::Ultra), "gates Ultra");
        match recommend_for(&mid) {
            Recommendation::Local(m) => {
                assert_ne!(m.name, "GPT-OSS 120B", "the 120B can't fit a 64 GB machine");
                assert_eq!(m.name, "Qwen3.5 35B-A3B", "drops to the largest that fits");
            }
            other => panic!("expected Local, got {other:?}"),
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
    fn every_advertised_pick_is_guardable_4b_plus_and_subfloor_is_3b() {
        for m in all_models() {
            let b: u32 = m.params.trim_end_matches('B').parse().unwrap_or(0);
            assert!(
                b >= 4,
                "advertised pick {} is {} — below the 4B guardable floor",
                m.name,
                m.params
            );
        }
        assert_eq!(
            SUBFLOOR_PICK.params, "3B",
            "the forced-local fallback is the tiny 3B"
        );
    }

    #[test]
    fn reported_ram_and_vram_just_under_nameplate_keep_their_tier() {
        // RAM tolerance: a 32 GB box reads ~31.3, a 16 GB one ~15.7 — must not drop a rung.
        assert_eq!(
            tier_for(&spec(31.3, 8, Some(80.0), Some(24.0))),
            Some(Tier::Strong)
        );
        assert_eq!(
            tier_for(&spec(15.7, 6, Some(60.0), Some(16.0))),
            Some(Tier::Medium)
        );
        // 12 GB is genuinely below a tolerated 16 → not Medium.
        assert_ne!(
            tier_for(&spec(12.0, 6, Some(60.0), Some(16.0))),
            Some(Tier::Medium)
        );
        // VRAM tolerance (the blocker fix): a 24 GB 4090 reports ~23.99 and must still clear Strong;
        // a 32 GB 5090 reports ~31.75 and must still clear Ultra. Without VRAM_TOLERANCE both fail.
        assert_eq!(
            tier_for(&spec(64.0, 12, Some(200.0), Some(23.99))),
            Some(Tier::Strong)
        );
        assert_eq!(
            tier_for(&spec(128.0, 16, Some(400.0), Some(31.75))),
            Some(Tier::Ultra)
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

    #[test]
    fn vram_budget_discrete_unified_and_none() {
        let discrete = GpuInfo {
            name: "RTX".into(),
            vram_gb: 16.0,
            unified: false,
        };
        assert_eq!(vram_budget(Some(&discrete), 32.0), Some(16.0));
        // Unified (Apple Silicon) uses a fraction of RAM, not the whole pool — OS headroom.
        let unified = GpuInfo {
            name: "Apple M3".into(),
            vram_gb: 0.0,
            unified: true,
        };
        assert_eq!(vram_budget(Some(&unified), 32.0), Some(32.0 * 0.7));
        // No GPU → unknown budget.
        assert_eq!(vram_budget(None, 32.0), None);
    }

    #[test]
    fn quant_from_filename_handles_shards_ud_and_model_names() {
        assert_eq!(
            quant_from_filename("gpt-oss-20b-Q4_K_M.gguf").as_deref(),
            Some("Q4_K_M")
        );
        assert_eq!(
            quant_from_filename("model-F16.gguf").as_deref(),
            Some("F16")
        );
        assert_eq!(
            quant_from_filename("m-UD-Q4_K_XL.gguf").as_deref(),
            Some("Q4_K_XL")
        );
        // Sharded: the shard suffix (…-00001-of-00002) is skipped, the quant is still found.
        assert_eq!(
            quant_from_filename("gpt-oss-120b-Q4_K_M-00001-of-00002.gguf").as_deref(),
            Some("Q4_K_M")
        );
        // A model name starting with Q+letter (qwen2.5) is NOT mistaken for a quant.
        assert_eq!(
            quant_from_filename("qwen2.5-9b-Q5_K_M.gguf").as_deref(),
            Some("Q5_K_M")
        );
        // DOT separator before the quant (MaziyarPanahi/TheBloke style) — must still parse.
        assert_eq!(
            quant_from_filename("Qwen3-14B.Q4_K_M.gguf").as_deref(),
            Some("Q4_K_M")
        );
        assert_eq!(
            quant_from_filename("Meta-Llama-3.1-8B-Instruct.Q6_K.gguf").as_deref(),
            Some("Q6_K")
        );
        assert_eq!(quant_from_filename("README.md"), None);
    }

    #[test]
    fn parse_repo_tree_sums_shards_and_ignores_non_gguf() {
        let json = r#"[
            {"path":"gpt-oss-20b-Q4_K_M.gguf","lfs":{"size":12884901888}},
            {"path":"gpt-oss-20b-Q8_0.gguf","size":23622320128},
            {"path":"gpt-oss-20b-F16-00001-of-00002.gguf","lfs":{"size":21474836480}},
            {"path":"gpt-oss-20b-F16-00002-of-00002.gguf","lfs":{"size":21474836480}},
            {"path":"README.md","size":1234}
        ]"#;
        let opts = parse_repo_tree(json);
        let get = |q: &str| opts.iter().find(|o| o.quant == q).map(|o| o.size_gb);
        assert!((get("Q4_K_M").unwrap() - 12.0).abs() < 0.1);
        assert!((get("Q8_0").unwrap() - 22.0).abs() < 0.1);
        assert!((get("F16").unwrap() - 40.0).abs() < 0.2, "shards summed");
        assert_eq!(opts.len(), 3, "README ignored");
    }

    #[test]
    fn best_gguf_for_largest_that_fits_and_skips_full_precision() {
        let files = vec![
            GgufOption {
                quant: "Q4_K_M".into(),
                size_gb: 12.0,
            },
            GgufOption {
                quant: "Q5_K_M".into(),
                size_gb: 14.0,
            },
            GgufOption {
                quant: "Q6_K".into(),
                size_gb: 16.0,
            },
            GgufOption {
                quant: "Q8_0".into(),
                size_gb: 22.0,
            },
            GgufOption {
                quant: "F16".into(),
                size_gb: 40.0,
            },
        ];
        // Budget 13.6 → only Q4_K_M fits (Q5_K_M is 14); the highest-quality quant under budget.
        assert_eq!(best_gguf_for(&files, 13.6).unwrap().quant, "Q4_K_M");
        // A big budget picks Q8_0 (largest quant), NOT F16 (full precision is excluded).
        assert_eq!(best_gguf_for(&files, 100.0).unwrap().quant, "Q8_0");
        // Nothing fits a tiny budget → the smallest quant (least likely to swap).
        assert_eq!(best_gguf_for(&files, 5.0).unwrap().quant, "Q4_K_M");
        // Only full precision → last resort.
        let f16 = vec![GgufOption {
            quant: "F16".into(),
            size_gb: 40.0,
        }];
        assert_eq!(best_gguf_for(&f16, 100.0).unwrap().quant, "F16");
        assert_eq!(best_gguf_for(&[], 100.0), None);
    }

    #[test]
    fn device_budget_prefers_vram_then_ram() {
        let s = spec(31.0, 24, Some(100.0), Some(16.0));
        let discrete = GpuInfo {
            name: "RTX 5080".into(),
            vram_gb: 16.0,
            unified: false,
        };
        assert!(
            (device_model_budget_gb(&s, Some(&discrete)) - 16.0 * VRAM_USABLE_FRACTION).abs()
                < 0.01
        );
        assert!((device_model_budget_gb(&s, None) - 31.0 * FIT_FRACTION).abs() < 0.01);
        let unified = GpuInfo {
            name: "Apple M3".into(),
            vram_gb: 0.0,
            unified: true,
        };
        assert!((device_model_budget_gb(&s, Some(&unified)) - 31.0 * FIT_FRACTION).abs() < 0.01);
    }

    #[test]
    fn quant_rank_orders_by_quality() {
        assert!(quant_rank("Q8_0") > quant_rank("Q6_K"));
        assert!(quant_rank("Q6_K") > quant_rank("Q5_K_M"));
        assert!(quant_rank("Q5_K_M") > quant_rank("Q4_K_M"));
        assert!(quant_rank("Q4_K_M") > quant_rank("Q4_K_S"));
        assert!(quant_rank("Q4_K_M") > quant_rank("Q3_K_M"));
        // Full precision outranks any quant (but best_gguf_for excludes it from the pick).
        assert!(quant_rank("F16") > quant_rank("Q8_0"));
    }

    #[test]
    fn best_gguf_prefers_quality_not_size_when_sizes_are_close() {
        // GPT-OSS-like: every quant is ~11 GB. Q5_K_M should win over the slightly-BIGGER but
        // lower-quality Q2_K_L — the bug that picked a big-by-size quant that then OOM'd.
        let files = vec![
            GgufOption {
                quant: "Q2_K_L".into(),
                size_gb: 10.95,
            },
            GgufOption {
                quant: "Q5_K_M".into(),
                size_gb: 10.91,
            },
            GgufOption {
                quant: "Q4_K_M".into(),
                size_gb: 10.83,
            },
            GgufOption {
                quant: "Q8_0".into(),
                size_gb: 12.29,
            },
        ];
        // Budget 11.0 → Q8_0 (12.29) doesn't fit; the highest-quality quant under budget is Q5_K_M,
        // NOT the marginally-bigger Q2_K_L (that was the OOM bug: size-ranked, not quality-ranked).
        assert_eq!(best_gguf_for(&files, 11.0).unwrap().quant, "Q5_K_M");
        // A tighter budget where even Q5_K_M (10.91) is out: the pick steps down to Q4_K_M (10.83),
        // NOT the fallback smallest-by-size — quality ranking still governs among what fits.
        assert_eq!(best_gguf_for(&files, 10.88).unwrap().quant, "Q4_K_M");
    }

    #[test]
    fn parse_list_devices_reads_cuda_and_vulkan() {
        // Real CUDA build output (with a leading timestamped log line that must be ignored).
        let cuda = "0.02.5 E ggml_cuda_init: found 1 device\nAvailable devices:\n  CUDA0: NVIDIA GeForce RTX 5080 Laptop GPU (16302 MiB, 15067 MiB free)\n";
        let d = parse_list_devices(cuda);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].token, "CUDA0");
        assert_eq!(d[0].backend, Backend::Cuda);
        assert_eq!(d[0].name, "NVIDIA GeForce RTX 5080 Laptop GPU");
        assert_eq!(d[0].total_mib, 16302);
        assert_eq!(d[0].free_mib, 15067);
        // The winget Vulkan build on the hybrid laptop — the iGPU trap.
        let vk = "Available devices:\n  Vulkan0: AMD Radeon(TM) 890M Graphics (16276 MiB, 15462 MiB free)\n";
        let d = parse_list_devices(vk);
        assert_eq!(d[0].backend, Backend::Vulkan);
        assert_eq!(d[0].name, "AMD Radeon(TM) 890M Graphics");
    }

    #[test]
    fn select_device_prefers_discrete_cuda_over_igpu_vulkan() {
        // A hybrid where a multi-backend build sees BOTH: must pick the CUDA dGPU, never the Vulkan iGPU.
        let devs = vec![
            Device {
                token: "Vulkan0".into(),
                backend: Backend::Vulkan,
                name: "AMD Radeon 890M".into(),
                total_mib: 16276,
                free_mib: 15462,
            },
            Device {
                token: "CUDA0".into(),
                backend: Backend::Cuda,
                name: "RTX 5080".into(),
                total_mib: 16302,
                free_mib: 15067,
            },
        ];
        assert_eq!(select_device(&devs).unwrap().token, "CUDA0");
        // Two CUDA devices → the higher-VRAM one.
        let two = vec![
            Device {
                token: "CUDA0".into(),
                backend: Backend::Cuda,
                name: "A".into(),
                total_mib: 8192,
                free_mib: 8000,
            },
            Device {
                token: "CUDA1".into(),
                backend: Backend::Cuda,
                name: "B".into(),
                total_mib: 24576,
                free_mib: 24000,
            },
        ];
        assert_eq!(select_device(&two).unwrap().token, "CUDA1");
        assert_eq!(select_device(&[]), None);
    }

    #[test]
    fn select_device_picks_discrete_over_igpu_within_vulkan() {
        // The real post-reboot case: a Vulkan build sees BOTH GPUs, and the AMD iGPU reports MORE
        // "VRAM" (shared RAM) than the discrete NVIDIA — must still pick the NVIDIA (dedicated).
        let devs = vec![
            Device {
                token: "Vulkan0".into(),
                backend: Backend::Vulkan,
                name: "AMD Radeon(TM) 890M Graphics".into(),
                total_mib: 16276,
                free_mib: 15462,
            },
            Device {
                token: "Vulkan1".into(),
                backend: Backend::Vulkan,
                name: "NVIDIA GeForce RTX 5080 Laptop GPU".into(),
                total_mib: 16003,
                free_mib: 15235,
            },
        ];
        assert_eq!(select_device(&devs).unwrap().token, "Vulkan1");
        assert_eq!(
            name_discreteness_rank("NVIDIA GeForce RTX 5080 Laptop GPU"),
            2
        );
        assert_eq!(name_discreteness_rank("AMD Radeon(TM) 890M Graphics"), 0);
        assert_eq!(name_discreteness_rank("Intel(R) UHD Graphics"), 0);
        assert_eq!(name_discreteness_rank("AMD Radeon RX 7900 XTX"), 2);
    }

    #[test]
    fn gpu_serve_args_pins_the_device() {
        let cuda = Device {
            token: "CUDA0".into(),
            backend: Backend::Cuda,
            name: "x".into(),
            total_mib: 16302,
            free_mib: 15067,
        };
        assert_eq!(
            gpu_serve_args(Some(&cuda)),
            vec!["-ngl", "99", "-dev", "CUDA0"]
        );
        let metal = Device {
            token: "Metal0".into(),
            backend: Backend::Metal,
            name: "m".into(),
            total_mib: 0,
            free_mib: 0,
        };
        assert_eq!(gpu_serve_args(Some(&metal)), vec!["-ngl", "99"]); // Metal: no -dev
        assert!(gpu_serve_args(None).is_empty()); // CPU-only: no GPU flags
    }

    // ─── ensure_runtime: the pure pick logic ─────────────────────────────────────────────────────

    /// The REAL asset list of release b10064 (captured 2026-07-17), so the pick table is tested
    /// against what the release actually ships, not what we hope it ships.
    const B10064: [&str; 25] = [
        "cudart-llama-bin-win-cuda-12.4-x64.zip",
        "cudart-llama-bin-win-cuda-13.3-x64.zip",
        "llama-b10064-bin-android-arm64.tar.gz",
        "llama-b10064-bin-macos-arm64.tar.gz",
        "llama-b10064-bin-macos-x64.tar.gz",
        "llama-b10064-bin-ubuntu-arm64.tar.gz",
        "llama-b10064-bin-ubuntu-openvino-2026.2.1-x64.tar.gz",
        "llama-b10064-bin-ubuntu-rocm-7.2-x64.tar.gz",
        "llama-b10064-bin-ubuntu-s390x.tar.gz",
        "llama-b10064-bin-ubuntu-sycl-fp16-x64.tar.gz",
        "llama-b10064-bin-ubuntu-sycl-fp32-x64.tar.gz",
        "llama-b10064-bin-ubuntu-vulkan-arm64.tar.gz",
        "llama-b10064-bin-ubuntu-vulkan-x64.tar.gz",
        "llama-b10064-bin-ubuntu-x64.tar.gz",
        "llama-b10064-bin-win-cpu-arm64.zip",
        "llama-b10064-bin-win-cpu-x64.zip",
        "llama-b10064-bin-win-cuda-12.4-x64.zip",
        "llama-b10064-bin-win-cuda-13.3-x64.zip",
        "llama-b10064-bin-win-hip-radeon-x64.zip",
        "llama-b10064-bin-win-opencl-adreno-arm64.zip",
        "llama-b10064-bin-win-openvino-2026.2.1-x64.zip",
        "llama-b10064-bin-win-sycl-x64.zip",
        "llama-b10064-bin-win-vulkan-x64.zip",
        "llama-b10064-ui.tar.gz",
        "llama-b10064-xcframework.zip",
    ];

    fn gpu_named(name: &str) -> GpuInfo {
        GpuInfo {
            name: name.into(),
            vram_gb: 16.0,
            unified: false,
        }
    }

    #[test]
    fn vendors_classify_by_name() {
        assert_eq!(
            gpu_vendor(Some(&gpu_named("NVIDIA GeForce RTX 5080 Laptop GPU"))),
            GpuVendor::Nvidia
        );
        assert_eq!(
            gpu_vendor(Some(&gpu_named("AMD Radeon RX 7900 XTX"))),
            GpuVendor::Amd
        );
        assert_eq!(
            gpu_vendor(Some(&gpu_named("Intel(R) Arc(TM) A770 Graphics"))),
            GpuVendor::Intel
        );
        let apple = GpuInfo {
            name: "Apple M3 Max".into(),
            vram_gb: 0.0,
            unified: true,
        };
        assert_eq!(gpu_vendor(Some(&apple)), GpuVendor::Apple);
        assert_eq!(gpu_vendor(None), GpuVendor::None);
    }

    #[test]
    fn cuda_major_follows_the_card_generation() {
        // Blackwell (RTX 50-series, RTX PRO) exists only in CUDA 13; Pascal exists only in ≤ 12.
        assert!(nvidia_wants_cuda13("NVIDIA GeForce RTX 5080 Laptop GPU"));
        assert!(nvidia_wants_cuda13("NVIDIA RTX PRO 6000 Blackwell"));
        assert!(!nvidia_wants_cuda13("NVIDIA GeForce RTX 4090"));
        assert!(!nvidia_wants_cuda13("NVIDIA GeForce GTX 1080 Ti"));
        assert!(!nvidia_wants_cuda13("Tesla V100-SXM2-16GB"));
        // The Ada laptop-workstation "RTX 500" is < 5000 → 12; "RTX 5000 Ada" ≥ 5000 → 13, and
        // CUDA 13 kept Ada, so both roads are safe.
        assert!(!nvidia_wants_cuda13(
            "NVIDIA RTX 500 Ada Generation Laptop GPU"
        ));
        assert!(nvidia_wants_cuda13("NVIDIA RTX 5000 Ada Generation"));
    }

    #[test]
    fn windows_nvidia_gets_cuda_plus_cudart() {
        // A Blackwell card: the CUDA 13 build + its version-matched cudart bundle.
        let p = pick_runtime_asset_for(
            TargetOs::Windows,
            TargetArch::X64,
            &B10064,
            Some(&gpu_named("NVIDIA GeForce RTX 5080 Laptop GPU")),
        )
        .unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-win-cuda-13.3-x64.zip");
        assert_eq!(
            p.companion.as_deref(),
            Some("cudart-llama-bin-win-cuda-13.3-x64.zip")
        );
        // An Ada card: CUDA 12 line, 12-matched cudart.
        let p = pick_runtime_asset_for(
            TargetOs::Windows,
            TargetArch::X64,
            &B10064,
            Some(&gpu_named("NVIDIA GeForce RTX 4070")),
        )
        .unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-win-cuda-12.4-x64.zip");
        assert_eq!(
            p.companion.as_deref(),
            Some("cudart-llama-bin-win-cuda-12.4-x64.zip")
        );
    }

    #[test]
    fn windows_amd_and_intel_get_vulkan_not_hip_or_sycl() {
        for name in ["AMD Radeon RX 7800 XT", "Intel(R) Arc(TM) B580"] {
            let p = pick_runtime_asset_for(
                TargetOs::Windows,
                TargetArch::X64,
                &B10064,
                Some(&gpu_named(name)),
            )
            .unwrap();
            assert_eq!(p.asset, "llama-b10064-bin-win-vulkan-x64.zip", "{name}");
            assert_eq!(p.companion, None);
        }
    }

    #[test]
    fn no_gpu_means_cpu_never_vulkan() {
        let p = pick_runtime_asset_for(TargetOs::Windows, TargetArch::X64, &B10064, None).unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-win-cpu-x64.zip");
        let p = pick_runtime_asset_for(TargetOs::Linux, TargetArch::X64, &B10064, None).unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-ubuntu-x64.tar.gz");
    }

    #[test]
    fn macs_pick_by_arch() {
        let apple = GpuInfo {
            name: "Apple M2".into(),
            vram_gb: 0.0,
            unified: true,
        };
        let p = pick_runtime_asset_for(TargetOs::Mac, TargetArch::Arm64, &B10064, Some(&apple))
            .unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-macos-arm64.tar.gz");
        let p = pick_runtime_asset_for(TargetOs::Mac, TargetArch::X64, &B10064, None).unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-macos-x64.tar.gz");
    }

    #[test]
    fn linux_gpu_gets_vulkan() {
        let p = pick_runtime_asset_for(
            TargetOs::Linux,
            TargetArch::X64,
            &B10064,
            Some(&gpu_named("AMD Radeon RX 6700 XT")),
        )
        .unwrap();
        assert_eq!(p.asset, "llama-b10064-bin-ubuntu-vulkan-x64.tar.gz");
    }

    #[test]
    fn cuda_minor_versions_compare_numerically() {
        // A hypothetical 12.10 must beat 12.4 — lexicographic comparison would invert this.
        let names = [
            "llama-b1-bin-win-cuda-12.4-x64.zip",
            "llama-b1-bin-win-cuda-12.10-x64.zip",
        ];
        let (asset, ver) = best_cuda_asset(&names, "12").unwrap();
        assert_eq!(asset, "llama-b1-bin-win-cuda-12.10-x64.zip");
        assert_eq!(ver, "12.10");
    }

    #[test]
    fn a_release_without_cuda_falls_back_to_vulkan() {
        let names = [
            "llama-b1-bin-win-vulkan-x64.zip",
            "llama-b1-bin-win-cpu-x64.zip",
        ];
        let p = pick_runtime_asset_for(
            TargetOs::Windows,
            TargetArch::X64,
            &names,
            Some(&gpu_named("NVIDIA GeForce RTX 4090")),
        )
        .unwrap();
        assert_eq!(p.asset, "llama-b1-bin-win-vulkan-x64.zip");
    }

    #[test]
    fn release_json_parses_tag_and_assets() {
        let json = r#"{
            "tag_name": "b10064",
            "assets": [
                {"name": "llama-b10064-bin-win-cpu-x64.zip",
                 "browser_download_url": "https://example.invalid/a.zip",
                 "size": 18000000},
                {"name": "no-url-asset"}
            ]
        }"#;
        let r = parse_runtime_release(json).unwrap();
        assert_eq!(r.tag, "b10064");
        assert_eq!(r.assets.len(), 1, "an asset without a URL is skipped");
        assert_eq!(r.assets[0].name, "llama-b10064-bin-win-cpu-x64.zip");
        assert_eq!(r.assets[0].size, 18_000_000);
        assert!(parse_runtime_release("not json").is_none());
    }

    #[test]
    fn find_bin_handles_flat_and_nested_layouts() {
        // The Windows zips are flat; the macOS/Linux tarballs nest `llama-bNNNNN/`. Both must
        // resolve, and the depth cap must hold.
        let root = std::env::temp_dir().join(format!("ziqpu-find-bin-{}", std::process::id()));
        let nested = root
            .join("llama-b10064-bin-macos-arm64")
            .join("llama-b10064");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("llama-server"), b"stub").unwrap();
        let found = find_bin_under(&root, "llama-server", 3).expect("nested layout resolves");
        assert!(
            found.ends_with("llama-b10064/llama-server")
                || found.ends_with("llama-b10064\\llama-server")
        );
        // Too deep for the cap → not found (the cap is what keeps a bad root from a disk crawl).
        let deep = root.join("a").join("b").join("c").join("d");
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("other-bin"), b"stub").unwrap();
        assert_eq!(find_bin_under(&root, "other-bin", 3), None);
        let _ = std::fs::remove_dir_all(&root);
    }
}
