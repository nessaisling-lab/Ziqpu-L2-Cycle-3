# Ziqpu database

A contained, isolated Postgres holding each choice's **birth moment** (IPO date/time + exchange
coordinates) plus chart/reading caches. Start locally: `docker compose up -d --wait db`
(then `scripts/db-apply.sh` to re-apply schema + seed to an already-running instance).

## Files
- `init/01_schema.sql` — schema: `company_metadata`, `natal_positions`, `synastry_readings`.
- `init/02_seed.sql` — **GENERATED** seed: 5,271 US-market tickers. Do not hand-edit.

On a first `up` with an empty volume, both files run automatically (filename order).
`docker compose down -v` wipes the volume so the next `up` re-seeds cleanly.

## Data provenance
`company_metadata` originates from the Nisaba engine's Polygon-sourced dataset
(NYSE / NASDAQ / NYSE American / CBOE), exported from the private Nisaba Postgres and
transformed here. Every ticker keeps its exchange coordinates as the chart location.

## Validation / transform rules (the "double-check")
- Exchange → `tz` = `America/New_York` (all listings are US exchanges).
- IPO dates outside `[1792-01-01, today]` are treated as **UNKNOWN** (`NULL`) — impossible for a
  US-listed security, so not fabricated into a bogus chart (3 such rows were caught).
- ~2,351 tickers have a usable IPO date (fully chartable); ~2,920 are date-unknown (charted
  partially, flagged). Filling the unknowns from public sources (SEC EDGAR / exchange
  directories) is **route-3** follow-up work.

## Regenerate the seed
1. Export from the source DB:
   ```
   psql "$NISABA_DATABASE_URL" -c "\copy (SELECT ticker, company_name, ipo_date, ipo_time, exchange, latitude, longitude, founding_date, data_source, notes FROM company_metadata ORDER BY ticker) TO 'company_metadata.csv' CSV HEADER"
   ```
2. Load into a text staging table, apply the plausibility gate + tz mapping into
   `company_metadata`, then dump:
   ```
   docker compose exec -T db pg_dump -U ziqpu -d ziqpu --data-only --table=company_metadata --no-owner --no-privileges > db/init/02_seed.sql
   ```
The raw `company_metadata.csv` is gitignored; only the generated seed is committed.
