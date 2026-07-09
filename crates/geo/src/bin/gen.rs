//! Gazetteer blob generator — the *only* code that reads GeoNames text.
//!
//! Reads a local GeoNames `cities5000.txt` (19 tab-separated columns) and writes the compact,
//! committed `crates/geo/data/cities.bin` that `geo::lib` embeds via `include_bytes!`. Run it
//! once, by hand, to (re)produce the artifact; it is never invoked at build or test time and
//! touches no network:
//!
//! ```text
//! cargo run -p geo --bin gen -- path/to/cities5000.txt crates/geo/data/cities.bin
//! ```
//!
//! GeoNames columns used (0-indexed): 1 name · 2 asciiname · 4 lat · 5 lon · 8 country ·
//! 14 population · 17 IANA timezone. Output layout is documented on `geo::BLOB`.

use std::collections::BTreeSet;
use std::collections::HashMap;

/// One kept city row.
struct Rec {
    id: u64,
    asciiname: String,
    name: String,
    lat: f32,
    lon: f32,
    country: [u8; 2],
    population: u32,
    tz: String,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let in_path = args.next().unwrap_or_else(|| "cities5000.txt".to_string());
    let out_path = args
        .next()
        .unwrap_or_else(|| "crates/geo/data/cities.bin".to_string());

    let text = std::fs::read_to_string(&in_path)
        .unwrap_or_else(|e| panic!("failed to read {in_path}: {e}"));

    let mut recs: Vec<Rec> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 18 {
            continue;
        }
        let name = f[1];
        let asciiname = f[2];
        let country = f[8];
        let tz = f[17];
        if asciiname.is_empty() || name.is_empty() || tz.is_empty() || country.len() != 2 {
            continue;
        }
        // A u8 length prefix caps each string at 255 bytes; skip the vanishingly rare longer ones.
        if asciiname.len() > 255 || name.len() > 255 {
            continue;
        }
        let (lat, lon) = match (f[4].parse::<f32>(), f[5].parse::<f32>()) {
            (Ok(la), Ok(lo)) if la.is_finite() && lo.is_finite() => (la, lo),
            _ => continue,
        };
        let id = f[0].parse::<u64>().unwrap_or(u64::MAX);
        let population = f[14].parse::<u32>().unwrap_or(0);
        let cc = country.as_bytes();
        recs.push(Rec {
            id,
            asciiname: asciiname.to_string(),
            name: name.to_string(),
            lat,
            lon,
            country: [cc[0], cc[1]],
            population,
            tz: tz.to_string(),
        });
    }

    // Deterministic row order (by GeoNames id) so the same input yields a byte-identical blob.
    recs.sort_by_key(|r| r.id);

    // Intern timezones into a sorted string table → stable u16 indices.
    let tz_set: BTreeSet<&str> = recs.iter().map(|r| r.tz.as_str()).collect();
    let tz_list: Vec<&str> = tz_set.into_iter().collect();
    let tz_index: HashMap<&str, u16> = tz_list
        .iter()
        .enumerate()
        .map(|(i, &tz)| (tz, i as u16))
        .collect();

    let mut out: Vec<u8> = Vec::new();
    out.extend_from_slice(b"ZGEO");
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&(tz_list.len() as u32).to_le_bytes());
    out.extend_from_slice(&(recs.len() as u32).to_le_bytes());
    for tz in &tz_list {
        out.extend_from_slice(&(tz.len() as u16).to_le_bytes());
        out.extend_from_slice(tz.as_bytes());
    }
    for r in &recs {
        out.push(r.asciiname.len() as u8);
        out.extend_from_slice(r.asciiname.as_bytes());
        out.push(r.name.len() as u8);
        out.extend_from_slice(r.name.as_bytes());
        out.extend_from_slice(&r.lat.to_le_bytes());
        out.extend_from_slice(&r.lon.to_le_bytes());
        out.extend_from_slice(&tz_index[r.tz.as_str()].to_le_bytes());
        out.extend_from_slice(&r.country);
        out.extend_from_slice(&r.population.to_le_bytes());
    }

    std::fs::write(&out_path, &out).unwrap_or_else(|e| panic!("failed to write {out_path}: {e}"));
    println!(
        "wrote {out_path}: {} rows, {} timezones, {} bytes",
        recs.len(),
        tz_list.len(),
        out.len()
    );
}
