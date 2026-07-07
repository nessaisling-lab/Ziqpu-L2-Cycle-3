-- Ziqpu contained schema — entity ("choice") charts + reading cache.
--
-- A choice's "birth moment" is its IPO: local date + time at the listing exchange.
-- Times use the market-open convention (09:30 at the exchange's local time) UNLESS a
-- real first-trade time is known. A NULL `ipo_time` means the time is genuinely UNKNOWN;
-- the engine then flags houses/Ascendant as approximate rather than fabricating a time.
-- Local time is converted to UTC at compute time using `tz` (DST-aware), not stored pre-converted.

CREATE TABLE IF NOT EXISTS company_metadata (
    ticker        TEXT PRIMARY KEY,
    company_name  TEXT NOT NULL,
    ipo_date      DATE,                                  -- nullable: unknown listing date
    ipo_time      TIME,                                  -- nullable: unknown time (flagged, not faked)
    exchange      TEXT NOT NULL DEFAULT 'NYSE',
    tz            TEXT NOT NULL DEFAULT 'America/New_York', -- IANA zone of the exchange
    latitude      DOUBLE PRECISION NOT NULL,
    longitude     DOUBLE PRECISION NOT NULL,
    founding_date DATE,
    data_source   TEXT NOT NULL DEFAULT 'manual',
    notes         TEXT
);

-- Optional cache of computed natal positions (the sidecar can also compute live via the
-- ephemeris backend). Bodies: Sun..Pluto, MeanNode, TrueNode, Chiron, Asc, MC.
CREATE TABLE IF NOT EXISTS natal_positions (
    ticker     TEXT NOT NULL REFERENCES company_metadata(ticker) ON DELETE CASCADE,
    body       TEXT NOT NULL,
    longitude  DOUBLE PRECISION NOT NULL,   -- ecliptic longitude 0..360
    sign       TEXT NOT NULL,               -- 'Aries'..'Pisces'
    degree     DOUBLE PRECISION NOT NULL,   -- 0..30 within the sign
    retrograde BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (ticker, body)
);
CREATE INDEX IF NOT EXISTS natal_positions_ticker_idx ON natal_positions (ticker);

-- Ziqpu-specific: readings are keyed by (user chart, choice), not by (ticker, date).
CREATE TABLE IF NOT EXISTS synastry_readings (
    user_chart_hash TEXT NOT NULL,          -- stable hash of the user's birth inputs
    choice_ticker   TEXT NOT NULL REFERENCES company_metadata(ticker) ON DELETE CASCADE,
    score           DOUBLE PRECISION NOT NULL,          -- 0..100
    dominant_theme  TEXT NOT NULL,
    confidence      DOUBLE PRECISION NOT NULL DEFAULT 50.0,
    reading         JSONB NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_chart_hash, choice_ticker)
);
CREATE INDEX IF NOT EXISTS synastry_readings_theme_idx ON synastry_readings (dominant_theme);
