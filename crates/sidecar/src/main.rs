//! Ziqpu read-only sidecar.
//!
//! A small axum API over the interpretation engine, the ephemeris backend, and the
//! contained Postgres. It only ever READS `company_metadata` — no endpoint mutates the DB.
//! Endpoints: `/health`, `/chart/:ticker`, `/synastry/:a/:b`, `/transits/:date`.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use engine::{compute_chart, find_aspect, NatalChart};
use ephemeris::{julian_day, AnalyticBackend};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

const BACKEND: &str = "analytic (VSOP87 + Meeus)";
const SYNASTRY_ORB: f64 = 6.0;
const NOT_ADVICE: &str =
    "Symbolic reflection of how these charts aspect — not a prediction, not financial advice.";

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    let db = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://ziqpu:ziqpu_dev@localhost:5433/ziqpu".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db)
        .await
        .expect("connect to Postgres");

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/chart/:ticker", get(chart_handler))
        .route("/synastry/:a/:b", get(synastry_handler))
        .route("/transits/:date", get(transits_handler))
        .with_state(AppState { pool });

    let addr = "0.0.0.0:8787";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    println!("ziqpu sidecar listening on http://{addr}  (read-only)");
    axum::serve(listener, app).await.expect("serve");
}

// --- serialization ---------------------------------------------------------

#[derive(Serialize)]
struct BodyJson {
    body: String,
    longitude: f64,
    sign: String,
    degree: f64,
    retrograde: bool,
    speed: f64,
}

#[derive(Serialize)]
struct ChartResponse {
    ticker: String,
    company_name: String,
    exchange: String,
    ipo_date: String,
    ipo_time: Option<String>,
    time_known: bool,
    backend: String,
    ascendant: Option<f64>,
    midheaven: Option<f64>,
    bodies: Vec<BodyJson>,
    note: String,
}

#[derive(Serialize)]
struct AspectJson {
    body_a: String,
    body_b: String,
    aspect: String,
    orb: f64,
    harmonious: bool,
}

#[derive(Serialize)]
struct SynastryResponse {
    a: String,
    b: String,
    backend: String,
    aspects: Vec<AspectJson>,
    note: String,
}

#[derive(Serialize)]
struct TransitsResponse {
    date: String,
    backend: String,
    bodies: Vec<BodyJson>,
    note: String,
}

// --- helpers ---------------------------------------------------------------

fn bodies_json(chart: &NatalChart) -> Vec<BodyJson> {
    chart
        .bodies
        .iter()
        .map(|b| BodyJson {
            body: b.body.name().to_string(),
            longitude: round2(b.longitude),
            sign: b.sign.to_string(),
            degree: round2(b.degree),
            retrograde: b.retrograde,
            speed: round2(b.speed),
        })
        .collect()
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

/// Convert a local birth date/time at an exchange timezone into (JD UT, time_known).
/// An unknown time uses local noon and reports `time_known = false` so angles are withheld.
fn birth_jd(date: NaiveDate, time: Option<NaiveTime>, tz: Tz) -> (f64, bool) {
    let (naive_time, known) = match time {
        Some(t) => (t, true),
        None => (NaiveTime::from_hms_opt(12, 0, 0).unwrap(), false),
    };
    let local = NaiveDateTime::new(date, naive_time);
    let utc = tz
        .from_local_datetime(&local)
        .earliest()
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Utc.from_utc_datetime(&local));
    let hour = utc.hour() as f64 + utc.minute() as f64 / 60.0 + utc.second() as f64 / 3600.0;
    (julian_day(utc.year(), utc.month(), utc.day(), hour), known)
}

struct ChartMeta {
    ticker: String,
    company_name: String,
    exchange: String,
    ipo_date: NaiveDate,
    ipo_time: Option<NaiveTime>,
}

/// Load a ticker's metadata and compute its natal chart, or an HTTP error.
async fn load_chart(
    pool: &PgPool,
    ticker: &str,
) -> Result<(ChartMeta, NatalChart), (StatusCode, String)> {
    let t = ticker.to_uppercase();
    let row = sqlx::query(
        "SELECT company_name, ipo_date, ipo_time, exchange, tz, latitude, longitude \
         FROM company_metadata WHERE ticker = $1",
    )
    .bind(&t)
    .fetch_optional(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, format!("unknown ticker {t}")))?;

    let company_name: String = row.get("company_name");
    let exchange: String = row.get("exchange");
    let ipo_date: Option<NaiveDate> = row.get("ipo_date");
    let ipo_time: Option<NaiveTime> = row.get("ipo_time");
    let tz_name: String = row.get("tz");
    let latitude: f64 = row.get("latitude");
    let longitude: f64 = row.get("longitude");

    let ipo_date = ipo_date.ok_or((
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("{t}: birth date unknown — not enough data to chart"),
    ))?;
    let tz: Tz = tz_name.parse().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("bad tz {tz_name}"),
        )
    })?;

    let (jd, time_known) = birth_jd(ipo_date, ipo_time, tz);
    let chart = compute_chart(&AnalyticBackend, jd, latitude, longitude, time_known);
    Ok((
        ChartMeta {
            ticker: t,
            company_name,
            exchange,
            ipo_date,
            ipo_time,
        },
        chart,
    ))
}

// --- handlers --------------------------------------------------------------

async fn chart_handler(
    State(st): State<AppState>,
    Path(ticker): Path<String>,
) -> Result<Json<ChartResponse>, (StatusCode, String)> {
    let (meta, chart) = load_chart(&st.pool, &ticker).await?;
    Ok(Json(ChartResponse {
        ticker: meta.ticker,
        company_name: meta.company_name,
        exchange: meta.exchange,
        ipo_date: meta.ipo_date.to_string(),
        ipo_time: meta.ipo_time.map(|t| t.format("%H:%M").to_string()),
        time_known: chart.time_known,
        backend: BACKEND.to_string(),
        ascendant: chart.ascendant.map(round2),
        midheaven: chart.midheaven.map(round2),
        bodies: bodies_json(&chart),
        note: NOT_ADVICE.to_string(),
    }))
}

async fn synastry_handler(
    State(st): State<AppState>,
    Path((a, b)): Path<(String, String)>,
) -> Result<Json<SynastryResponse>, (StatusCode, String)> {
    let (ma, ca) = load_chart(&st.pool, &a).await?;
    let (mb, cb) = load_chart(&st.pool, &b).await?;
    let mut aspects = Vec::new();
    for pa in &ca.bodies {
        for pb in &cb.bodies {
            if let Some((aspect, orb)) = find_aspect(pa.longitude, pb.longitude, SYNASTRY_ORB) {
                aspects.push(AspectJson {
                    body_a: pa.body.name().to_string(),
                    body_b: pb.body.name().to_string(),
                    aspect: aspect.name().to_string(),
                    orb: round2(orb),
                    harmonious: aspect.is_harmonious(),
                });
            }
        }
    }
    aspects.sort_by(|x, y| x.orb.total_cmp(&y.orb));
    Ok(Json(SynastryResponse {
        a: ma.ticker,
        b: mb.ticker,
        backend: BACKEND.to_string(),
        aspects,
        note: NOT_ADVICE.to_string(),
    }))
}

async fn transits_handler(
    Path(date): Path<String>,
) -> Result<Json<TransitsResponse>, (StatusCode, String)> {
    let d: NaiveDate = date.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            format!("bad date {date}, want YYYY-MM-DD"),
        )
    })?;
    // Transits are sky positions (bodies only), computed for noon UT at Greenwich.
    let jd = julian_day(d.year(), d.month(), d.day(), 12.0);
    let chart = compute_chart(&AnalyticBackend, jd, 0.0, 0.0, false);
    Ok(Json(TransitsResponse {
        date: d.to_string(),
        backend: BACKEND.to_string(),
        bodies: bodies_json(&chart),
        note: NOT_ADVICE.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birth_jd_converts_est_to_utc() {
        // 1980-12-12 09:30 EST (no DST) = 14:30 UT.
        let d = NaiveDate::from_ymd_opt(1980, 12, 12).unwrap();
        let t = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
        let (jd, known) = birth_jd(d, Some(t), chrono_tz::America::New_York);
        assert!(known);
        assert!((jd - julian_day(1980, 12, 12, 14.5)).abs() < 1e-6);
    }

    #[test]
    fn birth_jd_handles_summer_dst() {
        // 2010-06-29 09:30 EDT (DST, -4) = 13:30 UT.
        let d = NaiveDate::from_ymd_opt(2010, 6, 29).unwrap();
        let t = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
        let (jd, _) = birth_jd(d, Some(t), chrono_tz::America::New_York);
        assert!((jd - julian_day(2010, 6, 29, 13.5)).abs() < 1e-6);
    }

    #[test]
    fn birth_jd_unknown_time_flagged() {
        let d = NaiveDate::from_ymd_opt(1919, 9, 5).unwrap();
        let (_, known) = birth_jd(d, None, chrono_tz::America::New_York);
        assert!(!known);
    }
}
