#!/usr/bin/env bash
# Apply Ziqpu schema + seed to the Postgres at $DATABASE_URL (or the local dev default).
# Used both locally and in CI (where the DB is a service container, so initdb.d is not used).
# Idempotent: schema uses IF NOT EXISTS, seed uses ON CONFLICT DO NOTHING.
set -euo pipefail
: "${DATABASE_URL:=postgres://ziqpu:ziqpu_dev@localhost:5433/ziqpu}"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
for f in "$here"/db/init/*.sql; do
  echo "applying $(basename "$f")"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -q -f "$f"
done
echo "schema + seed applied."
