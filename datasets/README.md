# Datasets — domain "birth moment" data for Ziqpu

Ziqpu's synastry engine is **domain-agnostic**: it charts any entity with a *birth moment*
(a date/time + a location). Stocks use their IPO. Each new domain is just a dataset in this shape
(plus a small adapter later). This folder is where partners collect that data for the agents to use.

## The shared schema (every domain dataset uses this)

| column | meaning |
|---|---|
| `id` | stable identifier — the ticker-equivalent (ICAO code, NCES id, NAIC id) |
| `name` | display name |
| `birth_date` | the domain's founding / inception / first-event date (`YYYY-MM-DD`; empty = unknown) |
| `birth_time` | local clock time at the location (`HH:MM`; empty = unknown → the chart flags houses/Ascendant approximate) |
| `location` | human label (airport, city, campus) |
| `tz` | IANA timezone of the location (e.g. `America/New_York`) |
| `latitude` | decimal degrees, North positive |
| `longitude` | decimal degrees, East positive |
| `data_source` | where the row came from (e.g. `wikidata`, `openflights`, `nces`, `naic`) |
| `notes` | anything worth flagging (approximation, conflicting sources) |

**Honesty rule** (same as the stock data): if a date or time is unknown, **leave it empty** — never
invent one. A missing date just means that entity charts partially. Keep dates plausible for the
domain (no 1600s airlines).

## Domains
- [`aviation/`](aviation/) — airlines & first flights (Nathan)
- [`schools/`](schools/) — schools & founding/charter (Ahsan)
- [`insurance/`](insurance/) — carriers & inception (Ahsan)

## How to contribute
1. Work on a branch named **`<yourfirstname>/<topic>`** — e.g. `nathan/airlines-v1`, `ahsan/schools-v1`.
2. Curate CSVs under your domain folder (start from that folder's `entities.template.csv`).
3. Open a pull request. **CI must be green, and the repo owner approves & merges** — you cannot
   push straight to `main`.
4. Sign your commits: `git commit -s` (the DCO check requires a `Signed-off-by` line).
