//! VIN → vehicle **identity**, via NHTSA vPIC (free, keyless, U.S.-Gov public domain).
//!
//! The **resolver** half of N3 "chart anything" for cars. vPIC tells us WHAT the car is and WHERE it
//! was built (the plant = the origin *place*) — but deliberately **NOT** a chartable date: a VIN
//! carries only the *model year*, never a day-precise build date (verified in the source research —
//! see the project's N3 origin-sources note). The day-precise origin *moment* is the **model's launch
//! date**, resolved separately by joining the Wikidata item for the model (Increment 2). So this
//! module hands back identity + plant; the date + geocoding + synastry wiring build on top.
//!
//! Two entry points: [`resolve_vin`] for the deterministic path (the UI decodes a VIN the seeker
//! typed), and [`DecodeVinTool`] — an [`crate::tools::Tool`] — so the agent loop can decode a VIN
//! mid-conversation ("chart my car") and chain it with other lookups.

use crate::tools::Tool;
use serde_json::{json, Value};

/// A vehicle's identity as vPIC decodes it. Everything past make/model is `Option` because a partial
/// or older VIN can leave fields blank. `year` is **model-year precision only** — kept for the label
/// and for the Wikidata model join, never used as a chartable moment on its own.
#[derive(Debug, Clone, PartialEq)]
pub struct VehicleId {
    pub vin: String,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub trim: Option<String>,
    pub plant_city: Option<String>,
    pub plant_state: Option<String>,
    pub plant_country: Option<String>,
    pub manufacturer: Option<String>,
}

impl VehicleId {
    /// A human label, e.g. `"2003 Honda Accord EX-V6"`. Make is title-cased (vPIC returns it upper).
    pub fn label(&self) -> String {
        let mut s = String::new();
        if let Some(y) = self.year {
            s.push_str(&y.to_string());
            s.push(' ');
        }
        s.push_str(&title_case(&self.make));
        s.push(' ');
        s.push_str(&self.model);
        if let Some(trim) = &self.trim {
            s.push(' ');
            s.push_str(trim);
        }
        s.trim().to_string()
    }

    /// The origin **place** — where it was assembled — as a readable string, e.g.
    /// `"Marysville, Ohio, United States"`. `None` if the plant is unknown. Increment 2 geocodes this
    /// to lat/lon for the chart.
    pub fn plant_place(&self) -> Option<String> {
        let city = self.plant_city.as_ref().map(|c| title_case(c));
        let parts: Vec<String> = [
            city,
            self.plant_state.as_ref().map(|s| title_case(s)),
            self.plant_country.as_ref().map(|c| clean_country(c)),
        ]
        .into_iter()
        .flatten()
        .collect();
        (!parts.is_empty()).then(|| parts.join(", "))
    }
}

/// A VIN is exactly 17 characters, digits + uppercase letters, **excluding I, O, Q** (banned to avoid
/// confusion with 1/0). Cheap gate before spending a network call on obvious garbage.
pub fn is_valid_vin(vin: &str) -> bool {
    let vin = vin.trim();
    vin.len() == 17
        && vin.chars().all(|c| {
            c.is_ascii_digit() || (c.is_ascii_uppercase() && !matches!(c, 'I' | 'O' | 'Q'))
        })
}

/// Parse a vPIC `DecodeVinValues` response (`{Results:[{...}]}`, a flat object) into a [`VehicleId`].
/// Pure; unit-tested. `None` when the response has no usable identity (a bad VIN comes back with an
/// empty `Make`/`Model`), so a caller can honestly say "couldn't decode that VIN".
pub fn parse_vpic(json: &str, vin: &str) -> Option<VehicleId> {
    let v: Value = serde_json::from_str(json).ok()?;
    let r = v.get("Results")?.as_array()?.first()?;

    // vPIC uses "" (and sometimes "Not Applicable") for absent fields — treat both as None.
    let field = |k: &str| -> Option<String> {
        r.get(k)
            .and_then(|x| x.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty() && *s != "Not Applicable")
            .map(str::to_string)
    };

    let make = field("Make")?;
    let model = field("Model")?;
    // Both present = a real decode. (An invalid VIN returns the object with these blank.)
    Some(VehicleId {
        vin: vin.trim().to_string(),
        make,
        model,
        year: field("ModelYear").and_then(|y| y.parse::<i32>().ok()),
        trim: field("Trim"),
        plant_city: field("PlantCity"),
        plant_state: field("PlantState"),
        plant_country: field("PlantCountry"),
        manufacturer: field("Manufacturer"),
    })
}

/// The vPIC endpoint for a VIN (flat key/value decode, JSON).
fn vpic_url(vin: &str) -> String {
    format!("https://vpic.nhtsa.dot.gov/api/vehicles/DecodeVinValues/{vin}?format=json")
}

/// Resolve a VIN to a [`VehicleId`] via NHTSA vPIC — **thin I/O, not unit-tested** (the parser is).
/// Keyless, in-process HTTPS (a descriptive User-Agent, no auth). `None` on an invalid VIN, a
/// transport failure, or an undecodable VIN.
pub fn resolve_vin(vin: &str) -> Option<VehicleId> {
    let vin = vin.trim();
    if !is_valid_vin(vin) {
        return None;
    }
    let body = crate::llm_http::get_json(
        &vpic_url(vin),
        &[(
            "User-Agent",
            "Ziqpu vehicle-origin resolver (ness.aisling@nisabacapitalcharting.com)",
        )],
    )?;
    parse_vpic(&body, vin)
}

/// Title-case an ALL-CAPS or lower token run (vPIC returns `"HONDA"`, `"MARYSVILLE"`). Handles the
/// common hyphen/space word boundaries; leaves already-mixed strings sensible.
fn title_case(s: &str) -> String {
    s.split_inclusive([' ', '-'])
        .map(|word| {
            let mut cs = word.chars();
            match cs.next() {
                Some(first) => {
                    first.to_ascii_uppercase().to_string() + &cs.as_str().to_ascii_lowercase()
                }
                None => String::new(),
            }
        })
        .collect()
}

/// vPIC country strings look like `"UNITED STATES (USA)"`; drop the parenthetical and title-case.
fn clean_country(c: &str) -> String {
    let base = c.split('(').next().unwrap_or(c).trim();
    title_case(base)
}

/// A [`Tool`] wrapping [`resolve_vin`], so the agent loop can decode a VIN when a seeker asks to chart
/// their car. The result is a compact JSON blob the model can read + chain (e.g. into a Wikidata
/// model-launch lookup for the date).
pub struct DecodeVinTool;

impl Tool for DecodeVinTool {
    fn name(&self) -> &str {
        "decode_vin"
    }

    fn spec(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "decode_vin",
                "description": "Decode a 17-character vehicle VIN into its make, model, model year, \
                                and assembly plant (city/state/country) using the free NHTSA vPIC \
                                database. Returns identity + origin place, NOT a build date.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "vin": { "type": "string", "description": "The 17-character VIN." }
                    },
                    "required": ["vin"]
                }
            }
        })
    }

    fn call(&self, args: &Value) -> String {
        let vin = args
            .get("vin")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if !is_valid_vin(vin) {
            return json!({ "error": "not a valid 17-character VIN" }).to_string();
        }
        match resolve_vin(vin) {
            Some(v) => json!({
                "make": title_case(&v.make),
                "model": v.model,
                "year": v.year,
                "trim": v.trim,
                "plant": v.plant_place(),
                "label": v.label(),
            })
            .to_string(),
            None => json!({ "error": "couldn't decode that VIN" }).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A trimmed real vPIC `DecodeVinValues` response (VIN 1HGCM82633A004352 — a 2003 Honda Accord,
    /// captured live), so the parser is tested against the true field shape, not a guess.
    const HONDA: &str = r#"{
        "Count": 1,
        "Message": "Results returned successfully",
        "Results": [{
            "Make": "HONDA",
            "Model": "Accord",
            "ModelYear": "2003",
            "Trim": "EX-V6",
            "BodyClass": "Coupe",
            "VehicleType": "PASSENGER CAR",
            "Manufacturer": "AMERICAN HONDA MOTOR CO., INC.",
            "PlantCity": "MARYSVILLE",
            "PlantState": "OHIO",
            "PlantCountry": "UNITED STATES (USA)",
            "ErrorText": "0 - VIN decoded clean. Check Digit (9th position) is correct",
            "VIN": "1HGCM82633A004352"
        }]
    }"#;

    #[test]
    fn parses_the_real_vpic_shape() {
        let v = parse_vpic(HONDA, "1HGCM82633A004352").unwrap();
        assert_eq!(v.make, "HONDA");
        assert_eq!(v.model, "Accord");
        assert_eq!(v.year, Some(2003));
        assert_eq!(v.trim.as_deref(), Some("EX-V6"));
        assert_eq!(v.plant_city.as_deref(), Some("MARYSVILLE"));
        // Label + place are the reader-facing strings — title-cased, country cleaned.
        assert_eq!(v.label(), "2003 Honda Accord EX-V6");
        assert_eq!(
            v.plant_place().as_deref(),
            Some("Marysville, Ohio, United States")
        );
    }

    #[test]
    fn an_undecodable_vin_yields_none() {
        // A bad VIN comes back with blank Make/Model — no usable identity.
        let blank = r#"{"Results":[{"Make":"","Model":"","ErrorText":"11 - Incorrect VIN"}]}"#;
        assert_eq!(parse_vpic(blank, "00000000000000000"), None);
        assert_eq!(parse_vpic("not json", "x"), None);
    }

    #[test]
    fn vin_validation_matches_the_standard() {
        assert!(is_valid_vin("1HGCM82633A004352")); // 17, valid chars
        assert!(!is_valid_vin("1HGCM82633A00435")); // 16 — too short
        assert!(!is_valid_vin("1HGCM82633A0043521")); // 18 — too long
        assert!(!is_valid_vin("1HGCM82633A0043I2")); // contains I (banned)
        assert!(!is_valid_vin("1hgcm82633a004352")); // lowercase not accepted
        assert!(!is_valid_vin("1HGCM8263 A004352")); // space
    }

    #[test]
    fn the_tool_spec_and_result_are_well_formed() {
        let t = DecodeVinTool;
        assert_eq!(t.name(), "decode_vin");
        let spec = t.spec();
        assert_eq!(spec["function"]["name"], "decode_vin");
        // An invalid VIN yields a readable error result, not a panic or a network call.
        let out = t.call(&json!({ "vin": "bad" }));
        assert!(out.contains("valid"), "{out}");
    }

    /// LIVE — proves the real NHTSA vPIC endpoint still returns the shape [`parse_vpic`] expects.
    /// Ignored by default (needs network). Run: `cargo test -p agents live_vpic -- --ignored`.
    #[test]
    #[ignore = "hits the live NHTSA vPIC API"]
    fn live_vpic_decode() {
        let v = resolve_vin("1HGCM82633A004352").expect("vPIC should decode a known VIN");
        assert_eq!(title_case(&v.make), "Honda");
        assert!(v.plant_place().is_some());
        println!("live: {} — {}", v.label(), v.plant_place().unwrap());
    }
}
