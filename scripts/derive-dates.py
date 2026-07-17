#!/usr/bin/env python3
"""Re-derive the choice universe and both of its dates from CC0 + public-domain sources ONLY.

WHY THIS EXISTS
---------------
The shipped dataset came from Polygon via a private export, and three separate problems converged:

1. LICENCE. Polygon's Market Data ToS s5(c) bars redistributing Market Data "or any data derived
   from" it. We publish thousands of its dates in a public repo headed for a paid product. Owner's
   call: purge and re-derive rather than seek permission.
2. THE COLUMN WAS MISLABELLED. Polygon's `list_date` is not a listing date — it is an inception
   field. Proven byte-for-byte against Wikidata's founding property (P571): Ford 1903-06-16, IBM
   1911-06-16, JNJ 1886-01-01 are identical in both. We were reading a CONCEPTION date and calling
   it a BIRTH.
3. THE 8-A BACKFILL WAS THE SAME MISTAKE TWICE. An SEC Form 8-A registers *a security*, not a
   listing: our rows include bonds, a poison pill, a poison pill's EXPIRATION (the Micron row we
   shipped), and 12(g) filings by companies that were not listed at all.

THE MODEL (owner's framing)
---------------------------
  CONCEPTION = founding.  BIRTH = listing/IPO.  Two different dates. We want both, with provenance.

SOURCES, and why each
---------------------
  universe/CIK/exchange : SEC company_tickers_exchange.json  (US-gov public domain)
  conception            : Wikidata P571 inception            (CC0; 0% Polygon-sourced — verified)
  birth                 : SEC EDGAR 424B4/424B1 + discriminator (public domain)

  REJECTED: Wikidata P580 (ticker tenure) — 86% of it is bulk-imported FROM Polygon, so it would
  hand our own broken data back to us wearing a CC0 hat and we would score it as independent
  corroboration. It is also 0-for-6 on the anchors.

THE DISCRIMINATOR (the whole design)
------------------------------------
EDGAR begins 1994/95; earlier is paper. So the question is not "what is the earliest prospectus"
but "did EDGAR WATCH this company go public, or did it show up already public?"

    first registration (S-1/F-1/424B4/...) EARLIER than first periodic report (10-K/10-Q/...)
        -> EDGAR saw the IPO. Date it from the earliest 424B4/424B1.
        -> else NULL, provenance "pre-EDGAR: not datable from SEC".

Validated 7/7 before this script was written. Ford's naive earliest-424B4 is 2002-01-25 — 16,810
days (46 years) off its true 1956 listing, because Ford filed a securities prospectus in 2002. The
discriminator rejects it: Ford's 1994 10-K predates its 2002 registration. A truthful blank beats a
confident wrong date, and it tells the next source exactly what to go find.

PRECISION
---------
Dates are ISO 8601 reduced precision: `1978` | `1978-10` | `1978-10-05` | blank. NEVER YYYY-01-01
for a year-only fact. The 904 Jan-1 rows we are deleting existed because the schema had no way to
say "year", so year-only knowledge got rounded up into a lie at write time. Here the lie is
inexpressible.

Usage:  python scripts/derive-dates.py [--limit N]
Output: scripts/out/derived.csv  (+ a resumable HTTP cache under scripts/out/cache/)
Needs:  python 3.9+, stdlib only. No API keys. Respects SEC's 10 req/s and UA policy.
"""
import argparse, csv, gzip, io, json, os, re, sys, time, urllib.parse, urllib.request
from datetime import date

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "out")
CACHE = os.path.join(OUT, "cache")

# SEC requires a contact that reaches a human. Same address the app sends (see grounded.rs).
UA = os.environ.get("ZIQPU_EDGAR_UA", "Ziqpu research (ness.aisling@nisabacapitalcharting.com)")
SEC_RPS = 8.0  # SEC's published limit is 10/s; stay under it. Being rate-limited is our own fault.

# "We are going public."
REG = {"S-1", "S-1/A", "F-1", "F-1/A", "S-11", "S-11/A", "SB-2", "DRS", "424B1", "424B4",
       "N-2", "10-12B", "10-12G"}
# "We are ALREADY public." Their presence BEFORE any registration means EDGAR missed the IPO.
PERI = {"10-K", "10-K405", "10-Q", "DEF 14A", "20-F", "40-F", "11-K"}
PROSPECTUS = ("424B4", "424B1")

# Registered-fund form vocabulary. A CIK that files these is a fund/trust, and its birth cannot be
# read from EDGAR's structured filings for a reason worth stating precisely:
#
# The CIK is the TRUST, not the fund, and a trust holds many funds launched on different days. Tested:
# JEPI (true inception 2020-05-20) sits on the J.P. Morgan ETF Trust CIK whose earliest 485BPOS is
# 2014-06-09 — six years early, because the TRUST launched in 2014 and JEPI was added in 2020. QQQ,
# ARKK show the same; AAAU (a grantor trust) files no 485 at all. So "take the fund's 485BPOS" is the
# Micron/Ford failure again: a real filing date for the wrong event. The individual fund's launch
# lives in unstructured prospectus text (497/497K) or a dedicated fund-data source, not here.
#
# We therefore refuse to date these from EDGAR and label them so — an accurate blank a fund source
# can later fill, not the misleading "no prospectus found" they used to share with operating
# companies.
FUND_FORMS = {"N-1A", "N-1A/A", "485APOS", "485BPOS", "485BXT", "497", "497K", "NPORT-P",
              "N-CEN", "24F-2NT", "N-8A", "N-8A/A"}

_last = [0.0]


def _throttle(rps):
    gap = 1.0 / rps
    dt = time.time() - _last[0]
    if dt < gap:
        time.sleep(gap - dt)
    _last[0] = time.time()


def fetch(url, rps=SEC_RPS, cache_key=None):
    """GET -> parsed JSON, disk-cached so a re-run costs nothing and a crash resumes."""
    if cache_key:
        p = os.path.join(CACHE, cache_key)
        if os.path.exists(p):
            with open(p, "rb") as f:
                return json.loads(gzip.decompress(f.read()))
    _throttle(rps)
    req = urllib.request.Request(url, headers={"User-Agent": UA, "Accept-Encoding": "gzip"})
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=30) as r:
                raw = r.read()
                if r.headers.get("Content-Encoding") == "gzip":
                    raw = gzip.decompress(raw)
            data = json.loads(raw)
            if cache_key:
                os.makedirs(CACHE, exist_ok=True)
                with open(os.path.join(CACHE, cache_key), "wb") as f:
                    f.write(gzip.compress(raw))
            return data
        except Exception as e:
            if attempt == 3:
                raise
            time.sleep(1.5 * (attempt + 1))


# ---------------------------------------------------------------- universe (SEC, public domain)
def sec_universe():
    d = fetch("https://www.sec.gov/files/company_tickers_exchange.json", cache_key="universe.json.gz")
    i = {f: n for n, f in enumerate(d["fields"])}
    rows = {}
    for r in d["data"]:
        ex = r[i["exchange"]]
        if ex not in ("NYSE", "Nasdaq", "CBOE"):   # OTC is out of scope for v1
            continue
        rows[r[i["ticker"]].strip().upper()] = {
            "cik": int(r[i["cik"]]),
            "name": r[i["name"]].strip(),
            "exchange": {"Nasdaq": "NASDAQ"}.get(ex, ex),
        }
    return rows


# ------------------------------------------------------- conception (Wikidata P571, CC0)
WD_QUERY = """
SELECT ?ticker ?co ?coLabel ?t ?prec ?rank WHERE {
  ?co p:P414 ?st . ?st ps:P414 ?exch ; pq:P249 ?ticker .
  VALUES ?exch { wd:Q13677 wd:Q82059 }
  ?co p:P571 ?s . ?s psv:P571 ?tv ; wikibase:rank ?rank .
  ?tv wikibase:timeValue ?t ; wikibase:timePrecision ?prec .
  SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
}
"""


def wikidata_inceptions():
    """ticker -> [ {date, prec, rank, entity, label} ]  — ALL statements, conflicts included.

    Deliberately does NOT pick one. Coca-Cola has 1886-05-08 (DAY, PreferredRank — the day the first
    Coke was SOLD) beside 1892-01-01 (YEAR, NormalRank — the true incorporation). Precision picks
    wrong; rank picks wrong. There is no automatic rule, so conflicts are surfaced, not resolved.
    """
    p = os.path.join(CACHE, "wikidata.json.gz")
    if os.path.exists(p):
        with open(p, "rb") as f:
            d = json.loads(gzip.decompress(f.read()))
    else:
        url = "https://query.wikidata.org/sparql?" + urllib.parse.urlencode({"query": WD_QUERY})
        req = urllib.request.Request(url, headers={
            "User-Agent": "Ziqpu/0.1 (https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3; " + UA + ")",
            "Accept": "application/sparql-results+json"})
        with urllib.request.urlopen(req, timeout=120) as r:
            raw = r.read()
        d = json.loads(raw)
        os.makedirs(CACHE, exist_ok=True)
        with open(p, "wb") as f:
            f.write(gzip.compress(raw))
    out = {}
    for b in d["results"]["bindings"]:
        t = b["ticker"]["value"].strip().upper()
        prec = int(b["prec"]["value"])
        iso = b["t"]["value"][:10]
        # Reduced precision: say what we know, and NOTHING more. 9=year, 10=month, 11=day.
        val = iso if prec >= 11 else (iso[:7] if prec == 10 else (iso[:4] if prec == 9 else None))
        if not val:
            continue
        out.setdefault(t, []).append({
            "date": val,
            "prec": {11: "day", 10: "month", 9: "year"}[prec],
            "rank": b["rank"]["value"].rsplit("#", 1)[-1].replace("Rank", "").lower(),
            "entity": b["co"]["value"].rsplit("/", 1)[-1],
            "label": b.get("coLabel", {}).get("value", ""),
        })
    return out


# ------------------------------------------------------------- birth (SEC EDGAR, public domain)
def edgar_filings(cik):
    """Every (form, filingDate) for a CIK — recent PLUS the shards.

    The shards are the bug that broke the old 8-A import: `filings.recent` holds only the most
    recent ~1000 filings, so for any long-lived company the OLD filings — the only ones that can
    answer "did EDGAR watch the IPO" — live in `filings.files[]` and were never read. Micron's
    recent covers 2017-2026; its 2009 and 2016 8-As were invisible.
    """
    d = fetch(f"https://data.sec.gov/submissions/CIK{cik:010d}.json", cache_key=f"cik{cik}.json.gz")
    rec = d["filings"]["recent"]
    out = list(zip(rec["form"], rec["filingDate"]))
    for sh in d["filings"].get("files", []):
        s = fetch("https://data.sec.gov/submissions/" + sh["name"], cache_key=f"sh_{sh['name']}.gz")
        out += list(zip(s["form"], s["filingDate"]))
    return out


def derive_birth(cik):
    """(date|None, source, note) — the discriminator, plus an honest fund carve-out."""
    try:
        f = edgar_filings(cik)
    except Exception as e:
        return None, "", f"edgar error: {type(e).__name__}"
    forms = {fm for fm, _ in f}

    # A registered fund/trust: its birth is a per-fund inception EDGAR does not carry at the trust
    # CIK (see FUND_FORMS). Detected BEFORE the operating-company path so a fund is never dated from
    # a filing that belongs to a sibling fund. `needs-fund-source` is a truthful blank, addressed to
    # whatever fund-inception source we adopt next — not a claim that no date exists.
    if len(forms & FUND_FORMS) >= 2 and not (forms & {"424B4", "424B1"}):
        return None, "", "needs-fund-source: registered fund; inception not in EDGAR structured filings"

    reg = sorted(d for fm, d in f if fm in REG)
    peri = sorted(d for fm, d in f if fm in PERI)
    r0, p0 = (reg[0] if reg else None), (peri[0] if peri else None)
    if not r0 or (p0 and p0 <= r0):
        # EDGAR met this company already public. Its real listing is on paper, pre-1994.
        return None, "", "pre-EDGAR: not datable from SEC"
    pro = sorted(d for fm, d in f if fm in PROSPECTUS)
    if not pro:
        return None, "", "registered but no 424B prospectus on file"
    return pro[0], "sec-424b", "first final prospectus; +-2d of first trade"


def acquire_lock():
    """Refuse to run twice at once.

    Two copies of this script writing one CSV interleave at buffer boundaries and produce rows like
    `BlackRBBLGW` and `Butterfly NetworkBFRG,BullFrog AI Holdings` — two records stitched into one.
    That happened: a backgrounded run was assumed dead, wasn't, and quietly corrupted 63 rows of a
    7,671-row extract while a second run appeared to succeed. Nothing errored; the file just had
    lies in it, and only a field-count check caught it.

    An advisory lock is cheap; a silently-corrupted dataset is what this whole task is about.
    """
    os.makedirs(OUT, exist_ok=True)
    lock = os.path.join(OUT, ".derive.lock")
    if os.path.exists(lock):
        with open(lock) as f:
            who = f.read().strip()
        sys.exit(f"another extraction is running (pid {who}, {lock}).\n"
                 f"If you are sure it is dead, delete that file and re-run.")
    with open(lock, "w") as f:
        f.write(str(os.getpid()))
    import atexit
    atexit.register(lambda: os.path.exists(lock) and os.remove(lock))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=0, help="only the first N tickers (smoke test)")
    a = ap.parse_args()
    os.makedirs(CACHE, exist_ok=True)
    acquire_lock()

    print(f"UA: {UA}", file=sys.stderr)
    uni = sec_universe()
    print(f"SEC universe (NYSE/NASDAQ/CBOE): {len(uni)}", file=sys.stderr)
    wd = wikidata_inceptions()
    print(f"Wikidata tickers with inception: {len(wd)}", file=sys.stderr)

    tickers = sorted(uni)
    if a.limit:
        tickers = tickers[: a.limit]

    path = os.path.join(OUT, "derived.csv")
    n_birth = n_conc = 0
    with open(path, "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh)
        w.writerow(["ticker", "name", "exchange", "cik",
                    "conception_date", "conception_prec", "conception_source", "conception_note",
                    "birth_date", "birth_prec", "birth_source", "birth_note"])
        for n, t in enumerate(tickers, 1):
            u = uni[t]
            cands = wd.get(t, [])
            c_date = c_prec = c_src = ""
            c_note = "no wikidata match"
            if cands:
                # One entity, one statement -> take it. Anything else is a conflict a human owns.
                ents = {c["entity"] for c in cands}
                if len(cands) == 1:
                    c = cands[0]
                    c_date, c_prec, c_src, c_note = c["date"], c["prec"], "wikidata-p571", c["entity"]
                else:
                    c_note = ("CONFLICT: " +
                              "; ".join(f"{c['date']}/{c['prec']}/{c['rank']}/{c['label']}" for c in cands[:4]))
                    c_src = "needs-review"
            b_date, b_src, b_note = derive_birth(u["cik"])
            if c_date:
                n_conc += 1
            if b_date:
                n_birth += 1
            w.writerow([t, u["name"], u["exchange"], u["cik"],
                        c_date, c_prec, c_src, c_note,
                        b_date or "", "day" if b_date else "", b_src, b_note])
            if n % 100 == 0:
                fh.flush()
                print(f"  {n}/{len(tickers)}  conception={n_conc}  birth={n_birth}", file=sys.stderr)

    # Verify the ARTEFACT, not the pipeline. This repo has been burned twice by trusting a
    # transformation that reported success while the file it wrote was wrong — the SEC enrichment
    # that never reached the CSV, and the interleaved rows above. Read it back and count fields.
    with open(path, newline="", encoding="utf-8") as fh:
        got = list(csv.reader(fh))
    body = got[1:]
    if len(body) != len(tickers):
        sys.exit(f"CORRUPT: wrote {len(tickers)} rows, read back {len(body)}")
    bad = [n for n, r in enumerate(body, 2) if len(r) != 12]
    if bad:
        sys.exit(f"CORRUPT: {len(bad)} rows have the wrong field count (first at line {bad[0]})")

    print(f"\nwrote {path}  [verified: {len(body)} rows x 12 fields]", file=sys.stderr)
    print(f"  conception : {n_conc}", file=sys.stderr)
    print(f"  birth      : {n_birth}", file=sys.stderr)


if __name__ == "__main__":
    main()
