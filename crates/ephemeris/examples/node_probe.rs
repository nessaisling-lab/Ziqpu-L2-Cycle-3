//! Measure the osculating node against the known behaviour of the real one.
use ephemeris::{julian_day, true_node, Body, Ephemeris};

fn diff(a: f64, b: f64) -> f64 {
    let mut d = a - b;
    if d > 180.0 {
        d -= 360.0
    } else if d < -180.0 {
        d += 360.0
    }
    d
}

fn run<E: Ephemeris + ?Sized>(label: &str, eph: &E) {
    let start = julian_day(2024, 1, 1, 0.0);
    let (mut max_lib, mut min_lib) = (f64::MIN, f64::MAX);
    let (mut direct_days, mut n) = (0usize, 0usize);
    let (mut sum_lib, mut prev) = (0.0, None::<f64>);

    for step in 0..730 {
        let jd = start + step as f64;
        let Ok(t) = true_node(eph, jd) else { continue };
        let m = match eph.position(Body::MeanNode, jd) {
            Ok(p) => p.longitude,
            Err(_) => continue,
        };
        let lib = diff(t, m);
        max_lib = max_lib.max(lib);
        min_lib = min_lib.min(lib);
        sum_lib += lib;
        n += 1;
        if let Some(p) = prev {
            if diff(t, p) > 0.0 {
                direct_days += 1
            }
        }
        prev = Some(t);
    }
    if n == 0 {
        println!("{label}: no positions");
        return;
    }
    let first = true_node(eph, start).unwrap();
    let last = true_node(eph, start + 729.0).unwrap();
    println!("{label}");
    println!(
        "  libration vs mean node   {min_lib:+.3} .. {max_lib:+.3} deg   (real node: about +/-1.6)"
    );
    println!(
        "  mean libration           {:+.4} deg   (should hover near 0)",
        sum_lib / n as f64
    );
    println!("  days moving DIRECT       {direct_days} of {n}   (real node: a minority, not zero)");
    println!(
        "  net motion over 2 years  {:+.2} deg   (mean node covers about -38.7 over 2 years)",
        diff(last, first)
    );
}

fn main() {
    let e = ephemeris::shared();
    println!("resolved engine: {}\n", e.source.badge());
    run("resolved engine", e.backend.as_ref());
    println!();
    run("analytic floor", &ephemeris::AnalyticBackend);
}
