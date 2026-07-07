#!/usr/bin/env bash
# Regenerate crates/ephemeris/data/chiron.bin from JPL Horizons.
#
# Chiron (asteroid 2060) has no analytic theory, and Horizons emits it as an SPK "Type 21"
# that ANISE cannot read. So we sample its geocentric ecliptic longitude (1900-2100, 2-day
# steps) and bundle an f32 little-endian table the ephemeris crate interpolates. Public-domain
# US-gov data. Requires: curl, awk, node.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="$here/crates/ephemeris/data/chiron.bin"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

url="https://ssd.jpl.nasa.gov/api/horizons.api?format=text&COMMAND=%272060%3B%27&EPHEM_TYPE=OBSERVER&CENTER=%27500@399%27&START_TIME=%271900-01-01%27&STOP_TIME=%272100-01-01%27&STEP_SIZE=%272%20d%27&QUANTITIES=%2731%27"
echo "fetching Chiron ephemeris from JPL Horizons..."
curl -sSfL "$url" -o "$tmp/raw.txt"
awk '/\$\$SOE/{f=1;next} /\$\$EOE/{f=0} f {print $(NF-1)}' "$tmp/raw.txt" > "$tmp/lons.txt"

mkdir -p "$(dirname "$out")"
node -e "
const fs=require('fs');
const lons=fs.readFileSync('$tmp/lons.txt','utf8').trim().split(/\s+/).map(Number);
if(lons.length<30000||lons.some(v=>!isFinite(v))) throw new Error('unexpected Horizons output');
const buf=Buffer.alloc(lons.length*4);
lons.forEach((v,i)=>buf.writeFloatLE(v,i*4));
fs.writeFileSync('$out',buf);
console.log('wrote',lons.length,'samples to',fs.realpathSync('$out'));
"
