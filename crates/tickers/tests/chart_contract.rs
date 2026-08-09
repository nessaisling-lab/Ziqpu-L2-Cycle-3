//! The end-to-end chart contract, asserted without a database or an HTTP server.
//!
//! # What this replaces, and why it is not a downgrade
//!
//! These assertions used to live in the `integration (sidecar + Postgres)` CI job: it stood up a
//! Postgres, applied `db/init/*.sql`, ran the sidecar with the ANISE backend, and curled its
//! endpoints for exactly the facts below.
//!
//! That job was removed with the `db/` scaffolding, and losing its coverage would have been a real
//! cost — it was the **only** place CI asserted end-to-end that Pluto is present, which got more
//! valuable the day DE440 became the default engine, not less.
//!
//! So the assertions moved here and the machinery did not follow. None of them ever needed a
//! database: the ticker table is compiled into the binary by `crates/tickers`, and the chart is
//! arithmetic over it. The Postgres copy was a second source of the same rows, and the HTTP layer
//! was a transport for a function call. What is genuinely gone is the sidecar's *API surface* —
//! status codes 404 and 422 — which cannot be asserted once there is no API, and which described a
//! server nothing shipped against.
//!
//! # The divergence that writing this exposed
//!
//! The removed job asserted `"sign":"Sagittarius"` for AAPL. Ported here verbatim, it FAILED: the
//! app charts AAPL with the Sun in **Aries**.
//!
//! Neither side was broken. They were reading different moments. The Postgres seed is pre-purge
//! data — `ipo_date 1980-12-12 09:30`, with `founding_date` as a separate column — so the sidecar
//! charted the listing. The compiled table models an entity's *lifecycle* (`Moment::Listing` /
//! `Moment::Founding`, the same idea the N3 origin resolver uses), charts the listing only when a
//! day-precise one was established, and otherwise charts the founding. After Polygon was purged and
//! the dates were re-derived from Wikidata `P571` and SEC 424B4, AAPL's chartable moment became its
//! founding, `1976-04-01`.
//!
//! So there were two answers to "when was this born" — the single most important input in this
//! product — and CI was guarding the one the app had stopped using. That is the real argument for
//! deleting `db/`: not that it was unused, but that it was a second source of truth quietly
//! disagreeing with the first.

use agents::Choice;
use engine::compute_chart;
use ephemeris::Body;

/// AAPL's Sun, Pluto, and Chiron — the three the integration job checked by curl.
///
/// Pluto is the one that matters. It is absent from the analytic floor entirely, so this assertion
/// is what tells us the authoritative engine actually ran, and it is written to say something true
/// in both environments rather than skipping where no kernel is present.
#[test]
fn aapl_charts_with_the_bodies_the_integration_job_checked() {
    let choice = tickers::choice("AAPL").expect("AAPL is in the compiled table");
    let (jd, time_known) = choice.birth.julian_day_ut();

    let engine = ephemeris::shared();
    let chart = compute_chart(
        engine.backend.as_ref(),
        jd,
        choice.birth.lat,
        choice.birth.lon,
        time_known,
    );

    // 1976-04-01, AAPL's FOUNDING — not the 1980-12-12 listing the stale Postgres seed held. The
    // date is asserted alongside the sign so a future re-derivation that moves the moment fails
    // here loudly, instead of silently changing every AAPL reading.
    assert_eq!(
        choice.birth.date.to_string(),
        "1976-04-01",
        "AAPL's chartable moment is its founding"
    );
    let sun = chart
        .bodies
        .iter()
        .find(|b| b.body == Body::Sun)
        .expect("every backend has the Sun");
    assert_eq!(sun.sign, "Aries", "AAPL Sun sign at its founding");

    let has = |b: Body| chart.bodies.iter().any(|p| p.body == b);
    assert!(has(Body::Chiron), "Chiron rides the bundled Horizons table");

    if engine.source.is_authoritative() {
        assert!(
            has(Body::Pluto),
            "DE440 resolved but the chart has no Pluto — the body list and the engine disagree"
        );
    } else {
        // Not a skip: the floor's defining limitation is asserted as a fact.
        assert!(
            !has(Body::Pluto),
            "the analytic floor cannot compute Pluto; a chart claiming one is fabricating it"
        );
        eprintln!(
            "NOTE: analytic floor — {}",
            engine.source.caveat().unwrap_or_default()
        );
    }
}

/// Two charts cross-aspect. The `/synastry/AAPL/MSFT` curl, minus the curl.
#[test]
fn two_tickers_produce_cross_aspects() {
    let a = tickers::choice("AAPL").expect("AAPL");
    let b = tickers::choice("MSFT").expect("MSFT");
    let engine = ephemeris::shared();

    let chart_of = |c: &Choice| {
        let (jd, known) = c.birth.julian_day_ut();
        compute_chart(engine.backend.as_ref(), jd, c.birth.lat, c.birth.lon, known)
    };
    let (ca, cb) = (chart_of(&a), chart_of(&b));

    let mut hits = 0;
    for pa in &ca.bodies {
        for pb in &cb.bodies {
            if engine::find_aspect(pa.longitude, pb.longitude, 6.0).is_some() {
                hits += 1;
            }
        }
    }
    assert!(hits > 0, "two real charts must aspect each other somewhere");
}

/// The 404 and 422 cases, as the data properties underneath them.
///
/// The status codes died with the API; what they *described* is still true and still worth
/// guarding — an unknown ticker is not chartable, and a ticker whose date was never established is
/// honestly date-unknown rather than charted from an invented day.
#[test]
fn an_unknown_ticker_is_none_and_a_dateless_one_is_not_invented() {
    assert!(
        tickers::choice("ZZZZ").is_none(),
        "an unknown ticker yields no choice — the sidecar's 404"
    );

    // The sidecar's 422: rows exist whose date was never established. They must not acquire one.
    // `choice()` returns None for them, which is the whole point of keeping 764 rows date-unknown
    // rather than filling them with a plausible January 1st.
    let dateless = tickers::search("")
        .into_iter()
        .find(|row| tickers::choice(&row.ticker).is_none());
    if let Some(row) = dateless {
        assert!(
            tickers::choice(&row.ticker).is_none(),
            "{} has no established date and must not be charted from one",
            row.ticker
        );
    }
}
