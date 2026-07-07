-- Ziqpu company_metadata seed — GENERATED, do not hand-edit. See db/README.md to regenerate.
-- 5,271 US-market tickers (NYSE / NASDAQ / NYSE American / CBOE). IPO dates outside
-- [1792, today] were gated to NULL (unknown birth moment, flagged not fabricated).

--
-- PostgreSQL database dump
--

\restrict xJJerdLIfcHqggtu7OImMYAji3fq07KfbwUrD1idNA4JIG2D2SwYhvcgjauuOZM

-- Dumped from database version 16.14
-- Dumped by pg_dump version 16.14

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: company_metadata; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.company_metadata (ticker, company_name, ipo_date, ipo_time, exchange, tz, latitude, longitude, founding_date, data_source, notes) FROM stdin;
A	Agilent Technologies Inc.	1999-08-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AA	Alcoa Corporation	2016-10-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AACB	Artius II Acquisition Inc. Class A Ordinary Shares	2024-11-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AACI	Armada Acquisition Corp. III Class A Ordinary Share	2025-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AACO	Abony Acquisition Corp. I Class A Ordinary Share	2025-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAL	American Airlines Group Inc.	1994-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAME	Atlantic American Corp	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAMI	Acadian Asset Management Inc.	2020-03-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AAOI	Applied Optoelectronics, Inc.	2013-08-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAON	Aaon Inc	1996-03-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAP	ADVANCE AUTO PARTS INC	2002-02-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AAPL	Apple Inc.	1980-12-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	1976-04-01	manual	First day of trading on NASDAQ; opened at $22/share
AARD	Aardvark Therapeutics, Inc. Common Stock	2025-01-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AAT	AMERICAN ASSETS TRUST, INC.	2011-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AAUC	Allied Gold Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AB	AllianceBernstein Holding, L.P.	1994-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABAT	American Battery Technology Company Common Stock	2013-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABBV	ABBVIE INC.	2012-12-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABCB	Ameris Bancorp	1996-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABCL	AbCellera Biologics Inc. Common Shares	2020-11-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABEO	Abeona Therapeutics Inc. Common Stock	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABG	Asbury Automotive Group, Inc.	2001-07-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABLV	Able View Global Inc. Class B Ordinary Shares	2023-11-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABM	ABM Industries, Inc.	1995-01-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABNB	Airbnb, Inc. Class A Common Stock	2020-11-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABOS	Acumen Pharmaceuticals, Inc. Common Stock	2021-06-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABR	Arbor Realty Trust, Inc.	2005-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABSI	Absci Corporation Common Stock	2021-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABT	Abbott Laboratories	1994-03-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ABTC	American Bitcoin Corp. Class A Common Stock	2019-09-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABTS	Abits Group Inc Ordinary Shares	2022-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABUS	Arbutus Biopharma Corporation Common Stock	2011-06-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABVC	ABVC BioPharma, Inc. Common Stock	2002-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABVE	Above Food Ingredients Inc. Common Stock	2024-07-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ABX	Abacus Global Management, Inc.	2020-07-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACA	Arcosa, Inc. Common Stock	2019-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACAA	Averin Capital Acquisition Corp. Class A Ordinary Shares	2026-01-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACAD	Acadia Pharmaceuticals Inc.	2000-12-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACB	Aurora Cannabis Inc. Common Shares	2013-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACCL	Acco Group Holdings Limited Class A Ordinary Shares	2025-07-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACCO	Acco Brands Corporation	2006-03-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACCS	ACCESS Newswire Inc.	1996-03-29	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACDC	ProFrac Holding Corp. Class A Common Stock	2021-11-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACEL	Accel Entertainment, Inc.	2017-06-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACET	Adicet Bio, Inc. Common Stock	2017-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACFN	Acorn Energy, Inc. Common Stock	1997-04-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACGL	Arch Capital Group Ltd	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACGLN	Arch Capital Group Ltd. Depositary Shares, each Representing a 1/1,000th Interest in a 4.550% Non-Cumulative Preferred Share, Series G	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACH	Accendra Health, Inc.	1994-03-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACHC	Acadia Healthcare Company, Inc.	2011-11-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACHR	Archer Aviation Inc.	2020-10-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACHV	Achieve Life Sciences, Inc.	1997-03-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACI	Albertsons Companies, Inc.	1939-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACIC	American Coastal Insurance Corporation Common Stock	2007-06-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACIU	AC Immune SA	2016-05-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACIW	ACI Worldwide, Inc.	1996-12-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACLS	Axcelis Technologies Inc	2000-05-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACLX	Arcellx, Inc. Common Stock	2022-01-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACM	Aecom	1990-04-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACMR	ACM Research, Inc. Class A Common Stock	2017-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACN	Accenture PLC	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACNB	ACNB Corp	1995-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACNT	Ascent Industries Co. Common Stock	1996-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACOG	Alpha Cognition Inc. Common Stock	2024-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACON	Aclarion, Inc. Common Stock	2022-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACR	ACRES Commercial Realty Corp.	2006-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACRE	Ares Commercial Real Estate Corporation	2013-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACRS	Aclaris Therapeutics, Inc.	2015-08-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACRV	Acrivon Therapeutics, Inc. Common Stock	2022-10-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACT	Enact Holdings, Inc. Common Stock	1984-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACTG	Acacia Research Corporation	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACTU	Actuate Therapeutics, Inc. Common stock	2024-05-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ACU	Acme United Corporation	1996-03-29	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACVA	ACV Auctions Inc.	2021-02-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ACXP	Acurx Pharmaceuticals, Inc. Common Stock	2021-05-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AD	Array Digital Infrastructure, Inc.	1994-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADAC	American Drive Acquisition Company Class A Ordinary Shares	2025-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAM	Adamas Trust, Inc. Common Stock	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAMG	Adamas Trust, Inc. 9.125% Senior Notes Due 2030	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAMH	Adamas Trust, Inc. 9.875% Senior Notes Due 2030	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAMI	Adamas Trust, Inc. 9.125% Senior Notes Due 2029	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAML	Adamas Trust, Inc. 6.875% Series F Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock, $0.01 par value per share	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAMO	Adamas Trust, Inc. 9.250% Senior Notes Due 2031	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADAMZ	Adamas Trust, Inc. 7.000% Series G Cumulative Redeemable Preferred Stock, $0.01 par value per share	2005-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADBE	Adobe Inc.	1995-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADC	Agree Realty Corporation	1997-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADCT	ADC Therapeutics SA	2019-09-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADEA	Adeia Inc. Common Stock	2021-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADGM	Adagio Medical Holdings, Inc Common Stock	2024-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADI	Analog Devices, Inc.	1995-01-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADIL	Adial Pharmaceuticals, Inc	2017-09-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADM	Archer Daniels Midland Company	1902-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADMA	ADMA Biologics, Inc.	2009-09-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADNT	Adient plc Ordinary Shares	2016-10-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADP	Automatic Data Processing	1994-09-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADPT	Adaptive Biotechnologies Corporation Common Stock	2019-05-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADSE	ADS-TEC ENERGY PLC Ordinary Shares	2021-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADSK	Autodesk Inc	1995-05-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADT	ADT Inc.	1874-08-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ADTN	ADTRAN Holdings, Inc. Common Stock	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADTX	Aditxt, Inc. Common Stock	2020-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADUR	Aduro Clean Technologies Inc. Common Stock	2024-07-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADUS	Addus HomeCare Corp.	2009-07-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADV	Advantage Solutions Inc. Class A Common Stock	2019-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ADVB	Advanced Biomed Inc. Common Stock	2023-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEAQ	Activate Energy Acquisition Corp. Class A Ordinary Share	2025-11-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEBI	Aebi Schmidt Holding AG Common Stock	2026-03-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEC	Anfield Energy Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEE	Ameren Corporation	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEG	Aegon Ltd.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEHL	Antelope Enterprise Holdings Limited Class A Ordinary Shares	2009-12-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEHR	Aehr Test Systems	1997-06-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEI	Alset Inc. Common Stock (TX)	2019-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEIS	Advanced Energy Industries Inc	1997-03-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEM	Agnico Eagle Mines Ltd.	1953-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEMD	AETHLON MEDICAL INC	2000-12-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AENT	Alliance Entertainment Holding Corporation Class A Common Stock	2020-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEO	American Eagle Outfitters	1977-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEON	AEON Biopharma, Inc.	2021-01-21	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEP	American Electric Power Company, Inc.	1927-08-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AER	Aercap Holdings N.V.	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AERT	Aeries Technology, Inc. Class A Ordinary Share	2021-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AES	AES Corporation	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AESI	Atlas Energy Solutions Inc.	2024-02-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEVA	Aeva Technologies, Inc.	2016-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AEXA	American Exceptionalism Acquisition Corp. A	2025-08-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AEYE	AudioEye, Inc. Common Stock	2011-10-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFBI	Affinity Bancshares, Inc. Common Stock (MD)	2020-09-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFCG	Advanced Flower Capital Inc. Common Stock	2022-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFG	American Financial Group, Inc.	1872-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AFJK	Aimei Health Technology Co., Ltd Ordinary Share	2023-05-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFL	Aflac Inc.	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AFRI	Forafric Global PLC Ordinary Shares	2022-07-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFRM	Affirm Holdings, Inc. Class A Common Stock	2020-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AFYA	Afya Limited Class A Common Shares	2019-06-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AG	FIRST MAJESTIC SILVER CORP	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGAE	Allied Gaming & Entertainment Inc. Common Stock	2017-09-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGBK	AGI Inc	2026-01-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGCC	Agencia Comercial Spirits Ltd Class A Ordinary Shares	2025-07-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGCO	AGCO Corporation	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGEN	Agenus Inc.	1999-11-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGH	Aureus Greenway Holdings Inc. Common Stock	2024-06-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGI	Alamos Gold Inc. Class A Common Shares	2003-06-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGIG	Abundia Global Impact Group Inc.	2003-09-10	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGIO	Agios Pharmaceuticals, Inc.	2013-06-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGL	agilon health, inc.	2021-03-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGM	Federal Agricultural Mortgage Corporation	1996-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGM.A	Federal Agricultural Mortgage Corporation Class A Voting	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGMH	AGM Group Holdings Inc. Class A Ordinary Shares	2017-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGNC	AGNC Investment Corp. Common Stock	2009-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGNCL	AGNC Investment Corp. Depositary Shares Each Representing a 1/1,000th Interest in a Share of 7.75% Series G Fixed-Rate Reset Cumulative Redeemable Preferred Stock	2009-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGNCO	AGNC Investment Corp. Depositary Shares, each representing a 1/1,000th interest in a share of 6.50% Series E Fixed-to-Floating Cumulative Redeemable Preferred Stock	2009-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGO	Assured Guaranty, LTD	2003-12-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGPU	Axe Compute Inc. Common Stock	2008-11-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGRO	ADECOAGRO S.A.	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGRZ	Agroz Inc. Ordinary Shares	2025-01-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AGX	Argan, Inc	1996-04-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AGYS	Agilysys, Inc. Common Stock (DE)	1994-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AHCO	AdaptHealth Corp. Common Stock	2017-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AHMA	Ambitions Enterprise Management Co. L.L.C Class A Ordinary Shares	2025-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AHR	American Healthcare REIT, Inc.	2016-03-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AHRT	AH Realty Trust, Inc.	2014-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AHT	Ashford Hospitality Trust, Inc.	2004-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AI	C3.ai, Inc.	2020-11-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIB	BlockchAIn Digital Infrastructure, Inc	2026-03-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIDX	20/20 Biolabs, Inc. Common Stock	2025-12-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIFF	Firefly Neuroscience, Inc. Common Stock	2009-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIFU	AIFU Inc. Class A Ordinary Share	2007-10-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIG	American International Group, Inc.	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIHS	Senmiao Technology Limited Common Stock	2017-10-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AII	American Integrity Insurance Group, Inc.	2025-04-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIIA	AI Infrastructure Acquisition Corp.	2025-08-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIIO	Robo.ai Inc. Class B Ordinary Shares	2023-05-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIM	AIM ImmunoTech Inc.	1996-07-26	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIMD	Ainos, Inc. Common Stock	1996-05-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIN	Albany International Corp Class A	1895-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIOS	AIOS Tech Inc. Class A Common Shares	2015-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIOT	PowerFleet, Inc. Common Stock	2020-04-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIP	Arteris, Inc. Common Stock	2021-10-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIR	AAR Corp.	1951-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIRE	reAlpha Tech Corp. Common Stock	2023-04-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIRG	Airgain, Inc. Common Stock	2016-07-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIRI	Air Industries Group	2006-02-09	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIRJ	AirJoule Technologies Corporation Class A Common Stock	2021-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIRO	AIRO Group Holdings, Inc. Common Stock	2025-02-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIRS	AirSculpt Technologies, Inc. Common Stock	2021-10-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIRT	Air T Inc	1996-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AISP	Airship AI Holdings, Inc. Class A Common Stock	2021-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIT	Applied Industrial Technologies, Inc.	1923-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIV	Apartment Investment and Management Company	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AIXC	AIxCrypto Holdings, Inc. Common Stock	2015-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AIZ	Assurant, Inc.	1977-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AJG	Arthur J. Gallagher & Co.	1927-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AKA	a.k.a. Brands Holding Corp.	2021-08-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AKAM	Akamai Technologies Inc	1999-08-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AKAN	Akanda Corp. Common Shares	2022-01-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AKBA	Akebia Therapeutics, Inc.	2014-02-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AKR	Acadia Realty Trust	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AKTS	Aktis Oncology, Inc. Common stock	2025-12-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALAB	Astera Labs, Inc. Common Stock	2024-02-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALB	Albemarle Corporation	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALBT	Avalon GloboCare Corp. Common Stock	2015-02-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALC	Alcon Inc. Ordinary Shares	1945-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALCO	Alico Inc	1995-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALCY	Alchemy Investments Acquisition Corp 1 Class A Ordinary Shares	2022-12-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALDF	Aldel Financial II Inc. Class A Ordinary Shares	2024-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALDX	Aldeyra Therapeutics, Inc.	2014-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALEC	Alector, Inc. Common Stock	2019-01-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALF	Centurion Acquisition Corp. Class A Ordinary Shares	2024-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALG	Alamo Group, Inc.	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALGM	Allegro MicroSystems, Inc. Common Stock	2007-08-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALGN	Align Technology Inc	2000-11-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALGS	Aligos Therapeutics, Inc. Common Stock	2020-09-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALGT	Allegiant Travel Company	2006-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALH	Alliance Laundry Holdings Inc.	2025-09-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALHC	Alignment Healthcare, Inc. Common Stock	2021-03-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALIS	Calisa Acquisition Corp Ordinary shares	2024-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALIT	Alight, Inc.	2017-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALK	Alaska Air Group, Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALKS	Alkermes Inc. plc	2012-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALKT	Alkami Technology, Inc. Common Stock	2021-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALL	The Allstate Corporation	1931-04-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALLE	Allegion Public Limited Company	2013-12-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALLO	Allogene Therapeutics, Inc. Common Stock	2018-09-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALLR	Allarity Therapeutics, Inc. Common Stock	2021-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALLT	Allot Ltd. Ordinary Shares	2006-10-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALLY	Ally Financial Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALM	Almonty Industries Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALMR	Alamar Biosciences, Inc. Common Stock	2026-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALMS	Alumis Inc. Common Stock	2024-06-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALMU	Aeluma, Inc. Common Stock	2021-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALNT	Allient Inc. Common Stock	1995-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALNY	Alnylam Pharmaceuticals, Inc.	2004-02-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALOT	AstroNova, Inc. Common Stock	1995-04-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALOV	Aldabra 4 Liquidity Opportunity Vehicle, Inc. Class A Ordinary Shares	2025-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALOY	REalloys Inc. Common Stock	2015-03-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALPS	ALPS Group Inc Ordinary Share	2025-10-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALRM	Alarm.com Holdings, Inc.	2015-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALRS	Alerus Financial Corporation Common Stock	2019-08-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALSN	ALLISON TRANSMISSION HOLDINGS, INC.	1916-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALT	Altimmune, Inc. Common Stock	2005-05-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALTG	Alta Equipment Group Inc.	2019-03-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALTI	AlTi Global, Inc. Class A Common Stock	2021-02-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALTO	Alto Ingredients, Inc. Common Stock	2000-10-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALTS	ALT5 Sigma Corporation Common Stock	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALUB	Alussa Energy Acquisition Corp. II	2025-10-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALV	Autoliv, Inc.	1953-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALVO	Alvotech Ordinary Shares	2022-06-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALX	Alexander's Inc.	1928-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ALXO	ALX Oncology Holdings Inc. Common Stock	2020-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ALZN	Alzamend Neuro, Inc. Common Stock	2021-05-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AM	Antero Midstream Corporation Common Stock	1906-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMAL	Amalgamated Financial Corp. Common Stock (DE)	2021-03-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMAT	Applied Materials Inc	1994-12-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMBA	Ambarella, Inc. Ordinary Shares	2011-06-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMBP	Ardagh Metal Packaging S.A.	2021-08-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMBQ	Ambiq Micro, Inc.	2025-07-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMC	AMC ENTERTAINMENT HOLDINGS, INC.	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMCI	AMC Robotics Corporation Common Stock	2022-11-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMCR	Amcor plc Ordinary Shares	1860-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMCX	AMC Global Media Inc. Class A Common Stock	2012-03-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMD	Advanced Micro Devices	1969-05-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AME	Ametek, Inc.	1930-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMG	Affiliated Managers Group	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMGN	Amgen Inc	1995-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMH	AMERICAN HOMES 4 RENT	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMIX	Autonomix Medical, Inc. Common Stock	2024-05-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMKR	Amkor Technology Inc	1997-10-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMLX	Amylyx Pharmaceuticals, Inc. Common Stock	2021-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMN	AMN Healthcare Services	2001-07-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMOD	Alpha Modus Holdings, Inc. Class A Common Stock	2021-08-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMP	Ameriprise Financial, Inc.	1894-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMPG	AMPLITECH GROUP INC. COM	2012-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMPH	Amphastar Pharmaceuticals, Inc.	2005-02-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMPL	Amplitude, Inc. Class A Common Stock	2021-08-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMPX	Amprius Technologies, Inc.	2022-01-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMPY	Amplify Energy Corp.	2011-11-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMR	Alpha Metallurgical Resources, Inc.	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMRC	Ameresco, Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMRX	Amneal Pharmaceuticals, Inc. Class A Common Stock	2018-03-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMRZ	Amrize Ltd	2026-02-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMS	American Shared Hospital Services	1995-10-26	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMSC	American Superconductor Corp	1997-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMSF	AMERISAFE, Inc.	1996-08-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMST	Amesite Inc.	2020-08-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMT	American Tower Corporation	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMTB	Amerant Bancorp Inc.	2018-10-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMTM	Amentum Holdings, Inc.	2024-12-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMTX	Aemetis, Inc. (DE) Common Stock	2008-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMWD	American Woodmark Corp	1995-07-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AMWL	American Well Corporation	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMZE	Amaze Holdings, Inc.	2021-11-12	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AMZN	Amazon.com Inc.	1997-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	1994-07-05	manual	IPO price $18, Bezos rang the opening bell from Seattle
AN	AutoNation, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANAB	AnaptysBio, Inc. Common Stock	2015-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANABV	AnaptysBio, Inc. Common Stock When Issued	2015-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANDE	Andersons Inc/The	1994-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANDG	Andersen Group Inc.	2025-09-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANET	Arista Networks	2004-10-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANF	Abercrombie & Fitch Co.	1892-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANGH	Anghami Inc. Ordinary Shares	2022-02-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANGI	Angi Inc. Class A Common Stock	2018-03-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANGO	AngioDynamics, Inc.	2004-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANGX	Angel Studios, Inc.	2021-11-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANIK	Anika Therapeutics Inc	1997-10-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANIP	ANI Pharmaceuticals, Inc.	2001-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANIX	Anixa Biosciences, Inc.	1996-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANNA	AleAnna, Inc. Class A Common Stock	2021-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANNX	Annexon, Inc. Common Stock	2020-07-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANPA	Rich Sparkle Holdings Limited Ordinary Shares	2025-03-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANRO	Alto Neuroscience Inc.	2024-01-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANSC	Agriculture & Natural Solutions Acquisition Corporation Class A Ordinary Shares	2021-10-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANTA	Antalpha Platform Holding Company Class A Ordinary Shares	2025-04-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANTX	AN2 Therapeutics, Inc. Common Stock	2022-03-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ANVS	Annovis Bio, Inc.	2019-07-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ANY	Sphere 3D Corp. Common Shares	2017-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AOMR	Angel Oak Mortgage REIT, Inc.	2022-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AON	Aon plc Class A	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AORT	Artivion, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AOS	A.O. Smith Corporation	1889-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AOSL	Alpha and Omega Semiconductor Limited	2010-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AOUT	American Outdoor Brands, Inc. Common Stock	2021-07-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AP	Ampco-Pittsburgh Corp.	1929-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APA	APA Corporation Common Stock	1954-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APAC	StoneBridge Acquisition II Corporation Class A Ordinary Shares	2025-05-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APAD	A Paradise Acquisition Corp. Class A Ordinary Shares	2025-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APAM	ARTISAN PARTNERS ASSET MANAGEMENT INC.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APC	ARKO Petroleum Corp. Class A Common Stock	1959-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APD	Air Products & Chemicals, Inc.	1940-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APEI	American Public Education, Inc.	2007-08-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APG	APi Group Corporation	2021-03-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APGE	Apogee Therapeutics, Inc. Common Stock	2023-06-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APH	Amphenol Corporation	1932-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APLD	Applied Digital Corporation Common Stock	2001-08-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APLE	Apple Hospitality REIT, Inc.	2009-03-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APLM	Apollomics Inc. Class A Ordinary Shares	2023-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APLS	Apellis Pharmaceuticals, Inc. Common Stock	2015-10-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APM	Aptorum Group Limited Class A Ordinary Shares	2018-09-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APO	Apollo Global Management, Inc.	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APOG	Apogee Enterprises Inc	1994-05-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APP	Applovin Corporation Class A Common Stock	2021-03-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APPF	AppFolio, Inc. Class A	2015-05-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APPN	Appian Corporation Class A Common Stock	2017-04-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APPS	Digital Turbine, Inc.	1996-02-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APRE	Aprea Therapeutics, Inc. Common stock	2019-09-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APT	Alpha Pro Tech, Ltd.	1997-04-14	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
APTV	Aptiv PLC	2009-10-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
APUS	Apimeds Pharmaceuticals US, Inc.	2024-09-25	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
APVO	Aptevo Therapeutics Inc	2017-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APWC	Asia Pacific Wire & Cable Corp	2002-07-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APXT	Apex Treasury Corporation Class A Ordinary Share	2025-08-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
APYX	Apyx Medical Corporation Common Stock	2007-03-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AQB	AquaBounty Technologies, Inc.	2017-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AQMS	Aqua Metals, Inc. Common Stock	2015-06-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AQN	Algonquin Power & Utilities Corp	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AQST	Aquestive Therapeutics, Inc. Common Stock	2007-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AR	ANTERO RESOURCES CORPORATION	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARAI	Arrive AI Inc. Common Stock	2024-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARAY	Accuray Incorporated	2006-11-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARBB	ARB IOT Group Limited Ordinary Shares	2022-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARBE	Arbe Robotics Ltd. Ordinary Shares	2021-09-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARCB	ArcBest Corporation	1994-03-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARCC	Ares Capital Corporation	2005-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARCI	Archimedes Tech SPAC Partners III Co. Ordinary Share	2025-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARCO	ARCOS DORADOS HOLDINGS INC.	2007-08-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARCT	Arcturus Therapeutics Holdings Inc. Common Stock	2020-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARDT	Ardent Health, Inc.	2018-12-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARDX	Ardelyx, Inc.	2014-05-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARE	Alexandria Real Estate Equities, Inc.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AREB	American Rebel Holdings, Inc. Common Stock	2015-08-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AREC	AMERICAN RESOURCES CORP	2013-11-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AREN	The Arena Group Holdings, Inc.	1996-07-30	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARES	Ares Management Corporation Class A Common Stock	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARHS	Arhaus, Inc. Class A Common Stock	2021-10-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARI	APOLLO COMMERCIAL REAL ESTATE FINANCE, INC.	2010-03-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARIS	Aris Mining Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARKO	ARKO Corp. Common Stock	2021-01-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARKR	Ark Restaurants Corp	1995-12-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARL	American Realty Investors, Inc.	2002-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARLO	Arlo Technologies, Inc.	2018-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARLP	Alliance Resource Partners LP	1999-05-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARMK	ARAMARK	1959-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARMP	Armata Pharmaceuticals, Inc. Common Stock	1997-03-17	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AROC	Archrock Inc	2008-02-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AROW	Arrow Financial Corp	1996-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARQ	Arq, Inc. Common Stock	2016-02-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARQQ	Arqit Quantum Inc. Ordinary Shares	2021-09-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARQT	Arcutis Biotherapeutics, Inc. Common Stock	2020-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARR	ARMOUR Residential REIT, Inc.	2010-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARRY	Array Technologies, Inc. Common Stock	2020-09-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARTC	Art Technology Acquisition Corp. Class A Ordinary Shares	2025-12-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARTL	Artelo Biosciences, Inc. Common Stock	2014-10-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARTNA	Artesian Resources Corp	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARTV	Artiva Biotherapeutics, Inc. Common Stock	2021-04-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARTW	Arts-Way Manufacturing Co Inc	1997-05-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARVN	Arvinas, Inc	2018-08-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARW	Arrow Electronics, Inc.	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARWR	Arrowhead Research Corporation	2006-12-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ARX	Accelerant Holdings	1937-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ARXS	Arxis, Inc. Class A Common Stock	2026-03-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AS	Amer Sports, Inc.	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASAN	Asana, Inc. Class A Common Stock	2008-12-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASB	Associated Banc-Corp	1861-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASBP	Aspire Biopharma Holdings, Inc. Common Stock	2021-12-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASC	ARDMORE SHIPPING CORPORATION	2013-06-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASGN	ASGN Incorporated	1997-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASH	Ashland Inc.	1924-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASIC	Ategrity Specialty Insurance Company Holdings	2025-03-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASIX	AdvanSix Inc.	2017-03-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASLE	AerSale Corporation Common Stock	2018-11-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASM	Avino Silver & Gold Mines Ltd. (Canada)	2005-07-12	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASMB	Assembly Biosciences, Inc	2010-07-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASO	Academy Sports and Outdoors, Inc.	2020-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASPC	A SPAC III Acquisition Corp. Class A Ordinary Shares	2024-10-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASPI	ASP Isotopes Inc. Common Stock	2022-09-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASPN	Aspen Aerogels, Inc.	2011-06-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ASPS	Altisource Portfolio Solutions S.A.	2010-03-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASRT	Assertio Holdings, Inc. Common Stock	2021-03-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASRV	AmeriServ Financial Inc	1994-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASST	Strive, Inc. Class A Common Stock	2022-09-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTC	Astrotech Corporation (DE) Common Stock	1996-09-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTE	Astec Industries Inc	1995-03-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTH	Astrana Health Inc. Common Stock	2009-05-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTI	Ascent Solar Technologies, Inc. Common Stock	2006-01-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTL	Algoma Steel Group Inc. Common Shares	2021-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASTS	AST SpaceMobile, Inc. Class A Common Stock	2019-08-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASUR	Asure Software, Inc	1996-11-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ASYS	Amtech Systems Inc	1996-01-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATAI	AtaiBeckley Inc. Common Stock	2026-03-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATCH	AtlasClear Holdings, Inc.	2024-04-16	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATCX	Atlas Critical Minerals Corporation Common Stock	2016-12-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATEC	Alphatec Holdings, Inc.	2006-02-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATEN	A10 NETWORKS INC	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATER	Aterian, Inc. Common Stock	2019-05-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATEX	Anterix Inc. Common Stock	2014-12-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATGL	Alpha Technology Group Limited Class A Ordinary Shares	2023-07-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATHR	Aether Holdings, Inc. Common Stock	2024-12-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATI	ATI Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATII	Archimedes Tech SPAC Partners II Co. Ordinary Shares	2024-10-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATKR	Atkore Inc.	2016-03-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATLC	Atlanticus Holdings Corporation	2010-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATLCL	Atlanticus Holdings Corporation 6.125% Senior Notes due 2026	2010-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATLCZ	Atlanticus Holdings Corporation 9.25% Senior Notes due 2029	2010-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATLN	Atlantic International Corp. Common Stock	2019-04-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATLO	AMES National Corp	2002-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATLX	Atlas Lithium Corporation Common Stock	2012-04-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATMU	Atmus Filtration Technologies Inc.	2023-02-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATNI	ATN International, Inc	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATNM	Actinium Pharmaceuticals, Inc	2009-02-20	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATO	Atmos Energy Corporation	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATOM	Atomera Incorporated Common Stock	2016-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATON	AlphaTON Capital Corp. Common Stock	2000-08-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATOS	Atossa Therapeutics, Inc. Common Stock	2010-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATPC	Agape ATP Corporation Common Stock	2017-08-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATR	AptarGroup, Inc.	1992-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATRA	Atara Biotherapeutics, Inc	2014-06-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATRC	AtriCure, Inc.	2005-04-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATRO	Astronics Corp	1997-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATS	ATS Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ATXG	Addentax Group Corp. Common Stock	2015-08-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ATYR	aTyr Pharma, Inc. Common Stock	2015-04-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AU	AngloGold Ashanti plc	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AUB	Atlantic Union Bankshares Corporation	1902-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AUBN	Auburn National Bancorporation	2000-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUDC	AudioCodes Ltd	2002-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUGO	Aura Minerals Inc. Common Shares	2025-06-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUID	authID Inc. Common Stock	2012-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUNA	Auna S.A.	2020-09-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AUPH	Aurinia Pharmaceuticals Inc	2021-02-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUR	Aurora Innovation, Inc. Class A Common Stock	2021-02-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AURA	Aura Biosciences, Inc. Common Stock	2021-10-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AURE	Aurelion Inc. Class A Ordinary Shares	2022-10-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AUST	Austin Gold Corp.	2021-10-21	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AUUD	Auddia Inc. Common Stock	2020-01-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVA	Avista Corporation	1889-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVAH	Aveanna Healthcare Holdings Inc. Common Stock	2021-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVAV	AeroVironment, Inc.	2006-09-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVB	AvalonBay Communities, Inc.	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVBC	Avidia Bancorp, Inc.	2025-03-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVBH	Avidbank Holdings, Inc. Common stock	2025-07-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVBP	ArriVent BioPharma, Inc. Common Stock	2024-01-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVD	American Vanguard Corporation	1996-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVEX	AEVEX Corp.	2026-03-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVGO	Broadcom Inc. Common Stock	2018-12-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVIR	Atea Pharmaceuticals, Inc. Common Stock	2020-10-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVNS	Avanos Medical, Inc.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVNT	Avient Corporation	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVNW	Aviat Networks, Inc.	2007-01-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVO	Mission Produce, Inc. Common Stock	2020-09-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVPT	AvePoint, Inc. Class A Common Stock	2019-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVR	Anteris Technologies Global Corp. Common Stock	2024-11-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVT	Avnet, Inc.	1921-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVTR	Avantor, Inc.	2019-02-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AVTX	Avalo Therapeutics, Inc. Common Stock	2015-06-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVX	Avax One Technology Ltd. Common Shares	2020-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVXL	Anavex Life Sciences	2005-01-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AVY	Avery Dennison Corp.	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AWI	Armstrong World Industries, Inc.	1891-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AWK	American Water Works Company, Inc	1914-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AWR	American States Water Company	1929-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AWRE	Aware Inc	1996-06-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AWX	Avalon Holdings Corp.	1999-03-23	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AX	Axos Financial, Inc. Common Stock	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXG	Solowin Holdings Class A Ordinary Share	2023-04-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AXGN	Axogen, Inc. Common Stock	1995-09-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AXIL	AXIL Brands, Inc.	2017-10-06	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXIN	Axiom Intelligence Acquisition Corp 1 Class A Ordinary Shares	2025-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AXON	Axon Enterprise, Inc. Common Stock	2001-02-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AXP	American Express Company	1850-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXR	AMREP Corporation	1961-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXS	Axis Capital Holders Limited	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXSM	Axsome Therapeutics, Inc	2015-10-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AXTA	Axalta Coating Systems Ltd.	1866-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AXTI	AXT Inc	1998-03-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AYI	Acuity Inc.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AYTU	AYTU BioPharma, Inc. Common Stock	2007-01-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AZ	A2Z Cust2Mate Solutions Corp. Common Shares	2023-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AZI	Autozi Internet Technology (Global) Ltd. Class A Ordinary Shares	2023-07-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AZN	AstraZeneca PLC	1913-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AZO	AutoZone, Inc.	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
AZTA	Azenta, Inc.	1996-12-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
AZTR	Azitra Inc	2023-02-21	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
AZZ	AZZ Inc.	1996-05-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
B	Barrick Mining Corporation	1896-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BA	Boeing Company	1916-07-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BAC	Bank of America Corporation	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BACC	Blue Acquisition Corp. Class A Ordinary Shares	2025-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAER	Bridger Aerospace Group Holdings, Inc. Common Stock	2023-01-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAFN	BayFirst Financial Corp. Common Stock	2021-05-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAH	Booz Allen Hamilton Holding Corporation	1914-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BALL	Ball Corporation	1880-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BALY	Bally's Corporation	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BAM	Brookfield Asset Management Ltd.	1899-04-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BANC	Banc of California, Inc.	2002-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BAND	Bandwidth Inc. Class A Common Stock	2017-10-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BANF	Bancfirst Corp	1996-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BANL	CBL International Limited Class B Ordinary Shares	2022-08-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BANR	Banner Corp.	1996-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BANX	ArrowMark Financial Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAOS	Baosheng Media Group Holdings Limited Ordinary shares	2020-07-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAP	Credicorp LTD	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BARK	BARK, Inc.	2020-09-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BATL	Battalion Oil Corporation	2004-03-12	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BATRA	Atlanta Braves Holdings, Inc. Series A Common Stock	2023-09-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BATRK	Atlanta Braves Holdings, Inc. Series C Common Stock	2023-09-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BAX	Baxter International Inc.	1931-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BAYA	Bayview Acquisition Corp Class A Ordinary Shares	2023-11-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BB	BlackBerry Limited	1984-03-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBAI	BigBear.ai Holdings, Inc.	2021-01-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBBY	Bed Bath & Beyond, Inc.	2002-03-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBCP	Concrete Pumping Holdings, Inc. Common Stock	2017-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBCQ	Bleichroeder Acquisition Corp. II Class A Ordinary Shares	2025-10-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBDC	Barings BDC, Inc.	2007-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBGI	Beasley Broadcasting Group Inc	1999-11-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBIO	BridgeBio Pharma, Inc. Common Stock	2019-05-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBLG	Bone Biologics Corp Common Stock	2009-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBNX	Beta Bionics, Inc. Common Stock	2025-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBOT	BridgeBio Oncology Therapeutics, Inc. Common Stock	2024-01-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBSI	Barrett Business Services	1994-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BBT	Beacon Financial Corporation	1872-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBUC	Brookfield Business Corporation Class A Subordinate Voting Shares	2015-10-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBW	Build-A-Bear Workshop, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBWI	Bath & Body Works, Inc.	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BBY	Best Buy Company, Inc.	1966-08-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BC	Brunswick Corporation	1845-09-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCAB	BioAtla, Inc. Common Stock	2020-11-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCAL	California BanCorp Common Stock	2024-03-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCAR	D. Boral ARC Acquisition I Corp. Class A Ordinary Shares	2025-04-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCAX	Bicara Therapeutics Inc. Common Stock	2024-08-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCBP	BCB Bancorp Inc (NJ)	2004-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCC	Boise Cascade Company	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCDA	BioCardia, Inc. Common Stock	1996-08-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCE	BCE, Inc.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCG	Binah Capital Group, Inc. Common Stock	2024-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCHT	Birchtech Corp.	2009-04-13	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCIC	BCP Investment Corporation Common Stock	2007-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCML	BayCom Corp Common Stock	2018-04-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCO	The Brink's Company	1859-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCPC	Balchem Corporation	1999-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCRX	BioCryst Pharmaceuticals Inc	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCSF	Bain Capital Specialty Finance, Inc.	2017-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCSS	Bain Capital GSS Investment Corp.	2025-09-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BCTX	Briacell Therapeutics Corp. Common Shares	2019-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BCTXL	BriaCell Therapeutics Corp. Warrant expiring 2031	2019-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BDC	Belden Inc.	1902-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BDCI	BTC Development Corp. Class A Ordinary Shares	2025-08-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BDL	Flanigan's Enterprises Inc	1999-01-07	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BDMD	Baird Medical Investment Holdings Ltd Ordinary Share	2024-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BDN	Brandywine Realty Trust	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BDSX	Biodesix, Inc. Common Stock	2020-10-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BDTX	Black Diamond Therapeutics, Inc. Common Stock	2020-01-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BDX	Becton, Dickinson and Co.	1897-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BE	Bloom Energy Corporation	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BEAG	Bold Eagle Acquisition Corp. Class A Ordinary Shares	2021-06-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEAM	Beam Therapeutics Inc. Common Stock	2019-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEAT	Heartbeam, Inc. Common Stock	2021-09-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEBE	TGE Value Creative Solutions Corp	2025-08-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BEEM	Beam Global Common Stock	2007-11-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEEP	Mobile Infrastructure Corporation Common Stock	2021-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BELFA	Bel Fuse Inc	1995-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BELFB	Bel Fuse Inc	1995-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEN	Franklin Resources, Inc.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BENF	Beneficient Class A Common Stock	2023-07-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BEP	Brookfield Renewable Partners L.P.	2013-06-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BEPC	Brookfield Renewable Corporation Class A Exchangeable Subordinate Voting Shares	2019-11-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BESS	Bimergen Energy Corporation	2009-04-15	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BETA	Beta Technologies, Inc.	2025-09-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BETR	Better Home & Finance Holding Company Class A Common Stock	2021-02-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BF.A	Brown-Forman Corporation Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BF.B	Brown-Forman Corporation Class B	1870-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BFAM	BRIGHT HORIZONS FAMILY SOLUTIONS INC.	2012-10-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BFC	Bank First Corporation Common Stock	2019-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BFH	Bread Financial Holdings, Inc.	2000-01-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BFLY	Butterfly Network, Inc.	2020-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BFRG	Bullfrog AI Holdings, Inc. Common Stock	2022-10-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BFRI	Biofrontera Inc. Common Stock	2021-07-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BFS	Saul Centers, Inc.	1997-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BFST	Business First Bancshares, Inc. Common Stock	2015-05-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BG	Bunge Global SA	1818-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BGC	BGC Group, Inc. Class A Common Stock	1999-09-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGI	Birks Group Inc	2006-07-19	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BGIN	Bgin Blockchain Limited Class A Ordinary Shares	2025-02-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGL	Blue Gold Limited Class A Ordinary Shares	2025-07-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGLC	BioNexus Gene Lab Corp Common stock	2019-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGM	BGM Group Ltd. Class A Ordinary Shares	2019-11-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGMS	Bio Green Med Solution, Inc. Common Stock	2000-12-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BGS	B&G Foods, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BGSF	BGSF, Inc.	2013-10-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BGSI	Boyd Group Services Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BH	Biglari Holdings Inc. Class B Common Stock	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BH.A	Biglari Holdings Inc. Class A Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHAV	BHAV Acquisition Corp Class A Ordinary Shares	2026-02-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHB	Bar Harbor Bankshares	1996-03-28	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHC	Bausch Health Companies Inc	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHE	Benchmark Electronics	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHF	Brighthouse Financial, Inc.	2018-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHFAL	Brighthouse Financial, Inc. 6.25% Junior Subordinated Debentures due 2058	2018-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHFAM	Brighthouse Financial, Inc. Depositary shares each representing a 1/1,000th Interest in a Share of 4.625% Non-Cumulative Preferred Stock, Series D	2018-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHFAN	Brighthouse Financial, Inc. Depositary shares, each representing a 1/1,000th interest in a share of 5.375% Non-Cumulative Preferred Stock, Series C	2018-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHFAO	Brighthouse Financial, Inc. Depositary Shares 6.75% Non-Cum Pfd Series B	2018-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHM	Bluerock Homes Trust, Inc.	2023-03-22	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHR	Braemar Hotels & Resorts Inc. Common Stock	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BHRB	Burke & Herbert Financial Services Corp. Common Stock	2024-03-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHST	BioHarvest Sciences Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BHVN	Biohaven Ltd.	2022-10-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIAF	bioAffinity Technologies, Inc. Common Stock	2022-04-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIIB	Biogen Inc. Common Stock	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIII	Black Spade Acquisition III Co	2025-09-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BILL	BILL Holdings, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIO	Bio-Rad Laboratories, Inc.Class A	1952-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIO.B	Bio-Rad Laboratories, Inc. Class B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIOA	BioAge Labs, Inc. Common Stock	2024-09-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIOX	Bioceres Crop Solutions Corp. Ordinary Shares	2019-05-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIP	Brookfield Infrastructure Partners L.P. Limited Partnership Units	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIPC	Brookfield Infrastructure Corporation Class A Exchangeable Subordinate Voting Shares	2019-09-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIRD	Allbirds, Inc. Class A Common Stock	2021-08-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIRK	Birkenstock Holding plc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BIVI	BioVie, Inc. Common Stock	2013-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIXI	Bitcoin Infrastructure Acquisition Corp Ltd. Class A Ordinary Shares	2025-08-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BIYA	Baiya International Group Inc. Ordinary Shares	2023-10-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BJ	BJs Wholesale Club Holdings, Inc. Common Stock	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BJDX	Bluejay Diagnostics, Inc. Common Stock	2021-10-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BJRI	BJ's Restaurants, Inc.	2000-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BK	Bank of New York Mellon Corporation	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKD	Brookdale Senior Living, Inc.	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKE	The Buckle, Inc.	1948-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKH	Black Hills Corporation	1941-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKHA	Black Hawk Acquisition Corporation Class A Ordinary Shares	2024-02-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BKKT	Bakkt, Inc.	2018-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKNG	Booking Holdings Inc. Common Stock	1998-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BKR	Baker Hughes Company	1987-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BKSY	BlackSky Technology Inc.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKTI	BK Technologies Corporation	1996-04-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKU	Bankunited, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKV	BKV Corporation	2022-11-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BKYI	BIO-key International, Inc. Common Stock	1996-11-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BL	BlackLine, Inc. Common Stock	2016-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLBD	Blue Bird Corporation Common Stock	2013-12-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLCO	Bausch + Lomb Corporation	1853-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLD	TopBuild Corp. Common Stock	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLDP	Ballard Power Systems Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLDR	Builders FirstSource, Inc.	2005-02-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLFS	BioLife Solutions Inc.	1995-10-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLIN	Bridgeline Digital Inc.	2006-12-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLIV	BeLive Holdings Ordinary Share	2024-07-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLK	Blackrock, Inc.	1988-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLKB	Blackbaud, Inc.	2004-02-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLLN	BillionToOne, Inc. Class A Common Stock	2025-10-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLMN	Bloomin' Brands, Inc. Common Stock	2012-04-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLND	Blend Labs, Inc.	2021-06-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLNE	Beeline Holdings, Inc. Common Stock	2011-11-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLNK	Blink Charging Co. Common Stock	2008-03-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLRK	Bluerock Acquisition Corp. Class A Ordinary Shares	2025-11-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLSH	Bullish	2025-07-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLUW	Blue Water Acquisition Corp. III Class A Ordinary Shares	2025-02-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLX	Banco Latinoamericano de Comercio Exterior, S.A	2002-07-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BLZE	Backblaze, Inc. Class A Common Stock	2021-10-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BLZR	Trailblazer Acquisition Corp. Class A Ordinary Shares	2025-07-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMBL	Bumble Inc. Class A Common Stock	2021-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMEA	Biomea Fusion, Inc. Common Stock	2021-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMGL	Basel Medical Group Ltd Ordinary Shares	2024-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMHL	Bluemount Holdings Limited Class B Ordinary Shares	2025-03-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMI	Badger Meter, Inc.	1996-03-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BMM	Blue Moon Metals Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMNR	BitMine Immersion Technologies, Inc.	2021-12-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BMO	Bank of Montreal	1817-06-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BMR	Beamr Imaging Ltd. Ordinary Share	2022-02-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMRA	BIOMERICA INC	1999-09-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMRC	Bank of Marin Bancorp	2008-03-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMRN	BioMarin Pharmaceuticals Inc	1999-05-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BMY	Bristol-Myers Squibb Co.	1989-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BN	Brookfield Corporation	2002-04-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BNAI	Brand Engagement Network Inc. Common Stock	2021-02-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNBX	BNB Plus Corp. Common Stock	1983-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNC	CEA Industries Inc. Common Stock	2010-01-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNED	Barnes & Noble Education, Inc	2015-02-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BNGO	Bionano Genomics, Inc. Common Stock	2018-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNKK	Bonk, Inc. Common Stock	2020-06-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNL	Broadstone Net Lease, Inc.	2018-03-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BNRG	Brenmiller Energy Ltd Ordinary Shares	2022-04-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNS	Bank of Nova Scotia	1997-08-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BNT	Brookfield Wealth Solutions Ltd.	2021-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BNTC	Benitec Biopharma Inc. Common Stock	2020-08-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BNZI	Banzai International, Inc. Class A Common Stock	2020-12-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOBS	Bobs Discount Furniture, Inc.	2026-01-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BOC	Boston Omaha Corporation	2010-10-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BODI	The Beachbody Company, Inc. Class A Common Stock	2020-10-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOF	BranchOut Food Inc. Common Stock	2023-04-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOH	Bank of Hawaii Corp.	1897-12-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BOKF	BOK Financial Corp	1996-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOLD	Boundless Bio, Inc. Common Stock	2024-03-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOLT	Bolt Biotherapeutics, Inc. Common Stock	2021-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BON	Bon Natural Life Limited Class A Ordinary Shares	2020-12-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOOM	DMC Global Inc. Common Stock	1999-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOOT	Boot Barn Holdings, Inc.	2014-09-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BORR	Borr Drilling Limited	2019-07-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BOSC	B.O.S. Better On-Line Solutions Ltd.	2002-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOTJ	Bank of the James Financial Group, Inc	2006-09-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BOW	Bowhead Specialty Holdings Inc.	2024-04-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BOX	BOX, INC.	2009-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BOXL	Boxlight Corporation Class A Common Stock	2015-06-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPAC	Blueport Acquisition Ltd Class A Ordinary Shares	2025-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPOP	Popular Inc	1994-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPRE	Bluerock Private Real Estate Fund	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BPRN	Princeton Bancorp, Inc. Common Stock (PA)	2023-03-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPYPM	Brookfield Property Partners L.P. 6.25% Class A Cumulative Redeemable Preferred Units, Series 1	2013-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPYPN	Brookfield Property Partners L.P. 5.750% Class A Cumulative Redeemable Perpetual Preferred Units, Series 3	2013-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BPYPO	Brookfield Property Partners L.P. 6.375% Class A Cumulative Redeemable Perpetual Preferred Units, Series 2	2013-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BQ	Boqii Holding Limited	2020-09-08	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BR	Broadridge Financial Solutions Inc	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRAG	Bragg Gaming Group Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRAI	Braiin Limited Common Stock	2025-11-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRBR	BellRing Brands, Inc.	2019-09-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRBS	Blue Ridge Bankshares, Inc.	2020-04-14	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRC	Brady Corporation	1914-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRCB	Black Rock Coffee Bar, Inc. Class A Common Stock	2025-08-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRCC	BRC Inc.	2014-12-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRFH	Barfresh Food Group Inc. Common Stock	2010-08-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRIA	BrilliA Inc	2024-09-12	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRID	Bridgford Foods Corp	1996-02-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRK.A	Berkshire Hathaway Inc.	1839-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRK.B	BERKSHIRE HATHAWAY Class B	1839-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRKR	Bruker Corporation	2000-04-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRLS	Borealis Foods Inc. Class A Common Shares	2021-07-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRLT	Brilliant Earth Group, Inc. Class A Common Stock	2021-08-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRN	Barnwell Industries, Inc.	1995-12-22	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRO	Brown & Brown, Inc.	1939-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BROS	Dutch Bros Inc.	1992-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRR	ProCap Financial, Inc. Common Stock	2026-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRSL	Brightstar Lottery PLC	2015-05-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRSP	BrightSpire Capital, Inc.	2018-03-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRT	BRT Apartments Corp	1994-12-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRTX	BioRestorative Therapies, Inc. Common Stock (NV)	2012-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BRX	BRIXMOR PROPERTY GROUP INC.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BRZE	Braze, Inc. Class A Common Stock	2021-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSAA	BEST SPAC I Acquisition Corp. Class A Ordinary Shares	2025-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSBK	Bogota Financial Corp. Common Stock	2019-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSET	Bassett Furniture Industries I	1995-02-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSM	Black Stone Minerals, L.P.	1876-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BSRR	Sierra Bancorp	2002-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSVN	Bank7 Corp. Common stock	2018-08-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BSX	Boston Scientific Corp.	1984-05-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BSY	Bentley Systems, Incorporated Class B Common Stock	2002-04-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTAI	BioXcel Therapeutics, Inc. Common Stock	2018-02-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTBD	BT Brands, Inc. Common Stock	2019-08-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTBT	Bit Digital, Inc. Ordinary Shares	2017-12-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTCS	BTCS Inc. Common Stock	2008-05-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTCT	BTC Digital Ltd. Ordinary Shares	2020-06-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTDR	Bitdeer Technologies Group Class A Ordinary Shares	2021-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTE	Baytex Energy Corp.	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BTG	B2Gold Corp.	2007-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BTGO	BitGo Holdings, Inc.	2025-09-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BTM	Bitcoin Depot Inc. Class A Common Stock	1998-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTMD	Biote Corp. Class A Common Stock	2021-02-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTOC	Armlogi Holding Corp. common stock	2023-09-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTOG	Bit Origin Limited Class A Ordinary Shares	2018-08-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTQ	BTQ Technologies Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTSG	BrightSpring Health Services, Inc. Common Stock	2021-10-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTTC	Black Titan Corporation Ordinary Shares	2025-11-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BTU	Peabody Energy Corporation	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BUDA	Buda Juice, Inc.	2025-08-27	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BULL	Webull Corporation Class A Ordinary Shares	2025-04-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BUR	Burford Capital Limited	2021-03-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BURL	BURLINGTON STORES, INC.	1972-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BURU	Nuburu, Inc.	2020-08-18	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
BUSE	First Busey Corporation Class A Common Stock	1996-03-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BUUU	BUUU Group Limited Class A Ordinary Share	2025-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BV	BrightView Holdings, Inc. Common Stock	2018-05-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BVC	BitVentures Limited Ordinary Shares	2021-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BVFL	BV Financial, Inc. Common Stock	2004-09-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BVS	Bioventus Inc. Class A Common Stock	2016-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BW	Babcock & Wilcox Enterprises, Inc.	1867-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BWA	BorgWarner Inc.	1880-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BWB	Bridgewater Bancshares, Inc. Common Stock	2018-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWBBP	Bridgewater Bancshares, Inc. Depositary Shares, Each Representing a 1/100th Interest in a Share of 5.875% Non-Cumulative Perpetual Preferred Stock, Series A	2018-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWEN	Broadwind, Inc. Common Stock	2009-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWFG	Bankwell Financial Group Inc	2014-04-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWIN	The Baldwin Insurance Group, Inc. Class A Common Stock	2019-09-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWLP	BW LPG Limited	2025-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BWMN	Bowman Consulting Group Ltd. Common Stock	2021-04-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BWMX	Betterware de Mexico, S.A.P.I. de C.V.	2019-11-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BWXT	BWX Technologies, Inc.	2011-03-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BX	Blackstone Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BXC	BlueLinx Holdings Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BXMT	Blackstone Mortgage Trust, Inc. (NEW)	1999-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BXP	BXP, Inc.	1970-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BXSL	Blackstone Secured Lending Fund	2019-03-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BY	Byline Bancorp, Inc. Common Stock	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BYAH	Park Ha Biological Technology Co., Ltd. Ordinary Shares	2024-08-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BYD	Boyd Gaming Corporation	1941-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
BYFC	Broadway Financial Corp/Del	2009-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BYND	Beyond Meat, Inc. Common Stock	2018-11-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BYRN	Byrna Technologies, Inc. Common Stock	2006-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BYSI	BeyondSpring Inc. Ordinary Shares	2016-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BZAI	Blaize Holdings, Inc. Common Stock	2021-08-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BZFD	BuzzFeed, Inc. Class A Common Stock	2020-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
BZH	Beazer Homes USA, Inc. New	1996-12-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
C	Citigroup Inc.	1925-06-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAAP	Corporacion America Airports S.A.	2017-12-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAAS	China Automotive Systems, Inc. Ordinary Share	2026-04-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CABA	Cabaletta Bio, Inc. Common Stock	2019-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CABO	Cable One, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CABR	Caring Brands, Inc. Common Stock	2025-03-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAC	Camden National Corporation	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CACC	Credit Acceptance Corp	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CACI	CACI INTERNATIONAL CLA	1962-07-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CADL	Candel Therapeutics, Inc. Common Stock	2021-06-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAE	CAE INC	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAEP	Cantor Equity Partners III, Inc. Class A Ordinary Shares	2025-06-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAG	Conagra Brands, Inc.	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAH	Cardinal Health, Inc.	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAI	Caris Life Sciences, Inc. Common Stock	2025-05-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAKE	Cheesecake Factory (The)	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAL	Caleres Inc	2015-05-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CALC	CalciMedica, Inc. Common Stock	2020-09-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CALM	Cal-Maine Foods Inc	1996-10-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CALX	CALIX, INC.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CALY	Callaway Golf Company	1997-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAMP	CAMP4 Therapeutics Corporation Common Stock	2024-09-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAMT	Camtek Ltd	2001-07-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CANG	Cango Inc.	2018-06-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAPL	CrossAmerica Partners LP Common units representing limited partner interests	2012-05-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CAPN	Cayson Acquisition Corp Ordinary shares	2024-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAPR	Capricor Therapeutics Inc	2001-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAPS	Capstone Holding Corp. Common Stock	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAQ	Cambridge Acquisition Corp. Class A Ordinary Shares	2025-12-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAR	Avis Budget Group, Inc.	1996-04-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CARE	Carter Bankshares, Inc. Common Stock	2021-03-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CARG	CarGurus, Inc. Class A Common Stock	2017-09-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CARL	Carlsmed, Inc. Common Stock	2025-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CARR	Carrier Global Corporation	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CARS	Cars.com Inc. Common Stock	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CART	Maplebear Inc. Common Stock	2023-08-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CASH	Pathward Financial, Inc. Common Stock	1997-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CASS	Cass Information Systems Inc	1997-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAST	FreeCast, Inc. Class A Common Stock	2020-02-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CASY	Casey's General Stores Inc	1994-07-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAT	Caterpillar Inc.	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CATO	CATO CORP	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CATX	Perspective Therapeutics, Inc.	1996-02-14	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CATY	Cathay General Bancorp	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CAVA	CAVA Group, Inc.	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CB	Chubb Limited	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBAN	Colony Bankcorp Inc.	1998-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBAT	CBAK Energy Technology, Inc. Common Stock	2005-01-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBC	Central Bancompany, Inc. Class A Common Stock	2025-10-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBFV	CB Financial Services, Inc. (PA)	2015-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CELC	Celcuity Inc. Common Stock	2017-08-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBIO	Crescent Biopharma, Inc. Common Stock	2013-10-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBK	Commercial Bancgroup, Inc. Common Stock	2025-08-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBL	CBL & Associates Properties, Inc.	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBLL	CeriBell, Inc. Common Stock	2024-08-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBNA	Chain Bridge Bancorp, Inc.	2024-09-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBNK	Capital Bancorp, Inc.	2018-08-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBOE	Cboe Global Markets, Inc.	2010-03-11	09:30:00	CBOE/BATS	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBRE	CBRE GROUP, INC.	1906-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBRL	Cracker Barrel Old Country Store, Inc,.	1999-10-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBSH	Commerce Bancshares Inc	1996-03-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBT	Cabot Corporation	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBU	Community Financial System, Inc.	1866-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CBUS	Cibus, Inc. Class A Common Stock	2017-06-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CBZ	CBIZ, Inc.	1997-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CC	The Chemours Company	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCAP	Crescent Capital BDC, Inc. Common stock	2016-03-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCB	Coastal Financial Corporation	2018-06-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCBG	Capital City Bank Group Inc	1994-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCC	CCC Intelligent Solutions Holdings Inc. Common Stock	1942-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCCC	C4 Therapeutics, Inc.	2020-09-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCD	Calamos Dynamic Convertible & Income Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCEC	Capital Clean Energy Carriers Corp. Common Share	2007-03-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCEL	Cryo-Cell International Inc.	2008-02-11	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCEP	Coca-Cola Europacific Partners plc Ordinary Shares	2017-04-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCG	Cheche Group Inc. Class A Ordinary Shares	2023-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCHH	CCH Holdings Ltd Ordinary Shares	2025-08-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCI	Crown Castle Inc.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCII	Cohen Circle Acquisition Corp. II Class A Ordinary Shares	2025-05-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCIX	Churchill Capital Corp IX Ordinary Shares	2024-03-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCJ	Cameco Corporation	1988-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCK	Crown Holdings Inc.	1892-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCL	Carnival Corporation	1972-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCLD	CareCloud, Inc. Common Stock	2013-12-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCNE	CNB Financial Corp/PA	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCO	Clear Channel Outdoor Holdings, Inc. Common Stock	1972-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCOI	Cogent Communications Holdings, Inc.	2002-01-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCRN	Cross Country Healthcare Inc	2001-07-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCS	CENTURY COMMUNITIES, INC.	2014-05-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CCSI	Consensus Cloud Solutions, Inc. Common Stock	2022-04-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCTG	CCSC Technology International Holdings Limited Class A Ordinary Shares	2023-03-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CCXI	Churchill Capital Corp XI Class A Ordinary Shares	2025-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CD	Chaince Digital Holdings Inc. Ordinary Shares	2015-01-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDE	Coeur Mining, Inc.	1928-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CDIO	Cardio Diagnostics Holdings Inc. Common stock	2021-10-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDLX	Cardlytics, Inc. Common Stock	2018-01-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDNA	CareDx, Inc.	2007-10-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDNL	Cardinal Infrastructure Group Inc. Class A Common Stock	2025-10-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDNS	Cadence Design Systems	1994-06-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDP	COPT Defense Properties	1998-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CDRE	Cadre Holdings, Inc.	2021-07-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CDRO	Codere Online Luxembourg, S.A. Ordinary Shares	2022-02-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDT	CDT Equity Inc. Common Stock	2022-01-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDTG	CDT Environmental Technology Investment Holdings Limited ordinary shares	2021-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDW	CDW Corporation	2011-03-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDXS	Codexis, Inc.	2008-04-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CDZI	CADIZ, Inc.	1995-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CE	Celanese Corporation Common Stock	1918-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CECO	Ceco Environmental Corp	2002-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEG	Constellation Energy Corporation Common Stock	1999-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CELH	Celsius Holdings, Inc. Common Stock	2005-11-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CELU	Celularity Inc. Class A Common Stock	2019-04-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CELZ	Creative Medical Technology Holdings, Inc. Common Stock	2002-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CENN	Cenntro Inc. Common Stock	2018-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CENT	Central Garden and Pet Co	1996-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CENTA	Central Garden & Pet Company	1996-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CENX	Century Aluminum Co	1997-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEPF	Cantor Equity Partners IV, Inc. Class A Ordinary Shares	2025-07-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEPO	Cantor Equity Partners I, Inc. Class A Ordinary Shares	2024-11-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEPS	Cantor Equity Partners VI, Inc. Class A Ordinary Shares	2026-01-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEPT	Cantor Equity Partners II, Inc. Class A Ordinary Share	2025-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEPU	Central Puerto S.A. American Depositary Shares (each represents ten Common Shares)	2018-01-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CEPV	Cantor Equity Partners V, Inc. Class A Ordinary Shares	2025-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CERS	Cerus Corp	1996-09-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CERT	Certara, Inc. Common Stock	2020-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CETX	CEMTREX INC.	2010-01-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CETY	Clean Energy Technologies, Inc. Common Stock	2005-06-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CEVA	CEVA Inc.	2002-07-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CF	CF Industries Holding, Inc.	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CFBK	CF Bankshares Inc. Common Stock	1998-09-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CFFI	C&F Financial Corp	1998-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CFFN	Capitol Federal Financial, Inc.	2010-05-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CFG	Citizens Financial Group, Inc.	1871-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CFND	C1 Fund Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CFR	Cullen/Frost Bankers Inc.	1868-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CG	The Carlyle Group Inc. Common Stock	2011-09-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGABL	The Carlyle Group Inc. 4.625% Subordinated Notes due 2061	2011-09-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGAU	Centerra Gold Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CGBD	Carlyle Secured Lending, Inc. Common Stock	2014-03-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGC	Canopy Growth Corporation Common Shares	2020-06-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGCT	Cartesian Growth Corporation III Class A Ordinary Shares	2025-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGEM	Cullinan Therapeutics, Inc. Common Stock	2020-12-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGEN	Compugen Ltd	2002-04-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGNT	Cognyte Software Ltd. Ordinary Shares	2021-04-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGNX	Cognex Corp	1995-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGO	Calamos Global Total Return Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGON	CG Oncology, Inc. Common stock	2024-01-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGTL	Creative Global Technology Holdings Limited Class A Ordinary Shares	2023-07-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CGTX	Cognition Therapeutics, Inc. Common Stock	2021-07-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHAI	Core AI Holdings, Inc. Common Shares	2020-08-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHAR	Charlton Aria Acquisition Corporation Class A Ordinary Shares	2024-09-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHCI	Comstock Holding Companies, Inc. Class A	2004-08-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHCO	City Holding Co	1995-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHCT	Community Healthcare Trust Incorporated Common Stock, $0.01 par value per share	2016-02-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHD	Church & Dwight Co., Inc.	1847-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHDN	Churchill Downs Inc	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHE	Chemed Corporation	1994-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHEC	Chenghe Acquisition III Co. Class A Ordinary Share	2025-07-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHEF	The Chef's Warehouse Inc	2011-04-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHGG	CHEGG, INC.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHH	Choice Hotels Intnl.	1939-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHI	Calamos Convertible Opportunities and Income Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHKP	Check Point Software Technologies Ltd	2000-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHMG	Chemung Financial Corp	1995-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHMI	CHERRY HILL MORTGAGE INVESTMENT CORPORATION	2014-03-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHNR	China Natural Resources, Inc.	1997-04-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHOW	ChowChow Cloud International Holdings Limited	2025-04-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHPG	ChampionsGate Acquisition Corporation Class A Ordinary Share	2024-12-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHPT	ChargePoint Holdings, Inc.	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHR	Cheer Holding, Inc. Class A Ordinary Share	2018-07-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHRD	Chord Energy Corporation Common Stock	2010-03-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHRS	Coherus Oncology, Inc. Common Stock	2014-09-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHRW	C.H. Robinson Worldwide, Inc.	1997-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHSN	Chanson International Holding Class A Ordinary Shares	2021-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHTR	Charter Comm Inc Del CL A New	1999-07-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHW	Calamos Global Dynamic Income Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHWY	Chewy, Inc.	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CHY	Calamos Convertible and High Income Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CHYM	Chime Financial, Inc. Class A Common Stock	2025-05-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CI	The Cigna Group	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CIA	Citizens, Inc.	1996-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CIEN	Ciena Corporation	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CIFR	Cipher Digital Inc. Common Stock	2020-08-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CIGI	Colliers International Group Inc. Subordinate Voting Shares	2000-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CIIT	Tianci International, Inc. Common Stock	2012-09-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CIM	Chimera Investment Corp.	2008-03-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CINF	Cincinnati Financial Corp	1994-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CING	Cingulate Inc. Common Stock	2021-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CINT	CI&T Inc	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CION	CION Investment Corporation	2013-03-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CISO	CISO Global, Inc. Common Stock	2020-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CISS	C3is Inc. Common Stock	2023-04-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CITR	CitroTech Inc.	1997-03-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CIVB	Civista Bancshares, Inc.	1997-03-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CIX	Comp X International Inc.	1997-12-18	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CJMB	Callan JMB Inc. Common Stock	2024-10-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CKX	CKX Lands, Inc.	1996-03-29	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CL	Colgate-Palmolive Company	1953-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLAR	Clarus Corporation Common Stock	1998-02-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLB	Core Laboratories Inc.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLBK	Columbia Financial, Inc. Common Stock	2017-12-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLBR	Colombier Acquisition Corp. III	2025-10-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLBT	Cellebrite DI Ltd. Class A Ordinary Shares	2021-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLDI	Calidi Biotherapeutics, Inc.	2021-08-24	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLDT	CHATHAM LODGING TRUST	2011-03-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLDX	Celldex Therapeutics, Inc	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLF	Cleveland-Cliffs Inc.	1846-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLFD	Clearfield, Inc.	1999-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLGN	CollPlant Biotechnologies Ltd Ordinary Shares	2016-10-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLH	Clean Harbors, Inc	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLIK	Click Holdings Limited Class A Ordinary Share	2024-06-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLIR	ClearSign Technologies Corporation Common Stock (DE)	2011-11-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLMB	Climb Global Solutions, Inc. Common Stock	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLMT	Calumet, Inc. Common Stock	2025-03-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLNE	Clean Energy Fuels Corp.	2006-09-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLNN	Clene Inc. Common Stock	2021-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLOV	Clover Health Investments, Corp	2020-02-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLPR	Clipper Realty Inc. Common Stock	2017-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLPS	CLPS Incorporation Common Stock	2018-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLPT	ClearPoint Neuro, Inc. Common Stock	2009-12-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLRB	Cellectar Biosciences INC NEW	2004-09-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLRO	ClearOne, Inc. (DE) Common Stock	2001-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLS	Celestica, Inc.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLSK	CLEANSPARK INC	2010-02-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLST	Catalyst Bancorp, Inc. Common Stock	2021-03-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLVT	Clarivate Plc	2019-05-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLW	Clearwater Paper Corporation	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLWT	Euro Tech Holdings Co Ltd. New	1996-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CLX	Clorox Company	1913-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CLYM	Climb Bio, Inc. Common Stock	2021-07-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CM	Canadian Imperial Bank of Commerce	1961-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMBT	CMB.TECH NV	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMC	Commercial Metals Company	1915-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMCL	Caledonia Mining Corporation Plc	2003-06-16	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMCO	Columbus McKinnon Corp/NY	1997-06-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMCSA	Comcast Corp	2003-03-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMCT	Creative Media & Community Trust Corporation Common stock	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMDB	Costamare Bulkers Holdings Limited	2026-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CME	CME Group Inc.	2002-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMG	Chipotle Mexican Grill, Inc.	1993-07-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMI	Cummins Inc.	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMII	Columbus Circle Capital Corp II Class A Ordinary Shares	2026-01-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMND	Clearmind Medicine Inc. Common Shares	2022-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMP	Compass Minerals International, Inc.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMPR	Cimpress PLC Ordinary Shares (Ireland)	2005-06-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMPX	Compass Therapeutics, Inc. Common Stock	2019-07-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMRC	Commerce.com, Inc. Series 1 Common Stock	2020-07-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMRE	Costamare Inc.	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMS	CMS Energy Corporation	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMT	Core Molding Technologies, Inc.	1997-03-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMTG	Claros Mortgage Trust, Inc.	2022-03-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CMTL	Comtech Telecommunications	1995-10-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CMTV	Community Bancorp. Common Stock	1999-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNA	CNA Financial Corporation	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNC	Centene Corporation	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNCK	Coincheck Group N.V. Ordinary Shares	2024-12-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNDT	Conduent Incorporated Common Stock	2017-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNET	ZW Data Action Technologies Inc. Common Stock	2008-04-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNEY	CN Energy Group Inc. Class A Ordinary Shares	2020-07-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNH	CNH INDUSTRIAL N.V.	2014-04-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNI	Canadian National Railway	1919-06-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNK	Cinemark Holdings, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNL	Collective Mining Ltd.	1906-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNM	Core & Main, Inc.	2017-08-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNMD	CONMED Corporation	1996-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNNE	Cannae Holdings, Inc. Common Stock	2018-03-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNO	CNO Financial Group, Inc.	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNOB	Center Bancorp Inc	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNOBP	ConnectOne Bancorp, Inc. Depositary Shares, each representing a 1/40th interest in a share of 5.25% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series A	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNP	CenterPoint Energy, Inc.	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNQ	Canadian Natural Resources Limited	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNR	Core Natural Resources, Inc.	2018-02-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNS	Cohen & Steers Inc.	2004-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNSP	CNS Pharmaceuticals, Inc. Common Stock	2019-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNTB	Connect Biopharma Holdings Limited Ordinary Shares	2021-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNTN	Canton Strategic Holdings, Inc. Common Stock	2021-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNTX	Context Therapeutics Inc. Common Stock	2021-05-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNTY	Century Casinos Inc	2003-03-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNVS	Cineverse Corp. Class A Common Stock	2003-08-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNX	CNX Resources Corporation	1998-12-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CNXC	Concentrix Corporation Common Stock	2021-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CNXN	PC Connection Inc	1997-11-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COCH	Envoy Medical, Inc. Class A Common Stock	2021-02-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COCO	The Vita Coco Company, Inc. Common Stock	2021-09-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COCP	Cocrystal Pharma, Inc. Common Stock	2007-09-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CODA	Coda Octopus Group, Inc. Common stock	2007-05-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CODI	Compass Diversified	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CODX	Co-Diagnostics, Inc. Common Stock	2017-04-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COEP	Coeptis Therapeutics Holdings Inc. Common Stock	2020-09-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COF	Capital One Financial	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COFS	CHOICEONE FINANCIAL	1998-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COGT	Cogent Biosciences, Inc. Common Stock	2018-03-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COHN	Cohen & Company Inc	2005-03-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
COHR	Coherent Corp.	1995-09-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COHU	Cohu Inc	1996-03-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COIN	Coinbase Global, Inc. Class A Common Stock	2021-02-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COKE	Coca-Cola Consolidated, Inc. Common Stock	1994-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COLA	Columbus Acquisition Corp Ordinary Shares	2024-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COLB	Columbia Banking Systems Inc	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COLD	Americold Realty Trust, Inc.	2018-03-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COLL	Collegium Pharmaceutical, Inc. Common Stock	2015-04-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COLM	Columbia Sportswear Co	1997-12-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COMP	Compass, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CON	Concentra Group Holdings Parent, Inc.	2024-06-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COO	The Cooper Companies, Inc. Common Stock	1958-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COOK	Traeger, Inc.	2021-07-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COOT	Australian Oilseeds Holdings Limited Ordinary Shares	2024-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COP	ConocoPhillips	1875-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COPL	Copley Acquisition Corp	2024-12-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COR	Cencora, Inc.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CORT	Corcept Therapeutics Inc.	2001-12-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CORZ	Core Scientific, Inc. Common Stock	2021-01-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COSM	Cosmos Holdings Inc. Common Stock	2009-10-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COSO	CoastalSouth Bancshares, Inc.	2025-06-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COST	Costco Wholesale Corp	1994-11-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
COTY	COTY INC	1904-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COUR	Coursera, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
COYA	Coya Therapeutics, Inc. Common Stock	2022-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CP	Canadian Pacific Kansas City Limited	2001-10-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPA	Copa Holdings, S.A.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPAY	Corpay, Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPB	The Campbell's Company Common Stock	1869-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPBI	Central Plains Bancshares, Inc. Common Stock	2023-06-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPF	Central Pacific Financial Corporation	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPHC	Canterbury Park Holding Corporation 'New' Common Stock	2017-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPHI	China Pharma Holdings, Inc.	2005-10-20	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPIX	Cumberland Pharmaceuticals Inc	2007-05-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPK	Chesapeake Utilities	1859-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPNG	Coupang, Inc.	2010-08-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPOP	Pop Culture Group Co., Ltd Class A Ordinary Shares	2021-03-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPRI	Capri Holdings Limited	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPRT	Copart Inc	1996-10-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPRX	Catalyst Pharmaceutical  Inc.	2006-07-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPS	Cooper-Standard Automotive Inc.	1936-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPSH	CPS Technologies Corp. Common Stock	1997-03-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPSS	Consumer Portfolio Services	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CPT	Camden Property Trust	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CPZ	Calamos Long/Short Equity & Dynamic Income Trust Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CQP	Cheniere Energy Partners, LP	2006-12-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CR	Crane Company	1855-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRAC	Crown Reserve Acquisition Corp. I Class A Ordinary Shares	2025-05-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRAI	CRA International, Inc.	1998-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRAN	Crane Harbor Acquisition Corp. II Class A Ordinary Shares	2025-11-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRAQ	Cal Redwood Acquisition Corp. Class A Ordinary Shares	2025-03-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRBG	Corebridge Financial, Inc.	2022-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRBP	Corbus Pharmaceuticals Holdings, Inc.	2014-09-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRBU	Caribou Biosciences, Inc. Common Stock	2021-07-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRC	California Resources Corporation	2015-02-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRCL	Circle Internet Group, Inc.	2025-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRCT	Cricut, Inc. Class A Common Stock	2021-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRD.A	Crawford & Company Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRD.B	Crawford & Company Class B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRDF	Cardiff Oncology, Inc. Common Stock	2003-02-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRDL	Cardiol Therapeutics Inc. Class A Common Shares	2023-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRDO	Credo Technology Group Holding Ltd Ordinary Shares	2022-01-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRE	Cre8 Enterprise Limited Class A Ordinary Shares	2024-08-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CREG	Smart Powerr Corp. Common Stock	2004-11-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CREX	CREATIVE REALITIES, INC.	2006-08-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRGO	Freightos Limited Ordinary shares	2023-02-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRGY	Crescent Energy Company	2022-03-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRH	CRH Public Limited Company	1970-10-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRI	Carter's Inc.	1865-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRIS	Curis Inc	2000-11-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRK	Comstock Resources, Inc.	1996-03-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRL	Charles River Laboratories International, Inc.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRM	Salesforce, Inc.	1999-02-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRMD	CorMedix Inc.	2009-11-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRML	Critical Metals Corp. Ordinary Shares	2024-03-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRMT	America's Car Mart Inc	1995-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRNC	Cerence Inc. Common Stock	2019-10-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRNT	Ceragon Networks Ltd	2001-07-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRNX	Crinetics Pharmaceuticals, Inc.	2018-06-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRON	Cronos Group Inc. Common Share	2020-03-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CROX	Crocs, Inc.	2005-08-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRS	Carpenter Technology Corp	1889-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRSP	CRISPR Therapeutics AG	2016-09-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRSR	Corsair Gaming, Inc. Common Stock	2020-08-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRT	Cross Timbers Royalty Trust	1997-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CRUS	Cirrus Logic Inc	1995-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRVL	Corvel Corp	1996-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRVO	CervoMed Inc. Common Stock	2009-04-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRVS	Corvus Pharmaceuticals, Inc.	2016-01-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRWD	CrowdStrike Holdings, Inc. Class A Common Stock	2019-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRWS	Crown Crafts Inc	1995-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CRWV	CoreWeave, Inc. Class A Common Stock	2025-03-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSAI	Cloudastructure, Inc. Class A Common Stock	2024-09-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSBR	Champions Oncology, Inc.	2009-08-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSCO	Cisco Systems, Inc. Common Stock (DE)	1995-10-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSGP	CoStar Group Inc	1998-03-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSGS	CSG Systems International	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSHR	CoinShares PLC Ordinary Shares	2026-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSIQ	Canadian Solar Inc. Common Shares (ON)	2006-10-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSL	Carlisle Companies, Inc.	1917-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CSPI	CSP Inc.	1995-11-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSQ	Calamos Strategic Total Return Fund	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSR	Centerspace	1995-07-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CSTE	Caesarstone Ltd.	2012-02-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSTL	Castle Biosciences, Inc. Common Stock	2019-06-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSTM	Constellium SE Class A Ordinary shares	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CSV	Carriage Services, Inc.	1996-06-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CSW	CSW Industrials, Inc.	2016-06-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CSWC	Capital Southwest Corp	1996-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CSX	CSX Corporation	1980-11-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTAA	ClearThink 1 Acquisition Corp. Class A Ordinary Shares	2026-01-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTAS	Cintas Corp	1994-08-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTBI	Community Trust Bancorp Inc	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTEV	Claritev Corporation	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTGO	Contango Silver & Gold Inc.	2011-09-19	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTKB	Cytek Biosciences, Inc. Common Stock	2021-07-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTLP	Cantaloupe, Inc. Common Stock	1996-08-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTM	Castellum, Inc.	2022-09-02	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTMX	CytomX Therapeutics, Inc.	2015-08-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTNM	Contineum Therapeutics, Inc. Class A Common Stock	2024-03-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTNT	Cheetah Net Supply Chain Service Inc. Class A Common Stock	2023-04-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTO	CTO Realty Growth, Inc.	1995-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTOR	Citius Oncology, Inc. Common Stock	2021-05-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTOS	Custom Truck One Source, Inc.	2017-07-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTRA	Coterra Energy Inc.	1989-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTRE	CareTrust REIT, Inc	2015-02-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTRI	Centuri Holdings, Inc.	2024-03-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTRM	Castor Maritime Inc. Common Stock	2019-01-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTRN	Citi Trends, Inc.	2005-02-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTS	CTS Corporation	1994-03-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTSH	Cognizant Technology Solutions	1998-04-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTSO	Cytosorbents Corp.	2004-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTVA	Corteva, Inc. Common Stock	2018-03-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CTW	CTW Cayman Class A Ordinary Shares	2025-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CTXR	Citius Pharmaceuticals Inc. Common	2010-11-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CUB	Lionheart Holdings Class A Ordinary Shares	2024-05-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CUBE	CubeSmart	2004-07-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CUBI	CUSTOMERS BANCORP INC	2010-04-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CUE	Cue Biopharma, Inc.	2017-09-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CULP	Culp, Inc. Common Stock	1995-07-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CUPR	Cuprina Holdings (Cayman) Limited Class A Ordinary Shares	2024-03-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CURB	Curbline Properties Corp.	2025-02-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CURI	CuriosityStream Inc. Class A Common Stock	2019-10-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CURR	Currenc Group Inc. Ordinary Shares	2021-05-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CURV	Torrid Holdings Inc.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CURX	Curanex Pharmaceuticals Inc Common Stock	2024-10-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CUZ	Cousins Properties Inc.	1994-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CV	CapsoVision, Inc. Common Stock	2025-05-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVBF	CVB Financial Corp	1994-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVCO	Cavco Industries Inc.	1995-12-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVE	Cenovus Energy Inc.	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVEO	Civeo Corporation	2015-03-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVGI	Commercial Vehicle Group, Inc.	2004-05-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVGW	Calavo Growers Inc	2002-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVI	CVR ENERGY, INC.	2006-09-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVKD	Cadrenal Therapeutics, Inc. Common Stock	2022-09-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVLG	Covenant Logistics Group, Inc.	1997-02-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVLT	Commault Systems, Inc.	2006-03-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVM	Cel-Sci Corporation	1996-06-18	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVNA	Carvana Co.	2017-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVR	Chicago Rivet & Machine Co.	1996-03-29	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVRX	CVRx, Inc. Common Stock	2021-06-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVS	CVS HEALTH CORPORATION	1963-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVSA	Covista Inc.	1996-09-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVU	CPI Aerostructures, Inc.	2002-12-17	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
CVV	CVD Equipment Corp.	2007-07-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CVX	Chevron Corporation	1994-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CW	Curtiss-Wright Corp.	1929-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWAN	Clearwater Analytics Holdings, Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWBC	Community West Bancshares Common Stock	2001-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CWCO	Consolidated Water Co Inc	2000-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CWD	CaliberCos Inc. Class A Common Stock	2022-09-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CWEN	Clearway Energy, Inc. Class C Common Stock	2013-06-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWEN.A	Clearway Energy, Inc.  Class A Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWH	Camping World Holdings, Inc.	1966-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWK	Cushman & Wakefield Ltd.	1917-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CWST	Casella Waste Systems Inc	1997-08-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CWT	California Water Service	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CXAI	CXApp Inc. Class A Common Stock	2020-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CXDO	CREXENDO INC	1999-06-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CXM	Sprinklr, Inc.	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CXT	Crane NXT, Co.	1994-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CXW	CoreCivic, Inc.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CYAB	Cyabra, Inc. Common Stock	2026-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYCN	Cyclerion Therapeutics, Inc. Common Stock	2019-04-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYCU	Cycurion, Inc. Common Stock	2021-10-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYD	China Yuchai International Ltd.	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CYH	Community Health Systems, Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
CYN	Cyngn Inc. Common Stock	1954-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYPH	Cypherpunk Technologies Inc. Common Stock	2017-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYRX	CryoPort, Inc. Common Stock	2007-11-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CYTK	Cytokinetics Inc.	2004-01-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CZFS	Citizens Financial Services, Inc. Common Stock	1995-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CZNC	Citizens & Northern Corporation	1995-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CZR	Caesars Entertainment, Inc. Common Stock	2015-03-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
CZWI	Citizens Community Bancorp, Inc.	2006-06-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
D	Dominion Energy, Inc Common Stock	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DAAQ	Digital Asset Acquisition Corp. Class A Ordinary shares	2025-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAC	Danaos Corporation	2006-09-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DAIC	CID HoldCo, Inc. Common Stock	2025-09-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAIO	Data I/O Corp	1995-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAKT	Daktronics Inc	1997-08-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAL	Delta Air Lines, Inc.	1925-03-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DAN	Dana Incorporated	1904-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DAR	DARLING INGREDIENTS INC.	1997-03-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DARE	Dare Bioscience, Inc. Common Stock	2014-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DASH	DoorDash, Inc. Class A Common Stock	2012-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAVE	Dave Inc. Class A Common Stock	2021-01-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DAWN	Day One Biopharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DB	Deutsche Bank Aktiengesellschaft	1870-03-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DBCA	D. Boral Acquisition I Corp. Class A Ordinary Shares	2025-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DBD	Diebold Nixdorf, Incorporated	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DBGI	Digital Brands Group, Inc. Common Stock	2021-04-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DBI	Designer Brands Inc.	2005-03-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DBRG	DigitalBridge Group, Inc.	1991-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DBVT	DBV Technologies S.A.	2014-09-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DBX	Dropbox, Inc. Class A	2018-02-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DC	Dakota Gold Corp.	2022-03-28	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
DCBO	Docebo Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DCGO	DocGo Inc. Common Stock	2020-09-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DCH	Dauch Corporation	1998-05-26	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DCI	Donaldson Company, Inc.	1915-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DCO	Ducommun Incorporated	1849-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DCOM	Dime Community Bancshares, Inc.	1999-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DCOY	Decoy Therapeutics Inc. Common Stock	2014-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DCTH	Delcath Systems Inc	2000-06-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DCX	Digital Currency X Technology Inc. Class A Ordinary Shares	2024-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DD	DuPont de Nemours, Inc. Common Stock	1802-07-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DDC	DDC Enterprise Limited	2023-06-16	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
DDD	3D Systems Corporation	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DDOG	Datadog, Inc. Class A Common Stock	2019-08-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DDS	Dillards Inc.	1938-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DE	Deere & Company	1868-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DEA	Easterly Government Properties, Inc.	2015-03-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DEC	Diversified Energy Company	2024-03-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DECK	Deckers Outdoor Corp	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DEFT	Defi Technologies, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DEI	Douglas Emmett, Inc.	2007-04-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DELL	Dell Technologies Inc.	2016-09-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DERM	Journey Medical Corporation Common Stock	2021-10-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DEVS	DevvStream Corp. Common Stock	2021-04-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DFDV	DeFi Development Corp. Common Stock	2022-10-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DFH	Dream Finders Homes, Inc.	2020-12-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DFIN	Donnelley Financial Solutions, Inc.	2017-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DFLI	Dragonfly Energy Holdings Corp. Common Stock (NV)	2021-03-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DFNS	T3 Defense Inc. Common Stock	2020-03-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DFSC	DEFSEC Technologies Inc. Common Stock	2022-08-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DFTX	Definium Therapeutics, Inc. Common Shares	2022-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DG	Dollar General Corp.	1939-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DGICA	Donegal Group Inc	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DGICB	Donegal Group Inc	1996-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DGII	Digi International Inc	1995-12-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DGNX	Diginex Limited Ordinary Shares	2024-09-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DGX	Quest Diagnostics Inc.	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DGXX	Digi Power X Inc. Subordinate Voting Shares	2023-07-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DH	Definitive Healthcare Corp. Class A Common Stock	2021-08-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DHCNI	Diversified Healthcare Trust 5.625% Senior Notes due 2042	2000-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DHCNL	Diversified Healthcare Trust 6.25% Senior Notes Due 2046	2000-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DHI	D.R. Horton Inc.	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DHIL	Diamond Hill Investment Group	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DHR	Danaher Corporation	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DHT	DHT HOLDINGS, INC.	2005-09-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DHX	DHI Group, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DIBS	1stdibs.com, Inc. Common Stock	2021-05-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DIN	Dine Brands Global, Inc.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DINO	HF Sinclair Corporation	2023-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DIOD	Diodes Inc	1996-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DIS	The Walt Disney Company	1923-10-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DIT	AMCON Distributing Co.	1996-12-23	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
DJCO	Daily Journal Corp	1995-12-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DJT	Trump Media & Technology Group Corp. Common Stock	2021-05-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DK	Delek US Holdings, Inc.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DKI	DarkIris Inc. Class A Ordinary Shares	2025-06-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DKL	DELEK LOGISTICS PARTNERS, LP	2012-07-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DKNG	DraftKings Inc. Class A Common Stock	2023-02-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DKS	Dick's Sporting Goods, Inc.	1948-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DLB	Dolby Laboratories, Inc.Class A	1965-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DLHC	DLH Holdings Corp.	1996-01-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DLNG	DYNAGAS LNG PARNERS LP	2013-10-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DLO	DLocal Limited Class A Common Shares	2021-05-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DLPN	Dolphin Entertainment, Inc.	2008-02-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DLR	Digital Realty Trust, Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DLTH	Duluth Holdings Inc. Class B Common Stock	2015-10-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DLTR	Dollar Tree Inc.	1997-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DLX	Deluxe Corporation	1915-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DLXY	Delixy Holdings Limited Class A Ordinary Shares	2024-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMAA	Drugs Made In America Acquisition Corp. Ordinary Shares	2024-08-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMAC	DiaMedica Therapeutics Inc. Common Stock	2018-11-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMII	Drugs Made In America Acquisition II Corp. Ordinary Shares	2025-07-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMLP	Dorchester Minerals LP	2003-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMRA	Damora Therapeutics, Inc. Common Stock	2020-10-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DMRC	Digimarc Corporation	2009-02-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DNA	Ginkgo Bioworks Holdings, Inc.	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DNLI	Denali Therapeutics Inc. Common Stock	2017-11-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DNMX	Dynamix Corporation III Class A Ordinary Shares	2025-08-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DNN	Denison Mines Corp	1999-03-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
DNOW	DNOW Inc.	2015-02-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DNTH	Dianthus Therapeutics, Inc. Common Stock	2018-05-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DNUT	Krispy Kreme, Inc. Common Stock	2021-06-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOC	Healthpeak Properties, Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOCN	DigitalOcean Holdings, Inc.	2011-08-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOCS	Doximity, Inc.	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOCU	DocuSign, Inc. Common Stock	2018-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOGZ	Dogness (International) Corporation Class A Common Stock	2017-09-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOLE	Dole plc	1851-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOMH	Dominari Holdings Inc. Common Stock	1998-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOMO	Domo, Inc. Class B Common Stock	2018-06-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOO	BRP Inc. Common Subordinate Voting Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DORM	Dorman Products, Inc. New	1997-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DOUG	Douglas Elliman Inc.	2021-12-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOV	Dover Corporation	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOW	Dow Inc.	2019-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DOX	Amdocs Limited	1982-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DPRO	Draganfly Inc. Common Shares	2022-04-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DPZ	Domino's Pizza Inc.	1960-06-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRCT	Direct Digital Holdings, Inc. Class A Common Stock	2021-11-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRDB	Roman DBDR Acquisition Corp. II Ordinary shares	2024-09-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRH	Diamondrock Hospitality Company Common Stock	2006-03-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRI	Darden Restaurants, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DRIO	DarioHealth Corp. Common Stock	2013-01-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRMA	Dermata Therapeutics, Inc. Common Stock	2021-06-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRS	Leonardo DRS, Inc. Common Stock	2021-02-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRTS	Alpha Tau Medical Ltd. Ordinary Shares	2022-03-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRUG	Bright Minds Biosciences Inc. Common Stock	2021-12-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DRVN	Driven Brands Holdings Inc. Common Stock	2020-12-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSAC	Daedalus Special Acquisition Corp. Class A Ordinary Shares	2025-09-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSGN	Design Therapeutics, Inc. Common Stock	2021-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSGR	Distribution Solutions Group, Inc. Common Stock	1996-03-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSGX	Descartes Systems Group Inc	2005-07-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSP	Viant Technology Inc. Class A Common Stock	2021-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSS	DSS, Inc.	1998-05-06	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
DSWL	Deswell Industries Inc	2001-07-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DSX	Diana Shipping, Inc.	2005-03-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DSY	Big Tree Cloud Holdings Limited Class A Ordinary Shares	2024-06-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DT	Dynatrace, Inc.	2005-02-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DTCX	Datacentrex, Inc. Common Stock	2021-04-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DTE	DTE Energy Company	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DTI	Drilling Tools International Corporation Common Stock	2021-11-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DTIL	Precision BioSciences, Inc. Common Stock	2019-03-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DTM	DT Midstream, Inc.	2022-02-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DTSQ	DT Cloud Star Acquisition Corporation Ordinary Shares	2024-04-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DTSS	Datasea Intelligent Technology Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DTST	Data Storage Corporation Common Stock	2007-12-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DUK	Duke Energy Corporation	1904-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DUO	Fangdd Network Group Ltd. Class A Ordinary Shares	2019-10-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DUOL	Duolingo, Inc. Class A Common Stock	2021-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DUOT	Duos Technologies Group, Inc. Common Stock	2007-04-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DV	DoubleVerify Holdings, Inc.	2021-03-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DVA	DaVita Inc.	1992-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DVLT	Datavault AI Inc. Common Stock	2018-04-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DVN	Devon Energy Corporation	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DWSN	Dawson Geophysical Company New Common Stock	1996-09-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DWTX	Dogwood Therapeutics, Inc. Common Stock	2020-08-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DX	Dynex Capital, Inc.	1994-03-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DXC	DXC Technology Company	2017-04-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DXCM	DexCom, Inc.	2005-02-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DXLG	Destination XL Group, Inc. Common Stock	1995-04-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DXPE	DXP Enterprises Inc	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DXR	Daxor Corporation Common Stock	1995-04-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DXST	Decent Holding Inc Class A Ordinary Shares	2024-10-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DXYZ	Destiny Tech100 Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DY	Dycom Industries, Inc.	1995-10-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
DYAI	Dyadic International, Inc.	2003-01-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DYN	Dyne Therapeutics, Inc. Common Stock	1984-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
DYOR	Insight Digital Partners II Class A Ordinary Shares	2025-08-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EA	Electronic Arts Inc	1995-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EAF	GrafTech International Ltd.	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EARN	Ellington Credit Company	2014-03-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EAT	Brinker International, Inc.	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EBAY	eBay Inc	1998-07-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EBC	Eastern Bankshares, Inc. Common Stock	2020-06-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EBF	Ennis, Inc.	1995-05-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EBMT	Eagle Bancorp Montana, Inc	2009-12-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EBON	Ebang International Holdings Inc. Class A Ordinary Shares	2020-04-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EBS	Emergent Biosolutions, Inc.	2003-12-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ECBK	ECB Bancorp, Inc. Common Stock	2022-03-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ECG	Everus Construction Group, Inc.	2025-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ECL	Ecolab, Inc.	1923-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ECO	Okeanis Eco Tankers Corp.	2024-04-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ECOR	electroCore, Inc. Common Stock	2018-05-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ECPG	Encore Capital Group, Inc.	1999-04-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ECVT	Ecovyst Inc.	2017-06-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ECX	ECARX Holdings Inc. Class A Ordinary shares	2023-04-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ED	Consolidated Edison, Inc.	1823-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EDBL	Edible Garden AG Incorporated Common Stock	2021-11-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDHL	Everbright Digital Holding Limited Ordinary Shares	2025-02-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDIT	Editas Medicine, Inc. Common Stock	2016-01-04	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDRY	EuroDry Ltd. Common Shares	2018-05-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDSA	Edesa Biotech, Inc. Common Shares	2013-01-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDTK	Skillful Craftsman Education Technology Limited Ordinary Share	2020-04-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EDUC	Educational Development Corp	1997-05-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EE	Excelerate Energy, Inc.	1901-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EEFT	Euronet Worldwide Inc	1996-12-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EEIQ	EpicQuest Education Group International Limited Common Stock	2020-12-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EEX	Emerald Holding, Inc.	2017-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EFC	Ellington Financial Inc. Common Stock	2011-03-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EFOI	Energy Focus, Inc.	2000-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EFSC	Enterprise Financial Services Corporation	1996-10-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EFSI	Eagle Financial Services Inc Common Stock	1997-04-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EFTY	Etoiles Capital Group Co., Ltd. Class A Ordinary Shares	2025-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EFX	Equifax, Incorporated	1899-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EFXT	Enerflex Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EG	Everest Group, Ltd.	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EGAN	eGain Corporation	1999-07-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EGBN	Eagle Bancorp Inc	1997-12-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EGG	Enigmatig Limited	2025-03-28	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EGHA	EGH Acquisition Corp. Class A Ordinary Shares	2025-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EGHT	8x8, Inc. Common Stock	1987-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EGO	Eldorado Gold Corporation	1991-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EGP	EastGroup Properties Inc.	1996-03-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EGY	Vaalco Energy, Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EHAB	Enhabit, Inc.	2023-04-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EHC	Encompass Health Corporation Common Stock	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EHGO	Eshallgo Inc. Class A Ordinary Shares	2023-04-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EHLD	Euroholdings Ltd. Common Stock	2025-05-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EHTH	eHealth, Inc.	2006-04-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EIG	Employers Holdings, Inc.	2006-12-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EIKN	Eikon Therapeutics, Inc. Common Stock	2026-01-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EIX	Edison International	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FCNCA	First Citizens BancShares Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EJH	E-Home Household Service Holdings Limited Ordinary Shares	2019-08-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EKSO	Ekso Bionics Holdings, Inc. Common Stock	2012-05-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EL	The Estee Lauder Companies Inc. Class A	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELA	Envela Corporation	2005-04-15	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELAB	PMGC Holdings Inc. Common Stock	2023-09-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELAN	Elanco Animal Health Incorporated Common Stock	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELBM	Electra Battery Materials Corporation Common Stock	2024-05-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELDN	Eledon Pharmaceuticals, Inc. Common Stock	2014-08-11	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELE	Elemental Royalty Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELF	e.l.f. Beauty, Inc.	2004-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELLO	Ellomay Capital LTD	1987-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELMD	Electromed, Inc.	2010-05-03	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELME	Elme Communities	1995-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELOG	Eastern International Ltd. Ordinary Shares	2024-09-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELPW	Elong Power Holding Limited Class A Ordinary Shares	2024-11-27	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELS	Equity Lifestyle Properties, Inc.	1997-03-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELSE	Electro-Sensors Inc	2009-03-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELTK	Eltek Ltd	2002-07-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELTX	Elicio Therapeutics, Inc. Common Stock	2014-04-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELUT	Elutia Inc. Class A Common Stock	2020-09-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELV	Elevance Health, Inc.	1940-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ELVA	Electrovaya Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELVN	Enliven Therapeutics, Inc. Common Stock	2020-02-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ELWT	Elauwit Connection, Inc. Common Stock	2025-08-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EMA	Emera Incorporated	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EMAT	Evolution Metals & Technologies Corp. Common Stock	2021-12-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EMBC	Embecta Corp. Common Stock	2022-12-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EME	EMCOR Group, Inc.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EMIS	Emmis Acquisition Corp. Class A Ordinary Shares	2025-07-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EML	Eastern Company	1995-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EMN	Eastman Chemical Company	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EMP	Entergy Mississippi, LLC First Mortgage Bonds, 4.90% Series due October 1, 2066	1994-03-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EMPD	Empery Digital Inc. Common stock	2021-09-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EMPG	Empro Group Inc. Ordinary shares	2024-09-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EMR	Emerson Electric Co.	1890-09-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENB	Enbridge, Inc	1987-12-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENGN	enGene Therapeutics Inc. Common Stock	2023-11-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENGS	Energys Group Limited Ordinary Shares	2023-12-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENLT	Enlight Renewable Energy Ltd. Ordinary Shares	2023-01-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENLV	Enlivex Ltd. Ordinary Shares	2014-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENOV	Enovis Corporation	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENPH	Enphase Energy, Inc.	2011-06-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENR	Energizer Holdings, Inc	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENS	EnerSys, Inc.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENSC	Ensysce Biosciences, Inc. Common Stock	2017-11-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENSG	The Ensign Group, Inc.	2007-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENTA	Enanta Pharmaceuticals, Inc	2012-11-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENTG	Entegris Inc	2000-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENTX	Entera Bio Ltd. Ordinary Shares	2017-11-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENVA	Enova International, Inc.	2011-09-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ENVB	Enveric Biosciences, Inc. Common Stock	1997-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ENVX	Enovix Corporation Common Stock	2020-11-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EOG	EOG Resources, Inc.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EOLS	Evolus, Inc. Common Stock	2018-01-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EONR	EON Resources Inc.	2021-01-29	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EOSE	Eos Energy Enterprises, Inc. Class A Common Stock	2020-04-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EP	Empire Petroleum Corporation	2007-07-03	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPAC	Enerpac Tool Group Corp.	1910-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPAM	EPAM SYSTEMS, INC.	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPC	Edgewell Personal Care Company	2015-07-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPD	Enterprise Products Partners L.P.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPM	Evolution Petroleum Corporation	1998-03-30	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPOW	E-Power Inc. Class A Ordinary Shares	2019-09-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EPR	EPR Properties	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPRT	Essential Properties Realty Trust, Inc.	2019-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EPRX	Eupraxia Pharmaceuticals Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EPSM	Epsium Enterprise Limited Class A Ordinary Shares	2022-02-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EPSN	Epsilon Energy Ltd.	2019-03-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EQ	Equillium, Inc. Common Stock	2018-09-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EQBK	Equity Bancshares, Inc.	2015-10-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EQH	Equitable Holdings, Inc.	1859-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EQIX	Equinix, Inc. Common Stock REIT	2000-06-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EQPT	EquipmentShare.com Inc Class A Common Stock	2025-12-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EQR	Equity Residential	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EQS	Equus Total Return, Inc.	1996-03-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EQT	EQT CORP	1888-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EQX	Equinox Gold Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ERAS	Erasca, Inc. Common Stock	2021-06-25	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ERIE	Erie Indemnity Co	1997-03-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ERII	Energy Recovery, Inc.	2008-04-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ERNA	Ernexa Therapeutics Inc. Common Stock	1996-04-16	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ERO	Ero Copper Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ES	Eversource Energy	1966-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESAB	ESAB Corporation	1904-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESCA	Escalade Inc	1996-03-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESE	ESCO Technologies, Inc.	1996-12-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESEA	Euroseas Ltd.(Marshall Islands)	2005-10-20	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESHA	ESH Acquisition Corp. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESI	Element Solutions Inc.	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESLA	Estrella Immunopharma, Inc. Common Stock	2021-02-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESLT	Elbit Systems Ltd	2002-06-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESNT	Essent Group LTD	2013-09-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESOA	Energy Services of America Corporation Common Stock	2006-04-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESP	Espey Mfg. & Electronics Corp	1995-09-22	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESPR	Esperion Therapeutics, Inc.	2013-05-14	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESQ	Esquire Financial Holdings, Inc. Common Stock	2017-05-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESRT	EMPIRE STATE REALTY TRUST, INC.	2014-03-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESS	Essex Property Trust, Inc	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ESTA	Establishment Labs Holdings Inc.	2018-06-21	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ESTC	Elastic N.V.	2012-02-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ET	Energy Transfer LP Common Units representing limited partner interests	2005-09-02	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ETD	Ethan Allen Interiors Inc	1932-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ETHM	Dynamix Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ETN	Eaton Corporation, plc Ordinary Shares	1911-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ETON	Eton Pharmaceutcials, Inc. Common Stock	2018-08-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ETOR	eToro Group Ltd. Class A Common Shares	2025-03-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ETR	Entergy Corporation	1913-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ETS	Elite Express Holding Inc. Class A Common Stock	2025-05-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ETSY	Etsy, Inc.	2015-03-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EU	enCore Energy Corp.	2025-03-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EUDA	EUDA Health Holdings Limited Ordinary Shares	2021-05-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EURK	Eureka Acquisition Corp Class A Ordinary Share	2024-03-08	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVAC	EQV Ventures Acquisition Corp. II	2025-06-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVC	Entravision Communication	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVCM	EverCommerce Inc. Common Stock	2021-05-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVER	EverQuote, Inc. Class A Common Stock	2018-06-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVEX	Eve Holding, Inc.	2020-10-22	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVGN	EVOGENE LTD.	2002-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVGO	EVgo Inc. Class A Common Stock	2020-09-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVH	Evolent Health, Inc Class A Common Stock	2015-05-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVI	EVI Industries, Inc.	2009-09-25	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVLV	Evolv Technologies Holdings, Inc. Class A Common Stock	2020-07-10	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVMN	Evommune, Inc.	2025-10-09	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVOX	Evolution Global Acquisition Corp Class A Ordinary Shares	2025-07-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVR	Evercore Inc.	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVRG	Evergy, Inc.	1909-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EVTC	EVERTEC, INC.	2013-02-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVTL	Vertical Aerospace Ltd.	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EVTV	Envirotech Vehicles, Inc. Common Stock	2014-11-12	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EW	Edwards Lifesciences Corp	1958-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EWBC	East-West Bancorp Inc	1999-03-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EWCZ	European Wax Center, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EWTX	Edgewise Therapeutics, Inc. Common Stock	2021-03-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXC	Exelon Corporation	2000-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXE	Expand Energy Corporation Common Stock	1996-09-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXEL	Exelixis Inc	2000-02-07	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXFY	Expensify, Inc. Class A Common Stock	2021-10-15	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXK	Endeavour Silver Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EXLS	ExlService Holdings, Inc.	2004-12-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXOD	Exodus Movement, Inc.	2025-03-06	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
EXOZ	eXoZymes Inc. Common Stock	2024-02-09	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXP	Eagle Materials, Inc.	1963-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EXPD	Expeditors International of Washington, Inc.	1996-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EXPE	Expedia Group, Inc. Common Stock	2006-03-31	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXPI	eXp World Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXPO	Exponent Inc	1997-04-03	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EXR	Extra Space Storage, Inc.	1977-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
EXTR	Extreme Networks	1999-02-05	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EYE	National Vision Holdings, Inc. Common Stock	2017-09-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EYPT	EyePoint, Inc. Common Stock	2006-01-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EZGO	EZGO Technologies Ltd. Ordinary Shares	2020-10-28	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EZPW	Ezcorp Inc	1996-12-24	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
EZRA	Reliance Global Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
F	Ford Motor Company	1903-06-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FA	First Advantage Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FACT	FACT II Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FAF	First American Financial Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FAMI	Farmmi, Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FANG	Diamondback Energy, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FARM	Farmer Bros Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FAST	Fastenal Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FATE	Fate Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FATN	FatPipe, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBGL	FBS Global Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBIN	Fortune Brands Innovations, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FBIO	Fortress Biotech, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBIZ	First Business Financial Services, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBK	FB Financial Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FBLA	FB Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBLG	FibroBiologics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBNC	First Bancorp/NC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBP	First BanCorp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FBRT	Franklin BSP Realty Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FBRX	Forte Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FBYD	Falcon's Beyond Global, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FC	Franklin Covey Company	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FCAP	First Capital Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCBC	First Community Bankshares, Inc. (VA)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCCO	First Community Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCEL	FuelCell Energy Inc  NEW (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCF	First Commonwealth Financial Corporation	1857-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FCFS	FirstCash Holdings, Inc. Common Stock	1984-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCHL	Fitness Champs Holdings Limited Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCN	FTI Consulting, Inc.	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GERN	Geron Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCNCN	First Citizens BancShares, Inc. Depositary Shares, each representing a 1/40th interest in a share of 6.625% Non-Cumulative Perpetual Preferred Stock, Series E	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCPT	Four Corners Property Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FCRS	FutureCrest Acquisition Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FCUV	Focus Universal Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FCX	Freeport-McMoran Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FDBC	Fidelity D & D Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FDMT	4D Molecular Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FDP	Fresh Del Monte Produce Inc.	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FDS	Factset Research Systems	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FDSB	Fifth District Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FDUS	Fidus Investment Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FDX	FedEx Corporation	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FE	FirstEnergy Corp.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FEAM	5E Advanced Materials, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FEBO	Fenbo Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FEED	ENvue Medical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FEIM	Frequency Electronics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FELE	Franklin Electric Co Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FEMY	Femasys Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FENC	Fennec Pharmaceuticals Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FER	Ferrovial SE Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FERA	Fifth Era Acquisition Corp I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FERG	Ferguson Enterprises Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FET	Forum Energy Technologies, Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FF	Future Fuel Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FFAI	Faraday Future Intelligent Electric Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FFBC	First Financial Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FFIC	Flushing Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FFIN	First Financial Bankshares Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FFIV	F5, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FG	F&G Annuities & Life, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FGBI	FIRST GUARANTY BANCSHARES INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FGI	FGI Industries Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FGII	FG Imperii Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FGL	Founder Group Limited Class A Ordinary Shares	1959-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FGMC	FG Merger II Corp. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FGNX	FG Nexus Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FHB	First Hawaiian, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FHI	Federated Hermes, Inc.	1957-10-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FHN	First Horizon Corporation	1864-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FHTX	Foghorn Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIBK	First Interstate BancSystem, Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FICO	Fair Isaac Corporation	1956-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FIEE	FiEE, Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIG	Figma, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FIGR	Figure Technology Solutions, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIGS	FIGS, Inc.	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FIGX	FIGX Capital Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIHL	Fidelis Insurance Holdings Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FINW	FinWise Bancorp Common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIP	FTAI Infrastructure Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIS	Fidelity National Information Services, Inc.	2006-02-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FISI	Financial Institutions Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FISV	Fiserv, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FITB	Fifth Third Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTA	Metalla Royalty & Streaming Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FITBM	Fifth Third Bancorp Depositary Shares Representing a 1/40th Ownership Interest in a Share of 6.875% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series M	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FITBO	Fifth Third Bancorp Depositary Shares each representing a 1/1000th ownership interest in a share of Non-Cumulative Perpetual Preferred Stock, Series K	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIVE	Five Below, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIVN	FIVE9, INC.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FIX	Comfort Systems USA, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FIZZ	National Beverage Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FJET	Starfighters Space, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FKWL	Franklin Wireless Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLD	Fold Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLEX	Flex Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLG	Flagstar Bank, National Association	1993-11-23	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLGT	Fulgent Genetics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLL	Full House Resorts, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLNA	Filana Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLNC	Fluence Energy, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLNG	FLEX LNG Ltd. Ordinary Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLNT	Fluent, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLO	Flowers Foods, Inc.	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLOC	Flowco Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLR	Fluor Corporation	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLS	Flowserve Corporation	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLUT	Flutter Entertainment plc	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FLUX	Flux Power Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLWS	1-800-FLOWERS.COM Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLXS	Flexsteel Industries	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLY	Firefly Aerospace Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLYE	Fly-E Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLYW	Flywire Corporation Voting Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FLYX	flyExclusive, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FMAO	Farmers & Merchants Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FMBH	First Mid Bancshares, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FMC	FMC Corporation	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FMFC	Kandal M Venture Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FMNB	Farmers National Banc Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FMST	Foremost Clean Energy Ltd. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FN	Fabrinet	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FNB	F.N.B. Corp	1864-02-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FND	Floor & Decor Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FNF	Fidelity National Financial, Inc.	1847-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FNGR	FingerMotion, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FNKO	Funko, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FNLC	First Bancorp, Inc. (ME)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FNUC	Frontier Nuclear and Minerals Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FNV	Franco-Nevada Corporation	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FNWB	First Northwest Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FNWD	Finward Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOA	Finance of America Companies Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FOFO	Hang Feng Technology Innovation Co., Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOLD	Amicus Therapeutics, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FONR	Fonar Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOR	Forestar Group Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FORA	Forian Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FORM	FormFactor Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FORR	Forrester Research Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOSL	Fossil Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOUR	Shift4 Payments, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FOX	Fox Corporation Class B Common Stock	1985-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOXA	Fox Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOXF	Fox Factory Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FOXX	Foxx Development Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FPH	Five Point Holdings, LLC Class A Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FPI	Farmland Partners Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FPS	Forgent Power Solutions, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FR	First Industrial Realty Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FRAF	Franklin Financial Services Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRBA	First Bank	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRD	Friedman Industries Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRGT	Freight Technologies, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRHC	Freedom Holding Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRME	First Merchants Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRMEP	First Merchants Corporation Depository Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRMI	Fermi Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRMM	Forum Markets, Incorporated Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRO	Frontline Plc	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FROG	JFrog Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRPH	FRP Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRPT	Freshpet, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRSH	Freshworks Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRST	Primis Financial Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FRT	Federal Realty Investment Trust	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSBC	Five Star Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSBW	FS Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSCO	FS Credit Opportunities Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSEA	First Seacoast Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSHP	Flag Ship Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSI	Flexible Solutions International, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSK	FS KKR Capital Corp. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSLR	First Solar, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSLY	Fastly, Inc. Class A Common Stock	2011-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSM	Fortuna Mining Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSP	Franklin Street Properties Corp	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSS	Federal Signal Corp.	1901-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSSL	FS Specialty Lending Fund	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FSTR	Foster (Lb) Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSUN	FirstSun Capital Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FSV	FirstService Corporation Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTAI	FTAI Aviation Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTCI	FTC Solar, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTDR	Frontdoor, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTEK	Fuel Tech, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTFT	Future FinTech Group Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTHM	Fathom Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTI	TechnipFMC plc Ordinary Share	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FTK	Flotek Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FTLF	FitLife Brands, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTNT	Fortinet, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTRE	Fortrea Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTRK	FAST TRACK GROUP Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FTS	Fortis Inc. Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FTV	Fortive Corporation	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FTW	Presidio Production Company	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FUBO	FuboTV Inc.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FUFU	BitFuFu Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FUL	H.B. Fuller Company	1887-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FULC	Fulcrum Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FULT	Fulton Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FULTP	Fulton Financial Corporation Depositary Shares, Each Representing a 1/40th Interest in a Share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FUN	Six Flags Entertainment Corporation	2024-07-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FUNC	First United Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FUND	Sprott Focus Trust, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FURY	Fury Gold Mines Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
FUSB	First US Bancshares, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FUSE	Fusemachines Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FVAV	Fortress Value Acquisition Corp. V Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FVCB	FVCBankcorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FVN	Future Vision II Acquisition Corporation Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FVR	FrontView REIT, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FVRR	Fiverr International Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
FWDI	Forward Industries, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FWONA	Liberty Media Corporation Series A Liberty Formula One Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FWONK	Liberty Media Corporation Series C Liberty Formula One Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FWRD	Forward Air Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FWRG	First Watch Restaurant Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
FXNC	FIRST NATL CORP STRASBURG VA	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
G	GENPACT LIMITED	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GABC	German American Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAIA	Gaia, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAIN	Gladstone Investment Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAING	Gladstone Investment Corporation 7.125% Notes due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAINI	Gladstone Investment Corporation 7.875% Notes due 2030	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAINN	Gladstone Investment Corporation 5.00% Notes Due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAINZ	Gladstone Investment Corporation 4.875% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GALT	Galectin Therapeutics Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAMB	Gambling.com Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAME	GameSquare Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GANX	Gain Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GAP	The Gap, Inc.	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GASS	StealthGas, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GATX	GATX Corporation	1933-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GAU	Galiano Gold Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GAUZ	Gauzy Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GBCI	Glacier Bancorp Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GBDC	Golub Capital BDC, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GBFH	GBank Financial Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GBLI	Global Indemnity Group, LLC Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GBR	New Concept Energy Inc	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GBTG	Global Business Travel Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GBX	The Greenbrier Companies, Inc.	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GCBC	Greene County Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GCDT	Green Circle Decarbonize Technology Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GCL	GCL Global Holdings Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GCMG	GCM Grosvenor Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GCO	Genesco Inc.	1924-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GCT	GigaCloud Technology Inc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GCTK	GlucoTrack, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GCTS	GCT Semiconductor Holding, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GD	General Dynamics Corporation	1952-02-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GDC	GD Culture Group Limited Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDDY	GoDaddy Inc	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GDEN	Golden Entertainment, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDEV	GDEV Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDHG	Golden Heaven Group Holdings Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDOT	Green Dot Corporation	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GDRX	GoodRx Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDTC	CytoMed Therapeutics Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GDYN	Grid Dynamics Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GE	GE Aerospace	1917-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GECC	Great Elm Capital. Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GECCG	Great Elm Capital Corp. 7.75% Notes Due 2030	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GECCH	Great Elm Capital Corp. 8.125% Notes Due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GECCI	Great Elm Capital Corp. 8.50% NOTES DUE 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GECCO	Great Elm Capital Corp. 5.875% Notes due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEF	Greif, Inc.	1877-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GEF.B	Greif, Inc. Class B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GEG	Great Elm Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEGGL	Great Elm Group, Inc. 7.25% Notes due 2027	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEHC	GE HealthCare Technologies Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEL	Genesis Energy, L.P.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GELS	Gelteq Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEMI	Gemini Space Station, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEN	Gen Digital Inc. Common Stock	2001-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GENB	Generate Biomedicines, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GENC	Gencor Industries, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GENI	Genius Sports Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GENK	GEN Restaurant Group, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GEO	The GEO Group, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GEOS	Geospace Technologies Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GETY	Getty Images Holdings, Inc.	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GEV	GE Vernova Inc.	2023-02-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GEVO	Gevo, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GFAI	Guardforce AI Co., Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GFF	Griffon Corp	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GFL	GFL Environmental Inc. Subordinate Voting Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GFR	Greenfire Resources Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GFS	GlobalFoundries Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GGG	Graco Inc	1926-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GGR	Gogoro Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GGRP	The Glimpse Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GH	Guardant Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GHC	GRAHAM HOLDINGS COMPANY	1889-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GHM	Graham Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GHRS	GH Research PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIB	CGI Inc.	1976-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GIBO	GIBO Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIC	Global Industrial Company	1949-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GIFT	Giftify, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIG	GigCapital7 Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIGM	GigaMedia Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIII	G-Iii Apparel Group Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIL	Gildan Activewear Inc.	1984-05-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GILD	Gilead Sciences Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GILT	Gilat Satellite Networks Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIPR	Generation Income Properties Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIS	General Mills, Inc.	1856-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GITS	Global Interactive Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIW	GigCapital8 Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GIX	GigCapital9 Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GKOS	Glaukos Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GL	Globe Life Inc.	1900-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLAD	Gladstone Capital Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLBE	Global-E Online Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLBS	Globus Maritime Limited	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLDG	GoldMining Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLE	Global Engine Group Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLED	GalaxyEdge Acquisition Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLIBA	GCI Liberty, Inc. Series A GCI Group Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLIBK	GCI Liberty, Inc. Series C GCI Group Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLMD	Galmed Pharmaceuticals Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLND	Greenland Energy Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLNG	Golar LNG Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLOB	GLOBANT S.A.	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLOO	Gloo Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLP	Global Partners LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLPI	Gaming and Leisure Properties, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLRE	Greenlight Captial RE, LTD. Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLSI	Greenwich LifeSciences, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLUE	Monte Rosa Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLW	Corning Incorporated	1851-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GLXG	Galaxy Payroll Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GLXY	Galaxy Digital Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GM	General Motors Company	1908-09-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GME	GameStop Corp. Class A	1980-08-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GMED	GLOBUS MEDICAL INC	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GMEX	GMEX ROBOTICS CORPORATION Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GMHS	Gamehaus Holdings Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GMM	Global Mofy AI Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNE	GENIE ENERGY LTD	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GNK	GENCO SHIPPING & TRADING LTD	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GNL	Global Net Lease, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GNLN	Greenlane Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNLX	Genelux Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNPX	Genprex, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNRC	GENERAC HOLDINGS INC	1959-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GNS	Genius Group Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GNSS	Genasys Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNTX	Gentex Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GNW	Genworth Financial, Inc.	2004-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GO	Grocery Outlet Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOAI	Eva Live Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOCO	GoHealth, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOGO	Gogo Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOLD	Gold.com, Inc.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GOLF	Acushnet Holdings Corp.	1910-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GOOD	Gladstone Commercial Corporation - REIT	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOOG	Alphabet Inc. Class C Capital Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOOGL	Alphabet Inc.	2004-08-19	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	1998-09-04	manual	Dutch auction IPO, opened at $100.01
GOOS	Canada Goose Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GORO	Gold Resource Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GOSS	Gossamer Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GOVX	GeoVax Labs, Inc. New	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GP	GreenPower Motor Company Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GPAC	General Purpose Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GPAT	GP-Act III Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GPC	Genuine Parts Company	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPGI	GPGI, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPI	Group 1 Automotive, Inc.	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPK	Graphic Packaging Holding Company	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPMT	Granite Point Mortgage Trust Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPN	Global Payments, Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPOR	Gulfport Energy Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPRE	Green Plains, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GPRK	GEOPARK LIMITED	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GPRO	GoPro, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GPUS	Hyperscale Data, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRAB	Grab Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRAF	Graf Global Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRAL	GRAIL, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRAN	Grande Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRBK	Green Brick Partners, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRC	The Gorman-Rupp Company Common Shares	1933-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRCE	Grace Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRDN	Guardian Pharmacy Services, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRDX	GridAI Technologies Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GREE	Greenidge Generation Holdings Inc. Class A Common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GREEL	Greenidge Generation Holdings Inc. 8.50% Senior Notes due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRI	GRI Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRML	Greenland Mines Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRMN	Garmin Ltd	1989-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRND	Grindr Inc.	2009-03-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRNQ	Greenpro Capital Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRNT	Granite Ridge Resources, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRO	Brazil Potash Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GROV	Grove Collaborative Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GROW	US Global Investors Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GROY	Gold Royalty Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GRPN	Groupon, Inc.Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRRR	Gorilla Technology Group Inc. Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GRWG	GROW GENERATION CORP	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GS	Goldman Sachs Group Inc.	1869-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GSAT	Globalstar, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSBC	Great Southern Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSBD	Goldman Sachs BDC, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GSHD	Goosehead Insurance, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSHR	Gesher Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSIT	GSI Technology	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSIW	Garden Stage Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSL	Global Ship Lease, Inc.	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GSM	Ferroglobe PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSRF	GSR IV Acquisition Corp. Class A ordinary share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GSUN	Golden Sun Technology Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GT	Goodyear Tire & Rubber	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTBP	GT Biopharma Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTE	Gran Tierra Energy Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
GTEC	Greenland Technologies Holding Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTEN	Gores Holdings X, Inc. Class A ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTERA	Globa Terra Acquisition Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTES	Gates Industrial Corporation plc	1911-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GTIM	Good Times Restaurants Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTLB	GitLab Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTLS	Chart Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GTM	ZoomInfo Technologies Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTN	Gray Media, Inc.	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GTN.A	Gray Media, Inc. Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GTX	Garrett Motion Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GTY	Getty Realty Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GURE	Gulf Resources, Inc. (NV) Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GUTS	Fractyl Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GV	Visionary Holdings Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GVA	Granite Construction Inc.	1922-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GVH	Globavend Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GWAV	Greenwave Technology Solutions, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GWH	ESS Tech, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GWRE	GUIDEWIRE SOFTWARE, INC.	2001-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GWRS	Global Water Resources, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GWW	W.W. Grainger, Inc.	1927-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GXAI	Gaxos.ai Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GXO	GXO Logistics, Inc.	2021-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
GYRE	Gyre Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
GYRO	Gyrodyne, LLC Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
H	Hyatt Hotels Corporation	1957-09-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HACQ	HCM IV Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAE	Haemonetics Corporation	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HAFC	Hanmi Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAFN	Hafnia Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HAIN	Hain Celestial Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAL	Halliburton Company	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HALO	Halozyme Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAO	Haoxi Health Technology Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAS	Hasbro, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HASI	HA Sustainable Infrastructure Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HAVA	Harvard Ave Acquisition Corporation Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAVAR	Harvard Ave Acquisition Corporation Rights that convert on a 1/10th of 1 basis to Class A ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HAYW	Hayward Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HBAN	Huntington Bancshares Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBANL	Huntington Bancshares Incorporated Depositary Shares, Each Representing a 1/40th Interest in a Share of 6.875% Series J Non-Cumulative Perpetual Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBANM	Huntington Bancshares Incorporated Depositary Shares each representing a 1/1000th interest in a share of Huntington Series I Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBANZ	Huntington Bancshares Incorporated Depositary Shares, Each Representing a 1/1000th Interest in a Share of 5.50% Series L Non-Cumulative Perpetual Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBB	Hamilton Beach Brands Holding Company Class A Common Stock	1910-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HBCP	Home Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBIO	Harvard Bioscience Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBM	Hudbay Minerals Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HBNB	Hotel101 Global Holdings Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBNC	Horizon Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HBT	HBT Financial, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCA	HCA Healthcare, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HCAC	Hall Chadwick Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCAI	Huachen AI Parking Management Technology Holding Co., Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCAT	Health Catalyst, Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCC	Warrior Met Coal, Inc.	1974-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HCHL	Happy City Holdings Limited Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCI	HCI Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HCIC	Hennessy Capital Investment Corp. VIII Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCKT	Hackett Group Inc (The).	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCMA	HCM III Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCSG	Healthcare Services Group	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCTI	Healthcare Triangle, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCWB	HCW Biologics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HCWC	Healthy Choice Wellness Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
HD	Home Depot, Inc.	1978-02-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HDSN	Hudson Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HE	Hawaiian Electric Industries, Inc.	1891-10-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HEI	HEICO Corporation	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HEI.A	HEICO CORP CL A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HELE	Helen Of Troy Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HELP	Cybin Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HERZ	Herzfeld Credit Income Fund, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HESM	Hess Midstream LP Class A Share representing a limited partner Interest	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HFBL	Home Federal Bancorp, Inc. of Louisiana	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HFFG	HF Foods Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HFWA	Heritage Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HG	Hamilton Insurance Group, Ltd. Class B Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HGBL	HERITAGE GLOBAL INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HGTY	Hagerty, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HGV	Hilton Grand Vacations Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HHH	Howard Hughes Holdings Inc.	1913-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HHS	Harte-Hanks, Inc. Common Stock	1923-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HIFS	Hingham Institution for Saving	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HIG	The Hartford Insurance Group, Inc.	1810-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HIHO	Highway Holdings Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HII	Huntington Ingalls Industries, Inc.	2011-03-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HIMS	Hims & Hers Health, Inc.	2017-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HIND	Vyome Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HIPO	Hippo Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HIT	Health In Tech, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HITI	High Tide Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HIVE	HIVE Digital Technologies Ltd. Common Shares	2006-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HIW	Highwoods Properties Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HKIT	Hitek Global Inc. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HKPD	Cellyan Biotechnology Co., Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HL	Hecla Mining Company	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLF	Herbalife Ltd.	1980-02-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLI	Houlihan Lokey, Inc.	1972-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLIO	Helios Technologies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLIT	Harmonic Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HLLY	Holley Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLMN	Hillman Solutions Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HLNE	Hamilton Lane Incorporated Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HLP	Hongli Group Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HLT	Hilton Worldwide Holdings Inc.	1919-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLX	Helix Energy Solutions Group, Inc.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HLXC	Helix Acquisition Corp. III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HMH	HMH Holding Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HMN	Horace Mann Educators Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HMR	Heidmar Maritime Holdings Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HNGE	Hinge Health, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HNI	HNI Corporation	1944-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HNNA	Hennessy Advisors, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HNNAZ	Hennessy Advisors, Inc. 4.875% Notes due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HNRG	Hallador Energy Company	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HNST	The Honest Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HNVR	Hanover Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOFT	Hooker Furnishings Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOG	Harley-Davidson, Inc.	1903-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HOLO	MicroCloud Hologram Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOMB	Home BancShares, Inc.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HON	Honeywell International, Inc.	1885-04-23	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOOD	Robinhood Markets, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOPE	Hope Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOTH	Hoth Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOUR	Hour Loop, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOV	Hovnanian Enterprises, Inc. Class A	1959-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HOVR	New Horizon Aircraft Ltd. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HOWL	Werewolf Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HP	Helmerich & Payne, Inc.	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HPAI	Helport AI Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HPE	Hewlett Packard Enterprise Company	2015-11-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HPK	HighPeak Energy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HPP	Hudson Pacific Properties, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HPQ	HP Inc.	2015-11-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HQ	Horizon Quantum Holdings Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HQI	HireQuest, Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HQY	HealthEquity, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HR	Healthcare Realty Trust Incorporated	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HRB	H&R Block, Inc.	1955-01-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HRI	Herc Holdings Inc.	1965-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HRL	Hormel Foods Corporation	1891-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HRMY	Harmony Biosciences Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HROW	Harrow, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HRTG	HERITAGE INSURANCE HOLDINGS INC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HRTX	Heron Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HRZN	Horizon Technology Finance Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSAI	Hesai Group American Depositary Share, each ADS represents one Class B ordinary share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSCS	HeartSciences Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSDT	Solana Company Class A Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSHP	Himalaya Shipping Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HSIC	Henry Schein Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSLV	Highlander Silver Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
HSPT	Horizon Space Acquisition II Corp. Ordinary share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HST	Host Hotels & Resorts, Inc.	1993-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSTM	HealthStream Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HSY	The Hershey Company	1894-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HTB	HomeTrust Bancshares, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HTBK	Heritage Commerce Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTCO	High-Trend International Group Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTCR	Heartcore Enterprises, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTFL	Heartflow, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTGC	Hercules Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HTH	HILLTOP HOLDINGS INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HTLD	Heartland Express Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTLM	HomesToLife Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTO	H2O America Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTOO	Fusion Fuel Green PLC Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HTZ	Hertz Global Holdings, Inc Common Stock	1977-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUBB	Hubbell Incorporated	1905-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HUBC	Hub Cyber Security Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUBG	HUB Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUBS	HUBSPOT, INC.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HUDI	Huadi International Group Co., Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUHU	HUHUTECH International Group Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUM	Humana Inc.	1961-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HUMA	Humacyte, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUN	Huntsman Corporation	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HURA	TuHURA Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HURC	Hurco Cos Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HURN	Huron Consulting Group Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HUT	Hut 8 Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HVII	Hennessy Capital Investment Corp. VII Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HVMC	Highview Merger Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HVT	Haverty Furniture Companies, Inc.	1885-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HVT.A	Haverty Furniture Companies, Inc. Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HWBK	Hawthorn Bancshars Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HWC	Hancock Whitney Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HWCPZ	Hancock Whitney Corporation 6.25% Subordinated Notes due 2060	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HWH	HWH International Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HWKN	Hawkins Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HWM	Howmet Aerospace Inc.	2016-11-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HXHX	Haoxin Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HXL	Hexcel Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HY	Hyster-Yale, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
HYFM	Hydrofarm Holdings Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HYFT	MindWalk Holdings Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HYLN	Hyliion Holdings Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
HYMC	Hycroft Mining Holding Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HYNE	Hoyne Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HYPD	Hyperion DeFi, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HYPR	Hyperfine, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
HZO	MarineMax, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IAC	IAC Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IACO	Idea Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IAG	IAMGold Corporation	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IART	Integra LifeSciences Holdings	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IAUX	i-80 Gold Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IBAC	IB Acquisition Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBCP	Independent Bank Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBEX	IBEX Limited Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBG	Innovation Beverage Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBIO	iBio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBKR	Interactive Brokers Group, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBM	International Business Machines Corporation	1911-06-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IBO	Impact BioMedical, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IBOC	International Bancshares Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBP	INSTALLED BUILDING PRODUCTS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IBRX	ImmunityBio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IBTA	Ibotta, Inc.	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ICCC	Immucell Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICCM	IceCure Medical Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICE	Intercontinental Exchange  Inc.	2000-05-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ICFI	ICF International, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICHR	Ichor Holdings	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICL	ICL Group Ltd.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ICLR	Icon Plc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICMB	Investcorp Credit Management BDC, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICON	Icon Energy Corp. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICU	SeaStar Medical Holding Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ICUI	ICU Medical Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IDA	IDACORP, Inc.	1998-10-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IDAI	T Stamp Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IDCC	InterDigital, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IDN	Intellicheck, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IDR	Idaho Strategic Resources, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IDT	IDT Corporation Class B	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IDXX	Idexx Laboratories Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IDYA	IDEAYA Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IE	Ivanhoe Electric Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IEAG	Infinite Eagle Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IEP	Icahn Enterprises L.P	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IESC	IES Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IEX	IDEX Corporation	1988-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IFBD	Infobird Co., Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IFF	International Flavors & Fragrances Inc.	1958-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IFRX	InflaRx N.V. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IFS	Intercorp Financial Services Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IGAC	Invest Green Acquisition Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IGC	IGC Pharma, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IGIC	International General Insurance Holdings Ltd. Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IHRT	iHeartMedia, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IHS	IHS Holding Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IHT	InnSuites Hospitality Trust	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
III	Information Services Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IIIN	Insteel Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IIIV	i3 Verticals, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IINN	Inspira Technologies Oxy B.H.N. Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IIPR	Innovative Industrial Properties, Inc. Common stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IKT	Inhibikase Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ILLR	Triller Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ILMN	Illumina Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMA	ImageneBio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMAX	Imax Corp	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IMCC	IM Cannabis Corp. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMDX	Insight Molecular Diagnostics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMKTA	Ingles Markets Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMMR	Immersion Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMMX	Immix Biopharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMNM	Immunome, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMNN	Imunon, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMO	Imperial Oil Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IMPP	Imperial Petroleum Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMPPP	Imperial Petroleum Inc. 8.75% Series A Cumulative Redeemable Perpetual Preferred Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMRX	Immuneering Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMSR	Terrestrial Energy Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMTE	Integrated Media Technology Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMTX	Immatics N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMUX	Immunic, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMVT	Immunovant, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IMXI	International Money Express, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INAB	IN8bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INAC	Indigo Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INBK	First Internet Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INBKZ	First Internet Bancorp 6.0% Fixed-to-Floating Rate Subordinated Notes Due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INBS	Intelligent Bio Solutions Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INBX	Inhibrx Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INCR	Intercure Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INCY	Incyte Genomics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INDB	Independent Bank Corp/MA	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INDI	indie Semiconductor, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INDO	Indonesia Energy Corporation Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
INDP	Indaptus Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INDV	Indivior Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INEO	INNEOVA Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INFQ	Infleqtion, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INFU	InfuSystem Holdings, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
INGM	Ingram Micro Holding Corporation	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INGN	Inogen Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INGR	Ingredion Incorporated	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INHD	Inno Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INKT	MiNK Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INLF	INLIF LIMITED Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INLX	Intellinetics, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
INM	InMed Pharmaceuticals Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INMB	INmune Bio Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INMD	InMode Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INN	Summit Hotel Properties, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INNV	InnovAge Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INO	Inovio Pharmaceuticals, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INOD	Innodata Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INR	Infinity Natural Resources, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INSE	Inspired Entertainment, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INSG	Inseego Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INSM	Insmed, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INSP	Inspire Medical Systems, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INSW	International Seaways, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INTA	Intapp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTC	Intel Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTG	Intergroup Corporation (The)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTJ	Intelligent Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTR	Inter & Co. Inc. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTS	Intensity Therapeutics, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTT	inTEST Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
INTU	Intuit Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INTZ	Intrusion Inc New	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INUV	Inuvo, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
INV	Innventure, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INVA	Innoviva, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INVE	Identive, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
INVH	Invitation Homes Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INVX	Innovex International, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
INVZ	Innoviz Technologies Ltd. Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IONQ	IonQ, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IONS	Ionis Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IOR	Income Opportunity Realty Investors, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
IOSP	Innospec Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IOT	Samsara Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IOTR	iOThree Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IOVA	Iovance Biotherapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IP	International Paper Co.	1898-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IPAR	Interparfums, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPCX	Inflection Point Acquisition Corp. III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPDN	Professional Diversity Network, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPEX	Inflection Point Acquisition Corp. V Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPGP	IPG Photonics Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPI	Intrepid Potash, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IPM	Intelligent Protection Management Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPOD	Dune Acquisition Corporation II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPSC	Century Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPST	IP Strategy Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPW	iPower, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IPWR	Ideal Power Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IQST	IQSTEL INC. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IQV	IQVIA Holdings Inc.	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IR	Ingersoll Rand Inc. Common Stock	2020-02-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IRAB	Iris Acquisition Corp II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IRD	Opus Genetics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRDM	Iridium Communications Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IREN	IREN Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRHO	Iron Horse Acquisitions II Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRIX	Iridex Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRM	Iron Mountain Inc.	1951-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IRMD	iRadimed Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRON	Disc Medicine, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRT	Independence Realty Trust Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IRTC	iRhythm Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IRWD	Ironwood Pharmaceuticals, Inc. - Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISBA	Isabella Bank Corporation Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISOU	IsoEnergy Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ISPC	iSpecimen Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISPR	Ispire Technology Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISRG	Intuitive Surgical Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISSC	Innovative Solutions & Support	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ISTR	Investar Holding Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IT	Gartner, Inc.	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ITGR	Integer Holdings Corporation	1970-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ITHA	ITHAX Acquisition Corp III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ITIC	Investors Title Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ITOC	iTonic Holdings Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ITP	IT Tech Packaging, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ITRG	Integra Resources Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ITRI	Itron Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ITRN	Ituran Location and Control Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ITT	ITT Inc.	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ITW	Illinois Tool Works Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IVDA	Iveda Solutions, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IVF	INVO Fertility, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IVR	Invesco Mortgage Capital Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IVT	InvenTrust Properties Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IVVD	Invivyd, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IVZ	Invesco LTD	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
IZEA	IZEA Worldwide, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
IZM	ICZOOM Group Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
J	Jacobs Solutions Inc.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JACK	Jack in the Box Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JACS	Jackson Acquisition Company II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JAGU	Jaguar Uranium Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
JAGX	Jaguar Health, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JAKK	Jakks Pacific Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JAN	Janus Living, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JANX	Janux Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JATT	JATT II Acquisition Corp Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JAZZ	Jazz Pharmaceuticals, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBDI	JBDI Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBGS	JBG SMITH Properties Common Shares	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JBHT	JB Hunt Transport Services Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBI	Janus International Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JBIO	Jade Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBL	Jabil Inc.	1966-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JBLU	JetBlue Airways Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBS	JBS N.V.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JBSS	John B. Sanfilippo & SON	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JBTM	JBT Marel Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JCAP	Jefferson Capital, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JCI	Johnson Controls International plc	1885-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JCSE	JE Cleantech Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JCTC	Jewett-Cameron Trading	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JDZG	JIADE LIMITED Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JEF	Jefferies Financial Group Inc.	1962-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JELD	JELD-WEN Holding, Inc.	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JEM	707 Cayman Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JENA	Jena Acquisition Corporation II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JFB	JFB Construction Holdings Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JHG	Janus Henderson Group plc Ordinary Shares	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JHX	James Hardie Industries plc	1888-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JILL	J.Jill, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JJSF	J&J Snack Foods Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JKHY	Henry (Jack) & Associates	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JL	J-Long Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JLHL	Julong Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JLL	Jones Lang LaSalle, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JMG	JM Group Limited	1882-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
JMSB	John Marshall Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JNJ	Johnson & Johnson	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JOB	GEE Group Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
JOBY	Joby Aviation, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JOE	St. Joe Company	1936-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JOUT	Johnson Outdoors Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMPR	Kemper Corporation	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JPM	JPMorgan Chase & Co.	2000-12-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	1799-09-01	manual	Current entity formed by Chase Manhattan + J.P. Morgan merger Dec 2000
JRSH	Jerash Holdings (US), Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JRVR	James River Group Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JSM	Navient Corporation 6% Senior Notes due December 15, 2043	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JSPR	Jasper Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JTAI	Jet.AI Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JUNS	Jupiter Neurosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JVA	Coffee Holding Co., Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JWEL	Jowell Global Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JXG	JX Luxventure Limited Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JXN	Jackson Financial Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
JYD	Jayud Global Logistics Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JYNT	The Joint Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
JZXN	Jiuzi Holdings, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KAI	Kadant Inc.	1991-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KALA	KALA BIO, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KALU	Kaiser Aluminum Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KALV	KalVista Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KAPA	Kairos Pharma, Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
KARO	Karooooo Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KBDC	Kayne Anderson BDC, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KBH	KB Home	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KBON	Karbon Capital Partners Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KBR	KBR, Inc.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KBSX	FST Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KCHV	Kochav Defense Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KD	Kyndryl Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KDK	Kodiak AI, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KDP	Keurig Dr Pepper Inc.	2018-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KE	Kimball Electronics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KEEL	Keel Infrastructure Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KELYA	Kelly Services Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KELYB	Kelly Services Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KEN	KENON HOLDINGS LTD.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KEQU	Kewaunee Scientific Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KEX	Kirby Corporation	1921-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KEY	KeyCorp	1825-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KEYS	Keysight Technologies, Inc.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KFFB	Kentucky First Federal Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KFII	K&F Growth Acquisition Corp. II Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KFRC	Kforce Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KFS	Kingsway Financial Services, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KFY	Korn Ferry	1969-11-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KG	Kestrel Group, Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KGC	Kinross Gold Corporation	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KGEI	Kolibri Global Energy Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KGS	Kodiak Gas Services, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KHC	The Kraft Heinz Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KIDS	OrthoPediatrics Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KIDZ	Classover Holdings, Inc. Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KIM	Kimco Realty Corp.	1958-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KINS	Kingstone Companies, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KITT	Nauticus Robotics, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KKR	KKR & Co. Inc.	1976-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KLAC	KLA Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KLAR	Klarna Group plc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KLC	KinderCare Learning Companies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KLIC	Kulicke & Soffa Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KLRA	Kailera Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KLRS	Kalaris Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KLTR	Kaltura, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KLXE	KLX Energy Services Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMB	Kimberly-Clark Corp.	1872-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMDA	Kamada Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMI	Kinder Morgan, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KMRK	K-Tech Solutions Company Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMT	Kennametal Inc.	1938-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KMTS	Kestra Medical Technologies, Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KMX	CarMax Inc.	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KN	KNOWLES CORPORATION	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNDI	Kandi Technologies Group, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KNF	Knife River Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNOP	KNOT OFFSHORE PARTNERS LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNRX	KNOREX LTD.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNSA	Kiniksa Pharmaceuticals International, plc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KNSL	Kinsale Capital Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNTK	Kinetik Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KNX	Knight-Swift Transportation Holdings Inc. Class A Common Stock	2017-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KO	Coca-Cola Company	1886-05-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KOD	Kodiak Sciences Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KODK	EASTMAN KODAK COMPANY	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KOP	Koppers Holdings, Inc.	1988-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KOPN	Kopin Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KORE	KORE Group Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KOS	Kosmos Energy Ltd.	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KOSS	Koss Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KOYN	CSLM Digital Asset Acquisition Corp III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KPLT	Katapult Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KPRX	Kiora Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KPTI	Karyopharm Therapeutics Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KR	The Kroger Co.	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRAQ	KRAKacquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRC	Kilroy Realty Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KREF	KKR Real Estate Finance Trust Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRG	Kite Realty Group Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRMD	KORU Medical Systems, Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRMN	Karman Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRNT	Kornit Digital Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRNY	Kearny Financial Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRO	Kronos Worldwide, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KROS	Keros Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRP	Kimbell Royalty Partners, LP Common Units representing Limited Partner Interests	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRRO	Korro Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRSP	Rice Acquisition Corporation 3	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KRT	Karat Packaging Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRUS	Kura Sushi USA, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KRYS	Krystal Biotech, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KSCP	Knightscope, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KSS	Kohls Corporation	1962-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KTB	Kontoor Brands, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KTCC	KEY Tronic Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KTOS	Kratos Defense & Security Solutions, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KTTA	Pasithea Therapeutics Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KTWO	K2 Capital Acquisition Corporation Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KULR	KULR Technology Group, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
KURA	Kura Oncology, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KUST	Kustom Entertainment, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KVAC	Keen Vision Acquisition Corporation Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KVHI	KVH Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KVUE	Kenvue Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KVYO	Klaviyo, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KW	KENNEDY-WILSON HOLDINGS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KWM	K Wave Media, Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KWR	Quaker Houghton	1918-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
KXIN	Kaixin Holdings Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KYIV	Kyivstar Group Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KYMR	Kymera Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KYNB	Kyntra Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KYTX	Kyverna Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
KZR	Kezar Life Sciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
L	Loews Corporation	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LAB	Standard BioTools Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAC	Lithium Americas Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LAD	Lithia Motors, Inc.	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LADR	LADDER CAPITAL CORP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LAES	SEALSQ Corp Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAFA	LaFayette Acquisition Corp. Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAKE	Lakeland Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAMR	Lamar Advertising Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAND	Gladstone Land Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LANDO	Gladstone Land Corporation 6.00% Series B Cumulative Redeemable Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LANV	Lanvin Group Holdings Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LAR	Lithium Argentina AG	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LARK	Landmark Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LASE	Laser Photonics Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LASR	nLIGHT, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LATA	Galata Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAUR	Laureate Education, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LAW	CS Disco, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LAZ	Lazard, Inc.	1848-07-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LB	LandBridge Company LLC	1963-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LBGJ	Li Bang International Corporation Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBRDA	Liberty Broadband Corporation Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBRDK	Liberty Broadband Corporation Class C	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBRT	Liberty Energy Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LBRX	LB Pharmaceuticals Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBTYA	Liberty Global Ltd. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBTYB	Liberty Global Ltd. Class B Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LBTYK	Liberty Global Ltd. Class C Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LC	LendingClub Corporation	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LCCC	Lakeshore Acquisition III Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LCFY	Locafy Limited Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LCID	Lucid Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LCII	LCI Industries	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LCNB	LCNB Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LCTX	Lineage Cell Therapeutics, Inc.	1990-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LCUT	Lifetime Brands, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LDI	loanDepot, Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LDOS	Leidos Holdings, Inc.	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LE	Lands' End, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEA	Lear Corporation	1917-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LECO	Lincoln Electric Holdings Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEDS	SemiLEDS Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEE	Lee Enterprises, Inc.	1890-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEG	Leggett & Platt, Inc.	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEGH	Legacy Housing Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEGO	Legato Merger Corp. IV	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEGT	Legato Merger Corp. III	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEN	Lennar Corporation Class A	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEN.B	Lennar Corporation Class B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LENZ	LENZ Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LESL	Leslie's, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LEU	Centrus Energy Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEVI	Levi Strauss & Co. Class A Common Stock	1853-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LEXX	Lexaria Bioscience Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFAC	Leapfrog Acquisition Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFCR	Lifecore Biomedical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFMD	LifeMD, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFST	LifeStance Health Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFT	Lument Finance Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LFUS	Littelfuse Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFVN	Lifevantage Corporation Common Stock (Delaware)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LFWD	Lifeward Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGCB	Linkage Global Inc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGCL	Lucas GC Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGCY	Legacy Education Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LGHL	Lion Group Holding Ltd. American Depositary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGIH	LGI Homes, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGL	The LGL Group, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LGN	Legence Corp. Class A Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGND	Ligand Pharmaceuticals Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGO	Largo Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LGPS	LogProstyle Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LGVN	Longeveron Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LH	Labcorp Holdings Inc.	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LHAI	Linkhome Holdings Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LHSW	Lianhe Sowell International Group Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LHX	L3Harris Technologies, Inc.	2019-06-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LICN	Lichen China Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIDR	AEye, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIEN	Chicago Atlantic BDC, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIF	Life360, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIFE	Ethos Technologies Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LII	Lennox International Inc.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LILA	Liberty Latin America Ltd. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LILAK	Liberty Latin America Ltd. Class C Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIMN	Liminatus Pharma, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIN	Linde plc Ordinary Share	1994-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LINC	Lincoln Educational Services	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIND	Lindblad Expeditions Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LINE	Lineage, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LINK	Interlink Electronics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LION	Lionsgate Studios Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LIQT	LiqTech International, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LITE	Lumentum Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LITS	Lite Strategy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIVE	Live Ventures Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIVN	LivaNova PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LIXT	Lixte Biotechnology Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LKFN	Lakeland Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LKQ	LKQ Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LKSP	Lake Superior Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LLY	Eli Lilly & Co.	1875-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LLYVA	Liberty Live Holdings, Inc. Series A Liberty Live Group Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LLYVK	Liberty Live Holdings, Inc. Series C Liberty Live Group Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMAT	LeMaitre Vascular, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMB	Limbach Holdings, Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMFA	LM Funding America, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMND	Lemonade, Inc.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LMNR	Limoneira Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMRI	Lumexa Imaging Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LMT	Lockheed Martin Corp.	1995-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LNAI	Lunai Bioworks Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNC	Lincoln National Corp.	1905-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LNG	Cheniere Energy Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LNKB	LINKBANCORP, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNKS	Linkers Industries Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNN	Lindsay Corporation	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LNSR	LENSAR, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNT	Alliant Energy Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNTH	Lantheus Holdings, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LNZA	LanzaTech Global, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOAN	Manhattan Bridge Capital, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOAR	Loar Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LOB	Live Oak Bancshares, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LOBO	LOBO TECHNOLOGIES LTD. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOCL	Local Bounti Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LOCO	El Pollo Loco Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LODE	Comstock Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LOGI	Logitech International SA	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOKV	Live Oak Acquisition Corp. V Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LONA	LeonaBio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOOP	Loop Industries, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOPE	Grand Canyon Education, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOVE	The Lovesac Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LOW	Lowe's Companies Inc.	1921-03-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LPA	Logistic Properties of the Americas	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LPAA	Launch One Acquisition Corp. Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPBB	Launch Two Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPCN	Lipocine Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPCV	Launchpad Cadenza Acquisition Corp I Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPG	DORIAN LPG LTD	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LPLA	LPL Financial Holdings Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPRO	Open Lending Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPSN	LivePerson Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPTH	Lightpath Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LPX	Louisiana-Pacific Corp.	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LQDA	Liquidia Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LQDT	Liquidity Services, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LRCX	Lam Research Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LRHC	La Rosa Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LRMR	Larimar Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LRN	Stride, Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LSAK	Lesaka Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSBK	Lake Shore Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSCC	Lattice Semiconductor Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSE	Leishen Energy Holding Co., Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSF	Laird Superfood, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LSH	Lakeside Holding Limited Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSPD	Lightspeed Commerce Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LSTA	Lisata Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LSTR	Landstar System Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LTBR	Lightbridge Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LTC	LTC Properties, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LTH	Life Time Group Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LTRN	Lantern Pharma Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LTRX	Lantronix Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUCD	Lucid Diagnostics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUCK	Lucky Strike Entertainment Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LUCY	Innovative Eyewear, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUD	Luda Technology Group Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
LULU	lululemon athletica inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUMN	Lumen Technologies, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LUNG	Pulmonx Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUNR	Intuitive Machines, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LUV	Southwest Airlines Co.	1967-03-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LVLU	Lulu's Fashion Lounge Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LVO	LiveOne, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LVS	Las Vegas Sands Corp.	1937-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LVWR	LiveWire Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LW	Lamb Weston Holdings, Inc.	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LWAC	LightWave Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LWAY	Lifeway Foods Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LWLG	Lightwave Logic, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LXEO	Lexeo Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LXFR	Luxfer Holdings PLC Ordinary Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LXP	LXP Industrial Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LXRX	Lexicon Pharmaceuticals, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LXU	LSB INDUSTRIES INC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LYB	LyondellBasell Industries N.V. Class A	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LYEL	Lyell Immunopharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LYFT	Lyft, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LYTS	LSI Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LYV	Live Nation Entertainment Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LZ	LegalZoom.com, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
LZB	La-Z-Boy Incorporated	1927-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LZM	Lifezone Metals Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
LZMH	LZ Technology Holdings Limited Class B Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
M	Macy's Inc.	1929-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MA	Mastercard Incorporated	1966-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAA	Mid-America Apartment Communities, Inc.	1977-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAAS	Maase Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAC	The Macerich Company	1964-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MACI	Melar Acquisition Corp. I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAGH	Magnitude International Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAGN	Magnera Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAIA	MAIA Biotechnology, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAIN	Main Street Capital Corporation	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAIR	Madison Air Solutions Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAKO	Mako Mining Corp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAMA	Mama's Creations, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAMK	MaxsMaking Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAMO	Massimo Group Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAN	ManpowerGroup	1948-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MANE	Veradermics, Incorporated	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MANH	Manhattan Associates Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MANU	MANCHESTER UNITED PLC	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAPS	WM Technology, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAR	Marriott International Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MARA	MARA Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MARPS	Marine Petroleum Trust	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAS	Masco Corporation	1929-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MASI	Masimo Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MASK	3 E Network Technology Group Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MASS	908 Devices Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAT	Mattel, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MATH	Metalpha Technology Holding Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MATV	Mativ Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MATW	Matthews International Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MATX	Matsons, Inc.	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAX	MediaAlpha, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MAXN	Maxeon Solar Technologies, Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAYS	Mays (J.W.) Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MAZE	Maze Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MB	MasterBeef Group Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBAI	Check-Cap Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBAV	M3-Brigade Acquisition V Corp. Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBBC	Marathon Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBC	MasterBrand, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MBI	MBIA Inc.	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MBIN	Merchants Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBINL	Merchants Bancorp Depositary Shares, Each Representing a 1/40thInterest in a Share of 7.25% Fixed Rate Series E Non-CumulativePerpetual Preferred Stock, without par value	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBINM	Merchants Bancorp Depositary Shares, Each Representing a 1/40th Interest in a Share of 8.25% Fixed-Rate Reset Series D Non-Cumulative Perpetual Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBINN	Merchants Bancorp Depositary Shares Preferred Series C	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBIO	Mustang Bio, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBLY	Mobileye Global Inc. Class A Common Stock	1998-11-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBOT	Microbot Medical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBRX	Moleculin Biotech, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBUU	Malibu Boats, Inc. Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBVI	M3-Brigade Acquisition VI Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBWM	Mercantile Bank Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MBX	MBX Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MC	MOELIS & COMPANY	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCB	Metropolitan Bank Holding Corp. Common Stock, $0.01 par value per share	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCBS	METROCITY BANKSHARES INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCD	McDonald's Corporation	1940-05-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCFT	MasterCraft Boat Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCGA	Yorkville Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCHB	Mechanics Bancorp Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCHP	Microchip Technology Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCHPP	Microchip Technology Incorporated Depositary Shares Each Representing a 1/20th Interest in a Share of 7.50% Series A Mandatory Convertible Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCHX	Marchex, Inc. Class B	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCK	McKesson Corporation	1833-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCO	Moody's Corporation	1909-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCRB	Seres Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCRI	Monarch Casino & Resort Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCRP	Micropolis AI Robotics	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCS	The Marcus Corporation	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MCTA	Charming Medical Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCW	Mister Car Wash, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MCY	Mercury General Corp.	1961-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MD	Pediatrix Medical Group, Inc.	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MDA	MDA Space Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MDAI	Spectral AI, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDB	MongoDB, Inc. Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDBH	MDB Capital Holdings, LLC Class A common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDCX	Medicus Pharma Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDGL	Madrigal Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDIA	Mediaco Holding Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDLN	Medline Inc. Class A common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDLZ	Mondelez International, Inc. Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDRR	Medalist Diversified, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDT	Medtronic plc	1949-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MDU	MDU Resources Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MDV	Modiv Industrial, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MDWD	MediWound Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDXG	MiMedx Group, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MDXH	MDxHealth SA Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEC	Mayville Engineering Company, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MED	Medifast, Inc.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MEDP	Medpace Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEG	Montrose Environmental Group, Inc.	1940-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MEGL	Magic Empire Global Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEHA	Functional Brands, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEI	Methode Electronics	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MELI	Mercado Libre, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MENS	Jyong Biotech Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEOH	Methanex Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MERC	Mercer International Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MESH	Meshflow Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MET	MetLife, Inc.	1868-03-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
META	Meta Platforms Inc.	2012-05-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	2004-02-04	manual	Largest tech IPO at the time, $38/share open
METC	Ramaco Resources, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
METCB	Ramaco Resources, Inc. Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
METCI	Ramaco Resources, Inc. 8.250% Senior Notes due 2030	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
METCZ	Ramaco Resources, Inc. 8.375% Senior Notes due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MEVO	M Evo Global Acquisition Corp II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MFA	MFA Financial, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MFC	Manulife Financial Corp.	1887-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MFI	mF International Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MFIC	MidCap Financial Investment Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MFICL	MidCap Financial Investment Corporation 8.00% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MFIN	Medallion Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MG	Mistras Group Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MGA	Magna International	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MGEE	MGE Energy Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGIH	Millennium Group International Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGLD	The Marygold Companies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MGM	MGM RESORTS INTERNATIONAL	2000-05-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MGN	Megan Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGNI	Magnite, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGNX	MacroGenics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGPI	MGP Ingredients Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGRC	Mcgrath Rentcorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGRT	Mega Fortune Company Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGRX	Mangoceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGTX	MeiraGTx Holdings plc Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGX	Metagenomi Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MGY	Magnolia Oil & Gas Corporation Class A Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MGYR	Magyar Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MH	McGraw Hill, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MHH	Mastech Digital, Inc.	1986-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MHK	Mohawk Industries, Inc.	1878-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MHO	M/I Homes, Inc.	1976-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MI	NFT Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MIAX	Miami International Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MICC	The Magnum Ice Cream Company N.V.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MIDD	Middleby Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MIGI	Mawson Infrastructure Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MIMI	Mint Incorporation Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MIND	MIND Technology, Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MINE	Mayfair Gold Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MIR	Mirion Technologies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MIRA	MIRA Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MIRM	Mirum Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MIST	Milestone Pharmaceuticals Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MITK	Mitek Systems, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MITQ	Moving iMage Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MITT	TPG Mortgage Investment Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MKC	McCormick & Company, Incorporated Non-VTG CS	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MKC.V	McCormick & Company, Incorporated Voting CS	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MKDW	MKDWELL Tech Inc. Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MKL	Markel Group Inc.	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MKLY	McKinley Acquisition Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MKSI	MKS Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MKTW	MarketWise, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MKTX	MarketAxess Holdings Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MKZR	MacKenzie Realty Capital, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLAA	Mountain Lake Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLAB	Mesa Laboratories Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLAC	Mountain Lake Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLCI	Mount Logan Capital Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLCIL	Mount Logan Capital Inc. 8.00% Notes Due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLEC	Moolec Science SA Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLGO	MicroAlgo, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLI	Mueller Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MLKN	MillerKnoll, Inc. Common Stock	1938-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLM	Martin Marietta Materials	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MLP	Maui Land & Pineapple Co.	1903-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MLR	Miller Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MLSS	Milestone Scientific, Inc. Common Stock	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MLTX	MoonLake Immunotherapeutics Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MLYS	Mineralys Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MMA	Mixed Martial Arts Group Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MMED	MiniMed Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MMI	MARCUS & MILLICHAP	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MMLP	Martin Midstream Partners LP	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MMM	3M Company	1902-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MMS	MAXIMUS, Inc.	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MMSI	Merit Medical Systems Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MMTX	Miluna Acquisition Corp Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MMYT	MakeMyTrip Limited	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNDO	Mind CTI Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNDR	Mobile-health Network Solutions Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNDY	monday.com Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNKD	Mannkind Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNOV	Medicinova, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNPR	Monopar Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNRO	Monro, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNSB	MainStreet Bancshares, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNSBP	MainStreet Bancshares, Inc. Depositary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNST	Monster Beverage Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNTK	Montauk Renewables, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNTN	MNTN, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MNTS	Momentus Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MNY	MoneyHero Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MO	Altria Group, Inc.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MOBX	Mobix Labs, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MOD	Modine Manufacturing Co	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MODD	Modular Medical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MOG.A	Moog Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MOG.B	MOOG INC CL B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MOH	Molina Healthcare, Inc.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MORN	Morningstar, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MOS	The Mosaic Company	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MOV	Movado Group, Inc.	1881-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MOVE	Corvex, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MP	MP Materials Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPAA	Motorcar Parts of America, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MPB	Mid Penn Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MPC	MARATHON PETROLEUM CORPORATION	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPLT	MapLight Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MPLX	MPLX LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPT	Medical Properties Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPTI	M-tron Industries, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPU	Mega Matrix Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MPWR	Monolithic Power Systems, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MPX	Marine Products Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MQ	Marqeta, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRAM	Everspin Technologies, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRBK	Meridian Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRCY	Mercury Systems Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRDN	Meridian Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRK	Merck & Co., Inc.	1891-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MRKR	Marker Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRLN	Merlin, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRNA	Moderna, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRNO	Murano Global Investments PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRP	Millrose Properties, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MRSH	Marsh	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MRT	Marti Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MRTN	Marten Transport Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRVI	Maravai LifeSciences Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRVL	Marvell Technology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MRX	Marex Group plc Ordinary Shares	1988-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MS	Morgan Stanley	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSA	Mine Safety Incorporated	1914-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSAI	MultiSensor AI Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSB	Mesabi Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSBI	Midland States Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSCI	MSCI, Inc.	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSDL	Morgan Stanley Direct Lending Fund	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSEX	Middlesex Water Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSFT	Microsoft Corporation	1986-03-13	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	1975-04-04	manual	Listed on NASDAQ at market open
MSGE	Madison Square Garden Entertainment Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSGM	Motorsport Games Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSGS	Madison Square Garden Sports Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSGY	Masonglory Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSI	Motorola Solutions, Inc. New	2011-01-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSIF	MSC Income Fund, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSLE	Satellos Bioscience Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSM	MSC Industrial Direct Co., Inc. Class A	1941-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSN	Emerson Radio Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MSS	Maison Solutions Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSTR	Strategy Inc Common Stock Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MSW	Ming Shing Group Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MT	ArcelorMittal	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTAL	Metals Acquisition Corp. II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTB	M&T Bank Corp.	1856-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTC	MMTec, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTCH	Match Group, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTD	Mettler-Toledo International	1991-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTDR	MATADOR RESOURCES COMPANY	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTEK	Maris-Tech Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTEN	Mingteng International Corporation Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTEX	Mannatech Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTG	MGIC Investment Corp.	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTH	Meritage Homes Corporation	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTN	Vail Resorts, Inc.	1962-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTNB	Matinas BioPharma Holdings, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTR	Mesa Royalty Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTRN	Materion Corporation	1931-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTRX	Matrix Service Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTSI	MACOM Technology Solutions Holdings, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTUS	Metallus Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTVA	MetaVia Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MTW	The Manitowoc Company, Inc.	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTX	Minerals Technologies Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MTZ	MasTec, Inc.	1994-03-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MU	Micron Technology, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MUR	Murphy Oil Corp.	1964-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MUSA	MURPHY USA INC.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MUX	McEwen Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MUZE	Muzero Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MVBF	MVB Financial Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MVIS	Microvision Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MVO	MV Oil Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MVST	Microvast Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MWA	Mueller Water Products, Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MWG	Multi Ways Holdings Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MWH	SOLV Energy, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MWYN	Marwynn Holdings, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MX	Magnachip Semiconductor Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MXC	Mexco Energy Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MXCT	MaxCyte, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MXL	MaxLinear, Inc. Common Stock	2004-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYE	Myers Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
MYFW	First Western Financial, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYGN	Myriad Genetics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYO	Myomo Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
MYPS	PLAYSTUDIOS, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYRG	MYR Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYSE	Myseum.AI, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MYSZ	My Size, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
MZTI	The Marzetti Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NA	Nano Labs Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NABL	N-able, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NAGE	Niagen Bioscience, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAII	Natural Alternatives International Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAK	Northern Dynasty Minerals, Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NAKA	Nakamoto Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAMM	Namib Minerals Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAMS	NewAmsterdam Pharma Company N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAT	Nordic American Tanker	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NATH	Nathan's Famous Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NATL	NCR Atleos Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NATR	Nature's Sunshine Products Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAUT	Nautilus Biotechnolgy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAVI	Navient Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NAVN	Navan, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NB	NioCorp Developments Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBBK	NB Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBHC	NATIONAL BANK HOLDINGS CORP.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NBIS	Nebius Group N.V. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBIX	Neurocrine Biosciences Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBN	Northeast Bank Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBR	Nabors Industries Ltd.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NBRG	Newbridge Acquisition Limited Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NBTB	NBT Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NC	NACCO Industries, Inc.	1913-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NCDL	Nuveen Churchill Direct Lending Corp	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NCEL	NewcelX Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCEW	New Century Logistics (BVI) Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCI	Neo-Concept International Group Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCL	Northann Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NCLH	Norwegian Cruise Line Holdings Ltd. Ordinary Shares	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NCMI	National CineMedia, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCNO	nCino, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCPL	Netcapital Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCRA	Nocera, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCSM	NCS Multistage Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NCT	Intercont (Cayman) Limited Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NDAQ	Nasdaq, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NDLS	Noodles & Company Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NDRA	ENDRA Life Sciences Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NDSN	Nordson Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NE	Noble Corporation plc	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NECB	Northeast Community Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEE	NextEra Energy, Inc.	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEGG	Newegg Commerce, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEM	Newmont Corporation	1916-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEN	New England Realty Associates Limited Partnership	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEO	NeoGenomics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEOG	Neogen Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEON	Neonode Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEOV	NeoVolta Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEPH	Nephros Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NERV	Minerva Neurosciences, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NESR	National Energy Services Reunited Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NET	Cloudflare, Inc. Class A common stock, par value $0.001 per share	2009-11-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEU	NewMarket Corporation	1887-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEWP	New Pacific Metals Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEWT	NewtekOne, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEWTG	NewtekOne, Inc. 8.50% Fixed Rate Senior Notes due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEWTH	NewtekOne, Inc. 8.625% Fixed Rate Senior Notes due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEWTI	NewtekOne, Inc. 8.00% Fixed Rate Senior Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEWTO	NewtekOne, Inc. 8.50% Fixed Rate Senior Notes due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEXA	Nexa Resources S.A. Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NEXM	NexMetals Mining Corp. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEXN	Nexxen International Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEXR	Nexera Technologies Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NEXT	NextDecade Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NFBK	Northfield Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NFE	New Fortress Energy Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NFG	National Fuel Gas Co.	1902-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NFGC	New Found Gold Corp	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NFLX	NetFlix Inc	1997-08-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NG	NovaGold Resources Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NGEN	NervGen Pharma Corp. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NGL	NGL ENERGY PARTNERS LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NGNE	Neurogene, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NGS	Natural Gas Services Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NGVC	NATURAL GROCERS BY VITAMIN COTTAGE, INC	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NGVT	Ingevity Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NHC	National Healthcare Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NHI	National Health Investors	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NHIC	NewHold Investment Corp III Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NHTC	Natural Health Trends Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NI	NiSource Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NIC	Nicolet Bankshares,Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NINE	Nine Energy Service, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NIQ	NIQ Global Intelligence plc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NIVF	NewGenIvf Group Limited Class A ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NIXX	Nixxy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NJR	New Jersey Resources Corp	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NKE	Nike, Inc.	1964-01-25	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NKLR	Terra Innovatum Global N.V. Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NKSH	National Bankshares Inc/VA	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NKTR	Nektar Therapeutics	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NKTX	Nkarta, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NL	NL Industries, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NLOP	Net Lease Office Properties	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NLY	Annaly Capital Management. Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NMAX	Newsmax, Inc.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NMFC	NEW MOUNTAIN FINANCE CORPORATION	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMFCZ	New Mountain Finance Corporation 8.250% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMG	Nouveau Monde Graphite Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NMIH	NMI Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMM	Navios Maritime Partners L.P.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NMP	NMP Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMRA	Neumora Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMRK	Newmark Group, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NMTC	NeuroOne Medical Technologies Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NN	NextNav Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NNBR	NN Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NNE	Nano Nuclear Energy Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NNI	Nelnet, Inc. Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NNN	NNN REIT, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NNNN	Anbio Biotechnology Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NNOX	NANO-X IMAGING LTD Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NNVC	NanoViricides Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NOA	North American Construction Group Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NOC	Northrop Grumman Corp.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NODK	NI Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NOEM	CO2 Energy Transition Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NOG	Northern Oil and Gas, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NOMA	NOMADAR Corp. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NOMD	Nomad Foods Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NOTV	Inotiv, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NOV	NOV Inc.	1862-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NOVT	Novanta Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NOW	SERVICENOW, INC.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NP	Neptune Insurance Holdings Inc.	1873-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NPAC	New Providence Acquisition Corp. III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NPB	Northpointe Bancshares, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NPCE	Neuropace, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NPK	National Presto Industries, Inc.	1905-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NPKI	NPK International Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NPO	Enpro Inc.	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NPT	Texxon Holding Limited Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NPWR	NET Power Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRC	National Research Corporation Common Stock (Delaware)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRDS	NerdWallet, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRDY	Nerdy Inc.	2021-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NREF	NexPoint Real Estate Finance, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRG	NRG Energy, Inc.	1905-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRGV	Energy Vault Holdings, Inc.	2018-08-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRIM	Northrim BanCorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRIX	Nurix Therapeutics, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRP	Natural Resource Partners L.P.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRSN	NeuroSense Therapeutics Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRT	North European Oil Royalty Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NRXP	NRX Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NRXS	Neuraxis, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NSA	National Storage Affiliates Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NSC	Norfolk Southern Corp.	1894-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NSIT	Insight Enterprises Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NSP	Insperity, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NSPR	InspireMD, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NSRX	Nasus Pharma Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NSSC	Napco Security Technologies, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NSTS	NSTS Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NSYS	Nortech Systems Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTAP	NetApp, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTB	The Bank of N.T. Butterfield & Son Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NTCL	NETCLASS TECHNOLOGY INC Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTCT	Netscout Systems Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTGR	NETGEAR, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTHI	NeOnc Technologies Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTIC	Northern Technologies International Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTIP	Network-1 Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
NTLA	Intellia Therapeutics, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTNX	Nutanix, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTR	Nutrien Ltd. Common Shares	2018-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NTRA	Natera, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTRB	Nutriband Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTRP	NextTrip, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTRS	Northern Trust Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTSK	Netskope, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTST	NetSTREIT Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NTWK	NetSol Technologies, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NTWO	Newbury Street II Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NU	Nu Holdings Ltd.	2013-05-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NUAI	New Era Energy & Digital, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NUCL	Eagle Nuclear Energy Corp. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NUE	Nucor Corporation	1955-09-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NUS	NuSkin Enterprises, Inc.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NUTR	Nusatrip Incorporated Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NUTX	Nutex Health Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NUVB	Nuvation Bio Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NUVL	Nuvalent, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NUWE	Nuwellis, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVAX	Novavax Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVCR	NovoCure Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVCT	Nuvectis Pharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVDA	NVIDIA Corporation	1999-01-22	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	1993-04-05	manual	IPO price $12, raised $42M
NVEC	NVE Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVGS	NAVIGATOR HOLDINGS LTD.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NVMI	Nova Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVNI	Nvni Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVNO	enVVeno Medical Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVR	NVR, Inc.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NVRI	Enviri Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NVST	Envista Holdings Corporation Common stock, $0.01 par value per share	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NVT	nVent Electric plc Ordinary Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NVTS	Navitas Semiconductor Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NVVE	Nuvve Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWAX	New America Acquisition I Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NWBI	Northwest Bancshares, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWE	NorthWestern Energy Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWFL	Norwood Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWL	Newell Brands Inc.	1903-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWN	Northwest Natural Holding Company	1859-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NWPX	NWPX Infrastructure, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWS	News Corporation Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWSA	News Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NWTG	Newton Golf Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NX	Quanex Building Products Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NXDR	Nextdoor Holdings, Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NXE	NexGen Energy Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NXGL	NexGel, Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXL	Nexalin Technology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXPI	NXP Semiconductors N.V.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXPL	NextPlat Corp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXRT	NexPoint Residential Trust Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NXST	Nexstar Media Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXT	Nextpower Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXTC	NextCure, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXTS	Nexentis Technologies Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXTT	Next Technology Holding Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NXXT	NextNRG, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NYAX	Nayax Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
NYC	American Strategic Investment Co.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NYT	New York Times Co.	1851-09-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
NYXH	Nyxoah SA Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
O	Realty Income Corporation	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OABI	OmniAb, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OACC	Oaktree Acquisition Corp. III Life Sciences Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OBA	Oxley Bridge Acquisition Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OBAI	Our Bond, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OBDC	Blue Owl Capital Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OBE	Obsidian Energy Ltd.	2005-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OBIO	Orchestra BioMed Holdings, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OBK	Origin Bancorp, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OBT	Orange County Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OBTC	Osprey Bitcoin Trust Common Units of Beneficial Interest	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OC	Owens Corning	1938-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OCC	Optical Cable Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCCI	OFS Credit Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCCIN	OFS Credit Company, Inc. 5.25% Series E Term Preferred Stock Due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCFC	OceanFirst Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCG	Oriental Culture Holding LTD Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCGN	Ocugen, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCS	Oculis Holding AG Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCSL	Oaktree Specialty Lending Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OCUL	Ocular Therapeutix, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ODC	Oil-Dri Corporation of America	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ODD	ODDITY Tech Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ODFL	Old Dominion Freight Line	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ODV	Osisko Development Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ODYS	Odysight.ai Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OEC	Orion S.A.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OESX	Orion Energy Systems, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFAL	OFA Group Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFG	OFG BANCORP	1964-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OFIX	Orthofix Medical Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFLX	Omega Flex, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFRM	Once Upon a Farm, PBC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OFS	OFS Capital Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFSSH	OFS Capital Corporation 4.95% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OFSSO	OFS Capital Corporation 7.50% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OGC	OceanaGold Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OGE	OGE Energy Corp.	1902-02-27	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OGEN	Oragenics Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OGI	Organigram Global Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OGN	Organon & Co.	1923-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OGS	ONE GAS, INC.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OHI	Omega Healthcare Investors Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OI	O-I Glass, Inc.	1903-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OII	Oceaneering International Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OIM	OneIM Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OIMAU	OneIM Acquisition Corp. Units	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OIO	OIO Group Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OIS	OIL STATES INTERNATIONAL, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OKE	Oneok, Inc.	1906-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OKLO	Oklo Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OKTA	Okta, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OKUR	OnKure Therapeutics, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OKYO	OKYO Pharma Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLB	The OLB Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLED	Universal Display Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLLI	Ollie's Bargain Outlet Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLMA	Olema Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLN	Olin Corp.	1892-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OLOX	Olenox Industries Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OLP	One Liberty Properties, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OLPX	Olaplex Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OM	Outset Medical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMC	Omnicom Group Inc.	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OMCL	Omnicell Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMDA	Omada Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMER	Omeros Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMEX	Odyssey Marine Exploration, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMF	OneMain Holdings, Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OMH	Ohmyhome Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OMSE	OMS Energy Technologies Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ON	ON Semiconductor Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONB	Old National Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONBPO	Old National Bancorp Depositary Shares, Each Representing a 1/40th Interest in a Share of Series C Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONCH	1RT Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONCO	Onconetix, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONCY	Oncolytics Biotech, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONDS	Ondas Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONEG	OneConstruction Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONEW	OneWater Marine Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONFO	Onfolio Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONIT	Onity Group Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ONL	Orion Properties Inc.	2021-11-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ONMD	OneMedNet Corp Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ONON	On Holding AG	2012-09-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ONTO	Onto Innovation Inc.	1940-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OOMA	Ooma, Inc. Common Stock	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPAD	Offerpad Solutions Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPAL	OPAL Fuels Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPBK	OP Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPCH	Option Care Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPEN	Opendoor Technologies Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPFI	OppFi Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPHC	OptimumBank Holdings, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPK	Opko Health Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPLN	OPENLANE, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPRT	Oportun Financial Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPRX	OptimizeRx Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPTT	Ocean Power Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPTU	Optimum Communications, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OPTX	Syntec Optics Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPXS	Optex Systems Holdings, INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OPY	Oppenheimer Holdings, Inc.	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OR	OR Royalties Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORA	Ormat Technologies, Inc.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORBS	Eightco Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORC	Orchid Island Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORCL	Oracle Corp	1977-06-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORGN	Origin Materials, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORGO	Organogenesis Holdings Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORI	Old Republic International Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORIC	Oric Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORIO	Orion Digital Corp. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORIQ	Origin Investment Corp I Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORIS	Oriental Rise Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORKA	Oruka Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORKT	Orangekloud Technology Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORLA	Orla Mining Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORLY	O'Reilly Automotive, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORMP	Oramed Pharmaceuticals Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ORN	Orion Group Holdings, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ORRF	Orrstown Financial Services Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSBC	Old Second Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSCR	Oscar Health, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OSG	Octave Specialty Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OSIS	OSI Systems Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSK	Oshkosh Corp.	1917-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OSPN	OneSpan Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSRH	OSR Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSS	One Stop Systems, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OST	Ostin Technology Group Co., Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSTX	OS Therapies Incorporated	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OSUR	OraSure Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OSW	OneSpaWorld Holdings Limited Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OTEX	Open Text Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OTF	Blue Owl Technology Finance Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OTGA	OTG Acquisition Corp. I Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OTH	Off The Hook YS Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
OTIS	Otis Worldwide Corporation	1853-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OTLK	Outlook Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OTTR	Otter Tail Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OUST	Ouster, Inc.	2015-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OUT	OUTFRONT Media Inc.	1938-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OVBC	Ohio Valley Banc Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OVID	Ovid Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OVLY	Oak Valley Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OVV	Ovintiv Inc.	2020-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OWL	Blue Owl Capital Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OWLS	OBOOK Holdings Inc. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OWLT	Owlet, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OXBR	Oxbridge Re Holdings Limited	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLC	Oxford Lane Capital Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLCG	Oxford Lane Capital Corp. 7.95% Notes due 2032	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLCI	Oxford Lane Capital Corp. 8.75% Notes due 2030	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLCL	Oxford Lane Capital Corp. 6.75% Notes due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLCO	Oxford Lane Capital Corp. Preferred Stock Shares, 6.00% Series 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXLCZ	Oxford Lane Capital Corp. 5.00% Notes due 2027	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXM	Oxford Industries, Inc.	1942-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OXSQ	Oxford Square Capital Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXSQG	Oxford Square Capital Corp. 5.50% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXSQH	Oxford Square Capital Corp. 7.75% Notes due 2030	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OXY	Occidental Petroleum Corporation	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
OYSE	Oyster Enterprises II Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
OZK	Bank OZK	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
P	Everpure, Inc.	2000-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAA	Plains All American Pipeline, L.P. Common Units representing Limited Partner Interests	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAAC	Proem Acquisition Corp I Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAAS	Pan American Silver Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PACB	Pacific Biosciences of California, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PACH	Pioneer Acquisition I Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PACK	Ranpak Holdings Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PACS	PACS Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAG	Penske Automotive Group, Inc.	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAGP	Plains GP Holdings, L.P. Class A Units representing Limited Partner Interests	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAGS	PagSeguro Digital Ltd.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAHC	Phibro Animal Health Corporation Class A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAII	Pyrophyte Acquisition Corp. II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAL	Proficient Auto Logistics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PALI	Palisade Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PALO	Paloma Acquisition Corp I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAMT	PAMT CORP Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PANL	Pangaea Logistics Solutions Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PANW	Palo Alto Networks, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAPL	Pineapple Financial Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAR	PAR Technology Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PARK	Park Dental Partners, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PARR	Par Pacific Holdings, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PASG	Passage Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PASW	Ping An Biomedical Co., Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PATH	UiPath, Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PATK	Patrick Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAVM	PAVmed Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAVS	Paranovus Entertainment Technology Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAX	Patria Investments Limited Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAY	Paymentus Holdings, Inc.	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAYC	PAYCOM SOFTWARE, INC.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PAYO	Payoneer Global Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAYS	Paysign, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PAYX	Paychex Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PB	Prosperity Bancshares Inc	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBA	PEMBINA PIPELINE CORPORATION	1954-09-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBF	PBF ENERGY INC.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBFS	Pioneer Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PBH	Prestige Consumer Healthcare Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBHC	Pathfinder Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PBI	Pitney Bowes Inc.	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBM	Psyence Biomedical Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PBT	Permian Basin Royalty Trust	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PBYI	PUMA BIOTECHNOLOGY INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PC	Premium Catering (Holdings) Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCAP	ProCap Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCAR	Paccar Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCB	PCB Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCG	PG&E Corporation	1905-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PCOR	Procore Technologies, Inc.	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PCRX	Pacira BioSciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCSA	Processa Pharmaceuticals, Inc. Common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCSC	Perceptive Capital Solutions Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCT	PureCycle Technologies, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCTY	Paylocity Holding Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCVX	Vaxcyte, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PCYO	Pure Cycle Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PD	PagerDuty, Inc.	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PDCC	Pearl Diver Credit Company Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PDEX	Pro-Dex Inc New	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PDFS	PDF Solutions Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PDLB	Ponce Financial Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PDM	Piedmont Realty Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PDS	Precision Drilling Corporation	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PDSB	PDS Biotechnology Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PDYN	Palladyne AI Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEB	Pebblebrook Hotel Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PEBK	Peoples Bancorp of North Carol	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEBO	Peoples Bancorp Inc/OH	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PECO	Phillips Edison & Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PED	PEDEVCO Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PEG	Public Service Enterprise Group Incorporated	1903-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PEGA	Pegasystems Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEN	Penumbra, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PENG	Penguin Solutions, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PENN	PENN Entertainment, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEP	PepsiCo, Inc.	1961-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEPG	PepGen Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PERF	Perfect Corp.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PERI	Perion Network Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PESI	Perma-Fix Environmental Services, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PETS	PetMed Express, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PETZ	TDH Holdings, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PEW	GrabAGun Digital Holdings Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFAI	Pinnacle Food Group Limited Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFBC	Preferred Bank	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFE	Pfizer Inc.	1849-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFG	Principal Financial Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFGC	Performance Food Group Company	1885-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFIS	Peoples Financial Services Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFLT	PennantPark Floating Rate Capital Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFS	Provident Financial Services, Inc.	1839-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFSA	Profusa, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFSI	PennyMac Financial Services, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PFX	PhenixFIN Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PFXNZ	PhenixFIN Corporation 5.25% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PG	Procter & Gamble Company	1837-10-31	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PGAC	Pantages Capital Acquisition Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PGC	Peapack-Gladstone Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PGEN	Precigen, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PGNY	Progyny, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PGR	Progressive Corporation	1937-03-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PGY	Pagaya Technologies Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PH	Parker-Hannifin Corporation	1918-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHAR	Pharming Group N.V. ADS, each representing 10 ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PHAT	Phathom Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PHG	KONINKLIJKE PHILIPS  N.V.	1891-05-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHGE	BiomX Inc.	2005-01-01	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHIN	PHINIA Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHIO	Phio Pharmaceuticals Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PHM	Pultegroup, Inc.	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHOE	Phoenix Asia Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PHR	Phreesia, Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PHUN	Phunware, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PHVS	Pharvaris N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PI	Impinj, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PICS	PicS N.V. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PII	Polaris Inc.	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PIII	P3 Health Partners Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PINE	Alpine Income Property Trust, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PINS	Pinterest, Inc. Class A Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PIPR	Piper Sandler Companies	1895-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PJT	PJT Partners Inc.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PK	Park Hotels & Resorts Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PKBK	Parke Bancorp Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PKE	Park Aerospace Corp. Common Stock	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PKG	Packaging Corp of America	1959-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PKOH	Park-Ohio Holdings Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PKST	Peakstone Realty Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PL	Planet Labs PBC	2010-12-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLAB	Photronics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLAG	Planet Green Holdings Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLAY	Dave & Buster's Entertainment, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLBC	Plumas Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLBL	Polibeli Group Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLBY	Playboy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLCE	Children's Place, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLD	PROLOGIS, INC.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLG	Platinum Group Metals LTD.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLMK	Plum Acquisition Corp. IV Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLMR	Palomar Holdings, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLNT	Planet Fitness, Inc.	1992-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLOW	DOUGLAS DYNAMICS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLPC	Preformed Line Products Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLRX	Pliant Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLRZ	Polyrizon Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLSE	Pulse Biosciences, Inc Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLSM	Pulsenmore Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLTK	Playtika Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLTR	Palantir Technologies Inc. Class A Common Stock	2003-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLTS	Platinum Analytics Cayman Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLUG	Plug Power Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLUR	Pluri Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLUS	ePlus Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLUT	Plutus Financial Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLX	Protalix BioTherapeutics, Inc. Common Stock	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PLXS	Plexus Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PLYX	Polaryx Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PM	Philip Morris International Inc.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PMAX	Powell Max Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMCB	PharmaCyte Biotech, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMEC	Primech Holdings Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMI	Picard Medical, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PMN	ProMIS Neurosciences Inc. Common Shares (ON)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMNT	Perfect Moment Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PMT	PennyMac Mortgage Investment Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PMTR	Perimeter Acquisition Corp. I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMTS	CPI Card Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PMVP	PMV Pharmaceuticals, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PN	Skycorp Solar Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PNBK	Patriot National Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PNC	PNC Financial Services Group	1845-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PNFP	Pinnacle Financial Partners, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PNNT	Pennant Investment Corp	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PNR	Pentair plc	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PNRG	PrimeEnergy Resources Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PNTG	The Pennant Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PNW	Pinnacle West Capital Corporation	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
POAS	Phaos Technology Holdings (Cayman) Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
POCI	Precision Optics Corporation, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PODC	PodcastOne, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PODD	Insulet Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POET	POET Technologies Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POLA	Polar Power, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POLE	Andretti Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POOL	Pool Corporation	1993-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POR	Portland General Electric Company	1888-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
POST	POST HOLDINGS, INC.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
POWI	Power Integrations Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POWL	Powell Industries Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
POWW	Outdoor Holding Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPC	Pilgrims Pride Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPCB	Propanc Biopharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPG	PPG Industries, Inc.	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PPHC	Public Policy Holding Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPIH	Perma-Pipe International Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPL	PPL Corporation	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PPSI	Pioneer Power Solutions, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PPTA	Perpetua Resources Corp. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PR	Permian Resources Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRA	ProAssurance Corporation	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRAA	PRA Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRAX	Praxis Precision Medicines, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRCH	Porch Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRCT	PROCEPT BioRobotics Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRDO	Perdoceo Education Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRE	Prenetics Global Limited Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRFX	PRF Technologies Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRG	PROG Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRGO	PERRIGO COMPANY PLC	1887-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRGS	Progress Software Corp (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRHI	Presurance Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRHIZ	Presurance Holdings, Inc. 9.75% Senior Unsecured Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRI	PRIMERICA, INC.	1977-02-10	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRIM	Primoris Services Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRK	Park National Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRKS	United Parks & Resorts Inc.	1960-03-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRLB	PROTO LABS, INC.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRLD	Prelude Therapeutics Incorporated	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRM	Perimeter Solutions, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRMB	Primo Brands Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRME	Prime Medicine, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PROF	Profound Medical Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PROK	ProKidney Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PROP	Prairie Operating Co. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PROV	Provident Financial Hldgs	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRPL	Purple Innovation, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRPO	Precipio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRQR	ProQR Therapeutics N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRSO	Peraso, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRSU	Pursuit Attractions and Hospitality, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRTA	Prothena Corporation plc Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRTH	Priority Technology Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRTS	CarParts.com, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRU	Prudential Financial, Inc.	1875-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PRVA	Privia Health Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PRZO	ParaZero Technologies Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSA	Public Storage	1972-08-14	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSBD	Palmer Square Capital BDC Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSEC	Prospect Capital Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSFE	Paysafe Limited	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSHG	Performance Shipping Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSIG	PS International Group Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSIX	Power Solutions International, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSKY	Paramount Skydance Corporation Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSMT	Pricesmart Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSN	Parsons Corporation	1944-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSNL	Personalis, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSNY	Polestar Automotive Holding UK PLC Class A ADS	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSQH	PSQ Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSTL	Postal Realty Trust, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PSTV	PLUS THERAPEUTICS, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PSX	PHILLIPS 66	2012-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PTC	PTC, INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTCT	PTC Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTEN	Patterson-UTI Energy Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTGX	Protagonist Therapeutics, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTHS	Pelthos Therapeutics Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PTLE	PTL LTD Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTLO	Portillo's Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTN	Palatin Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PTNM	Pitanium Limited Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTON	Peloton Interactive, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTOR	Praetorian Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PTRN	Pattern Group Inc. Series A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PUBM	PubMatic, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PULM	Pulmatrix, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PUMP	ProPetro Holding Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PURR	Hyperliquid Strategies Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PVH	PVH Corp.	1881-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PVL	Permianville Royalty Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PVLA	Palvella Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PW	Power REIT	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PWP	Perella Weinberg Partners Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PWR	Quanta Services, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PXED	Phoenix Education Partners, Inc.	1976-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
PXLW	Pixelworks Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PXS	Pyxis Tankers Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PYPD	PolyPid Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PYPL	PayPal Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PYXS	Pyxis Oncology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
PZG	Paramount Gold Nevada Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
PZZA	Papa John's International Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
Q	Qnity Electronics, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QBTS	D-Wave Quantum Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QCLS	Q/C Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QCOM	Qualcomm Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QCRH	QCR Holdings Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QDEL	QuidelOrtho Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QETA	Quetta Acquisition Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QGEN	QIAGEN N.V.	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QLYS	Qualys, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QMCO	Quantum Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QMMM	QMMM Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QNC	Quantum eMotion Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
QNCX	Quince Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QNST	QuinStreet, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QNTM	Quantum Biopharma Ltd. Class B Subordinate Voting Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QRHC	Quest Resource Holding Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QRVO	Qorvo, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QS	QuantumScape Corporation Class A Common Stock	2010-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QSEA	Quartzsea Acquisition Corporation Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QSI	Quantum-Si Incorporated Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QSR	Restaurant Brands International Inc.	2014-12-15	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QTI	QT Imaging Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QTRX	Quanterix Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QTTB	Q32 Bio Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QTWO	Q2 Holdings Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QUAD	QUAD/GRAPHICS, INC.	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
QUBT	Quantum Computing Inc. Common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QUCY	Mainz Biomed N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QUIK	QuickLogic Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QUMS	Quantumsphere Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QURE	uniQure N.V.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QVCGA	QVC Group, Inc. Series A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
QXO	QXO, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
R	Ryder System, Inc.	1933-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RAAQ	Real Asset Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAC	Rithm Acquisition Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RACE	Ferrari N.V.	2013-05-24	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RAIL	FreightCar America, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAIN	Rain Enhancement Technologies Holdco, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAL	Ralliant Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RAMP	LiveRamp Holdings, Inc. Common Stock	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RAND	Rand Capital Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RANG	Range Capital Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RANI	Rani Therapeutics Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAPP	Rapport Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RARE	Ultragenyx Pharmaceutical Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAVE	Rave Restaurant Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAY	Raytech Holding Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RAYA	Erayak Power Solution Group Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBA	RB Global, Inc.	1958-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RBB	RBB Bancorp Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBBN	Ribbon Communications Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBC	RBC Bearings Incorporated	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RBCAA	Republic Bancorp Inc/KY	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBKB	Rhinebeck Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBLX	Roblox Corporation	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RBNE	Robin Energy Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RBRK	Rubrik, Inc.	2013-12-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RC	Ready Capital Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RCAT	Red Cat Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCI	Rogers Communications, Inc.	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RCKT	Rocket Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCKY	Rocky Brands, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCL	Royal Caribbean Group	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RCMT	RCM Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCON	Recon Technology, Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCT	RedCloud Holdings plc Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RCUS	Arcus Biosciences, Inc.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RDAC	Rising Dragon Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDAG	Republic Digital Acquisition Company Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDCM	Radcom Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDDT	Reddit, Inc.	2005-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RDGT	Ridgetech, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDI	Reading International, Inc Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDIB	Reading International, Inc (Class B	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDN	Radian Group Inc.	1977-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RDNT	RadNet, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDNW	RideNow Group, Inc. Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDVT	Red Violet, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDW	Redwire Corporation	2020-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RDWR	Radware Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RDZN	Roadzen, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REAL	The RealReal, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REAX	REAL BROKERAGE INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REBN	Reborn Coffee, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RECT	Rectitude Holdings Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REE	REE Automotive Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REED	Reed's, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
REFI	Chicago Atlantic Real Estate Finance, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REFR	Research Frontiers Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REG	Regency Centers Corporation	1963-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REGN	Regeneron Pharmaceuticals Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REI	Ring Energy Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
REKR	Rekor Systems, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RELL	Richardson Electronics Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RELY	Remitly Global, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RENT	Rent the Runway, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RENX	RenX Enterprises Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REPL	Replimune Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REPX	Riley Exploration Permian, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
RES	RPC, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RETO	ReTo Eco-Solutions, Inc. Class A Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REVB	Revelation Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REX	REX American Resources Corp.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
REXR	REXFORD INDUSTRIAL REALTY, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
REYN	Reynolds Consumer Products Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
REZI	Resideo Technologies, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RF	Regions Financial Corp.	1971-07-13	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RFAI	RF Acquisition Corp II Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RFAM	RF Acquisition Corp III Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RFIL	RF Industries Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RFL	Rafael Holdings, Inc. Class B Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RGA	Reinsurance Group of America, Incorporated	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RGC	Regencell Bioscience Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGCO	RGC Resources Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGEN	Repligen Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGLD	Royal Gold Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGNT	Regentis Biomaterials Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
RGNX	REGENXBIO Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGP	Resources Connection	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGR	Sturm, Ruger & Company, Inc.	1949-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RGS	Regis Corporation	1922-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RGTI	Rigetti Computing, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RH	RH	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RHI	Robert Half Inc.	1948-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RHLD	Resolute Holdings Management, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RHP	Ryman Hospitality Properties, Inc	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RIBB	Ribbon Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RICK	RCI Hospitality Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RIG	Transocean LTD.	1973-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RIGL	Rigel Pharmaceuticals Inc. (New)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILY	BRC Group Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILYG	BRC Group Holdings, Inc. 5.00% Senior Notes due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILYL	BRC Group Holdings, Inc. Depositary Shares each representing 1/1000th in a share of 7.375% Series B Cumulative Perpetual Preferred Stock, par value $0.0001	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILYN	BRC Group Holdings, Inc. 6.50% Senior Notes Due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILYT	BRC Group Holdings, Inc. 6.00% Senior Notes Due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RILYZ	BRC Group Holdings, Inc. 5.25% Senior Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RIME	Algorhythm Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RIOT	Riot Platforms, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RITM	Rithm Capital Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RITR	Reitar Logtech Holdings Limited Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RIVN	Rivian Automotive, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RJET	Republic Airways Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RJF	Raymond James Financial, Inc.	1962-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RKDA	Arcadia Biosciences, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RKLB	Rocket Lab Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RKT	Rocket Companies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RL	Ralph Lauren Corporation	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RLAY	Relay Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RLGT	Radiant Logistics, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
RLI	RLI Corp.	1965-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RLJ	RLJ Lodging Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RLMD	Relmada Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RLYB	Rallybio Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RM	REGIONAL MANAGEMENT CORP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RMAX	RE/MAX HOLDINGS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RMBI	Richmond Mutual Bancorporation, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMBS	Rambus Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMCF	Rocky Mountain Chocolate Factory, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMCO	Royalty Management Holding Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMD	ResMed Inc.	1989-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RMIX	Suncrete, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMNI	Rimini Street, Inc. (DE) Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMR	The RMR Group Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMSG	Real Messenger Corporation Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RMTI	Rockwell Medical, Inc. (DE) Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNA	Atrium Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNAC	Cartesian Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNAZ	TransCode Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNG	RINGCENTRAL, INC.	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RNGR	Ranger Energy Services, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RNGT	Range Capital Acquisition Corp II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNR	RenaissanceRe Holdings Ltd.	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RNST	Renasant Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RNTX	Rein Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNW	ReNew Energy Global plc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RNXT	RenovoRx, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROAD	Construction Partners, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROC	Rank One Computing Corporation Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROCK	Gibraltar Industries, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROG	Rogers Corporation	1832-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ROIV	Roivant Sciences Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROK	Rockwell Automation, Inc.	1903-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ROKU	Roku, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROL	Rollins, Inc.	1948-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ROLR	High Roller Technologies, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ROMA	Roma Green Finance Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROOT	Root, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROP	Roper Technologies, Inc. Common Stock	1808-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ROST	Ross Stores Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPAY	Repay Holdings Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPC	Ridgepost Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RPD	Rapid7, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPGL	Republic Power Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPID	Rapid Micro Biosystems, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPM	RPM International, Inc.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RPRX	Royalty Pharma plc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RPT	Rithm Property Trust Inc.	1950-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RR	Richtech Robotics Inc. Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RRBI	Red River Bancshares, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RRC	Range Resources Corp	1976-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RRGB	Red Robin Gourmet Burgers Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RRR	Red Rock Resorts, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RRX	Regal Rexnord Corporation	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RS	Reliance, Inc.	1939-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RSG	Republic Services Inc.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RSI	Rush Street Interactive, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RSKD	Riskified Ltd.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RSSS	RESEARCH SOLUTIONS INC	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RSVR	Reservoir Media, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RTAC	Renatus Tactical Acquisition Corp I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RTX	RTX Corporation	2020-04-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RUBI	Rubico Inc. Common Stock	2007-05-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RUM	Rumble Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RUN	Sunrun Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RUSHA	Rush Enterprises Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RUSHB	Rush Enterprises Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RVLV	Revolve Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RVMD	Revolution Medicines, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RVP	Retractable Technologies, Inc	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
RVPH	Reviva Pharmaceuticals Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RVSB	Riverview Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RVSN	Rail Vision Ltd. Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RVTY	Revvity, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RVYL	Ryvyl Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RWAY	Runway Growth Finance Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RWAYI	Runway Growth Finance Corp. 7.25% Notes due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RWAYL	Runway Growth Finance Corp. 7.50% Notes due 2027	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RWT	Redwood Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RXO	RXO, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RXRX	Recursion Pharmaceuticals, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RXST	RxSight, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RXT	Rackspace Technology, Inc. Common Stock	1998-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RY	Royal Bank of Canada	1864-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RYAM	Rayonier Advanced Materials Inc.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RYAN	Ryan Specialty Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RYDE	Ryde Group Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
RYET	Ruanyun Edai Technology Inc. Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RYM	RYTHM, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RYN	Rayonier Inc.	1926-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RYOJ	rYojbaba Co., Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RYTM	Rhythm Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RYZ	Ryerson Holding Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
RZLT	Rezolute, Inc. Common Stock (NV)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
RZLV	Rezolve AI PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
S	SentinelOne, Inc.	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SA	Seabridge Gold, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SAAQ	Space Asset Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SABR	Sabre Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SABS	SAB Biotherapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAC	Safeguard Acquisition Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SACH	Sachem Capital Corp. Common Shares	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SAFE	Safehold Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SAFT	Safety Insurance Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAFX	XCF Global, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAGT	SAGTEC GLOBAL LIMITED Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAH	Sonic Automotive, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SAIA	Saia, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAIC	Science Applications International Corporation Common Stock	2013-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAIH	SAIHEAT Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAIL	SailPoint, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAM	Boston Beer Company	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SAMG	Silvercrest Asset Management Group Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SANA	Sana Biotechnology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SANG	Sangoma Technologies Corporation Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SANM	Sanmina  Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SAR	SARATOGA INVESTMENT CORP. NEW	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SARO	StandardAero, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SATL	Satellogic Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SATS	EchoStar Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SB	Safe Bulkers, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBAC	SBA Communications Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBC	SBC Medical Group Holdings Incorporated Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBCF	Seacoast Banking Corp of Florida	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBET	Sharplink, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBEV	Splash Beverage Group, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBFG	SB Financial Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBFM	Sunshine Biopharma Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBGI	Sinclair, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBH	Sally Beauty Holdings, Inc.	1964-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBLK	Star Bulk Carriers Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBLX	StableX Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBR	Sabine Royalty Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBRA	Sabra Healthcare REIT, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBSI	Southside Bancshares, Inc.	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBUX	Starbucks Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SBXD	SilverBox Corp IV	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SBXE	SilverBox Corp V	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCCO	Southern Copper Corporation	1952-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCHL	Scholastic Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCHW	The Charles Schwab Corporation	1971-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCI	Service Corporation International	1962-07-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCII	SC II Acquisition Corp. Class A ordinary share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCKT	Socket Mobile, Inc. New	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCL	Stepan Co.	1932-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCLX	Scilex Holding Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCM	STELLUS CAPITAL INVESTMENT CORPORATION	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SCNX	Scienture Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCOR	comScore, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCPQ	Social Commerce Partners Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCSC	Scansource Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCVL	Shoe Carnival Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCWO	374Water Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCYX	SCYNEXIS, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SCZM	Santacruz Silver Mining Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SD	SandRidge Energy, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SDA	SunCar Technology Group Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SDEV	Stablecoin Development Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SDGR	Schrodinger, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SDHC	Smith Douglas Homes Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SDHI	Siddhi Acquisition Corp Class A Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SDHY	PGIM Short Duration High Yield Opportunities Fund	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SDM	Smart Digital Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SDOT	Sadot Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SDRL	Seadrill Limited	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SDST	Stardust Power Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEAT	Vivid Seats Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEB	Seaboard Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SEDG	SolarEdge Technologies, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEED	Origin Agritech Limited	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEER	Seer, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEG	Seaport Entertainment Group Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SEGG	Sports Entertainment Gaming Global Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEI	Solaris Energy Infrastructure, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SEIC	SEI Investments Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SELF	Global Self Storage, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SELX	Semilux International Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEM	SELECT MEDICAL HOLDINGS CORP	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SEMR	SEMrush Holdings, Inc.	2008-08-03	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SENEA	Seneca Foods Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SENEB	Seneca Foods Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SENS	Senseonics Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEPN	Septerna, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SER	Serina Therapeutics, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SERA	Sera Prognostics, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SERV	Serve Robotics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SES	SES AI Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SEV	Aptera Motors Corp. Class B Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEVN	Seven Hills Realty Trust Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SEZL	Sezzle Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SF	Stifel Financial Corp.	1890-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SFBC	Sound Financial Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFBS	ServisFirst Bancshares Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SFD	Smithfield Foods, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFHG	Samfine Creation Holdings Group Limited Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFIX	Stitch Fix, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFL	SFL Corporation Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SFM	Sprouts Farmers Market, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFNC	Simmons First National Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFST	Southern First Bancshares, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SFWL	Shengfeng Development Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SG	Sweetgreen, Inc.	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SGA	Saga Communications, Inc. Class A Common Stock (FL)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGC	Superior Group of Companies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGHC	Super Group (SGHC) Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SGHT	Sight Sciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGI	Somnigroup International Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SGLY	Singularity Future Technology Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGML	Sigma Lithium Corporation Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGMO	Sangamo Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGMT	Sagimet Biosciences Inc. Series A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGP	SpyGlass Pharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGRP	SPAR Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGRY	Surgery Partners, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SGU	Star Group, L.P. Common Units Representing Limited Partner Interest	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SHAK	Shake Shack Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SHAZ	SharonAI Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHBI	Shore Bancshares Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHC	Sotera Health Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHEN	Shenandoah Telecom Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHFS	SHF Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHIM	Shimmick Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHIP	Seanergy Maritime Holdings Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHLS	Shoals Technologies Group, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHMD	SCHMID Group N.V. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHO	Sunstone Hotel Investors, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SHOO	Steven Madden Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHOP	Shopify Inc. Class A subordinate voting shares	2006-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHPH	Shuttle Pharmaceuticals Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SHW	The Sherwin-Williams Company	1866-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SI	Shoulder Innovations, Inc.	1894-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SIBN	SI-BONE, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIDU	Sidus Space, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIEB	Siebert Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIF	SIFCO Industries, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SIG	Signet Jewelers Limited	1949-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SIGA	SIGA Technologies Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIGI	Selective Insurance Group	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SII	Sprott Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SILA	Sila Realty Trust, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SILC	Silicom Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SILO	Silo Pharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIMA	SIM Acquisition Corp. I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SINT	SiNtx Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SION	Sionna Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SIRI	Sirius XM Holdings,  Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SITC	SITE Centers Corp. Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SITE	SiteOne Landscape Supply, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SITM	SiTime Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SJ	Scienjoy Holding Corporation Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SJM	The J.M. Smucker Company	1897-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SJT	San Juan Basin Royalty Trust UBI	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKBL	Skyline Builders Group Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKE	Skeena Resources Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKIL	Skillsoft Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKIN	The Beauty Health Company Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKK	SKK Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKLZ	Skillz Inc.	2012-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKT	Tanger Inc.	1981-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKWD	Skyward Specialty Insurance Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKY	Champion Homes, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKYE	Skye Bioscience, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKYH	Sky Harbour Group Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SKYQ	Sky Quarry Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKYT	SkyWater Technology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKYW	Skywest Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SKYX	SKYX Platforms Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLAB	Silicon Laboratories Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLB	SLB Limited	1926-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLDB	Solid Biosciences Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLDE	Slide Insurance Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLDP	Solid Power, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLE	Super League Enterprise, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLF	Sun Life Financial Inc.	1865-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLG	SL Green Realty Corp.	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLGB	Smart Logistics Global Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLGL	Sol-Gel Technologies Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLGN	Silgan Holdings Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLI	Standard Lithium Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLM	SLM Corporation	1972-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLMT	Brera Holdings PLC Class B Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLND	Southland Holdings, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLNG	Stabilis Solutions, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLNH	Soluna Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLNO	Soleno Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLP	Simulations Plus, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLQT	SelectQuote, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLRC	SLR Investment Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLS	SELLAS Life Sciences Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLSN	Solesence, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SLSR	Solaris Resources Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLVM	Sylvamo Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SLXN	Silexion Therapeutics Corp Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SM	SM Energy Company	1908-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMA	SmartStop Self Storage REIT, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMBC	Southern Missouri Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMBK	SmartFinancial, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMC	Summit Midstream Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMCI	Super Micro Computer, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMG	The Scotts Miracle-Gro Company	1868-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMHI	SEACOR Marine Holdings Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMID	Smith-Midland Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMJF	SMJ International Holdings Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMMT	Summit Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMP	Standard Motor Products	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMPL	The Simply Good Foods Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMR	NuScale Power Corporation	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMRT	SmartRent, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMSI	Smith Micro Software Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMTC	Semtech Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMTI	Sanara MedTech Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMTK	SmartKem, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMWB	Similarweb Ltd.	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SMX	SMX (Security Matters) Public Limited Company Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SMXT	Solarmax Technology Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SN	SharkNinja, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNA	Snap-on Incorporated	1920-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNAL	Snail, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNAP	Snap Inc.	2011-09-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNBR	Sleep Number Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNCY	Sun Country Airlines Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SND	Smart Sand, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNDA	Sonida Senior Living, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNDK	Sandisk Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNDL	Sundial Growers Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNDR	Schneider National, Inc.	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNDX	Syndax Pharmaceuticals, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNES	SenesTech, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNEX	StoneX Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNFCA	Security National Financial Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNGX	Soligenix, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNOA	Sonoma Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNOW	Snowflake Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNPS	Synopsys Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNSE	Sensei Biotherapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNT	Senstar Technologies Corporation Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNTG	Sentage Holdings Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNTI	Senti Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNWV	SANUWAVE Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SNX	TD SYNNEX Corporation	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SNYR	Synergy CHC Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SO	The Southern Company	1945-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOAR	Volato Group, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOBO	South Bow Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOBR	SOBR Safe, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOC	Sable Offshore Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOCA	Solarius Capital Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOFI	SoFi Technologies, Inc. Common Stock	2011-04-26	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOHU	Sohu.com Limited American Depositary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOLS	Solstice Advanced Materials Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOLV	Solventum Corporation	2024-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SON	Sonoco Products Company	1899-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SONM	DNA X, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SONO	Sonos, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOPA	Society Pass Incorporated Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOPH	SOPHiA GENETICS SA Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SORA	AsiaStrategy Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SORN	Soren Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOS	SOS Limited	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOTK	Sono-Tek Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOUL	Soulpower Acquisition Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SOUN	SoundHound AI, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SOWG	Sow Good Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPAI	Safe Pro Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPB	Spectrum Brands Holdings, Inc.	1906-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPCB	SuperCom, Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPCE	Virgin Galactic Holdings, Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPEG	Silver Pegasus Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPFI	South Plains Financial, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPG	Simon Property Group, Inc.	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPGI	S&P Global Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPH	Suburban Propane Partners L P	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPHL	Springview Holdings Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPHR	Sphere Entertainment Co.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPIR	Spire Global, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPKL	Spark I Acquisition Corp. Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPMC	Sound Point Meridian Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPNT	SiriusPoint Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPOK	Spok Holdings, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPOT	Spotify Technology S.A.	2007-01-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPPL	SIMPPLE LTD. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPRB	Spruce Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPRC	SciSparc Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPRO	Spero Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPRU	Spruce Power Holding Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SPRY	ARS Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPSC	SPS Commerce, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPT	Sprout Social, Inc Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPWH	Sportsman's Warehouse Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPWR	SunPower Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SPXC	SPX Technologies, Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SQFT	Presidio Property Trust, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SR	Spire Inc.	1857-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRAD	Sportradar Group AG Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRBK	SR Bancorp, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRCE	1st Source Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRE	Sempra	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRFM	Surf Air Mobility Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRG	Seritage Growth Properties Class A common shares of beneficial interest, par value $0.01	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRI	Stoneridge, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRL	Scully Royalty Ltd. Common Shares	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRPT	Sarepta Therapeutics,, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRRK	Scholar Rock Holding Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRTA	Strata Critical Medical, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRTS	Sensus Healthcare, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SRXH	SRX Health Solutions, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SRZN	Surrozen, Inc. Common	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSAC	SPACSphere Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSB	SouthState Bank Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SSBI	Summit State Bank	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSD	Simpson Manufacturing Co., Inc.	1956-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SSEA	Starry Sea Acquisition Corp Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSII	SS Innovations International Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSM	Sono Group N.V. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSNC	SS&C Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSP	The E.W. Scripps Company	1878-11-02	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSRM	SSR Mining Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSSS	SuRo Capital Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSSSL	SuRo Capital Corp. 6.00% Notes due 2026	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SST	System1, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SSTI	SoundThinking, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SSTK	SHUTTERSTOCK, INC.	2003-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SSYS	Stratasys Inc (ISRAEL)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ST	Sensata Technologies Holding plc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STAA	Staar Surgical Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STAG	STAG INDUSTRIAL, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STAK	STAK Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STBA	S&T Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STC	Stewart Information Services Corporation	1893-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STE	STERIS plc	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STEL	Stellar Bancorp, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STEM	Stem, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STEP	StepStone Group Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STEX	Streamex Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STFS	Star Fashion Culture Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STGW	Stagwell Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STHO	Star Holdings Shares of Beneficial Interest	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STI	Solidion Technology, Inc. Common Stock	1811-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STIM	Neuronetics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STKE	Sol Strategies Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STKL	SunOpta, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STKS	The ONE Group Hospitality, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STLA	Stellantis N.V.	2021-01-16	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STLD	Steel Dynamics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STM	STMicroelectronics N.V.	1987-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STN	Stantec, Inc.	1954-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STNE	StoneCo Ltd. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STNG	Scorpio Tankers Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STOK	Stoke Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRA	Strategic Education, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRL	Sterling Infrastructure, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRO	Sutro Biopharma, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRR	Star Equity Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRS	Stratus Properties Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRT	Strattec Security Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STRW	Strawberry Fields REIT, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
STRZ	Starz Entertainment Corp. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STSS	Sharps Technology Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STT	State Street Corporation	1792-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STTK	Shattuck Labs, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STUB	StubHub Holdings, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STVN	Stevanato Group S.p.A.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STWD	STARWOOD PROPERTY TRUST, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
STX	Seagate Technology Holdings PLC Ordinary Shares (Ireland)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
STXS	Stereotaxis, Inc. Common Stock	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
STZ	Constellation Brands, Inc.	1945-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SU	Suncor Energy, Inc.	1978-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUGP	SU Group Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUI	Sun Communities, Inc	1975-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUIG	Sui Group Holdings Limited Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUN	SUNOCO L.P.	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUNB	Sunbelt Rentals Holdings, Inc.	1947-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUNC	SunocoCorp LLC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUNE	SUNation Energy, Inc. Common Stock	1959-08-06	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUNS	Sunrise Realty Trust, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUPN	Supernus Pharmaceuticals, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUPV	Grupo Supervielle S.A.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SUPX	SuperX AI Technology Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SURG	SurgePays, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SUUN	PowerBank Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVA	Sinovac Biotech, Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVAC	Spring Valley Acquisition Corp. III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVAQ	Silicon Valley Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVC	Service Properties Trust Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVCC	Stellar V Capital Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVCO	Silvaco Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVIV	Spring Valley Acquisition Corp. IV Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVM	Silvercorp Metals Inc. Common Shares	1991-10-31	09:30:00	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SVRA	Savara Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVRN	OceanPal Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SVV	Savers Value Village, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SW	Smurfit Westrock plc	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SWAG	Stran & Company, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWBI	Smith & Wesson Brands, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWIM	Latham Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWK	Stanley Black & Decker, Inc.	1843-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SWKHL	SWK Holdings Corporation 9.00% Senior Notes due 2027	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWKS	Skyworks Solutions Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWMR	Swarmer, Inc Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWVL	Swvl Holdings Corp Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SWX	Southwest Gas Holdings, Inc.	1931-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SXC	SUNCOKE ENERGY INC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SXI	Standex International Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SXT	Sensient Technology Corporation	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SXTC	China SXT Pharmaceuticals, Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SXTP	60 Degrees Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYBT	Stock Yards Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYF	SYNCHRONY FINANCIAL	2003-09-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SYK	Stryker Corporation	1946-02-20	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SYM	Symbotic Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYNA	Synaptics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYNX	Silynxcom Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
SYPR	Sypris Solutions Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYRE	Spyre Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
SYY	Sysco Corporation	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
SZZL	Sizzle Acquisition Corp. II Class A ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
T	AT&T Inc.	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TAC	TransAlta Corporation	1911-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TACH	Titan Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TACO	Berto Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TACT	Transact Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TALK	Talkspace, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TALO	Talos Energy, Inc. Common Stock	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TANH	Tantech Holdings Ltd. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TAOP	Taoping Inc. BVI Ordinary Shares (0 par)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TAOX	Tao Synergies Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TAP	Molson Coors Beverage Company Class B	2005-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TAP.A	Molson Coors Beverage Company Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TARA	Protara Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TARS	Tarsus Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TASK	TaskUs, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TATT	TAT Technologies Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TAVI	Tavia Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TAYD	Taylor Devices Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBBB	BBB Foods Inc.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TBBK	The Bancorp Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBCH	Turtle Beach Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBH	Brag House Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBI	Trueblue, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TBLA	Taboola.com Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBLD	Thornburg Income Builder Opportunities Trust Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBN	Tamboran Resources Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TBPH	Theravance Biopharma, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TBRG	TruBridge, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCBI	Texas Capital Bancshares, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCBK	Trico Bancshares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCBS	Texas Community Bancshares, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCBX	Third Coast Bancshares, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TCGL	TechCreate Group Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TCI	Transcontinental Realty Investors, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TCMD	Tactile Systems Technology, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCPC	BlackRock TCP Capital Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCRT	Alaunos Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCRX	TScan Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TCX	Tucows, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TD	Toronto Dominion Bank	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDAC	Translational Development Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TDAY	USA TODAY Co., Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDC	TERADATA CORPORATION	1979-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDG	TransDigm Group Incorporated	1993-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDIC	Dreamland Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TDOC	Teladoc Health, Inc.	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDS	Telephone and Data Systems Inc.	1969-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDUP	ThredUp Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TDW	Tidewater, Inc.	1956-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TDWD	Tailwind 2.0 Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TDY	Teledyne Technologies Incorporated	1960-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TE	T1 Energy Inc.	1899-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TEAD	Teads Holding Co. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TEAM	Atlassian Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TECH	Bio-Techne Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TECK	Teck Resources Limited	1908-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TECX	Tectonic Therapeutic, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TEL	TE Connectivity plc	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TELA	TELA Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TELO	Telomir Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TEM	Tempus AI, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TEN	Tsakos Energy Navigation Ltd.	1930-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TENB	Tenable Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TENX	Tenax Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TER	Teradyne, Inc. Common Stock	1960-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TERN	Terns Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TEX	Terex Corporation	1925-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TFC	Truist Financial Corporation	1872-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TFII	TFI International Inc.	1957-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TFIN	Triumph Financial, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TFPM	Triple Flag Precious Metals Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TFSL	TFS Financial Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TFX	Teleflex Incorporated	1943-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TG	Tredegar Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGB	Taseko Mines Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGE	The Generation Essentials Group	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGEN	Tecogen Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGHL	The GrowHub Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TGL	Treasure Global Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TGLS	Tecnoglass Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGT	Target Corporation	1962-05-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TGTX	TG Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TH	Target Hospitality Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
THC	Tenet Healthcare Corporation New	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
THCH	TH International Limited Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
THFF	First Financial Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
THG	The Hanover Insurance Group, Inc.	1852-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
THH	TryHard Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
THM	International Tower Hill Mines, Ltd.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
THO	Thor Industries, Inc.	1980-08-29	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
THR	THERMON GROUP HOLDINGS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
THRM	Gentherm Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
THRY	Thryv Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TIC	TIC Solutions, Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TIGO	Millicom International Cellular S.A. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TIGR	UP Fintech Holding Ltd American Depositary Share representing fifteen Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TII	Titan Mining Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TIL	Instil Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TILE	Interface Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TIPT	Tiptree Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TISI	Team, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TITN	Titan Machinery Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TIVC	Tivic Health Systems, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TJGC	TJGC Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TJX	TJX Companies, Inc. (The)	1956-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TK	Teekay Corporation Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TKNO	Alpha Teknova, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TKO	TKO Group Holdings, Inc.	2023-09-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TKR	The Timken Company	1899-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TLF	Tandy Leather Factory, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLIH	Ten-League International Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLN	Talen Energy Corporation Common Stock	2015-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLNC	Talon Capital Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLPH	Talphera, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLRY	Tilray Brands, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLS	Telos Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLSA	Tiziana Life Sciences Ltd. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLSI	TriSalus Life Sciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TLYS	Tilly's Inc.	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMC	TMC the metals company Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMCI	Treace Medical Concepts, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMCR	The Metals Royalty Company Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMDE	TMD Energy Limited	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMDX	TransMedics Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMHC	Taylor Morrison Home Corporation Common Stock	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMO	Thermo Fisher Scientific, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMP	Tompkins Financial Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMQ	Trilogy Metals Inc	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TMTS	Spartacus Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMUS	T-Mobile US, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMUSI	T-Mobile US, Inc. 5.500% Senior Notes due June 2070	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMUSL	T-Mobile US, Inc. 6.250% Senior Notes due 2069	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TMUSZ	T-Mobile US, Inc. 5.500% Senior Notes due March 2070	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNC	TENNANT COMPANY	1870-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TNDM	Tandem Diabetes Care, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNET	TRINET GROUP, INC.	1988-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TNGX	Tango Therapeutics, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNK	Teekay Tankers Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TNL	Travel + Leisure Co.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TNMG	TNL Mediagene Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNON	Tenon Medical, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNXP	Tonix Pharmaceuticals Holding Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TNYA	Tenaya Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOI	The Oncology Institute, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOL	Toll Brothers, Inc.	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TOMZ	TOMI Environmental Solutions, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TONX	TON Strategy Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOON	Kartoon Studios, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TOP	TOP Financial Group Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOPP	Toppoint Holdings Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TOPS	TOP Ships Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TORO	Toro Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOST	Toast, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TOVX	Theriva Biologics, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TOWN	Towne Bank	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TOYO	TOYO Co., Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TPB	Turning Point Brands, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPC	Tutor Perini Corporation	1894-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPCS	Techprecision Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TPET	Trio Petroleum Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPG	TPG Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TPGXL	TPG Operating Group II, L.P. 6.950% Fixed-Rate Junior Subordinated Notes due 2064	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TPH	Tri Pointe Homes, Inc.	2009-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPL	Texas Pacific Land Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPR	Tapestry, Inc. Common Stock	2017-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TPST	Tempest Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TPVG	TRIPLEPOINT VENTURE GROWTH BDC CORP.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TR	Tootsie Roll Industries, Inc.	1896-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRAK	ReposiTrak, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRAW	Traws Pharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRAXV	First Tracks Biotherapeutics, Inc. Ordinary Shares When-Issued	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRC	Tejon Ranch Co.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRDA	Entrada Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TREE	LendingTree, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TREX	Trex Company, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRGP	Targa Resources Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRI	Thomson Reuters Corporation Common Shares	2008-04-17	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRIN	Trinity Capital Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRINI	Trinity Capital Inc. 7.875% Notes Due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRINZ	Trinity Capital Inc. 7.875% Notes due 2029	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRIP	TripAdvisor, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRMB	Trimble Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRMD	TORM plc Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRMK	Trustmark Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRN	Trinity Industries, Inc.	1933-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRNO	Terreno Realty Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRNR	Interactive Strength Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRNS	Transcat Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRON	Tron Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TROO	TROOPS, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TROW	T Rowe Price Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TROX	TRONOX LIMITED CL A ORDINARY SHARES	2005-11-21	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRP	TC Energy Corporation	1951-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRS	Trimas Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRSG	Tungray Technologies Inc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRST	Trustco Bank Corp NY	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRT	Trio-Tech International	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRTX	TPG RE Finance Trust, Inc. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRU	TransUnion	1968-02-08	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRUG	TruGolf Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRUP	Trupanion, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRV	The Travelers Companies, Inc.	1853-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TRVI	Trevi Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TRX	TRX Gold Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
TSAT	Telesat Corporation Class A Common Shares and Class B Variable Voting Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TSBK	Timberland Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TSCO	Tractor Supply Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TSEM	Tower Semiconductor Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TSHA	Taysha Gene Therapies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TSLA	Tesla Inc.	2010-06-29	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	2003-07-01	manual	IPO price $17, first US automaker IPO since Ford in 1956
TSLX	Sixth Street Specialty Lending, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TSN	Tyson Foods, Inc.	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TSQ	TOWNSQUARE MEDIA, INC.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TSSI	TSS, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TT	Trane Technologies plc	1871-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TTAM	Titan America SA	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TTAN	ServiceTitan, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTC	Toro Company (The)	1914-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TTD	The Trade Desk, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTE	TotalEnergies SE	1924-03-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TTEC	TTEC Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTEK	Tetra Tech Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTGT	TechTarget, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTI	TETRA Technologies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TTMI	TTM Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTRX	Turn Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TTWO	Take-Two Interactive Software Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TU	Telus Corporation	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TULP	Bloomia Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TUSK	Mammoth Energy Services, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TVA	Texas Ventures Acquisition III Corp Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TVAI	Thayer Ventures Acquisition Corporation II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TVGN	Tevogen Bio Holdings Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TVRD	Tvardi Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TVTX	Travere Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TW	Tradeweb Markets Inc. Class A Common Stock	2010-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWAV	TaoWeave, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWFG	TWFG, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWG	Top Wealth Group Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWI	Titan International, Inc.(Delaware)	1990-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TWIN	Twin Disc, Incorporated	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWLO	Twilio Inc.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TWLV	Twelve Seas Investment Company III Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TWO	Two Harbors Investment Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TWST	Twist Bioscience Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TXG	10x Genomics, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TXMD	TherapeuticsMD, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TXN	Texas Instruments Incorporated	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TXNM	TXNM Energy, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TXRH	Texas Roadhouse, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TXT	Textron, Inc.	1923-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TY	TRI-Continental Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TYGO	Tigo Energy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TYL	Tyler Technologies, Inc.	1966-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
TYRA	Tyra Biosciences, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
TZOO	Travelzoo Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
U	Unity Software Inc.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UA	Under Armour, Inc. Class C Common Stock, $0.0003 1/3 par value	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UAA	Under Armour, Inc.	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UAC	United Acquisition Corp. I	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UAL	United Airlines Holdings, Inc. Common Stock	1968-12-30	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UAMY	United States Antimony Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UAN	CVR Partners, LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UAVS	AgEagle Aerial Systems, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UBCP	United Bancorp Inc/OH	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UBER	Uber Technologies, Inc.	2009-03-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UBS	UBS Group AG	1862-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UBSI	United Bankshares Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UBXG	U-BX Technology Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UCAR	U Power Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UCB	United Community Banks, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UCFI	CN Healthy Food Tech Group Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UCTT	Ultra Clean Holdings, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UDMY	Udemy, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UDR	UDR, Inc.	1972-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UE	UBRAN EDGE PROPERTIES	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UEC	Uranium Energy Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UEIC	Universal Electronics Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UFCS	United Fire Group Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UFG	Uni-Fuels Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UFI	UNIFI, Inc. New	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UFPI	UFP Industries, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UFPT	UFP Technologies Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UG	United-Guardian, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UGI	UGI Corporation	1882-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UGRO	urban-gro, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UHAL	U-Haul Holding Company	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UHAL.B	U-Haul Holding Company	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UHG	United Homes Group, Inc Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UHS	Universal Health Services, Inc. Class B	1978-09-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UHT	Universal Health Realty Income Trust	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UI	Ubiquiti Inc. Common Stock	2003-10-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UIS	Unisys Corporation	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UK	Ucommune International Ltd Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ULBI	Ultralife Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ULCC	Frontier Group Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ULH	Universal Logistics Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ULS	UL Solutions Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ULTA	Ulta Beauty, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UMAC	Unusual Machines, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UMBF	UMB Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UMH	UMH Properties, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UNB	Union Bankshares, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UNCY	Unicycive Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UNF	Unifirst Corp	1936-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UNFI	United Natural Foods Inc	1996-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UNH	UnitedHealth Group Inc.	1984-10-17	09:30:00	NYSE	America/New_York	40.7069	-74.0089	1977-01-01	manual	Formerly United HealthCare Corporation
UNIT	Uniti Group Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UNM	Unum Group	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UNP	Union Pacific Corp.	1969-01-30	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UNTY	Unity Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UONE	Urban One, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UONEK	Urban One, Inc. Class D Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UP	Wheels Up Experience Inc.	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UPB	Upstream Bio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPBD	Upbound Group, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPC	Universe Pharmaceuticals Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPLD	Upland Software, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPS	United Parcel Service, Inc. Class B	1907-08-28	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UPST	Upstart Holdings, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPWK	Upwork Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UPXI	Upexi, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
URBN	Urban Outfitters Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
URG	Ur-Energy Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
URGN	UroGen Pharma Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
URI	United Rentals, Inc.	1997-09-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UROY	Uranium Royalty Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USAC	USA COMPRESSION PARTNERS LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
USAR	USA Rare Earth, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USAS	Americas Gold and Silver Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
USAU	U.S. Gold Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USB	U.S. Bancorp	1850-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
USBC	USBC, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
USCB	USCB Financial Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USEA	United Maritime Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USEG	U.S. Energy Corp. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USFD	US Foods Holding Corp.	1989-08-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
USGO	U.S. GoldMining Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USIO	Usio, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USLM	United States Lime & Minerals	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
USNA	USANA Health Sciences Inc	1992-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
USPH	US Physical Therapy Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UTHR	United Therapeutics Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UTI	Universal Technical Institute, Inc.	1965-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UTL	Unitil Corporation	1984-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UTMD	Utah Medical Products Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UTSI	UTStarcom Holdings Corp Ordinary Shares (Cayman Islands)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UTZ	Utz Brands, Inc.	1921-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UUU	Universal Safety Products, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UUUU	Energy Fuels Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
UVE	UNIVERSAL INSURANCE HLDG, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UVSP	Univest Financial Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
UVV	Universal Corporation	1886-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UWMC	UWM Holdings Corporation	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
UYSC	UY Scuti Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
V	Visa Inc.	2008-03-19	09:30:00	NYSE	America/New_York	40.7069	-74.0089	1958-09-18	manual	Largest US IPO at the time, raised $17.9B
VABK	Virginia National Bankshares Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VAC	MARRIOTT VACATIONS WORLDWIDE CORPORATION	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VACH	Voyager Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VACI	Viking Acquisition Corp. I	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VAL	Valaris Limited	1806-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VALU	Value Line Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VANI	Vivani Medical, Inc. Common Stock (DE)	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VATE	INNOVATE Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VBIX	Viewbix Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VBNK	VersaBank Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VC	VISTEON CORPORATION	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VCEL	Vericel Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VCIG	VCI Global Limited Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VCTR	Victory Capital Holdings, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VCX	Fundrise Innovation Fund, LLC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VCYT	Veracyte, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VECO	Veeco Instruments Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VEEA	Veea Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VEEE	Twin Vee PowerCats Co. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VEEV	Veeva Systems Inc.	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VEL	Velocity Financial, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VELO	Velo3D, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VENU	Venu Holding Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
VERA	Vera Therapeutics, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VERI	Veritone, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VERU	Veru Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VERX	Vertex, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VET	VERMILION ENERGY INC.	1994-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VFC	V.F. Corporation	1899-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VFF	Village Farms International, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VFS	VinFast Auto Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VG	Venture Global, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VGAS	Verde Clean Fuels, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VGNT	Versigent PLC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VGZ	Vista Gold Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
VHC	VirnetX Holding Corp Common Stock	2005-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VHCP	Vine Hill Capital Investment Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VHI	Valhi, Inc.	1987-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VHUB	VenHub Global, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIA	Via Transportation, Inc.	2012-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VIAV	Viavi Solutions Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VICI	VICI Properties Inc. Common Stock	2017-10-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VICR	Vicor Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIK	Viking Holdings Ltd	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VINP	Vinci Compass Investments Ltd. Class A Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIR	Vir Biotechnology, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIRC	Virco Mfg. Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIRT	Virtu Financial, Inc. Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VISN	Vistance Networks, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VITL	Vital Farms, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIVO	VivoPower PLC Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VIVS	VivoSim Labs, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VKTX	Viking Therapeutics, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VLGEA	Village Super Market	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VLN	Valens Semiconductor Ltd.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VLO	Valero Energy Corporation	1980-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VLTO	Veralto Corporation	2022-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VLY	Valley National Bancorp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VMAR	Vision Marine Technologies Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VMC	Vulcan Materials Company(Holding Company)	1909-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VMD	Viemed Healthcare, Inc. Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VMET	Versamet Royalties Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VMI	Valmont Industries, Inc.	1946-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VNCE	Vince Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VNDA	Vanda Pharmaceuticals Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VNME	Vendome Acquisition Corporation I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VNO	Vornado Realty Trust	1982-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VNOM	Viper Energy, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VNRX	VolitionRX Limited Common Stock	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
VNT	Vontier Corporation	2020-09-04	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VNTG	Vantage Corp	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
VOC	VOC ENERGY TRUST	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VOR	Vor Biopharma Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VOXR	Vox Royalty Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VOYA	VOYA FINANCIAL, INC.	2013-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VOYG	Voyager Technologies, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VPG	Vishay Precision Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VRA	Vera Bradley, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRAX	Virax Biolabs Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRCA	Verrica Pharmaceuticals Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRDN	Viridian Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRE	Veris Residential, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VREX	Varex Imaging Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRM	Vroom, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRME	VerifyMe, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRNS	Varonis Systems, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRRM	Verra Mobility Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRSK	Verisk Analytics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRSN	VeriSign Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VRT	Vertiv Holdings Co Class A Common Stock	1965-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VRTS	Virtus Investment Partners, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VRTX	Vertex Pharmaceuticals Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VS	Versus Systems Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSAT	Viasat Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSCO	Victorias Secret & Co.	1977-06-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VSEC	VSE Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSEE	VSee Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSH	Vishay Intertechnology, Inc.	1962-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VSME	VS Media Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSNT	Versant Media Group, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VST	Vistra Corp.	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VSTD	Vestand Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSTM	Verastem, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VSTS	Vestis Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTAK	Catheter Precision, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTEX	VTEX	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTGN	Vistagen Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VTIX	Virtuix Holdings Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VTOL	Bristow Group Inc.	1953-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTR	Ventas, Inc.	1998-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTRS	Viatris Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VTS	Vitesse Energy, Inc..	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VTSI	VirTra, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VTVT	vTv Therapeutics Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VUZI	Vuzix Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VVOS	Vivos Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VVV	Valvoline Inc.	1866-09-06	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VVX	V2X, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VWAV	VisionWave Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VYGR	Voyager Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VYNE	VYNE Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
VYX	NCR Voyix Corporation	1884-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VZ	Verizon Communications	1983-10-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
VZLA	Vizsla Silver Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
W	Wayfair Inc.	2002-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WAB	Wabtec Inc.	1999-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WABC	Westamerica Bancorporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAFD	WaFd, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAFDP	WaFd, Inc. Depositary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAFU	Wah Fu Education Group Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAI	Top KingWin Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAL	Western Alliance Bancorporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WALD	Waldencast plc Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WASH	Washington Trust Bancorp Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAT	Waters Corp	1958-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WATT	Energous Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WAY	Waystar Holding Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WBD	Warner Bros. Discovery, Inc. Series A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WBI	WaterBridge Infrastructure LLC	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WBS	Webster Financial Corporation Waterbury	1935-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WBTN	WEBTOON Entertainment Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WBUY	WEBUY GLOBAL LTD. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WBX	Wallbox N.V.	2015-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WCC	Wesco International Inc.	1922-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WCN	Waste Connections, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WCT	Wellchange Holdings Company Limited Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WD	Walker & Dunlop, Inc.	1937-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WDAY	Workday, Inc. Class A Common Stock	2005-03-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WDC	Western Digital Corp.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WDFC	Wd-40 Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WEAV	Weave Communications, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WEC	WEC Energy Group, Inc.	1896-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WELL	Welltower Inc.	1970-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WEN	The Wendy's Company	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WENN	Wen Acquisition Corp Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WERN	Werner Enterprises Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WES	Western Midstream Partners, LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WEST	Westrock Coffee Company Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WETH	Wetouch Technology Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WETO	Wetour Robotics Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WEX	WEX Inc.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WEYS	Weyco Group Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WFC	Wells Fargo & Co.	1852-03-18	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WFCF	Where Food Comes From, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WFF	WF Holding Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WFG	West Fraser Timber Co. Ltd	2020-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WFRD	Weatherford International plc Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WGO	Winnebago Industries, Inc.	1958-02-12	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WGRX	Wellgistics Health, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WGS	GeneDx Holdings Corp. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WH	Wyndham Hotels & Resorts, Inc. Common Stock	2018-06-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WHD	Cactus, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WHF	WhiteHorse Finance, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WHFCL	WhiteHorse Finance, Inc. 7.875% Notes due 2028	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WHG	WESTWOOD HOLDINGS GROUP, INC.	1983-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WHLR	Wheeler Real Estate Investment Trust, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WHLRL	Wheeler Real Estate Investment Trust, Inc. 7.00% Senior Subordinated Convertible Notes Due 2031	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WHR	Whirlpool Corp.	1911-11-11	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WHWK	Whitehawk Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WILC	G Willi-Food International Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WIMI	WiMi Hologram Cloud Inc. Class B Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WINA	Winmark Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WING	Wingstop Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WIX	WIX.com Ltd.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WK	Workiva Inc.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WKC	World Kinect Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WKHS	Workhorse Group, Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WKSP	Worksport, Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLAC	Willow Lane Acquisition Corp. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLDN	Willdan Group, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLDS	Wearable Devices Ltd. Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLFC	Willis Lease Finance Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLII	Willow Lane Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLK	Westlake Corporation	1986-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WLKP	WESTLAKE CHEMICAL PARTNERS LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WLTH	Wealthfront Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WLY	John Wiley & Sons, Inc. Class A	1807-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WLYB	John Wiley & Sons, Inc. Class B	1807-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WM	Waste Management, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WMB	Williams Companies Inc.	1908-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WMG	Warner Music Group Corp. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WMK	Weis Markets, Inc.	1912-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WMS	ADVANCED DRAINAGE SYSTEMS, INC.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WMT	Walmart Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WNC	Wabash National Corp.	1985-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WNEB	Western New England Bancorp, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WNW	Meiwu Technology Company Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WOK	WORK Medical Technology Group LTD Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WOLF	Wolfspeed, Inc.	1987-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WOOF	Petco Health and Wellness Company, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WOR	Worthington Enterprises, Inc.	1955-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WPAC	White Pearl Acquisition Corp.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WPC	W.P. Carey Inc. (REIT)	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WPM	Wheaton Precious Metals Corp. Common Stock	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WPRT	Westport Fuel Systems Inc Common Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WRAP	Wrap Technologies, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WRB	W.R. Berkley Corporation	1967-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WRBY	Warby Parker Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WRLD	World Acceptance Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WRN	Western Copper and Gold Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
WS	Worthington Steel, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WSBC	WesBanco Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSBCO	WesBanco, Inc. Depositary Shares each representing 1/40th interest in a share of 7.375% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series B	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSBF	Waterstone Financial, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSBK	Winchester Bancorp, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSC	WillScot Holdings Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSFS	WSFS Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSHP	WeShop Holdings Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WSM	Williams-Sonoma, Inc.	1983-07-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WSO	Watsco, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WSO.B	Watsco, Inc. Class B	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WSR	Whitestone REIT	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WST	West Pharmaceutical Services, Inc.	1923-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WSTN	Westin Acquisition Corp Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WT	WisdomTree, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTBA	West Bancorporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WTF	Waton Financial Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WTFC	Wintrust Financial Corp	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WTG	Wintergreen Acquisition Corp. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WTI	W&T Offshore, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTM	White Mountains Insurance Group Ltd.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTO	UTime Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WTRG	Essential Utilities, Inc.	1968-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTS	Watts Water Technologies, Inc. Class A	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTTR	Select Water Solutions, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WTW	Willis Towers Watson Public Limited Company Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WU	The Western Union Company	1856-04-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WULF	TeraWulf Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WVE	Wave Life Sciences Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WVVI	Willamette Valley Vineyards	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WW	WW International, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WWD	Woodward, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WWR	Westwater Resources, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
WWW	Wolverine World Wide, Inc.	1883-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WXM	WF International Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WY	Weyerhaeuser Company	1900-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
WYFI	WhiteFiber, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WYNN	Wynn Resorts Ltd	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
WYY	WidePoint Corporation	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
XAIR	Beyond Air, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XBIO	Xenetic Biosciences, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XBIT	XBiotech Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XBP	XBP Global Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XCBE	X3 Acquisition Corp. Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XCUR	Exicure, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XEL	Xcel Energy, Inc.	1998-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XELB	XCEL BRANDS INC.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XELLL	Xcel Energy Inc. 6.25% Junior Subordinated Notes, Series due 2085	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XENE	Xenon Pharmaceuticals Inc	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XERS	Xeris Biopharma Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XFLH	XFLH Capital Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XFOR	X4 Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XGN	Exagen Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XHLD	TEN Holdings, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XHR	Xenia Hotels & Resorts, Inc.	2014-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XIFR	XPLR Infrastructure, LP	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XLO	Xilio Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XMTR	Xometry, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XNCR	Xencor, Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XNDU	Xanadu Quantum Technologies Limited Class B Subordinate Voting Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XOM	Exxon Mobil Corporation	1882-08-05	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XOMA	XOMA Royalty Corporation Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XOMAO	XOMA Royalty Corporation Depositary Shares Rep Series B 8.375% Cumulative Preferred Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XOS	Xos, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XP	XP Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XPEL	XPEL, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XPER	Xperi Inc	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XPL	Solitario Resources Corp.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
XPO	XPO, Inc.	1987-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XPOF	Xponential Fitness, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XPON	Expion360 Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XPRO	Expro Group Holdings N.V.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XRAY	DENTSPLY SIRONA Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XRN	Chiron Real Estate Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XRPN	Armada Acquisition Corp. II Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XRTX	XORTX Therapeutics Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XRX	Xerox Holdings Corporation Common Stock	1906-04-18	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XSLL	Xsolla SPAC 1 Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XTIA	XTI Aerospace, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XTNT	Xtant Medical Holdings, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
XWEL	XWELL, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XWIN	XMAX, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XXI	Twenty One Capital, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XXII	22nd Century Group Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
XYL	Xylem Inc	2011-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XYZ	Block, Inc.	2009-02-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
XZO	Exzeo Group, Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YAAS	Youxin Technology Ltd Class A Ordinary shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YCBD	cbdMD, Inc. Common Stock	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
YCY	AA Mission Acquisition Corp. II	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YDDL	One and One Green Technologies. Inc Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YDES	YD Bio Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YDKG	Yueda Digital Holding Class A Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YELP	YELP INC.	2004-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YETI	YETI Holdings, Inc. Common Stock	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YEXT	Yext, Inc.	2006-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YHC	LQR House Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YHGJ	Yunhong Green CTI Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YHNA	YHN Acquisition I Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YIBO	Planet Image International Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YMAT	J-Star Holding Co., Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YOOV	Concorde International Group Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YORW	York Water Co	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YOU	Clear Secure, Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YSS	York Space Systems Inc.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YSXT	YSX Tech. Co., Ltd Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YTRA	Yatra Online, Inc. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YUM	Yum! Brands, Inc.	1997-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YUMC	Yum China Holdings, Inc. Common Stock	2016-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
YYAI	AiRWA Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
YYGH	YY Group Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
Z	Zillow Group, Inc. Class C Capital Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZBAI	ATIF Holdings Limited Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZBAO	Zhibao Technology Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZBH	Zimmer Biomet Holdings, Inc.	1927-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZBIO	Zenas BioPharma, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZBRA	Zebra Technologies Corporation	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZCMD	Zhongchao Inc. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZD	Ziff Davis, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZDAI	DirectBooking Technology Co., Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZDGE	Zedge, Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZENA	ZenaTech, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZEO	Zeo Energy Corporation Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZETA	Zeta Global Holdings Corp.	2008-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZG	ZILLOW GROUP INC CLASS A	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZGN	Ermenegildo Zegna N.V.	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZIM	ZIM Integrated Shipping Services Ltd.	1945-06-07	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZION	Zions Bancorporation N.A.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZIP	ZipRecruiter, Inc.	2010-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZJK	ZJK Industrial Co., Ltd. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZJYL	JIN MEDICAL INTERNATIONAL LTD. Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZKIN	ZK International Group Co., Ltd Ordinary Share	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZKP	Lafayette Digital Acquisition Corp. I Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZM	Zoom Communications, Inc. Class A Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZNB	Zeta Network Group Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZNTL	Zentalis Pharmaceuticals, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZONE	CleanCore Solutions Inc.	\N	\N	NYSE American	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZOOZ	ZOOZ Strategy Ltd. Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZS	Zscaler, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZSPC	zSpace, Inc. Common stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZSTK	ZeroStack Corp. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZTEK	Zentek Ltd. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZTG	Zenta Group Company Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZTS	ZOETIS INC.	1952-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZUMZ	Zumiez Inc.	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZURA	Zura Bio Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZVIA	Zevia PBC	2007-01-01	09:30:00	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZVRA	Zevra Therapeutics, Inc. Common Stock	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZWS	Zurn Elkay Water Solutions Corporation	\N	\N	NYSE	America/New_York	40.7069	-74.0089	\N	polygon	\N
ZYBT	Zhengye Biotechnology Holding Limited Class A Ordinary Shares	\N	\N	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
ZYME	Zymeworks Inc.	2003-01-01	09:30:00	NASDAQ	America/New_York	40.7589	-73.9851	\N	polygon	\N
\.


--
-- PostgreSQL database dump complete
--

\unrestrict xJJerdLIfcHqggtu7OImMYAji3fq07KfbwUrD1idNA4JIG2D2SwYhvcgjauuOZM

