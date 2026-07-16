//! **Traction** — is an open model actually used and still cared for?
//!
//! OpenRouter's catalog says what exists, not what's alive. It publishes no usage or ranking data
//! (its `?order=` parameter is ignored and no rank field exists), so the long tail of abandoned
//! hobby finetunes sits in the list looking exactly like the models people actually run.
//!
//! Hugging Face answers it. Each open catalog entry carries a `hugging_face_id`, and the Hub
//! publishes `downloads`, `likes`, and `lastModified` per repo — real, current, third-party.
//!
//! **Downloads alone would be the wrong filter**, which is why all three signals are used together:
//!
//! | Model | Downloads | Last touched | Reality |
//! |---|---|---|---|
//! | `meta-llama/Llama-3.2-3B-Instruct` | 2.1M | 2024-10 | retiring within days |
//! | `tencent/Hy3` | 11.8K | today | actively developed |
//!
//! Popularity is a lagging indicator — it measures the past. A model is only dropped when it is
//! **unused AND unloved AND untouched**: below [`MIN_DOWNLOADS`], below [`MIN_LIKES`], and stale
//! past [`STALE_DAYS`]. Closed vendor models (Anthropic, OpenAI, Gemini — none carry a
//! `hugging_face_id`) are exempt: they're commercially supported by definition.
//!
//! Fetching all ~150 open repos takes ~3s across [`FETCH_THREADS`] workers, so the result is cached
//! on disk for [`CACHE_TTL_DAYS`] — the picker pays it once a week, not once a click.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

/// Below this many downloads a model is only kept if the community or its authors show it care.
const MIN_DOWNLOADS: u64 = 5_000;
/// Hub likes that count as community endorsement — enough to keep a low-download model listed.
/// Calibrated against the live distribution: it spares MiniMax-Text-01 (3.9K downloads but 656
/// likes, a real lab model) while still dropping the abandoned roleplay finetunes below it.
const MIN_LIKES: u64 = 500;
/// Untouched for this long counts as no longer under development.
const STALE_DAYS: i64 = 365;
/// Concurrent Hub fetches. ~150 repos land in ~3s at 8; higher risks looking like a scraper.
const FETCH_THREADS: usize = 8;
/// How long a cached sweep stays good. Downloads and likes move slowly; a week is plenty.
const CACHE_TTL_DAYS: i64 = 7;

/// What the Hub knows about one repo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Traction {
    pub downloads: u64,
    pub likes: u64,
    /// `YYYY-MM-DD`, as published. `None` when the Hub omits it.
    pub last_modified: Option<String>,
}

impl Traction {
    /// Days since the repo was last touched, or `None` when unknown/unparseable.
    fn staleness_days(&self) -> Option<i64> {
        let raw = self.last_modified.as_deref()?;
        let date = chrono::NaiveDate::parse_from_str(raw, "%Y-%m-%d").ok()?;
        Some((chrono::Utc::now().date_naive() - date).num_days())
    }

    /// Is this repo abandoned — unused, unloved, AND untouched?
    ///
    /// All three must hold. Any one alone is a false signal: a heavily-downloaded model can be
    /// months from retirement, and a brand-new one has barely any downloads yet precisely because
    /// it's new. Unknown staleness is treated as "not stale" — we don't drop on missing data.
    pub fn abandoned(&self) -> bool {
        let stale = self.staleness_days().is_some_and(|d| d > STALE_DAYS);
        self.downloads < MIN_DOWNLOADS && self.likes < MIN_LIKES && stale
    }
}

/// Descriptive User-Agent, matching the `model` crate's Hub convention (SEC-004) — the Hub is
/// entitled to know who's asking.
const HF_USER_AGENT: &str = "Ziqpu/1.0 (+https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3)";
/// A sweep that resolves less than this share of what it asked for was throttled or offline, not
/// informative. Caching it would poison the filter for a week over one bad minute.
const HEALTHY_SWEEP_RATIO: f64 = 0.5;

/// The on-disk sweep.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheFile {
    /// `YYYY-MM-DD` of the sweep.
    fetched_at: String,
    /// Every id the sweep asked about — **including** ones the Hub had no answer for.
    ///
    /// Without this, coverage would be judged by `entries` alone: any id the Hub doesn't resolve
    /// looks permanently "missing", so every call re-sweeps all ~150 repos and hammers the Hub into
    /// rate-limiting us (its policy is 500 requests per 5 minutes — a re-sweep loop burns that in
    /// seconds). Recording what we *asked* lets an unanswered id stay answered-as-nothing.
    #[serde(default)]
    attempted: Vec<String>,
    entries: HashMap<String, Traction>,
}

/// `<cache_dir>/hf_traction.json`.
fn cache_path(dir: &Path) -> PathBuf {
    dir.join("hf_traction.json")
}

/// Read a cached sweep if it exists and is still inside the TTL. Any error (missing, corrupt, old)
/// reads as "no cache" — this is an optimisation, never a source of truth.
fn load_cache(dir: &Path) -> Option<CacheFile> {
    let text = std::fs::read_to_string(cache_path(dir)).ok()?;
    let cache: CacheFile = serde_json::from_str(&text).ok()?;
    let fetched = chrono::NaiveDate::parse_from_str(&cache.fetched_at, "%Y-%m-%d").ok()?;
    let age = (chrono::Utc::now().date_naive() - fetched).num_days();
    (age <= CACHE_TTL_DAYS).then_some(cache)
}

/// Write the sweep. Best-effort — a failed write just means we fetch again next time.
fn save_cache(dir: &Path, attempted: &[String], entries: &HashMap<String, Traction>) {
    let cache = CacheFile {
        fetched_at: chrono::Utc::now()
            .date_naive()
            .format("%Y-%m-%d")
            .to_string(),
        attempted: attempted.to_vec(),
        entries: entries.clone(),
    };
    if let Ok(text) = serde_json::to_string(&cache) {
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(cache_path(dir), text);
    }
}

/// Traction for every id, from cache when fresh, otherwise swept from the Hub and cached.
///
/// `cache_dir` of `None` means "don't persist" (tests, or a machine with no data dir) — the sweep
/// still runs, it just isn't kept. A repo the Hub won't answer for is simply absent from the map,
/// and callers treat absence as "unknown", never as "abandoned".
pub fn for_ids(ids: &[String], cache_dir: Option<&Path>) -> HashMap<String, Traction> {
    if ids.is_empty() {
        return HashMap::new();
    }
    if let Some(dir) = cache_dir {
        if let Some(cached) = load_cache(dir) {
            // Reuse a sweep that ASKED about everything we care about — not one that happens to
            // hold an entry for each. An id the Hub had no answer for is legitimately absent from
            // `entries`; judging coverage by entries alone would re-sweep forever.
            if ids.iter().all(|id| cached.attempted.contains(id)) {
                return cached.entries;
            }
        }
    }
    let fetched = sweep(ids);
    // Only trust a sweep that mostly landed. A throttled or offline run resolves almost nothing —
    // caching that would silently disable the filter for a week (observed: the Hub rate-limits at
    // 500 requests per 5 minutes, and a throttled sweep returns an empty map that looks exactly
    // like "every model is unknown").
    let healthy = fetched.len() as f64 >= ids.len() as f64 * HEALTHY_SWEEP_RATIO;
    if healthy {
        if let Some(dir) = cache_dir {
            save_cache(dir, ids, &fetched);
        }
    }
    fetched
}

/// Fetch every id from the Hub across [`FETCH_THREADS`] workers pulling from a shared queue.
fn sweep(ids: &[String]) -> HashMap<String, Traction> {
    let queue = Arc::new(Mutex::new(ids.to_vec()));
    let out = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for _ in 0..FETCH_THREADS.min(ids.len()) {
        let queue = Arc::clone(&queue);
        let out = Arc::clone(&out);
        handles.push(std::thread::spawn(move || loop {
            // Hold the queue lock only long enough to claim one id — never across the HTTP call.
            let Some(id) = queue.lock().ok().and_then(|mut q| q.pop()) else {
                return;
            };
            if let Some(t) = fetch_one(&id) {
                if let Ok(mut map) = out.lock() {
                    map.insert(id, t);
                }
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    Arc::try_unwrap(out)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default()
}

/// One Hub lookup. `None` on any error — an unreachable repo is "unknown", not "abandoned".
fn fetch_one(id: &str) -> Option<Traction> {
    let url = format!("https://huggingface.co/api/models/{id}");
    let body = crate::llm_http::get_json(&url, &[("user-agent", HF_USER_AGENT)])?;
    parse_hub_model(&body)
}

/// Parse the Hub's model shape. Pure, so the thresholds and field handling are testable offline.
fn parse_hub_model(body: &str) -> Option<Traction> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    Some(Traction {
        downloads: v.get("downloads").and_then(|d| d.as_u64()).unwrap_or(0),
        likes: v.get("likes").and_then(|l| l.as_u64()).unwrap_or(0),
        // The Hub sends a full RFC3339 timestamp; we only need the date.
        last_modified: v
            .get("lastModified")
            .and_then(|m| m.as_str())
            .map(|s| s.chars().take(10).collect()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn days_ago(n: i64) -> String {
        (chrono::Utc::now().date_naive() - chrono::Duration::days(n))
            .format("%Y-%m-%d")
            .to_string()
    }

    #[test]
    fn parses_the_hub_shape_and_trims_the_timestamp() {
        let body = r#"{"id":"meta-llama/Llama-3.2-3B-Instruct","downloads":2103044,
                       "likes":2341,"lastModified":"2024-10-24T15:07:29.000Z"}"#;
        let t = parse_hub_model(body).expect("valid");
        assert_eq!(t.downloads, 2_103_044);
        assert_eq!(t.likes, 2341);
        assert_eq!(t.last_modified.as_deref(), Some("2024-10-24"));
    }

    /// The whole point: no single signal decides. These are the real shapes from the live Hub.
    #[test]
    fn abandoned_needs_unused_and_unloved_and_untouched() {
        // Barely used, barely liked, untouched for years — a genuinely dead roleplay finetune.
        let dead = Traction {
            downloads: 567,
            likes: 145,
            last_modified: Some(days_ago(700)),
        };
        assert!(dead.abandoned());

        // New and small: 11.8K downloads but touched today. Popularity lags; this is alive.
        let new_and_active = Traction {
            downloads: 11_849,
            likes: 811,
            last_modified: Some(days_ago(0)),
        };
        assert!(!new_and_active.abandoned());

        // Tiny downloads, but touched last month — under active development.
        let young = Traction {
            downloads: 200,
            likes: 5,
            last_modified: Some(days_ago(30)),
        };
        assert!(!young.abandoned());

        // Heavily used but ancient — still what people run. Not our call to drop.
        let old_workhorse = Traction {
            downloads: 2_103_044,
            likes: 2341,
            last_modified: Some(days_ago(700)),
        };
        assert!(!old_workhorse.abandoned());

        // Low downloads and stale, but the community clearly rates it (MiniMax-Text-01's real
        // numbers) — "supported in the community" counts as support.
        let loved = Traction {
            downloads: 3_961,
            likes: 656,
            last_modified: Some(days_ago(400)),
        };
        assert!(!loved.abandoned());
    }

    /// Missing data must never be read as abandonment.
    #[test]
    fn unknown_staleness_is_not_abandonment() {
        let t = Traction {
            downloads: 1,
            likes: 0,
            last_modified: None,
        };
        assert!(!t.abandoned());
    }

    fn t(downloads: u64) -> Traction {
        Traction {
            downloads,
            likes: 1,
            last_modified: Some("2026-01-01".into()),
        }
    }

    #[test]
    fn cache_round_trips_and_expires() {
        let dir = std::env::temp_dir().join(format!("ziqpu-traction-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let entries: HashMap<String, Traction> = [("a/b".to_string(), t(10))].into();
        let asked = vec!["a/b".to_string()];

        save_cache(&dir, &asked, &entries);
        assert_eq!(
            load_cache(&dir).map(|c| c.entries),
            Some(entries.clone()),
            "fresh cache reads back"
        );

        // An old sweep is ignored rather than trusted.
        let stale = CacheFile {
            fetched_at: days_ago(CACHE_TTL_DAYS + 1),
            attempted: asked,
            entries,
        };
        std::fs::write(cache_path(&dir), serde_json::to_string(&stale).unwrap()).unwrap();
        assert!(load_cache(&dir).is_none(), "expired cache is ignored");

        // Corrupt is treated as absent, never as a panic.
        std::fs::write(cache_path(&dir), "{not json").unwrap();
        assert!(load_cache(&dir).is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Regression: coverage is judged by what the sweep ASKED, not what it found.
    ///
    /// The Hub legitimately has no answer for some repos, so those ids never appear in `entries`.
    /// Judging coverage by `entries` made the cache look permanently incomplete — every call
    /// re-swept ~150 repos and walked straight into the Hub's 500-per-5-minutes limit, at which
    /// point the sweep returned nothing and the filter silently stopped working.
    #[test]
    fn an_unanswered_id_does_not_invalidate_the_cache() {
        let dir = std::env::temp_dir().join(format!("ziqpu-traction-cov-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        // We asked about two repos; the Hub only answered for one.
        let asked = vec!["known/one".to_string(), "silent/two".to_string()];
        let entries: HashMap<String, Traction> = [("known/one".to_string(), t(1))].into();
        save_cache(&dir, &asked, &entries);

        // Asking again for both must reuse the cache — NOT re-sweep because "silent/two" is absent.
        // (If it re-swept, the network call would fail in this offline test and return an empty
        // map; getting our one entry back proves the cached sweep was reused.)
        let got = for_ids(&asked, Some(&dir));
        assert_eq!(got.len(), 1);
        assert!(got.contains_key("known/one"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A throttled or offline sweep resolves almost nothing. Caching that would look identical to
    /// "every model is unknown" and silently disable the filter for a week.
    #[test]
    fn a_mostly_empty_sweep_is_not_cached() {
        let dir =
            std::env::temp_dir().join(format!("ziqpu-traction-poison-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        // Ids that resolve to nothing offline — the sweep will come back empty, like a 429 storm.
        let ids: Vec<String> = (0..10)
            .map(|i| format!("definitely/not-a-repo-{i}"))
            .collect();
        let got = for_ids(&ids, Some(&dir));
        assert!(got.is_empty());
        assert!(
            load_cache(&dir).is_none(),
            "an empty sweep must not be cached — it would poison the filter for a week"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
