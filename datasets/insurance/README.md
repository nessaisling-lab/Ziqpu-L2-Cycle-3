# Insurance datasets (Nathan)

**Birth moment — pick one convention and note it per row:**
- the carrier's **incorporation date** (the entity's birth), or
- a **policy's effective date** (the decision's birth).

Location = the carrier's headquarters (or the state-of-domicile capital).

Curate into `entities.csv` (copy the header from `entities.template.csv`).

## Suggested sources
- **NAIC** company directory — carrier ids + state of domicile: <https://content.naic.org/>.
- **State insurance department** filings (incorporation / licensing dates).
- **Wikidata** "inception" for large carriers.

## Notes
- `id` = NAIC company code where available.
- Insurance is the hardest domain for a clean inception source — flag low-confidence rows in `notes`,
  and leave `birth_date` empty rather than guessing.
