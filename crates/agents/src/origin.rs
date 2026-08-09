//! N3 — resolving a real-world entity to the moment(s) it came into the world.
//!
//! # Why this returns a lifecycle, not a birthday
//!
//! The obvious shape for this module is `fn origin(name) -> BirthMoment`. It is the wrong shape, and
//! Wikidata is what shows why: an entity does not carry *a* date, it carries several, each meaning
//! something different. A games console has a publication date per region. A company has an
//! inception and, separately, a listing. A building has a date of official opening that is not the
//! day construction started.
//!
//! Picking one and calling it "the birth" would be the app making a claim the data does not support
//! — and it would make that claim invisibly, which is the failure mode this project keeps finding in
//! itself. So [`resolve_origin`] returns every origin moment it found, ranked by a documented rule,
//! and the seeker chooses. The app's job is to show what is real, not to decide what is true.
//!
//! # Precision is the whole discipline
//!
//! Wikidata stamps every time value with a `precision`: 11 = day, 10 = month, 9 = year. **A
//! year-precision value is stored as January 1st.** Reading that as a day would silently reinvent the
//! exact fabrication this project deleted 904 charts to be rid of — a year dressed up as a moment.
//!
//! So precision travels *with* the date in [`OriginMoment`], and only [`DatePrecision::Day`] can
//! produce a chartable moment. A year-precision origin is still shown — it is real, and hiding it
//! would be its own dishonesty — but it is marked unchartable and cannot be turned into a
//! [`BirthMoment`] by any path through this module.
//!
//! # Scope
//!
//! Wikidata first because it is CC0, keyless, and — unlike every alternative surveyed — it models
//! the lifecycle natively rather than flattening it. GLEIF returns incorporation, which is not
//! founding. NHTSA vPIC resolves a VIN to a make/model but carries no origin date at all. Those are
//! resolvers or adjacent facts; this is the date layer.

use chrono::NaiveDate;
use serde_json::Value;

use crate::types::BirthMoment;

/// How precisely Wikidata knows a date.
///
/// The numeric values are Wikidata's own, kept rather than renamed so the mapping is checkable
/// against their documentation without a translation table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DatePrecision {
    /// Precision 9 — the year is known and nothing finer. **Stored as January 1st.**
    Year,
    /// Precision 10 — year and month. Stored as the 1st of the month.
    Month,
    /// Precision 11 or finer — a real day.
    Day,
}

impl DatePrecision {
    /// Map Wikidata's numeric precision. Anything below 9 (decade, century, millennium) is `None`:
    /// those are real values but too coarse to belong in an origin list at all, and rounding one up
    /// to "a year" would invent precision that was never claimed.
    pub fn from_wikidata(precision: u64) -> Option<Self> {
        match precision {
            9 => Some(DatePrecision::Year),
            10 => Some(DatePrecision::Month),
            11..=14 => Some(DatePrecision::Day),
            _ => None,
        }
    }

    /// How to write this date down without overstating it.
    pub fn render(self, date: NaiveDate) -> String {
        match self {
            DatePrecision::Day => date.format("%Y-%m-%d").to_string(),
            DatePrecision::Month => date.format("%Y-%m").to_string(),
            DatePrecision::Year => date.format("%Y").to_string(),
        }
    }
}

/// One origin property this module understands, and what it means in plain words.
///
/// Deliberately a small closed list rather than "any date property". Wikidata has hundreds of
/// time-valued properties and most are not origins — a death date, a dissolution, a last-updated
/// stamp. Widening this is a decision each time, not a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OriginProperty {
    /// The Wikidata P-number, e.g. `"P577"`.
    pub id: &'static str,
    /// The word a seeker reads, e.g. `"released"`.
    pub label: &'static str,
    /// What kind of entity this property speaks for — shown when several moments compete.
    pub means: &'static str,
}

/// The origin properties, in the order they are preferred when a tie must be broken.
///
/// The order encodes one judgement: when an entity carries several *equally precise* origin moments,
/// the one that best answers "when did this come into the world" wins. Publication first because a
/// released thing enters the world on release; opening next for places; inception last because it is
/// the broadest and most often a year-precision approximation of a process rather than an event.
///
/// This ordering only ever breaks ties among same-precision moments. It never suppresses one — all
/// of them are returned.
pub const ORIGIN_PROPERTIES: [OriginProperty; 5] = [
    OriginProperty {
        id: "P577",
        label: "published / released",
        means: "the day a work or product entered the world",
    },
    OriginProperty {
        id: "P1619",
        label: "officially opened",
        means: "the day a place or venue opened to the public",
    },
    OriginProperty {
        id: "P729",
        label: "entered service",
        means: "the day a vehicle or vessel began service",
    },
    OriginProperty {
        id: "P571",
        label: "founded / inception",
        means: "when an organization or object came into being",
    },
    OriginProperty {
        id: "P580",
        label: "started",
        means: "the start of the period this entity covers",
    },
];

/// One moment an entity could be said to have begun.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginMoment {
    pub property: OriginProperty,
    /// The stored date. For non-`Day` precision the day-of-month is Wikidata's placeholder and
    /// **must not be read as real** — [`DatePrecision::render`] is what puts it on screen.
    pub date: NaiveDate,
    pub precision: DatePrecision,
}

impl OriginMoment {
    /// Can this become a chart? Only a real day can.
    ///
    /// The single gate between "Wikidata knows roughly when" and "Ziqpu will cast a chart for it".
    pub fn is_chartable(&self) -> bool {
        self.precision == DatePrecision::Day
    }

    /// How this reads in a list the seeker is choosing from.
    pub fn describe(&self) -> String {
        let when = self.precision.render(self.date);
        if self.is_chartable() {
            format!("{}: {when}", self.property.label)
        } else {
            format!(
                "{}: {when} ({} precision only — not chartable)",
                self.property.label,
                match self.precision {
                    DatePrecision::Year => "year",
                    DatePrecision::Month => "month",
                    DatePrecision::Day => unreachable!("a day is chartable"),
                }
            )
        }
    }

    /// Turn this into a birth moment — **`None` unless the date is a real day**.
    ///
    /// The time is `None`, not noon: Wikidata records no clock time for any of these properties, and
    /// this project withholds angles rather than inventing an hour to fill them. `tz`/`lat`/`lon`
    /// come from the caller because Wikidata's date carries no place, and guessing one would be the
    /// same invention one field over.
    pub fn to_birth_moment(&self, tz: chrono_tz::Tz, lat: f64, lon: f64) -> Option<BirthMoment> {
        self.is_chartable().then_some(BirthMoment {
            date: self.date,
            time: None,
            tz,
            lat,
            lon,
        })
    }
}

/// Every origin moment an entity carries, ranked, plus what it resolved to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginLifecycle {
    /// The Wikidata QID, so a reader can verify the resolution rather than trust it.
    pub qid: String,
    /// Wikidata's own label for the entity.
    pub label: String,
    /// Ranked: chartable first, then by [`ORIGIN_PROPERTIES`], then earliest within a property.
    pub moments: Vec<OriginMoment>,
    /// The entity's English Wikipedia title, when it has one.
    ///
    /// Carried because it is the difference between a resolved entity and a *groundable* one: the
    /// existing Wikipedia worker keys off [`Choice::wiki`], so pulling this out here means a name
    /// the seeker typed arrives at the roster already able to be described, instead of resolving to
    /// a date and nothing else.
    pub wiki: Option<String>,
}

impl OriginLifecycle {
    /// The best moment to offer as a default — the first chartable one, or `None` if the entity has
    /// no day-precise origin at all.
    ///
    /// "Offer", not "use". A caller still shows the whole list; this only decides what is
    /// pre-selected. An entity with nothing chartable pre-selects nothing rather than falling back
    /// to a year, because a fallback here is a fabricated day by another name.
    pub fn default_moment(&self) -> Option<&OriginMoment> {
        self.moments.iter().find(|m| m.is_chartable())
    }

    /// The moments that exist but cannot be charted. Worth surfacing on their own: "we know when,
    /// just not precisely enough" is a different message from "we found nothing".
    pub fn unchartable(&self) -> Vec<&OriginMoment> {
        self.moments.iter().filter(|m| !m.is_chartable()).collect()
    }

    /// Wikidata page for the entity, so the seeker can check the source.
    pub fn source_url(&self) -> String {
        format!("https://www.wikidata.org/wiki/{}", self.qid)
    }

    /// Build the [`Choice`] the measuring loop already knows how to handle, from a chosen moment.
    ///
    /// `None` when the moment is not day-precise — the same gate as
    /// [`OriginMoment::to_birth_moment`], restated here so there is no route from a year to a chart
    /// through this constructor either.
    ///
    /// The **QID becomes the ticker**. That field is the entity's identifier everywhere downstream,
    /// and a QID is stable, unique, and checkable at [`Self::source_url`] — where a typed name is
    /// none of those. It also keeps `classify_entity` honest: a QID is not a VIN and not a GS1
    /// string, so the entity lands in `Named`, which is exactly what it is.
    ///
    /// `moment` is taken by reference and not required to come from `self.moments`; a caller that
    /// lets the seeker pick from a list is holding one of them anyway, and demanding an index would
    /// buy nothing but a way to pass the wrong one.
    pub fn to_choice(
        &self,
        moment: &OriginMoment,
        tz: chrono_tz::Tz,
        lat: f64,
        lon: f64,
    ) -> Option<crate::types::Choice> {
        let birth = moment.to_birth_moment(tz, lat, lon)?;
        Some(crate::types::Choice {
            ticker: self.qid.clone(),
            name: self.label.clone(),
            birth,
            // No CIK: this path is for entities that are not US public filers. One that IS will
            // arrive through the ticker table with its CIK already attached.
            cik: None,
            wiki: self.wiki.clone(),
        })
    }
}

/// Parse one Wikidata time value string into `(date, precision)`.
///
/// The format is `+2020-11-12T00:00:00Z`, with a leading sign and — this is the trap — a possible
/// **month or day of `00`** for coarse values, which `NaiveDate` rejects outright. Wikidata writes
/// `+1990-00-00T00:00:00Z` for some year-precision values and `+1990-01-01T00:00:00Z` for others;
/// both mean "1990". A `00` is normalised to `01` so the value survives to be *labelled* as
/// year-precision, rather than being dropped as unparseable and disappearing from the lifecycle.
///
/// Negative years (BCE, a leading `-`) are refused: chrono handles them but nothing downstream in
/// this product does, and an entity founded in 500 BCE is not a case this app has an honest answer
/// for.
pub fn parse_wikidata_time(time: &str, precision: u64) -> Option<(NaiveDate, DatePrecision)> {
    let precision = DatePrecision::from_wikidata(precision)?;
    let rest = time.strip_prefix('+')?;
    let datepart = rest.split('T').next()?;

    let mut parts = datepart.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if year <= 0 {
        return None;
    }

    // A `00` placeholder becomes `01` so the value is kept and labelled, not silently lost. This
    // does NOT make it a real day — `precision` is what decides that, and it is carried out intact.
    let date = NaiveDate::from_ymd_opt(year, month.max(1), day.max(1))?;
    Some((date, precision))
}

/// Read every origin moment out of a fetched Wikidata entity, ranked.
///
/// Pure: it takes the parsed JSON and returns the lifecycle, so the whole ranking and precision
/// discipline is testable against fixtures with no network at all. The fetching lives in
/// [`resolve_origin`].
pub fn lifecycle_from_entity(entity: &Value, qid: &str, label: &str) -> OriginLifecycle {
    let claims = &entity["entities"][qid]["claims"];
    let mut moments = Vec::new();

    for property in ORIGIN_PROPERTIES {
        let Some(values) = claims[property.id].as_array() else {
            continue;
        };
        for value in values {
            // `deprecated` claims are ones Wikidata itself marks as wrong or superseded. Reading one
            // would be quoting a correction as if it were the fact.
            if value["rank"].as_str() == Some("deprecated") {
                continue;
            }
            let datavalue = &value["mainsnak"]["datavalue"]["value"];
            let (Some(time), Some(precision)) =
                (datavalue["time"].as_str(), datavalue["precision"].as_u64())
            else {
                continue;
            };
            if let Some((date, precision)) = parse_wikidata_time(time, precision) {
                moments.push(OriginMoment {
                    property,
                    date,
                    precision,
                });
            }
        }
    }

    let rank = |p: &OriginProperty| {
        ORIGIN_PROPERTIES
            .iter()
            .position(|q| q.id == p.id)
            .unwrap_or(usize::MAX)
    };
    moments.sort_by(|a, b| {
        // 1. Chartable first — a day-precise moment beats a year-precise one whatever the dates
        //    say, because only one of them can actually produce a chart.
        b.is_chartable()
            .cmp(&a.is_chartable())
            // 2. Then the property that best answers "when did this enter the world" — NOT the
            //    earliest date.
            //
            //    This compared dates first at first, and the live probe caught it on the Eiffel
            //    Tower: Wikidata gives it inception 1887-01-28 (construction began) and official
            //    opening 1889-03-31. Date-first picked 1887 — a tower that did not exist yet. The
            //    module doc above already said opening should outrank inception for a place; the
            //    sort just did not implement the sentence written over it.
            .then_with(|| rank(&a.property).cmp(&rank(&b.property)))
            // 3. Then earliest WITHIN a single property, where "first" really does mean first
            //    existence: a later P577 is the same product arriving in another region.
            .then(a.date.cmp(&b.date))
    });
    moments.dedup();

    OriginLifecycle {
        qid: qid.to_string(),
        label: label.to_string(),
        moments,
        wiki: entity["entities"][qid]["sitelinks"]["enwiki"]["title"]
            .as_str()
            .map(str::to_string),
    }
}

/// Why an entity has no chartable origin — reported rather than returned as a bare `None`.
///
/// The same reasoning as `ephemeris::AnalyticReason` and `GroundedRung`: a caller that gets `None`
/// can only say "nothing found", which reads as "this entity has no origin" when the truth is
/// usually "the name did not resolve" or "we know the year and not the day". Those need different
/// words in front of a seeker, so they are different values here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginGap {
    /// The name matched nothing in Wikidata's search.
    Unresolved { searched: String },
    /// A match came back, but its label shares no significant word with what was asked for — almost
    /// always a search landing on something unrelated. Refused rather than reported, because a
    /// confidently-wrong origin is worse than none.
    MisResolved { searched: String, landed_on: String },
    /// Resolved cleanly and carries no origin property at all.
    NoOriginProperties(OriginLifecycle),
    /// Resolved, has origin moments, none of them day-precise. The lifecycle is carried so a caller
    /// can still show what IS known.
    NothingChartable(OriginLifecycle),
}

impl OriginGap {
    /// What to tell the seeker.
    pub fn explain(&self) -> String {
        match self {
            OriginGap::Unresolved { searched } => {
                format!("nothing in Wikidata matched \"{searched}\"")
            }
            OriginGap::MisResolved {
                searched,
                landed_on,
            } => format!(
                "the closest Wikidata match for \"{searched}\" was \"{landed_on}\", which looks \
                 like a different thing — refused rather than guessed"
            ),
            OriginGap::NoOriginProperties(life) => format!(
                "{} is in Wikidata but records no founding, release, or opening date",
                life.label
            ),
            OriginGap::NothingChartable(life) => format!(
                "{} records {} — known, but not to the day, so no chart can be cast from it",
                life.label,
                life.unchartable()
                    .iter()
                    .map(|m| m.describe())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        }
    }
}

/// A name resolved to a Wikidata entity, before any origin reading.
///
/// Exists so a caller that needs to inspect the entity for its *own* reasons — chiefly
/// [`crate::grounded::ProductSource`], which must refuse organizations because the company worker
/// already speaks for them — can do that without re-implementing the search, the label guard, and
/// the fetch. Those three were duplicated across two sources before this, which is the shape of
/// defect this codebase pays for most often.
pub struct ResolvedEntity {
    pub qid: String,
    pub label: String,
    pub entity: Value,
}

/// Search Wikidata for `name`, guard the match, and fetch the entity.
///
/// The label guard is not optional politeness. A name search is fuzzy, and a wrong entity does not
/// produce a vague answer — it produces a confident, day-precise, entirely fictional birth moment,
/// which is the worst possible failure for this product.
pub fn resolve_entity(name: &str, user_agent: &str) -> Result<ResolvedEntity, OriginGap> {
    let Some((qid, label)) = crate::grounded::wikidata_resolve_qid(name, user_agent) else {
        return Err(OriginGap::Unresolved {
            searched: name.to_string(),
        });
    };
    if !crate::grounded::label_overlaps(name, &label) {
        return Err(OriginGap::MisResolved {
            searched: name.to_string(),
            landed_on: label,
        });
    }
    let Some(entity) = crate::grounded::wikidata_entity(&qid, user_agent) else {
        return Err(OriginGap::Unresolved {
            searched: name.to_string(),
        });
    };
    Ok(ResolvedEntity { qid, label, entity })
}

/// Resolve a name to its origin lifecycle via Wikidata's keyless API.
///
/// Two network calls: a search for the QID, then the entity itself. No key, CC0 data, and the QID is
/// returned so the result is checkable at [`OriginLifecycle::source_url`] rather than taken on
/// trust.
///
/// `Err(OriginGap)` distinguishes the four ways this comes back empty — see [`OriginGap`].
pub fn resolve_origin(name: &str, user_agent: &str) -> Result<OriginLifecycle, OriginGap> {
    let resolved = resolve_entity(name, user_agent)?;
    let life = lifecycle_from_entity(&resolved.entity, &resolved.qid, &resolved.label);
    if life.moments.is_empty() {
        return Err(OriginGap::NoOriginProperties(life));
    }
    if life.default_moment().is_none() {
        return Err(OriginGap::NothingChartable(life));
    }
    Ok(life)
}

/// [`resolve_origin`] with the project's standard User-Agent.
///
/// Wikidata's API asks callers to identify themselves; the shared builder is reused so this cannot
/// drift from the one the other sources send — and so the owner's personal address stays out of it,
/// which was a real finding once.
pub fn resolve_origin_default(name: &str) -> Result<OriginLifecycle, OriginGap> {
    resolve_origin(name, &crate::grounded::sec_user_agent())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Build a claims fixture: `(property, time, precision, rank)`.
    fn entity(qid: &str, claims: &[(&str, &str, u64, &str)]) -> Value {
        let mut map = serde_json::Map::new();
        for (property, time, precision, rank) in claims {
            map.entry(property.to_string())
                .or_insert_with(|| json!([]))
                .as_array_mut()
                .unwrap()
                .push(json!({
                    "rank": rank,
                    "mainsnak": { "datavalue": { "value": {
                        "time": time, "precision": precision
                    }}}
                }));
        }
        json!({ "entities": { qid: { "claims": Value::Object(map) } } })
    }

    #[test]
    fn wikidata_precision_maps_to_what_we_may_claim() {
        assert_eq!(DatePrecision::from_wikidata(11), Some(DatePrecision::Day));
        assert_eq!(DatePrecision::from_wikidata(10), Some(DatePrecision::Month));
        assert_eq!(DatePrecision::from_wikidata(9), Some(DatePrecision::Year));
        // Decade and coarser are refused outright rather than rounded up to a year — rounding would
        // manufacture precision the source never claimed.
        for coarse in [0, 6, 7, 8] {
            assert_eq!(DatePrecision::from_wikidata(coarse), None, "{coarse}");
        }
    }

    /// The January-1st trap, which is the whole reason `precision` is carried around.
    #[test]
    fn a_year_precision_january_first_never_becomes_a_day() {
        let (date, precision) = parse_wikidata_time("+1990-01-01T00:00:00Z", 9).unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(1990, 1, 1).unwrap());
        assert_eq!(precision, DatePrecision::Year);

        let moment = OriginMoment {
            property: ORIGIN_PROPERTIES[3],
            date,
            precision,
        };
        assert!(!moment.is_chartable());
        assert_eq!(moment.to_birth_moment(chrono_tz::UTC, 0.0, 0.0), None);
        // And it renders as a year, so the placeholder day never reaches a screen either.
        assert_eq!(
            moment.describe(),
            "founded / inception: 1990 (year precision only — not chartable)"
        );
    }

    /// Wikidata writes `00` for unknown month/day; chrono rejects those outright.
    #[test]
    fn zero_month_and_day_survive_as_a_year_rather_than_vanishing() {
        let (date, precision) = parse_wikidata_time("+1888-00-00T00:00:00Z", 9).unwrap();
        assert_eq!(date.format("%Y").to_string(), "1888");
        assert_eq!(precision, DatePrecision::Year);
        // Kept and labelled, not dropped — a dropped value is indistinguishable from "we found
        // nothing", which is a different and false message.
        assert_eq!(precision.render(date), "1888");
    }

    #[test]
    fn bce_and_malformed_values_are_refused() {
        assert_eq!(parse_wikidata_time("-0500-01-01T00:00:00Z", 9), None);
        assert_eq!(parse_wikidata_time("+0000-01-01T00:00:00Z", 9), None);
        assert_eq!(parse_wikidata_time("garbage", 11), None);
        assert_eq!(parse_wikidata_time("+2020-13-45T00:00:00Z", 11), None);
    }

    /// The PlayStation 5 case named in the existing product source: three regional releases.
    #[test]
    fn the_earliest_day_leads_and_the_others_are_kept() {
        let e = entity(
            "Q63184502",
            &[
                ("P577", "+2020-11-19T00:00:00Z", 11, "normal"),
                ("P577", "+2020-11-12T00:00:00Z", 11, "normal"),
                ("P577", "+2020-12-11T00:00:00Z", 11, "normal"),
            ],
        );
        let life = lifecycle_from_entity(&e, "Q63184502", "PlayStation 5");

        assert_eq!(
            life.moments.len(),
            3,
            "later releases are kept, not discarded"
        );
        assert_eq!(
            life.default_moment().unwrap().date,
            NaiveDate::from_ymd_opt(2020, 11, 12).unwrap(),
            "the first release is when it entered the world"
        );
        assert!(life.unchartable().is_empty());
        assert_eq!(life.source_url(), "https://www.wikidata.org/wiki/Q63184502");
    }

    /// A day-precise moment outranks an *earlier* year-precise one. This is the ordering rule that
    /// matters: charting is the thing being chosen for, so chartability leads.
    #[test]
    fn a_chartable_day_beats_an_earlier_unchartable_year() {
        let e = entity(
            "Q1",
            &[
                ("P571", "+1900-01-01T00:00:00Z", 9, "normal"), // earlier, year only
                ("P577", "+1975-04-04T00:00:00Z", 11, "normal"), // later, real day
            ],
        );
        let life = lifecycle_from_entity(&e, "Q1", "Thing");

        assert_eq!(life.moments.len(), 2);
        assert_eq!(life.moments[0].property.id, "P577");
        assert!(life.moments[0].is_chartable());
        assert_eq!(life.default_moment().unwrap().property.id, "P577");

        // The year is still there and still says so — it is real, just not chartable.
        let coarse = life.unchartable();
        assert_eq!(coarse.len(), 1);
        assert_eq!(coarse[0].property.id, "P571");
        assert!(coarse[0].describe().contains("not chartable"));
    }

    /// The Eiffel Tower, with the real claim shape the live probe returned.
    ///
    /// Wikidata records inception 1887-01-28 (construction began) AND official opening 1889-03-31.
    /// Sorting by date first chose 1887 — a chart cast for a tower that was a hole in the ground.
    /// For a place, "officially opened" is the defensible origin, which is what `ORIGIN_PROPERTIES`
    /// says and what the sort now actually does.
    #[test]
    fn a_place_is_born_when_it_opens_not_when_digging_started() {
        let e = entity(
            "Q243",
            &[
                ("P571", "+1887-01-28T00:00:00Z", 11, "normal"),
                ("P571", "+1889-03-31T00:00:00Z", 11, "normal"),
                ("P1619", "+1889-03-31T00:00:00Z", 11, "normal"),
                ("P1619", "+1889-05-15T00:00:00Z", 11, "normal"),
            ],
        );
        let life = lifecycle_from_entity(&e, "Q243", "Eiffel Tower");
        let best = life.default_moment().unwrap();

        assert_eq!(
            best.property.id, "P1619",
            "opening outranks inception for a place"
        );
        assert_eq!(best.date, NaiveDate::from_ymd_opt(1889, 3, 31).unwrap());
        assert_ne!(
            best.date,
            NaiveDate::from_ymd_opt(1887, 1, 28).unwrap(),
            "1887-01-28 is when construction began, not when the tower existed"
        );
        // Every moment stays visible — the seeker can still choose the construction start.
        assert_eq!(life.moments.len(), 4);
    }

    /// The seam into the measuring loop: a chartable moment becomes a `Choice`, and only that.
    #[test]
    fn a_chartable_moment_becomes_a_choice_and_a_year_never_does() {
        let mut e = entity(
            "Q19610114",
            &[
                ("P577", "+2017-03-03T00:00:00Z", 11, "normal"),
                ("P571", "+2016-01-01T00:00:00Z", 9, "normal"),
            ],
        );
        e["entities"]["Q19610114"]["sitelinks"]["enwiki"]["title"] = json!("Nintendo Switch");
        let life = lifecycle_from_entity(&e, "Q19610114", "Nintendo Switch");

        let good = life.default_moment().unwrap();
        let choice = life
            .to_choice(good, chrono_tz::UTC, 0.0, 0.0)
            .expect("a day is chartable");

        // The QID is the identifier — stable, unique, and checkable, which a typed name is not.
        assert_eq!(choice.ticker, "Q19610114");
        assert_eq!(choice.name, "Nintendo Switch");
        assert_eq!(
            choice.birth.date,
            NaiveDate::from_ymd_opt(2017, 3, 3).unwrap()
        );
        assert_eq!(choice.cik, None, "this path is not for US public filers");
        // Carried so the existing Wikipedia worker can describe it without a second lookup.
        assert_eq!(choice.wiki.as_deref(), Some("Nintendo Switch"));

        // And the year-precision moment cannot become one, through this constructor either.
        let coarse = life.unchartable()[0];
        assert!(
            life.to_choice(coarse, chrono_tz::UTC, 0.0, 0.0).is_none(),
            "a year-precision moment must not reach the measuring loop"
        );
    }

    /// An entity with no English Wikipedia page still resolves — `wiki` is simply absent.
    #[test]
    fn a_missing_wikipedia_page_is_none_not_an_empty_string() {
        let e = entity("Q9", &[("P577", "+2001-02-03T00:00:00Z", 11, "normal")]);
        let life = lifecycle_from_entity(&e, "Q9", "Obscure Thing");
        assert_eq!(life.wiki, None);

        let choice = life
            .to_choice(life.default_moment().unwrap(), chrono_tz::UTC, 0.0, 0.0)
            .unwrap();
        assert_eq!(
            choice.wiki, None,
            "an empty string would look like a real title"
        );
    }

    /// Wikidata marks superseded claims `deprecated`. Quoting one repeats a known-wrong fact.
    #[test]
    fn deprecated_claims_are_not_read() {
        let e = entity(
            "Q2",
            &[
                ("P577", "+1999-01-02T00:00:00Z", 11, "deprecated"),
                ("P577", "+2001-05-06T00:00:00Z", 11, "normal"),
            ],
        );
        let life = lifecycle_from_entity(&e, "Q2", "Corrected Thing");
        assert_eq!(life.moments.len(), 1);
        assert_eq!(
            life.moments[0].date,
            NaiveDate::from_ymd_opt(2001, 5, 6).unwrap()
        );
    }

    /// An entity with nothing chartable pre-selects nothing — it does not fall back to a year.
    #[test]
    fn nothing_chartable_offers_nothing_rather_than_a_january_first() {
        let e = entity("Q3", &[("P571", "+1919-01-01T00:00:00Z", 9, "normal")]);
        let life = lifecycle_from_entity(&e, "Q3", "Old Company");

        assert!(life.default_moment().is_none(), "no day, no default");
        assert_eq!(life.unchartable().len(), 1);
        assert!(!life.moments.is_empty(), "but we still say what we DO know");
    }

    #[test]
    fn an_entity_with_no_origin_properties_is_empty_not_wrong() {
        let e = entity("Q4", &[("P1128", "+2020-01-01T00:00:00Z", 11, "normal")]);
        let life = lifecycle_from_entity(&e, "Q4", "Irrelevant");
        assert!(life.moments.is_empty());
        assert!(life.default_moment().is_none());
    }

    /// A chartable moment yields a birth moment with **no time** — angles get withheld downstream
    /// rather than an hour being invented to fill them.
    #[test]
    fn a_charted_origin_carries_no_invented_clock_time() {
        let e = entity("Q5", &[("P1619", "+1889-03-31T00:00:00Z", 11, "normal")]);
        let life = lifecycle_from_entity(&e, "Q5", "Eiffel Tower");
        let moment = life.default_moment().unwrap();

        let birth = moment
            .to_birth_moment(chrono_tz::Europe::Paris, 48.8584, 2.2945)
            .expect("a day is chartable");
        assert_eq!(birth.date, NaiveDate::from_ymd_opt(1889, 3, 31).unwrap());
        assert_eq!(birth.time, None, "Wikidata records no clock time for P1619");

        let (_, time_known) = birth.julian_day_ut();
        assert!(!time_known, "so the chart must withhold its angles");
    }
}
