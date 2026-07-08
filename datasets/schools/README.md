# Schools datasets (Ahsan)

**Birth moment:** a school's **founding or charter date**. Location = the school's campus.

Curate into `entities.csv` (copy the header from `entities.template.csv`).

## Suggested sources
- **NCES** — US schools with location data: <https://nces.ed.gov/> (ids + coordinates).
- **Wikidata** "inception" for notable institutions.
- **State charter records** for charter schools (charter grant dates).

## Notes
- `id` = NCES school id where available.
- Many schools list only a founding **year** — use `YYYY-01-01` and flag the imprecision in `notes`
  (or leave `birth_date` empty if the year itself is uncertain).
- Campus `latitude`/`longitude`/`tz` from NCES or a geocoder.
