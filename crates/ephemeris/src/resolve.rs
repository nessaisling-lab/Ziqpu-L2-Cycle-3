//! Which ephemeris engine computes a chart — and the honest reason when it isn't the good one.
//!
//! # The gap this closes
//!
//! Until this module existed there was no *selection* at all. `crates/agents/src/measure.rs` named
//! [`AnalyticBackend`] directly, so a build compiled with `anise-backend` **and** a DE440 kernel
//! sitting on disk still computed every chart from VSOP87 series. The high-accuracy backend had a
//! CI cross-check, a fetch script, and no caller.
//!
//! That was not only a precision question. The analytic backend cannot produce Pluto at all — it
//! returns an error, `compute_chart` skips the body, and the contact list comes out one planet
//! short. Every chart this app has ever shown a seeker was missing Pluto, silently, and a synastry
//! score is a sum over contacts: a body that never appears contributes nothing and no one is told.
//!
//! # The ladder
//!
//! [`EngineSource`] is deliberately shaped like `GroundedRung` one crate over. A degraded chart is
//! still a chart and still gets computed — but the reason is carried as data, not swallowed, so the
//! surface above can say which engine ran. The alternative designs were both wrong in ways this
//! project has already been burned by:
//!
//! - **Hard-fail on a missing or mismatched kernel** — one republished file at NAIF and every chart
//!   in the app stops working.
//! - **Fall back silently** — the exact defect fixed twice in one day in the interpreter: a chosen
//!   thing quietly replaced by a lesser thing, with the output still looking authoritative.
//!
//! # Why the digest is pinned
//!
//! The kernel is 32 MB of binary that is read and trusted to produce numbers a person makes a
//! decision against. Pinning its SHA-256 is the same discipline the model crate applies to the
//! llama.cpp runtime it downloads, for the same reason: a file that arrives over the network and
//! then drives output has to be the file that was reviewed.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::{AnalyticBackend, Ephemeris};

/// SHA-256 of `de440s.bsp` as published by NAIF and as cross-checked in CI.
///
/// DE440s ("small") covers 1849–2150 in ~32 MB — the full DE440 is over 100 MB and buys nothing for
/// a product whose subjects are companies, vehicles, and people with modern dates.
pub const DE440S_SHA256: &str = "c1c7feeab882263fc493a9d5a5b2ddd71b54826cdf65d8d17a76126b260a49f2";

/// The kernel file name, in every location it might live.
pub const KERNEL_FILE: &str = "de440s.bsp";

/// Why a chart fell back to the analytic floor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalyticReason {
    /// This build has no DE440 support compiled in (`anise-backend` off).
    NotCompiled,
    /// No kernel at any searched location — carries the paths tried, because "not found" without
    /// "where I looked" is the least actionable diagnostic there is.
    KernelMissing(Vec<PathBuf>),
    /// A kernel is there, and it is not the one that was reviewed.
    DigestMismatch { path: PathBuf, found: String },
    /// The digest matched and ANISE still refused it.
    LoadFailed { path: PathBuf, error: String },
    /// The file could not be read to hash it (permissions, a partial download).
    Unreadable { path: PathBuf, error: String },
}

impl AnalyticReason {
    /// One line, for a log or a tooltip. States what is missing and what to do, in that order.
    pub fn explain(&self) -> String {
        match self {
            AnalyticReason::NotCompiled => {
                "this build was compiled without DE440 support".to_string()
            }
            AnalyticReason::KernelMissing(tried) => format!(
                "no {KERNEL_FILE} found (looked in {})",
                tried
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            AnalyticReason::DigestMismatch { path, found } => format!(
                "{} is not the reviewed kernel (sha256 {}…, expected {}…)",
                path.display(),
                &found[..found.len().min(12)],
                &DE440S_SHA256[..12]
            ),
            AnalyticReason::LoadFailed { path, error } => {
                format!("{} could not be loaded: {error}", path.display())
            }
            AnalyticReason::Unreadable { path, error } => {
                format!("{} could not be read: {error}", path.display())
            }
        }
    }
}

/// Which engine computed a chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineSource {
    /// JPL DE440 — the authoritative one, digest-verified.
    De440 { path: PathBuf },
    /// The analytic floor: VSOP87 planets, Meeus Moon, **no Pluto**.
    Analytic(AnalyticReason),
}

impl EngineSource {
    /// Is this the authoritative engine?
    pub fn is_authoritative(&self) -> bool {
        matches!(self, EngineSource::De440 { .. })
    }

    /// A short label for a surface — the ephemeris equivalent of a rung badge.
    pub fn badge(&self) -> &'static str {
        match self {
            EngineSource::De440 { .. } => "JPL DE440",
            EngineSource::Analytic(_) => "ANALYTIC (reduced)",
        }
    }

    /// The full sentence a seeker is owed when the engine is not the authoritative one.
    ///
    /// Names the concrete consequence, not just the substitution. "Reduced accuracy" means nothing
    /// to a reader; "Pluto is missing from this chart" is checkable against what they are looking at.
    pub fn caveat(&self) -> Option<String> {
        match self {
            EngineSource::De440 { .. } => None,
            EngineSource::Analytic(why) => Some(format!(
                "This chart was computed by the fallback engine, so Pluto is absent from it and \
                 every position is approximate — {}.",
                why.explain()
            )),
        }
    }
}

/// A loaded engine and the record of which one it is.
pub struct Loaded {
    pub backend: Box<dyn Ephemeris + Send + Sync>,
    pub source: EngineSource,
}

/// The process-wide engine, loaded once.
///
/// Once, because loading means hashing and memory-mapping 32 MB — per chart that would be absurd,
/// and charts are computed per choice on a ranked list of five.
pub fn shared() -> &'static Loaded {
    static ENGINE: OnceLock<Loaded> = OnceLock::new();
    ENGINE.get_or_init(load)
}

/// Resolve the best available engine. Prefer [`shared`] unless a test needs a fresh resolution.
///
/// Announces the result on **stderr**, once. Never stdout: the MCP server speaks JSON-RPC there and
/// a stray line corrupts the stream — the interpreter's own banner learned this the same way.
pub fn load() -> Loaded {
    let (backend, source) = load_de440();
    if !cfg!(test) {
        match &source {
            EngineSource::De440 { path } => {
                eprintln!("ephemeris: JPL DE440 ({})", path.display())
            }
            EngineSource::Analytic(why) => eprintln!(
                "ephemeris: ANALYTIC FLOOR — no Pluto, positions approximate ({})",
                why.explain()
            ),
        }
    }
    match backend {
        Some(b) => Loaded { backend: b, source },
        None => Loaded {
            backend: Box::new(AnalyticBackend),
            source,
        },
    }
}

/// Where a kernel may live, in precedence order.
///
/// The installed location comes before the repository one so a developer with a checkout does not
/// accidentally test against a different file than the one that ships.
pub fn candidate_paths() -> Vec<PathBuf> {
    // An explicit override REPLACES the search — it does not lead it.
    //
    // It led it at first, and that was wrong in a way that hid itself: pointing the variable at a
    // nonexistent directory to force the floor still found the repository kernel further down the
    // list and computed with DE440 anyway. The variable looked honoured, the banner said DE440, and
    // a check meant to prove "this passes without a kernel" was quietly proving nothing. Anything
    // that cannot be turned off cannot be tested off.
    if let Some(p) = std::env::var_os("ZIQPU_EPHEMERIS_PATH") {
        let p = PathBuf::from(p);
        return vec![if p.extension().is_some() {
            p
        } else {
            p.join(KERNEL_FILE)
        }];
    }

    let mut out = Vec::new();

    // Beside the installed executable — where the release bundle puts it.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            out.push(dir.join("data").join("ephemeris").join(KERNEL_FILE));
            out.push(dir.join(KERNEL_FILE));
            // macOS puts non-code payload in `Contents/Resources`, one level up from
            // `Contents/MacOS` where the executable lives. Packaging it anywhere else inside a
            // .app works but fights the platform, and `codesign` treats the two locations
            // differently — a data file under MacOS/ is inside the signed code directory.
            if let Some(contents) = dir.parent() {
                out.push(contents.join("Resources").join(KERNEL_FILE));
            }
        }
    }

    // 3. The repository layout, for `cargo run` and CI.
    out.push(Path::new("data").join("ephemeris").join(KERNEL_FILE));

    out
}

#[cfg(feature = "anise-backend")]
fn load_de440() -> (Option<Box<dyn Ephemeris + Send + Sync>>, EngineSource) {
    let tried = candidate_paths();
    let Some(path) = tried.iter().find(|p| p.is_file()).cloned() else {
        return (
            None,
            EngineSource::Analytic(AnalyticReason::KernelMissing(tried)),
        );
    };

    let found = match sha256_of(&path) {
        Ok(d) => d,
        Err(error) => {
            return (
                None,
                EngineSource::Analytic(AnalyticReason::Unreadable { path, error }),
            )
        }
    };
    if found != DE440S_SHA256 {
        return (
            None,
            EngineSource::Analytic(AnalyticReason::DigestMismatch { path, found }),
        );
    }

    match crate::AniseBackend::from_kernel(&path.to_string_lossy()) {
        Ok(b) => (Some(Box::new(b)), EngineSource::De440 { path }),
        Err(e) => (
            None,
            EngineSource::Analytic(AnalyticReason::LoadFailed {
                path,
                error: e.to_string(),
            }),
        ),
    }
}

#[cfg(not(feature = "anise-backend"))]
fn load_de440() -> (Option<Box<dyn Ephemeris + Send + Sync>>, EngineSource) {
    (None, EngineSource::Analytic(AnalyticReason::NotCompiled))
}

/// Hash the kernel in fixed-size chunks.
///
/// Chunked rather than `fs::read`: the whole point is a 32 MB file, and reading it into a `Vec` to
/// hash it doubles the peak memory of app startup for no gain.
#[cfg(feature = "anise-backend")]
fn sha256_of(path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    /// Serialises the tests that read or write `ZIQPU_EPHEMERIS_PATH`.
    ///
    /// The environment is process-global and Rust runs tests on threads, so one test setting the
    /// override while another reads it is a race — and it fails intermittently, which is the worst
    /// way for a test to fail. This project has already paid for that once in the credential tests.
    fn env_guard() -> MutexGuard<'static, ()> {
        static LOCK: Mutex<()> = Mutex::new(());
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Run `f` with the override cleared, restoring whatever was there.
    fn without_override<T>(f: impl FnOnce() -> T) -> T {
        let _lock = env_guard();
        let restore = std::env::var_os("ZIQPU_EPHEMERIS_PATH");
        std::env::remove_var("ZIQPU_EPHEMERIS_PATH");
        let out = f();
        if let Some(v) = restore {
            std::env::set_var("ZIQPU_EPHEMERIS_PATH", v);
        }
        out
    }

    #[test]
    fn the_pin_is_a_real_sha256() {
        assert_eq!(DE440S_SHA256.len(), 64);
        assert!(DE440S_SHA256.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(
            DE440S_SHA256.chars().all(|c| !c.is_ascii_uppercase()),
            "lowercase, or the comparison against a formatted digest never matches"
        );
    }

    /// Every path the resolver *derives* must end at the kernel file.
    ///
    /// Scoped to the derived paths on purpose: an override that names a file directly means THAT
    /// file, whatever it is called, which is the point of naming it. An earlier version of this
    /// asserted over the override too and failed the moment someone pointed the variable at
    /// `none.bsp` to force the floor — the test was wrong, not the resolver.
    #[test]
    fn every_derived_path_ends_at_the_kernel() {
        without_override(|| {
            let derived = candidate_paths();
            assert!(
                !derived.is_empty(),
                "there must always be somewhere to look"
            );
            for p in derived {
                assert_eq!(
                    p.file_name().unwrap().to_string_lossy(),
                    KERNEL_FILE,
                    "a derived candidate that is not the kernel file will never match: {}",
                    p.display()
                );
            }
        })
    }

    /// An override naming the directory and one naming the file must both work.
    ///
    /// Both spellings are what people actually type, and the fetch script's own `EPHEMERIS_PATH` is
    /// a *directory* — so accepting only the file path would mean the documented variable and the
    /// runtime one disagree about what they hold.
    #[test]
    fn the_override_accepts_a_directory_or_a_file() {
        let _lock = env_guard();
        let key = "ZIQPU_EPHEMERIS_PATH";
        let restore = std::env::var_os(key);

        std::env::set_var(key, "/somewhere/kernels");
        let dir_form = candidate_paths();
        assert!(dir_form[0].ends_with(Path::new("kernels").join(KERNEL_FILE)));

        std::env::set_var(key, format!("/somewhere/{KERNEL_FILE}"));
        let file_form = candidate_paths();
        assert!(file_form[0].ends_with(KERNEL_FILE));

        // And it REPLACES the search rather than leading it. Pointing this at a path with no kernel
        // must actually yield the floor — otherwise the repository copy is found further down and
        // the override silently does nothing, which is how a "runs without a kernel" check passes
        // while running with one.
        assert_eq!(dir_form.len(), 1, "{dir_form:?}");
        assert_eq!(file_form.len(), 1, "{file_form:?}");

        match restore {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }

    /// The reason must survive to the seeker-facing sentence, and name the consequence.
    #[test]
    fn a_degraded_engine_says_what_is_missing_and_why() {
        let missing = EngineSource::Analytic(AnalyticReason::KernelMissing(vec![PathBuf::from(
            "data/ephemeris/de440s.bsp",
        )]));
        let caveat = missing.caveat().expect("a fallback owes a caveat");
        assert!(caveat.contains("Pluto"), "{caveat}");
        assert!(caveat.contains("de440s.bsp"), "{caveat}");
        assert!(!missing.is_authoritative());

        // And the authoritative engine apologises for nothing.
        let good = EngineSource::De440 {
            path: PathBuf::from("data/ephemeris/de440s.bsp"),
        };
        assert!(good.is_authoritative());
        assert_eq!(good.caveat(), None);
    }

    /// Pluto is the whole argument, so it is what gets asserted.
    ///
    /// Written to say something true in **both** environments rather than skipping when the kernel
    /// is absent. A test that quietly no-ops on CI is how a claim goes unchecked for months — this
    /// project already learned that from a PASS with no case behind it. So: the floor must be
    /// missing Pluto (always checkable), and the resolved engine must either produce Pluto or admit
    /// in its own caveat that it cannot.
    #[test]
    fn the_floor_has_no_pluto_and_says_so() {
        let jd = crate::julian_day(1980, 12, 12, 14.5);

        // The floor, named directly — this half runs everywhere, kernel or not.
        assert!(
            AnalyticBackend.position(crate::Body::Pluto, jd).is_err(),
            "if the analytic backend ever gains Pluto, the caveat that promises its absence is a lie"
        );

        let engine = load();
        match &engine.source {
            EngineSource::De440 { .. } => {
                let pluto = engine
                    .backend
                    .position(crate::Body::Pluto, jd)
                    .expect("DE440 is authoritative precisely because it has Pluto");
                assert!((0.0..360.0).contains(&pluto.longitude));
                assert_eq!(engine.source.caveat(), None);
            }
            EngineSource::Analytic(_) => {
                assert!(engine.backend.position(crate::Body::Pluto, jd).is_err());
                assert!(engine
                    .source
                    .caveat()
                    .expect("the floor owes a caveat")
                    .contains("Pluto"));
            }
        }
    }

    /// A truncated digest must not panic the explanation — the slice is the obvious place to do it.
    #[test]
    fn a_short_digest_does_not_panic_the_explanation() {
        let why = AnalyticReason::DigestMismatch {
            path: PathBuf::from("k.bsp"),
            found: "abc".to_string(),
        };
        assert!(why.explain().contains("abc"));
    }
}
