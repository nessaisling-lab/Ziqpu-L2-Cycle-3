#!/usr/bin/env bash
# Download the JPL DE440s planetary kernel for the ANISE backend.
# DE440/DE441 are U.S. Government works (public-domain-equivalent), freely redistributable.
set -euo pipefail
dir="${EPHEMERIS_PATH:-./data/ephemeris}"
out="$dir/de440s.bsp"
url="https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440s.bsp"
mkdir -p "$dir"
# Cache hit: a non-empty file already present is a no-op (matches the CI actions/cache restore).
if [ -s "$out" ]; then
  echo "already present: $out ($(du -h "$out" | cut -f1))"
  exit 0
fi
echo "downloading DE440s (~32 MB) -> $out"
# Resilient against transient network flakes (CI #23): retry on any error, fail on HTTP errors so a
# partial/error body is not written, and bound the connect time.
curl -sSfL \
  --retry 5 --retry-all-errors --retry-delay 5 \
  --connect-timeout 20 --fail \
  -o "$out" "$url"
echo "done: $(du -h "$out" | cut -f1)"
