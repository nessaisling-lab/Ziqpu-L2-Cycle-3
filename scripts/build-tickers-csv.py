#!/usr/bin/env python3
"""Transform scripts/out/derived.csv into the shipped crates/tickers/data/company_metadata.csv.

`derive-dates.py` produces the raw two-date extract from CC0 + public-domain sources. This applies
the owner's scope decisions (2026-07-17) and shapes it into what the `tickers` crate compiles in.

SCHEMA (N3-ready: two dated moments with per-date precision + provenance, so adding a car's
manufacture date or a product's release date later is a new source feeding an existing shape, not a
migration):

    ticker,name,exchange,cik,
    conception_date,conception_prec,conception_src,
    birth_date,birth_prec,birth_src,
    note

- Dates are ISO 8601 REDUCED precision: `1978` | `1978-10` | `1978-10-05` | blank. NEVER YYYY-01-01
  for a year-only fact — that lie is now inexpressible (it was the 904-row bug).
- Coordinates are NOT stored per row: they are the exchange's floor, a constant the parser maps
  (NYSE / NASDAQ / CBOE). Chart LOCATION = where the listing happened.

SCOPE DECISIONS APPLIED:
1. Keep every NYSE/NASDAQ/CBOE ticker (owner: "any ticker available to the retail investor is of
   providence"). Undatable rows are KEPT and marked, never dropped ("list them marked, don't hide").
2. Conception conflicts (209, incl. Coca-Cola) ship with a BLANK conception + note "conflict" — no
   automatic rule is honest (precision and rank both pick "first Coke sold"). Still chartable on
   birth if it has one.
3. Delisted / OTC tickers are simply absent from SEC's exchange universe — correctly gone.

Usage:  python scripts/build-tickers-csv.py
"""
import csv, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "out", "derived.csv")
DST = os.path.join(HERE, "..", "crates", "tickers", "data", "company_metadata.csv")


def compact_src(s):
    """Short, stable provenance token for the CSV."""
    return {"wikidata-p571": "wikidata", "sec-424b": "sec-424b", "needs-review": "conflict"}.get(s, s or "")


def main():
    if not os.path.exists(SRC):
        sys.exit(f"missing {SRC} — run scripts/derive-dates.py first")
    rows = list(csv.DictReader(open(SRC, encoding="utf-8")))

    out = []
    n_chartable = 0
    for r in rows:
        c_date = r["conception_date"]
        c_prec = r["conception_prec"]
        c_src = compact_src(r["conception_source"])
        # A conflict has no usable conception; keep the row, keep the flag, blank the date.
        if r["conception_source"] == "needs-review":
            c_date, c_prec, c_src = "", "", "conflict"

        b_date = r["birth_date"]
        b_prec = "day" if b_date else ""
        b_src = "sec-424b" if b_date else ""

        # A note that says WHY when both are blank — the honest surface for an unchartable row.
        note = ""
        if not b_date and not c_date:
            note = r["birth_note"] or r["conception_note"]

        # Chartable == we hold a DAY-precise moment for it (a chart needs a day for the angles).
        # Birth is preferred (the v1 "listing" framing); else a day-precise conception.
        chartable = bool(b_date) or c_prec == "day"
        if chartable:
            n_chartable += 1

        out.append([
            r["ticker"], r["name"], r["exchange"], r["cik"],
            c_date, c_prec, c_src,
            b_date, b_prec, b_src,
            note,
        ])

    out.sort(key=lambda x: x[0])
    dst = os.path.abspath(DST)
    with open(dst, "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["ticker", "name", "exchange", "cik",
                    "conception_date", "conception_prec", "conception_src",
                    "birth_date", "birth_prec", "birth_src", "note"])
        w.writerows(out)

    # Verify the artefact, not the pipeline (this repo's recurring lesson).
    back = list(csv.reader(open(dst, encoding="utf-8")))
    if len(back) - 1 != len(out):
        sys.exit("CORRUPT: row count mismatch on read-back")
    if any(len(row) != 11 for row in back):
        sys.exit("CORRUPT: wrong field count on read-back")

    print(f"wrote {dst}  [verified {len(out)} rows x 11 fields]")
    print(f"  chartable (day-precise moment) : {n_chartable} ({n_chartable/len(out):.1%})")
    print(f"  unchartable (listed, marked)   : {len(out)-n_chartable}")


if __name__ == "__main__":
    main()
