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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedProfile {
    pub date_str: String,
    pub time_str: String,
    pub time_unknown: bool,
    pub place: Option<SavedPlace>,
}

impl SavedProfile {
    /// The saved place as a [`geo::Place`], if one was picked — for seeding the form's `selected`.
    pub fn place(&self) -> Option<geo::Place> {
        self.place.as_ref().map(SavedPlace::to_place)
    }
}

/// `%APPDATA%\Ziqpu\profile.json` on Windows. Returns `None` if `APPDATA` is unset. Creates the
/// `Ziqpu` directory if it is missing (best-effort — a failure here still returns the intended
/// path, and the later read/write simply fails softly).
pub fn profile_path() -> Option<PathBuf> {
    let appdata = std::env::var("APPDATA").ok()?;
    let dir = PathBuf::from(appdata).join("Ziqpu");
    let _ = std::fs::create_dir_all(&dir);
    Some(dir.join("profile.json"))
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
        let _ = std::fs::write(&path, text);
    }
}
