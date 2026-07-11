//! Persist the seeker's **full birth-input draft** across launches — a single, best-effort JSON
//! file under the user's roaming app data. Loading is defensive: a missing, unreadable, or corrupt
//! file yields `None` (the caller falls back to the demo seeker and an empty form), and saving
//! swallows every error. Neither path ever panics — persistence is a convenience, never a hard
//! dependency.
//!
//! We persist the *form draft* (the raw date/time strings, the "time unknown" flag, and the picked
//! place), not just the derived [`agents::BirthMoment`]. That is what lets the birth form repopulate
//! exactly as the seeker typed it after a relaunch — a saved chart that could not round-trip back
//! into the form looked, to the product owner, "unsaved".

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A birthplace as saved to disk — mirrors [`geo::Place`] field-for-field so the form's `selected`
/// place round-trips. Kept as its own struct (rather than serializing `geo::Place` directly) so the
/// on-disk schema is owned here and independent of the gazetteer crate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedPlace {
    pub name: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz: String,
}

impl SavedPlace {
    /// Snapshot a picked [`geo::Place`] for persistence.
    pub fn from_place(p: &geo::Place) -> Self {
        SavedPlace {
            name: p.name.clone(),
            country: p.country.clone(),
            lat: p.lat,
            lon: p.lon,
            tz: p.tz.clone(),
        }
    }

    /// Reconstruct a [`geo::Place`] to seed the form's `selected` signal.
    pub fn to_place(&self) -> geo::Place {
        geo::Place {
            name: self.name.clone(),
            country: self.country.clone(),
            lat: self.lat,
            lon: self.lon,
            tz: self.tz.clone(),
        }
    }
}

/// The persisted birth-input draft — everything needed to repaint the form and re-derive the
/// seeker's [`agents::BirthMoment`]. The `place` is optional because a partially-filled draft is
/// still worth remembering (though the form only *submits*, and therefore only saves, a valid one).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SavedProfile {
    #[serde(default)]
    pub date_str: String,
    #[serde(default)]
    pub time_str: String,
    #[serde(default)]
    pub time_unknown: bool,
    #[serde(default)]
    pub place: Option<SavedPlace>,
    /// The mixed market basket the seeker built — **namespaced `"<slug>:<id>"` tokens** (e.g.
    /// `"stocks:AAPL"`, `"airlines:AAL"`), in pick order. The namespace is what lets the basket span
    /// universes and keeps a stock "AAL" distinct from an airline "AAL". `#[serde(default)]` so a
    /// profile written before baskets were saved still loads (empty basket); a legacy bare token
    /// with no `:` is read back as a stock by the UI. The type stays `Vec<String>`.
    #[serde(default)]
    pub basket: Vec<String>,
    /// The seeker's **anonymous handle** — the two-word cosmic name (e.g. `"Lapis Scribe"`), no
    /// email, no login. Empty means "never set": the UI then derives the chart-default via
    /// [`agents::anon_handle_for`]. A non-empty value is a handle the seeker **re-rolled** and chose
    /// to keep, which is why it must persist rather than always be recomputed. `#[serde(default)]`
    /// so profiles written before identities existed still load (empty → derive).
    #[serde(default)]
    pub handle: String,
}

impl SavedProfile {
    /// The saved place as a [`geo::Place`], if one was picked — for seeding the form's `selected`.
    pub fn place(&self) -> Option<geo::Place> {
        self.place.as_ref().map(SavedPlace::to_place)
    }
}

/// The OS user-data directory for Ziqpu — created best-effort, shared by every persisted file
/// (`profile.json`, `settings.json`). Hand-rolled over `std::env` so it needs no extra crate, and
/// branches per platform with `cfg!(target_os = …)`:
/// - **Windows** — `%APPDATA%\Ziqpu`
/// - **macOS** — `$HOME/Library/Application Support/Ziqpu`
/// - **Linux/other** — `$XDG_DATA_HOME/ziqpu` if set, else `$HOME/.local/share/ziqpu`
///
/// Returns `None` only when the underlying base variable (`APPDATA`/`HOME`) is unset. The directory
/// is created if missing (best-effort — a failure still returns the intended path, and the later
/// read/write simply fails softly). Never panics.
pub fn data_dir() -> Option<PathBuf> {
    let dir = if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").ok()?).join("Ziqpu")
    } else if cfg!(target_os = "macos") {
        PathBuf::from(std::env::var("HOME").ok()?)
            .join("Library")
            .join("Application Support")
            .join("Ziqpu")
    } else {
        // Linux/other: prefer $XDG_DATA_HOME (when non-empty), else ~/.local/share.
        match std::env::var("XDG_DATA_HOME") {
            Ok(x) if !x.is_empty() => PathBuf::from(x).join("ziqpu"),
            _ => PathBuf::from(std::env::var("HOME").ok()?)
                .join(".local")
                .join("share")
                .join("ziqpu"),
        }
    };
    let _ = std::fs::create_dir_all(&dir);
    Some(dir)
}

/// `<data_dir>/profile.json` — the birth-input draft file, on every OS. Returns `None` when
/// [`data_dir`] can't resolve a base directory. See [`data_dir`] for the per-OS paths.
pub fn profile_path() -> Option<PathBuf> {
    Some(data_dir()?.join("profile.json"))
}

/// Read + deserialize the saved draft. Returns `None` on any missing/corrupt-file error — never
/// panics — so the caller can fall back to the demo seeker and an empty form.
pub fn load_profile() -> Option<SavedProfile> {
    let path = profile_path()?;
    let text = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&text).ok()
}

/// Serialize + write the draft, best-effort. Any error (no path, serialize failure, write failure)
/// is ignored — this never panics and never blocks the loop.
pub fn save_profile(profile: &SavedProfile) {
    let Some(path) = profile_path() else {
        return;
    };
    if let Ok(text) = serde_json::to_string_pretty(profile) {
        // profile.json holds birth PII — date, time, place name, and coordinates to ~11 m. Restrict
        // it to owner-only on Unix (same protection settings.json gets) so it isn't world-readable on
        // a shared host (SEC-002). Best-effort; a failure just leaves the file at the default umask.
        if std::fs::write(&path, text).is_ok() {
            crate::settings::set_owner_only(&path);
        }
    }
}

/// Save just the birth-input draft, **preserving** any saved market basket. Loads the existing
/// profile (or a default), overwrites only the draft fields, and writes it back — so saving the
/// chart never wipes the basket, and vice versa.
pub fn save_draft(
    date_str: String,
    time_str: String,
    time_unknown: bool,
    place: Option<SavedPlace>,
) {
    let mut p = load_profile().unwrap_or_default();
    p.date_str = date_str;
    p.time_str = time_str;
    p.time_unknown = time_unknown;
    p.place = place;
    save_profile(&p);
}

/// Save just the market basket (namespaced `"<slug>:<id>"` tokens), **preserving** the birth draft.
pub fn save_basket(tokens: &[String]) {
    let mut p = load_profile().unwrap_or_default();
    p.basket = tokens.to_vec();
    save_profile(&p);
}

/// The saved basket's namespaced tokens (empty if none saved). The UI rebuilds each `Choice` via
/// `tickers::choice_in(Universe::from_slug(slug), id)`, treating a legacy bare token as a stock.
pub fn load_basket() -> Vec<String> {
    load_profile().map(|p| p.basket).unwrap_or_default()
}

/// Save just the anonymous handle, **preserving** the birth draft and basket. Called when the
/// seeker accepts a re-rolled name in the setup wizard.
pub fn save_handle(handle: &str) {
    let mut p = load_profile().unwrap_or_default();
    p.handle = handle.to_string();
    save_profile(&p);
}

/// The saved handle (empty if never set). Prefer [`handle_or_default`] for display — it fills the
/// chart-derived default when nothing was saved.
pub fn load_handle() -> String {
    load_profile().map(|p| p.handle).unwrap_or_default()
}

/// The handle to **show** the seeker: their saved (possibly re-rolled) handle if they kept one,
/// otherwise the stable chart-derived default. This is the single call the wizard and header use,
/// so display logic never has to special-case the empty back-compat value.
pub fn handle_or_default(birth: &agents::BirthMoment) -> String {
    let saved = load_handle();
    if saved.is_empty() {
        agents::anon_handle_for(birth)
    } else {
        saved
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A profile JSON written **before** handles existed (no `handle` key) still deserializes, with
    /// `handle` defaulting to empty — the back-compat contract that keeps old installs loading.
    #[test]
    fn legacy_profile_without_handle_loads_empty() {
        let legacy = r#"{"date_str":"1990-05-15","time_str":"14:30","time_unknown":false,"place":null,"basket":["stocks:AAPL"]}"#;
        let p: SavedProfile = serde_json::from_str(legacy).expect("legacy profile deserializes");
        assert_eq!(p.handle, "");
        assert_eq!(p.basket, vec!["stocks:AAPL".to_string()]);
    }

    /// The handle round-trips through serde alongside the rest of the profile.
    #[test]
    fn handle_round_trips() {
        let p = SavedProfile {
            handle: "Lapis Scribe".into(),
            ..Default::default()
        };
        let restored: SavedProfile =
            serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(restored.handle, "Lapis Scribe");
    }
}
