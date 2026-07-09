//! Aspect-pattern detection over a merged, chart-tagged placement set (PRD §7.3).
//!
//! Patterns are read across *both* charts at once — a Grand Trine whose legs belong to different
//! people is exactly the kind of cross-chart figure synastry cares about — so every placement
//! carries a [`Who`] tag and every reported member says which chart it came from.
//!
//! The Ptolemaic legs reuse [`crate::find_aspect`]; the Yod's 150° quincunx is measured with
//! [`crate::separation`] directly (a quincunx is deliberately *not* an [`crate::Aspect`], so it
//! never leaks into ordinary scoring). All roles are keyed and sorted on `label()` strings because
//! [`ephemeris::Body`] is not `Ord`.

use crate::{find_aspect, separation, Aspect};
use ephemeris::Body;
use std::collections::HashSet;

/// Which chart a placement belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Who {
    Seeker,
    Choice,
}

impl Who {
    pub fn label(self) -> &'static str {
        match self {
            Who::Seeker => "Seeker",
            Who::Choice => "Choice",
        }
    }
}

/// A body placed at a longitude, tagged with the chart it came from. The input unit for detection.
#[derive(Debug, Clone, Copy)]
pub struct Placed {
    pub who: Who,
    pub body: Body,
    pub longitude: f64,
}

/// One participant in a detected pattern — a body and the chart it belongs to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Member {
    pub who: Who,
    pub body: Body,
}

impl Member {
    /// A stable, orderable label like `"Seeker Sun"` — the basis of every dedup key and sort.
    pub fn label(&self) -> String {
        format!("{} {}", self.who.label(), self.body.name())
    }
}

/// A detected aspect pattern across the merged placement set.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Three placements in a pairwise Trine.
    GrandTrine { members: [Member; 3] },
    /// An Opposition pair with a third placement squaring both (the apex).
    TSquare { ends: [Member; 2], apex: Member },
    /// A Sextile base pair with a third placement quincunx to both (the apex).
    Yod { base: [Member; 2], apex: Member },
    /// A tight cluster of `min_stellium`+ placements within `stellium_span` degrees.
    Stellium { members: Vec<Member> },
}

/// Orb tolerances (degrees) and the stellium size floor. `Default` matches the sidecar's figures.
#[derive(Debug, Clone, Copy)]
pub struct PatternOrbs {
    pub conjunction: f64,
    pub stellium_span: f64,
    pub trine: f64,
    pub opposition: f64,
    pub square: f64,
    pub sextile: f64,
    pub quincunx: f64,
    pub min_stellium: usize,
}

impl Default for PatternOrbs {
    fn default() -> Self {
        Self {
            conjunction: 8.0,
            stellium_span: 8.0,
            trine: 7.0,
            opposition: 7.0,
            square: 6.0,
            sextile: 5.0,
            quincunx: 3.0,
            min_stellium: 3,
        }
    }
}

fn member(p: &Placed) -> Member {
    Member {
        who: p.who,
        body: p.body,
    }
}

/// Whether two placements form `target` within `orb`. `find_aspect` already guarantees the
/// returned aspect is within orb, so we only check the family matches.
fn is_leg(a: &Placed, b: &Placed, target: Aspect, orb: f64) -> bool {
    matches!(find_aspect(a.longitude, b.longitude, orb), Some((asp, _)) if asp == target)
}

/// Whether two placements are ~150° apart (a quincunx), measured directly since a quincunx is not
/// a Ptolemaic [`Aspect`].
fn is_quincunx(a: &Placed, b: &Placed, orb: f64) -> bool {
    (separation(a.longitude, b.longitude) - 150.0).abs() <= orb
}

/// Detect all Grand Trines, T-Squares, Yods, and Stellia in the merged set. Results are
/// deduplicated by a canonical key and sorted by (kind, first member label) for a stable order.
pub fn detect_patterns(placed: &[Placed], orbs: &PatternOrbs) -> Vec<Pattern> {
    let n = placed.len();
    let mut found: Vec<Pattern> = Vec::new();

    // Grand Trine: every pair among a triple is a Trine.
    for i in 0..n {
        for j in (i + 1)..n {
            if !is_leg(&placed[i], &placed[j], Aspect::Trine, orbs.trine) {
                continue;
            }
            for k in (j + 1)..n {
                if is_leg(&placed[i], &placed[k], Aspect::Trine, orbs.trine)
                    && is_leg(&placed[j], &placed[k], Aspect::Trine, orbs.trine)
                {
                    let mut members = [member(&placed[i]), member(&placed[j]), member(&placed[k])];
                    members.sort_by_key(|m| m.label());
                    found.push(Pattern::GrandTrine { members });
                }
            }
        }
    }

    // T-Square: an Opposition pair, plus a third placement squaring BOTH ends (the apex).
    // A Grand Cross is left to surface as its constituent T-Squares — no special case.
    for i in 0..n {
        for j in (i + 1)..n {
            if !is_leg(&placed[i], &placed[j], Aspect::Opposition, orbs.opposition) {
                continue;
            }
            for (k, apex) in placed.iter().enumerate() {
                if k == i || k == j {
                    continue;
                }
                if is_leg(apex, &placed[i], Aspect::Square, orbs.square)
                    && is_leg(apex, &placed[j], Aspect::Square, orbs.square)
                {
                    let mut ends = [member(&placed[i]), member(&placed[j])];
                    ends.sort_by_key(|m| m.label());
                    found.push(Pattern::TSquare {
                        ends,
                        apex: member(apex),
                    });
                }
            }
        }
    }

    // Yod: a Sextile base pair, plus a third placement quincunx to BOTH (the apex).
    for i in 0..n {
        for j in (i + 1)..n {
            if !is_leg(&placed[i], &placed[j], Aspect::Sextile, orbs.sextile) {
                continue;
            }
            for (k, apex) in placed.iter().enumerate() {
                if k == i || k == j {
                    continue;
                }
                if is_quincunx(apex, &placed[i], orbs.quincunx)
                    && is_quincunx(apex, &placed[j], orbs.quincunx)
                {
                    let mut base = [member(&placed[i]), member(&placed[j])];
                    base.sort_by_key(|m| m.label());
                    found.push(Pattern::Yod {
                        base,
                        apex: member(apex),
                    });
                }
            }
        }
    }

    // Stellium: tight longitude clusters, robust to the 360/0 wrap.
    found.extend(detect_stellia(placed, orbs));

    // Dedup by canonical key, then sort by (kind rank, first member label).
    let mut seen: HashSet<String> = HashSet::new();
    let mut unique: Vec<Pattern> = Vec::new();
    for p in found {
        if seen.insert(canonical_key(&p)) {
            unique.push(p);
        }
    }
    unique.sort_by(|a, b| {
        kind_rank(a)
            .cmp(&kind_rank(b))
            .then_with(|| first_label(a).cmp(&first_label(b)))
    });
    unique
}

/// Greedy longitude clustering. Sort by longitude, cut the circle at its widest empty arc, then
/// grow clusters while each forward gap stays within the conjunction orb; keep those of size
/// `min_stellium`+ whose total span fits `stellium_span`.
fn detect_stellia(placed: &[Placed], orbs: &PatternOrbs) -> Vec<Pattern> {
    let m = placed.len();
    if m < orbs.min_stellium {
        return Vec::new();
    }
    let mut items: Vec<&Placed> = placed.iter().collect();
    items.sort_by(|a, b| {
        a.longitude
            .rem_euclid(360.0)
            .total_cmp(&b.longitude.rem_euclid(360.0))
    });
    let lons: Vec<f64> = items
        .iter()
        .map(|p| p.longitude.rem_euclid(360.0))
        .collect();

    // Find the widest circular gap; start the walk on the placement just after it.
    let mut max_gap = -1.0;
    let mut start = 0;
    for g in 0..m {
        let next = (g + 1) % m;
        let gap = if next == 0 {
            lons[next] + 360.0 - lons[g]
        } else {
            lons[next] - lons[g]
        };
        if gap > max_gap {
            max_gap = gap;
            start = next;
        }
    }

    // Unroll longitudes to a strictly increasing sequence beginning at `start`.
    let base = lons[start];
    let mut order: Vec<(usize, f64)> = Vec::with_capacity(m);
    for step in 0..m {
        let idx = (start + step) % m;
        let mut lon = lons[idx];
        if lon < base {
            lon += 360.0;
        }
        order.push((idx, lon));
    }

    let mut result = Vec::new();
    let mut cluster: Vec<usize> = vec![order[0].0];
    let mut cluster_start = order[0].1;
    let mut prev = order[0].1;
    for &(idx, lon) in &order[1..] {
        if lon - prev <= orbs.conjunction {
            cluster.push(idx);
            prev = lon;
        } else {
            push_stellium(&items, &cluster, cluster_start, prev, orbs, &mut result);
            cluster = vec![idx];
            cluster_start = lon;
            prev = lon;
        }
    }
    push_stellium(&items, &cluster, cluster_start, prev, orbs, &mut result);
    result
}

fn push_stellium(
    items: &[&Placed],
    cluster: &[usize],
    start_lon: f64,
    end_lon: f64,
    orbs: &PatternOrbs,
    out: &mut Vec<Pattern>,
) {
    if cluster.len() >= orbs.min_stellium && (end_lon - start_lon) <= orbs.stellium_span {
        let mut members: Vec<Member> = cluster.iter().map(|&i| member(items[i])).collect();
        members.sort_by_key(|m| m.label());
        out.push(Pattern::Stellium { members });
    }
}

/// A stable dedup key: kind + sorted member labels (+ apex where the apex role is meaningful).
fn canonical_key(p: &Pattern) -> String {
    match p {
        Pattern::GrandTrine { members } => format!("GrandTrine|{}", sorted_labels(members)),
        Pattern::TSquare { ends, apex } => {
            format!("TSquare|{}|apex={}", sorted_labels(ends), apex.label())
        }
        Pattern::Yod { base, apex } => {
            format!("Yod|{}|apex={}", sorted_labels(base), apex.label())
        }
        Pattern::Stellium { members } => format!("Stellium|{}", sorted_labels(members)),
    }
}

fn sorted_labels(members: &[Member]) -> String {
    let mut labels: Vec<String> = members.iter().map(|m| m.label()).collect();
    labels.sort();
    labels.join(",")
}

fn kind_rank(p: &Pattern) -> u8 {
    match p {
        Pattern::GrandTrine { .. } => 0,
        Pattern::TSquare { .. } => 1,
        Pattern::Yod { .. } => 2,
        Pattern::Stellium { .. } => 3,
    }
}

fn first_label(p: &Pattern) -> String {
    let members: &[Member] = match p {
        Pattern::GrandTrine { members } => members,
        Pattern::TSquare { ends, .. } => ends,
        Pattern::Yod { base, .. } => base,
        Pattern::Stellium { members } => members,
    };
    members.iter().map(|m| m.label()).min().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(who: Who, body: Body, longitude: f64) -> Placed {
        Placed {
            who,
            body,
            longitude,
        }
    }

    #[test]
    fn cross_chart_grand_trine_is_tagged() {
        let placed = [
            p(Who::Seeker, Body::Sun, 0.0),
            p(Who::Choice, Body::Moon, 120.0),
            p(Who::Seeker, Body::Mars, 240.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        let trines: Vec<&Pattern> = out
            .iter()
            .filter(|x| matches!(x, Pattern::GrandTrine { .. }))
            .collect();
        assert_eq!(trines.len(), 1, "exactly one Grand Trine");
        if let Pattern::GrandTrine { members } = trines[0] {
            // The Moon leg is tagged to the Choice chart, the two others to the Seeker.
            let moon = members.iter().find(|m| m.body == Body::Moon).unwrap();
            assert_eq!(moon.who, Who::Choice);
            assert_eq!(members.iter().filter(|m| m.who == Who::Seeker).count(), 2);
        }
    }

    #[test]
    fn grand_trine_rejects_outside_orb() {
        // 0/130/240: only the 0–240 leg is a Trine — no figure.
        let placed = [
            p(Who::Seeker, Body::Sun, 0.0),
            p(Who::Choice, Body::Moon, 130.0),
            p(Who::Seeker, Body::Mars, 240.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        assert!(out.is_empty(), "no pattern expected, got {out:?}");
    }

    #[test]
    fn t_square_apex_is_the_square_planet() {
        let placed = [
            p(Who::Seeker, Body::Sun, 0.0),
            p(Who::Choice, Body::Sun, 180.0),
            p(Who::Seeker, Body::Mars, 90.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        let tsq: Vec<&Pattern> = out
            .iter()
            .filter(|x| matches!(x, Pattern::TSquare { .. }))
            .collect();
        assert_eq!(tsq.len(), 1);
        if let Pattern::TSquare { apex, .. } = tsq[0] {
            assert_eq!(apex.body, Body::Mars);
            assert_eq!(apex.who, Who::Seeker);
        }
    }

    #[test]
    fn yod_via_separation_finds_the_apex() {
        // Sextile base 0/60, apex quincunx to both at 210. A quincunx is not a Ptolemaic aspect.
        assert_eq!(find_aspect(0.0, 210.0, 3.0), None);
        let placed = [
            p(Who::Seeker, Body::Venus, 0.0),
            p(Who::Seeker, Body::Mercury, 60.0),
            p(Who::Choice, Body::Saturn, 210.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        let yods: Vec<&Pattern> = out
            .iter()
            .filter(|x| matches!(x, Pattern::Yod { .. }))
            .collect();
        assert_eq!(yods.len(), 1);
        if let Pattern::Yod { apex, .. } = yods[0] {
            assert_eq!(apex.body, Body::Saturn);
            assert_eq!(apex.who, Who::Choice);
        }
    }

    #[test]
    fn stellium_clusters_and_excludes_outlier() {
        let placed = [
            p(Who::Seeker, Body::Sun, 10.0),
            p(Who::Seeker, Body::Mercury, 14.0),
            p(Who::Seeker, Body::Venus, 17.0),
            p(Who::Choice, Body::Mars, 40.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        let stellia: Vec<&Pattern> = out
            .iter()
            .filter(|x| matches!(x, Pattern::Stellium { .. }))
            .collect();
        assert_eq!(stellia.len(), 1);
        if let Pattern::Stellium { members } = stellia[0] {
            assert_eq!(members.len(), 3, "the tight three, not the outlier");
            assert!(members.iter().all(|m| m.body != Body::Mars));
        }
    }

    #[test]
    fn grand_trine_triple_dedups_to_one() {
        // Feeding the Grand Trine triple must yield exactly one figure, never three.
        let placed = [
            p(Who::Seeker, Body::Sun, 0.0),
            p(Who::Choice, Body::Moon, 120.0),
            p(Who::Seeker, Body::Mars, 240.0),
        ];
        let out = detect_patterns(&placed, &PatternOrbs::default());
        assert_eq!(out.len(), 1);
        assert!(matches!(out[0], Pattern::GrandTrine { .. }));
    }
}
