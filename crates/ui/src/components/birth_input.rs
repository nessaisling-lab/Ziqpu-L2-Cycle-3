//! Birth-input form — the "enter my own birth details" alternative to the seeded demo. It builds
//! an [`agents::BirthMoment`] for the seeker and feeds it into the exact same `recommend` path.
//!
//! Everything here is a local draft; only the *validated* moment is promoted to `ctx.seeker`. The
//! place search calls the offline `geo` gazetteer synchronously (no async, no network), and an
//! unknown birth time is honest — `time: None` withholds the angles downstream, never invents them.

use agents::BirthMoment;
use chrono::{NaiveDate, NaiveTime};
use chrono_tz::Tz;
use dioxus::prelude::*;

use crate::state::{run_recommend, AppCtx};

/// A single field-level validation failure, rendered inline in the refusal palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldError {
    Date,
    Time,
    Place,
    Tz(String),
    Coord,
}

impl FieldError {
    fn message(&self) -> String {
        match self {
            FieldError::Date => "Enter a birth date.".to_string(),
            FieldError::Time => "Enter a time, or check \"time unknown\".".to_string(),
            FieldError::Place => "Search for and select a birthplace.".to_string(),
            FieldError::Tz(name) => format!("Unrecognized timezone \"{name}\"."),
            FieldError::Coord => "Coordinates are out of range.".to_string(),
        }
    }
}

/// Parse `HH:MM` (what a native time picker emits), tolerating an optional `:SS`.
fn parse_time(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(s, "%H:%M:%S"))
        .ok()
}

/// Validate the draft into a [`BirthMoment`], collecting every field error. Pure and cheap, so it
/// is recomputed each render to drive both the inline errors and the submit button's `disabled`.
///
/// Rules: date required; time required unless `time_unknown`; place required (it supplies lat/lon
/// and the default timezone); the timezone resolves from the manual override or the place's IANA
/// string (parsed via `chrono_tz::Tz`'s `FromStr`).
pub fn draft_to_moment(
    date_str: &str,
    time_str: &str,
    time_unknown: bool,
    selected: &Option<geo::Place>,
    tz_override: Option<Tz>,
) -> Result<BirthMoment, Vec<FieldError>> {
    let mut errs = Vec::new();

    let date = match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Ok(d) => Some(d),
        Err(_) => {
            errs.push(FieldError::Date);
            None
        }
    };

    let time = if time_unknown {
        Some(None) // honestly unknown → time = None
    } else {
        match parse_time(time_str) {
            Some(t) => Some(Some(t)),
            None => {
                errs.push(FieldError::Time);
                None
            }
        }
    };

    let place = match selected {
        Some(p) => Some(p),
        None => {
            errs.push(FieldError::Place);
            None
        }
    };

    let tz = match (tz_override, place) {
        (Some(tz), _) => Some(tz),
        (None, Some(p)) => match p.tz.parse::<Tz>() {
            Ok(tz) => Some(tz),
            Err(_) => {
                errs.push(FieldError::Tz(p.tz.clone()));
                None
            }
        },
        (None, None) => None,
    };

    // lat/lon come from the gazetteer (already in range); guard anyway for a future manual entry.
    if let Some(p) = place {
        if !(-90.0..=90.0).contains(&p.lat) || !(-180.0..=180.0).contains(&p.lon) {
            errs.push(FieldError::Coord);
        }
    }

    match (date, time, place, tz) {
        (Some(date), Some(time), Some(p), Some(tz)) if errs.is_empty() => Ok(BirthMoment {
            date,
            time,
            tz,
            lat: p.lat,
            lon: p.lon,
        }),
        _ => Err(errs),
    }
}

/// The birth form, in two modes. Default (Setup's "enter my own details") submits straight to the
/// graded loop. In **`reveal_mode`** (the first-run onboarding gate) it instead saves the chart, sets
/// the seeker, and fires [`on_continue`] so the wizard can advance to the handle reveal — the loop is
/// not run, and the standalone "Save chart" affordance is hidden to keep onboarding a single path.
#[component]
pub fn BirthInputForm(
    #[props(default)] reveal_mode: bool,
    #[props(default)] on_continue: EventHandler<()>,
) -> Element {
    let ctx = use_context::<AppCtx>();

    // Repopulate the form from the last saved draft (best-effort; loaded once on mount). A missing or
    // corrupt profile yields `None`, so every field simply starts empty — the form still works.
    let saved = use_hook(crate::profile::load_profile);
    let mut date_str = use_signal(|| {
        saved
            .as_ref()
            .map(|s| s.date_str.clone())
            .unwrap_or_default()
    });
    let mut time_str = use_signal(|| {
        saved
            .as_ref()
            .map(|s| s.time_str.clone())
            .unwrap_or_default()
    });
    let mut time_unknown = use_signal(|| saved.as_ref().map(|s| s.time_unknown).unwrap_or(false));
    let mut query = use_signal(|| {
        saved
            .as_ref()
            .and_then(|s| s.place.as_ref())
            .map(|p| p.name.clone())
            .unwrap_or_default()
    });
    let mut results = use_signal(Vec::<geo::Place>::new);
    let mut selected = use_signal(|| saved.as_ref().and_then(|s| s.place()));
    let mut tz_override = use_signal(|| None::<Tz>);
    let mut show_errors = use_signal(|| false);
    // Whether a chart is already persisted (drives the header "chart saved" indicator on mount).
    let had_saved = saved.is_some();
    // Flips true right after the explicit "Save chart" button persists the current draft, showing a
    // brief "Saved ✓" confirmation. Any edit to a field clears it, so it only ever affirms the
    // just-saved state.
    let mut saved_confirm = use_signal(|| false);

    // Recompute validation each render (pure + cheap) from the current draft.
    let date_now = date_str.read().clone();
    let time_now = time_str.read().clone();
    let unknown_now = *time_unknown.read();
    let selected_now = selected.read().clone();
    let tz_now = *tz_override.read();
    let validation = draft_to_moment(&date_now, &time_now, unknown_now, &selected_now, tz_now);
    let is_valid = validation.is_ok();
    let errors = if *show_errors.read() {
        validation.err().unwrap_or_default()
    } else {
        Vec::new()
    };

    rsx! {
        section { class: "setup birth-input",
            p { class: "eyebrow", "Begin · your birth moment" }
            div { class: "birth-title-row",
                h2 { class: "setup-title", "Your birth moment" }
                if had_saved {
                    span { class: "chart-saved-badge", "✦ chart saved" }
                }
            }
            p { class: "muted",
                "Enter your own birth details — everything is resolved offline (no keys, no network). "
                "An unknown time is honest: the angles are withheld, not guessed."
            }

            div { class: "birth-form",
                div { class: "field",
                    label { class: "field-label", "Birth date" }
                    input {
                        class: "field-input",
                        r#type: "date",
                        value: "{date_now}",
                        oninput: move |e| {
                            saved_confirm.set(false);
                            date_str.set(e.value());
                        },
                    }
                }

                div { class: "field",
                    label { class: "field-label", "Birth time" }
                    input {
                        class: "field-input",
                        r#type: "time",
                        value: "{time_now}",
                        disabled: unknown_now,
                        oninput: move |e| {
                            saved_confirm.set(false);
                            time_str.set(e.value());
                        },
                    }
                    label { class: "field-check",
                        input {
                            r#type: "checkbox",
                            checked: unknown_now,
                            onchange: move |_| {
                                saved_confirm.set(false);
                                let now = *time_unknown.read();
                                time_unknown.set(!now);
                            },
                        }
                        " Time unknown"
                    }
                }

                if unknown_now {
                    p { class: "honesty-note",
                        "Time unknown — Ascendant and Midheaven are withheld (not guessed), and confidence is reduced one band."
                    }
                }

                div { class: "field",
                    label { class: "field-label", "Birthplace" }
                    input {
                        class: "field-input",
                        placeholder: "Search a city — e.g. New York",
                        value: "{query}",
                        oninput: move |e| {
                            saved_confirm.set(false);
                            let q = e.value();
                            results.set(geo::lookup(&q));
                            query.set(q);
                        },
                    }
                    if !results.read().is_empty() {
                        div { class: "place-results",
                            {results.read().iter().cloned().map(|p| {
                                let label = format!("{} · {}", p.name, p.country);
                                let key = format!("{label} {} {}", p.lat, p.lon);
                                rsx! {
                                    button {
                                        key: "{key}",
                                        class: "place-result",
                                        onclick: move |_| {
                                            saved_confirm.set(false);
                                            query.set(p.name.clone());
                                            selected.set(Some(p.clone()));
                                            tz_override.set(None); // a fresh place resets the override
                                            results.set(Vec::new());
                                        },
                                        "{label}"
                                    }
                                }
                            })}
                        }
                    }
                }

                {selected_now.as_ref().map(|p| rsx! {
                    div { class: "place-selected",
                        span { class: "seeker-label", "Place" }
                        span { class: "seeker-detail", "{p.name} · {p.lat}, {p.lon} · {p.tz}" }
                    }
                })}

                if selected_now.is_some() {
                    div { class: "field",
                        label { class: "field-label", "Timezone (optional override)" }
                        select {
                            class: "field-input",
                            onchange: move |e| {
                                let v = e.value();
                                tz_override.set(if v.is_empty() { None } else { v.parse::<Tz>().ok() });
                            },
                            option { value: "", "Use place timezone" }
                            {chrono_tz::TZ_VARIANTS.iter().map(|tz| rsx! {
                                option { key: "{tz.name()}", value: "{tz.name()}", "{tz.name()}" }
                            })}
                        }
                    }
                }

                if !errors.is_empty() {
                    div { class: "field-errors",
                        {errors.iter().map(|err| rsx! {
                            p { key: "{err:?}", class: "field-error", "{err.message()}" }
                        })}
                    }
                }

                div { class: "actions", style: "justify-content:flex-start;align-items:center",
                    if !reveal_mode {
                        button {
                            class: "btn",
                            r#type: "button",
                            title: "Save this chart so it repopulates next time",
                            onclick: move |_| {
                                // Persist the current draft verbatim — even a partly-filled one — so it
                                // pins and round-trips on relaunch. Best-effort; never panics (see
                                // crate::profile). Does NOT require a valid moment.
                                let selected = selected.read().clone();
                                crate::profile::save_draft(
                                    date_str.read().clone(),
                                    time_str.read().clone(),
                                    *time_unknown.read(),
                                    selected.as_ref().map(crate::profile::SavedPlace::from_place),
                                );
                                saved_confirm.set(true);
                            },
                            "Save chart"
                        }
                        if *saved_confirm.read() {
                            span { class: "chart-saved-badge", "Saved ✓" }
                        }
                    }
                    button {
                        class: "btn btn--go",
                        r#type: "button",
                        disabled: !is_valid,
                        onclick: {
                            let mut ctx = ctx.clone();
                            move |_| {
                                show_errors.set(true);
                                let date_str = date_str.read().clone();
                                let time_str = time_str.read().clone();
                                let unknown = *time_unknown.read();
                                let selected = selected.read().clone();
                                let tz_override = *tz_override.read();
                                if let Ok(moment) = draft_to_moment(
                                    &date_str, &time_str, unknown, &selected, tz_override,
                                ) {
                                    // Persist the full draft (not just the moment) so the form
                                    // repopulates exactly as entered after a relaunch (best-effort,
                                    // never panics — see crate::profile).
                                    crate::profile::save_draft(
                                        date_str.clone(),
                                        time_str.clone(),
                                        unknown,
                                        selected.as_ref().map(crate::profile::SavedPlace::from_place),
                                    );
                                    ctx.seeker.set(moment);
                                    // Onboarding reveal mode advances to the handle reveal; Setup's
                                    // custom mode drives the graded loop straight to the ranked fits.
                                    if reveal_mode {
                                        on_continue.call(());
                                    } else {
                                        run_recommend(ctx.clone());
                                    }
                                }
                            }
                        },
                        if reveal_mode { "Continue →" } else { "Read the fits →" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_york() -> geo::Place {
        geo::Place {
            name: "New York City".to_string(),
            country: "United States".to_string(),
            lat: 40.7128,
            lon: -74.0060,
            tz: "America/New_York".to_string(),
        }
    }

    #[test]
    fn happy_path_builds_a_timed_moment() {
        let m = draft_to_moment("1990-05-15", "14:30", false, &Some(new_york()), None).unwrap();
        assert_eq!(m.date, NaiveDate::from_ymd_opt(1990, 5, 15).unwrap());
        assert_eq!(m.time, Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap()));
        assert_eq!(m.tz, chrono_tz::America::New_York);
    }

    #[test]
    fn time_unknown_yields_none() {
        let m = draft_to_moment("1990-05-15", "", true, &Some(new_york()), None).unwrap();
        assert_eq!(m.time, None);
    }

    #[test]
    fn missing_date_time_and_place_each_error() {
        assert!(draft_to_moment("", "14:30", false, &Some(new_york()), None)
            .unwrap_err()
            .contains(&FieldError::Date));
        assert!(
            draft_to_moment("1990-05-15", "nope", false, &Some(new_york()), None)
                .unwrap_err()
                .contains(&FieldError::Time)
        );
        assert!(draft_to_moment("1990-05-15", "14:30", false, &None, None)
            .unwrap_err()
            .contains(&FieldError::Place));
    }

    #[test]
    fn place_tz_parses_to_the_expected_zone() {
        let m = draft_to_moment("1990-05-15", "14:30", false, &Some(new_york()), None).unwrap();
        assert_eq!(m.tz, "America/New_York".parse::<Tz>().unwrap());
    }

    #[test]
    fn demo_values_through_the_form_equal_the_demo_seeker() {
        let m = draft_to_moment("1990-05-15", "14:30", false, &Some(new_york()), None).unwrap();
        assert_eq!(m, agents::demo_seeker());
    }
}
