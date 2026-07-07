-- Real, public IPO "birth moments." Dates are public record; times use the market-open
-- convention (09:30 exchange-local) except where a real time is known or genuinely unknown.
-- Exchange coordinates: NYSE 40.7069,-74.0089 (11 Wall St); NASDAQ 40.7589,-73.9851 (NYC).
-- Edge cases are intentional (see notes): unknown time, oldest listing, most recent IPO.

INSERT INTO company_metadata
    (ticker, company_name, ipo_date, ipo_time, exchange, tz, latitude, longitude, founding_date, data_source, notes)
VALUES
    ('MSFT',  'Microsoft Corporation',   '1986-03-13', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '1975-04-04', 'manual', 'Listed on NASDAQ at market open'),
    ('GOOGL', 'Alphabet Inc.',           '2004-08-19', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '1998-09-04', 'manual', 'Dutch-auction IPO, opened at $100.01'),
    ('AMZN',  'Amazon.com Inc.',         '1997-05-15', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '1994-07-05', 'manual', 'IPO price $18'),
    ('NVDA',  'NVIDIA Corporation',      '1999-01-22', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '1993-04-05', 'manual', 'IPO price $12, raised $42M'),
    ('META',  'Meta Platforms Inc.',     '2012-05-18', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '2004-02-04', 'manual', 'Largest tech IPO at the time'),
    ('TSLA',  'Tesla Inc.',              '2010-06-29', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '2003-07-01', 'manual', 'IPO price $17'),
    ('JPM',   'JPMorgan Chase & Co.',    '2000-12-31', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '1799-09-01', 'manual', 'Entity formed by Chase + J.P. Morgan merger Dec 2000'),
    ('V',     'Visa Inc.',               '2008-03-19', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '1958-09-18', 'manual', 'Largest US IPO at the time'),
    ('UNH',   'UnitedHealth Group Inc.', '1984-10-17', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '1977-01-01', 'manual', 'Formerly United HealthCare Corporation'),
    ('AAPL',  'Apple Inc.',              '1980-12-12', '09:30', 'NASDAQ', 'America/New_York', 40.7589, -73.9851, '1976-04-01', 'manual', 'The PRD running example'),
    ('AMD',   'Advanced Micro Devices',  '1979-09-27', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '1969-05-01', 'manual', 'Long-time NYSE listing'),
    ('DIS',   'The Walt Disney Company', '1957-11-12', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '1923-10-16', 'manual', 'Mid-century listing (edge: pre-1970 date)'),
    ('RDDT',  'Reddit, Inc.',            '2024-03-21', '09:30', 'NYSE',   'America/New_York', 40.7069, -74.0089, '2005-06-23', 'manual', 'Recent IPO (edge: very recent date)'),
    ('KO',    'The Coca-Cola Company',   '1919-09-05', NULL,    'NYSE',   'America/New_York', 40.7069, -74.0089, '1892-01-29', 'manual', 'Oldest listing; first-trade TIME UNKNOWN (edge: houses approximate)')
ON CONFLICT (ticker) DO NOTHING;
