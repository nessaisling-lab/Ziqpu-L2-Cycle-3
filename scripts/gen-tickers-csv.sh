#!/usr/bin/env bash
# Regenerate crates/tickers/data/company_metadata.csv from db/init/02_seed.sql.
#
# WHY THIS EXISTS
# ---------------
# The seed and the app's dataset drifted, and the app got the worse half.
#
# db/init/02_seed.sql carries the route-3 enrichment (PR #6): tickers Polygon left date-unknown
# were backfilled from their earliest SEC EDGAR Form 8-A filing, raising chartable coverage from
# ~2,351 to 4,507 of 5,271 (see db/README.md). That enrichment was applied to the DATABASE and
# never propagated to crates/tickers/data/company_metadata.csv, which is what actually ships —
# so the binary carried the PRE-enrichment Polygon export: 2,354 dated, 2,917 unknown. Worse,
# `tickers::choice_in` then charted every one of those 2,917 on a fabricated DEFAULT_LISTING_DATE
# (2000-01-01) rather than admitting it didn't know — while the database honestly NULLed them.
#
# The seed is the better source, so the CSV is generated FROM it. Running this is how the two stay
# in step; the drift happened because nothing did.
#
# Usage:  bash scripts/gen-tickers-csv.sh
# Needs:  nothing but awk. No database, no Docker — it reads the committed seed as text.

set -euo pipefail

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
seed="$repo/db/init/02_seed.sql"
out="$repo/crates/tickers/data/company_metadata.csv"

[ -f "$seed" ] || { echo "missing $seed" >&2; exit 1; }

# The seed is a pg_dump COPY block: tab-separated, \N for NULL, terminated by a lone "\.".
# Its columns:  ticker company_name ipo_date ipo_time exchange tz latitude longitude founding_date data_source notes
# The CSV drops `tz` — it is the constant America/New_York for every US listing (db/README.md), and
# `tickers` hardcodes it for the Stocks universe.
awk -F'\t' '
  function csv(s) {
    if (s == "\\N") return ""                       # SQL NULL -> empty cell -> honestly unknown
    if (s ~ /[",]/) { gsub(/"/, "\"\"", s); return "\"" s "\"" }
    return s
  }
  /^COPY public\.company_metadata/ { inblock = 1; next }
  inblock && /^\\\.$/             { inblock = 0; next }
  inblock && NF >= 11 {
    print csv($1) "," csv($2) "," csv($3) "," csv($4) "," csv($5) "," \
          csv($7) "," csv($8) "," csv($9) "," csv($10) "," csv($11)
  }
' "$seed" > "$out.rows"

{
  echo "ticker,company_name,ipo_date,ipo_time,exchange,latitude,longitude,founding_date,data_source,notes"
  cat "$out.rows"
} > "$out"
rm -f "$out.rows"

# Count from the SEED, not by re-splitting the CSV on commas: company names contain commas, so they
# are quoted, and a naive -F',' miscounts by ~1,900. (It did. The first run of this script reported
# 2,649 dated when the file it had just written held 4,507 — a verification that lies is worse than
# none, because it looks like evidence.)
rows=$(($(wc -l < "$out") - 1))
dated=$(awk -F'\t' '
  /^COPY public\.company_metadata/ { inblock = 1; next }
  inblock && /^\\\.$/             { inblock = 0; next }
  inblock && $3 ~ /^[0-9]{4}-/    { n++ }
  END { print n + 0 }
' "$seed")
echo "wrote $out"
echo "  rows:     $rows"
echo "  dated:    $dated        (expected 4,507 — see db/README.md)"
echo "  unknown:  $((rows - dated))          (blank ipo_date — must stay UNCHARTABLE, never fabricated)"

# The whole point of the regeneration is the enrichment, so prove it arrived.
if ! grep -q 'sec-8a' "$out"; then
  echo "ERROR: no sec-8a rows in the output — the route-3 enrichment did not carry over." >&2
  exit 1
fi
