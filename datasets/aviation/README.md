# Aviation datasets (Nathan)

**Birth moment:** an airline's **founding date**, or its **first-flight / air-operator-certificate**
date — pick one convention and note it per row. Location = the airline's primary hub or HQ airport.

Curate into `entities.csv` (copy the header from `entities.template.csv`).

## Suggested sources
- **Wikidata / Wikipedia** — airline "inception" / "founded" property.
- **OpenFlights** — airline + airport data with **coordinates and timezone** in one place:
  <https://openflights.org/data.html> (great for `latitude`/`longitude`/`tz` via the hub airport).
- **IATA / ICAO** airline & airport directories (codes + hubs).
- **FAA / EASA** operator certificates for certification dates.

## Notes
- `id` = the airline's ICAO code (or the hub airport's IATA/ICAO) — whatever is stable.
- `latitude` / `longitude` / `tz` come from the **hub airport** (OpenFlights has all three).
- If founding and first-flight differ a lot, keep founding in `birth_date` and mention the
  first-flight date in `notes`.
