//! Drive the Vedic layer from a real chart — sidereal frame, whole-sign houses, nakshatra, dasha.
use engine::{compute_chart_with, ChartOptions, DashaTimeline, WholeSignHouses};
use ephemeris::{julian_day, Ayanamsa, Body, Zodiac};

fn main() {
    let e = ephemeris::shared();
    println!("engine: {}\n", e.source.badge());

    // Apple's founding, the moment the app actually charts for AAPL.
    let jd = julian_day(1976, 4, 1, 12.0);
    let opts = ChartOptions {
        zodiac: Zodiac::Sidereal(Ayanamsa::Lahiri),
        ..Default::default()
    };
    let chart = compute_chart_with(e.backend.as_ref(), jd, 37.33, -122.03, false, opts);

    println!("zodiac: {}", chart.zodiac.label());
    let moon = chart
        .bodies
        .iter()
        .find(|b| b.body == Body::Moon)
        .expect("Moon");
    println!("Moon:   {:.2}° {} ", moon.longitude, moon.sign);

    let t = DashaTimeline::from_moon(moon.longitude, chart.time_known);
    let n = &t.moon_nakshatra;
    println!("\nnakshatra: {} (pada {}), lord {}", n.name, n.pada, n.lord);
    println!(
        "balance at birth: {:.2} years of {}",
        t.balance_years, t.periods[0].lord
    );
    println!("\nVimshottari from birth:");
    for p in &t.periods {
        println!(
            "  {:<8} {:>7.2} → {:>7.2} years",
            p.lord,
            p.starts_years.max(0.0),
            p.ends_years()
        );
    }
    println!(
        "\nat age 30 the ruling lord is: {}",
        t.lord_at(30.0).map(|p| p.lord).unwrap_or("—")
    );
    if let Some(c) = t.caveat() {
        println!("\ncaveat: {c}");
    }

    // Whole-sign houses need an ascendant, which needs a known time — so this chart has none.
    match chart.ascendant {
        Some(asc) => {
            let h = WholeSignHouses::from_ascendant(asc);
            println!("\nhouses (whole-sign, asc {:.1}°):", asc);
            for b in &chart.bodies { println!("  {:<10} house {}", b.body.name(), h.house_of(b.longitude)); }
        }
        None => println!("\nno ascendant: the birth time is unknown, so houses are withheld rather than invented."),
    }
}
