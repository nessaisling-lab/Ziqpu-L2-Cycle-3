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
        let ram_ok = spec.ram_gb >= self.min_ram_gb;
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
    },
    ModelPick {
        tier: Tier::Weak,
        name: "Qwen3.5 9B Instruct",
        params: "9B",
        quant: "Q4_K_M",
        download_gb: 6.6,
        min_ram_gb: 16.0,
    },
    ModelPick {
        tier: Tier::Medium,
        name: "GPT-OSS 20B",
        params: "20B",
        quant: "MXFP4",
        download_gb: 14.0,
        min_ram_gb: 16.0,
    },
    ModelPick {
        tier: Tier::Strong,
        name: "Qwen3.5 35B-A3B",
        params: "35B",
        quant: "Q4_K_M",
        download_gb: 24.0,
        min_ram_gb: 32.0,
    },
    ModelPick {
        tier: Tier::Ultra,
        name: "GPT-OSS 120B",
        params: "120B",
        quant: "MXFP4",
        download_gb: 65.0,
        min_ram_gb: 64.0,
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

/// Detect the real machine's capability. **Thin I/O, not unit-tested** — RAM via `sysinfo`, cores
/// via the std runtime hint. Disk-free and VRAM are left `None` for now (a later increment adds a
/// cross-platform GPU/disk probe); an unknown VRAM conservatively caps the pick at a CPU-runnable
/// tier, and an unknown disk never blocks a pick.
pub fn detect_spec() -> DeviceSpec {
    use sysinfo::System;
    let mut sys = System::new();
    sys.refresh_memory();
    // sysinfo 0.30+ reports memory in bytes.
    let ram_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let cores = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    DeviceSpec {
        ram_gb,
        cores,
        disk_free_gb: None,
        vram_gb: None,
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
}
