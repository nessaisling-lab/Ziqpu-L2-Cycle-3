//! Portable user profile — so the agent travels with the seeker (PRD §15, Phase 1b).
//!
//! A profile is a tiny, self-describing JSON blob of a seeker's birth inputs (only birth data —
//! no secrets). Any surface — the MCP server, a future desktop app — can [`import_profile`] it and
//! reconstruct the same chart with no re-onboarding. Small enough to paste, QR-encode, or carry.

use crate::types::BirthMoment;
use chrono::{NaiveDate, NaiveTime};
use chrono_tz::Tz;

/// Export a birth moment to a portable JSON profile string.
///
/// Shape: `{"v":1,"kind":"ziqpu.profile","date":"YYYY-MM-DD","time":"HH:MM"|null,"tz":"<IANA>","lat":..,"lon":..}`.
pub fn export_profile(birth: &BirthMoment) -> String {
    serde_json::json!({
        "v": 1,
        "kind": "ziqpu.profile",
        "date": birth.date.to_string(),
        "time": birth.time.map(|t| t.format("%H:%M").to_string()),
        "tz": birth.tz.name(),
        "lat": birth.lat,
        "lon": birth.lon,
    })
    .to_string()
}

/// Reconstruct a birth moment from a portable profile string.
pub fn import_profile(s: &str) -> Result<BirthMoment, ProfileError> {
    let v: serde_json::Value = serde_json::from_str(s).map_err(|_| ProfileError::Malformed)?;
    if v.get("kind").and_then(|k| k.as_str()) != Some("ziqpu.profile") {
        return Err(ProfileError::WrongKind);
    }
    let date = v
        .get("date")
        .and_then(|d| d.as_str())
        .and_then(|d| d.parse::<NaiveDate>().ok())
        .ok_or(ProfileError::BadField("date"))?;
    // `time` may be absent or JSON null (unknown birth time) — both mean "no time".
    let time = match v.get("time").and_then(|t| t.as_str()) {
        Some(t) => Some(
            NaiveTime::parse_from_str(t, "%H:%M").map_err(|_| ProfileError::BadField("time"))?,
        ),
        None => None,
    };
    let tz = v
        .get("tz")
        .and_then(|t| t.as_str())
        .and_then(|t| t.parse::<Tz>().ok())
        .ok_or(ProfileError::BadField("tz"))?;
    let lat = v
        .get("lat")
        .and_then(|x| x.as_f64())
        .ok_or(ProfileError::BadField("lat"))?;
    let lon = v
        .get("lon")
        .and_then(|x| x.as_f64())
        .ok_or(ProfileError::BadField("lon"))?;
    Ok(BirthMoment {
        date,
        time,
        tz,
        lat,
        lon,
    })
}

/// Build a portable profile from raw string/number inputs — for surfaces (the MCP server) that
/// only have loose fields and shouldn't depend on chrono directly. `time` may be `None`/empty.
pub fn make_profile(
    date: &str,
    time: Option<&str>,
    tz: &str,
    lat: f64,
    lon: f64,
) -> Result<String, ProfileError> {
    let date = date
        .parse::<NaiveDate>()
        .map_err(|_| ProfileError::BadField("date"))?;
    let time = match time {
        Some(t) if !t.is_empty() => Some(
            NaiveTime::parse_from_str(t, "%H:%M").map_err(|_| ProfileError::BadField("time"))?,
        ),
        _ => None,
    };
    let tz = tz.parse::<Tz>().map_err(|_| ProfileError::BadField("tz"))?;
    Ok(export_profile(&BirthMoment {
        date,
        time,
        tz,
        lat,
        lon,
    }))
}

/// Why a profile could not be imported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    /// Not valid JSON.
    Malformed,
    /// JSON, but not a `ziqpu.profile`.
    WrongKind,
    /// A required field is missing or unparseable.
    BadField(&'static str),
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileError::Malformed => write!(f, "profile is not valid JSON"),
            ProfileError::WrongKind => write!(f, "not a ziqpu.profile"),
            ProfileError::BadField(name) => write!(f, "profile field `{name}` missing or invalid"),
        }
    }
}

impl std::error::Error for ProfileError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_timed_profile() {
        let birth = crate::demo_seeker();
        let restored = import_profile(&export_profile(&birth)).unwrap();
        assert_eq!(restored, birth);
    }

    #[test]
    fn round_trips_an_unknown_time() {
        let birth = BirthMoment {
            date: NaiveDate::from_ymd_opt(1919, 9, 5).unwrap(),
            time: None,
            tz: chrono_tz::America::New_York,
            lat: 40.7,
            lon: -74.0,
        };
        let restored = import_profile(&export_profile(&birth)).unwrap();
        assert_eq!(restored, birth);
        assert!(restored.time.is_none());
    }

    #[test]
    fn rejects_junk_and_wrong_kind() {
        assert_eq!(import_profile("not json"), Err(ProfileError::Malformed));
        assert_eq!(
            import_profile(r#"{"kind":"something-else"}"#),
            Err(ProfileError::WrongKind)
        );
        assert!(matches!(
            import_profile(r#"{"kind":"ziqpu.profile","date":"nope"}"#),
            Err(ProfileError::BadField(_))
        ));
    }
}
