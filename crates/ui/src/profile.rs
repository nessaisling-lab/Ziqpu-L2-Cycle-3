//! Persist the seeker's birth chart across launches — a single, best-effort JSON file under the
//! user's roaming app data. Loading is defensive: a missing, unreadable, or corrupt file yields
//! `None` (the caller falls back to the demo seeker), and saving swallows every error. Neither path
//! ever panics — persistence is a convenience, never a hard dependency.

use std::path::PathBuf;

use agents::BirthMoment;

/// `%APPDATA%\Ziqpu\profile.json` on Windows. Returns `None` if `APPDATA` is unset. Creates the
/// `Ziqpu` directory if it is missing (best-effort — a failure here still returns the intended
/// path, and the later read/write simply fails softly).
pub fn profile_path() -> Option<PathBuf> {
    let appdata = std::env::var("APPDATA").ok()?;
    let dir = PathBuf::from(appdata).join("Ziqpu");
    let _ = std::fs::create_dir_all(&dir);
    Some(dir.join("profile.json"))
}

/// Read + deserialize the saved seeker. Returns `None` on any missing/corrupt-file error — never
/// panics — so the caller can fall back to the demo seeker.
pub fn load_seeker() -> Option<BirthMoment> {
    let path = profile_path()?;
    let text = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

/// Serialize + write the seeker, best-effort. Any error (no path, serialize failure, write failure)
/// is ignored — this never panics and never blocks the loop.
pub fn save_seeker(seeker: &BirthMoment) {
    let Some(path) = profile_path() else {
        return;
    };
    if let Ok(text) = serde_json::to_string_pretty(seeker) {
        let _ = std::fs::write(&path, text);
    }
}
