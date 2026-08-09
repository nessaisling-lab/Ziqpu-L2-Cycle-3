#!/usr/bin/env bash
# Download the JPL DE440s planetary kernel for the ANISE backend.
# DE440/DE441 are U.S. Government works (public-domain-equivalent), freely redistributable.
set -euo pipefail
dir="${EPHEMERIS_PATH:-./data/ephemeris}"
out="$dir/de440s.bsp"
url="https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440s.bsp"
mkdir -p "$dir"
# The pin lives in ONE place — the source constant the app checks at load. Reading it here instead
# of repeating the digest means the script cannot drift from what the runtime will accept, which is
# the failure mode where a download "succeeds" and then every chart silently falls to the floor.
repo="$(cd "$(dirname "$0")/.." && pwd)"
pin="$(grep -oE '"[0-9a-f]{64}"' "$repo/crates/ephemeris/src/resolve.rs" | head -1 | tr -d '"')"

verify() {
  [ -n "$pin" ] || return 0   # no pin readable (script copied elsewhere) — do not block on it
  have="$(sha256sum "$1" 2>/dev/null | cut -d" " -f1 || shasum -a 256 "$1" | cut -d" " -f1)"
  [ "$have" = "$pin" ]
}

# Cache hit: a file already present AND matching the pin is a no-op. Presence alone is not enough —
# a truncated download from an interrupted earlier run is non-empty and useless, and the old check
# would have treated it as done forever.
if [ -s "$out" ]; then
  if verify "$out"; then
    echo "already present and verified: $out ($(du -h "$out" | cut -f1))"
    exit 0
  fi
  echo "present but does NOT match the pinned digest — refetching: $out"
  rm -f "$out"
fi
echo "downloading DE440s (~32 MB) -> $out"
# Resilient against transient network flakes (CI #23): retry on any error, fail on HTTP errors so a
# partial/error body is not written, and bound the connect time.
curl -sSfL \
  --retry 5 --retry-all-errors --retry-delay 5 \
  --connect-timeout 20 --fail \
  -o "$out" "$url"
if verify "$out"; then
  echo "done: $(du -h "$out" | cut -f1), sha256 verified"
else
  echo "FAILED: downloaded kernel does not match the pinned digest in crates/ephemeris/src/resolve.rs" >&2
  echo "  expected $pin" >&2
  rm -f "$out"
  exit 1
fi
