//! Function smoke test: a minimal end-to-end synastry-style check over a doc fixture.
//! The CI `smoke` job runs this on all three OSes as the "function smoke testing" gate.

use engine::{find_aspect, separation, Aspect};

#[test]
fn scorpio_sun_trines_pisces_sun() {
    // PRD fixture: user natal Sun 14° Scorpio (224°) vs AAPL IPO Sun 12° Pisces (342°).
    let user_sun = 224.0;
    let entity_sun = 342.0;

    assert!((separation(user_sun, entity_sun) - 118.0).abs() < 1e-9);

    let (aspect, orb) =
        find_aspect(user_sun, entity_sun, 6.0).expect("Sun–Sun should register a trine within 6°");
    assert_eq!(aspect, Aspect::Trine);
    assert!(orb <= 6.0);
}
