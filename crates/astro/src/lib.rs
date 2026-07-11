//! Ziqpu **astrotopography** (issue #21) — location as a first-class astrological quantity.
//!
//! Kept in its own crate, on purpose: `engine` and `ephemeris` (the crown-jewel IP) stay
//! byte-for-byte unchanged while this soaks on the `nightfall` track. Everything here is additive
//! and pure `f64` — nothing existing is modified, and no new dependency enters the tree.
//!
//! Feature tiers (so promoting to stable is a config flip, never a rewrite):
//! - `relocation` (default) — relocation charts: re-house a chart for another place, same instant.
//! - `acg` (roadmap) — A\*C\*G planetary lines.
//! - `parans` (roadmap) — line-crossing latitude bands.

#[cfg(feature = "relocation")]
pub mod relocate;
#[cfg(feature = "relocation")]
pub use relocate::{house_cusps, relocate, HouseSystem};

#[cfg(feature = "acg")]
pub mod astrocartography;
#[cfg(feature = "acg")]
pub use astrocartography::{
    body_line, compute_acg, ecliptic_to_equatorial, horizon_longitude, mc_longitude,
    rising_hour_angle, AcgLine, AcgMap, AcgOptions, Equatorial, GeoPoint, LineKind,
};
