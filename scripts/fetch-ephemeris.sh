#!/usr/bin/env bash
# Download the JPL DE440s planetary kernel for the ANISE backend.
# DE440/DE441 are U.S. Government works (public-domain-equivalent), freely redistributable.
set -euo pipefail
dir="${EPHEMERIS_PATH:-./data/ephemeris}"
out="$dir/de440s.bsp"
url="https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440s.bsp"
mkdir -p "$dir"
if [ -f "$out" ]; then
  echo "already present: $out ($(du -h "$out" | cut -f1))"
  exit 0
fi
echo "downloading DE440s (~32 MB) -> $out"
curl -sSfL -o "$out" "$url"
echo "done: $(du -h "$out" | cut -f1)"
