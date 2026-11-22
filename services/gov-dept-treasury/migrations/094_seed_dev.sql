-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, treasury_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'TRE-100001'),
    ('did:gov:nz:test-citizen-002', 'TRE-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO treasury_budget (citizen_id, fiscal_year, portfolio, appropriation, amount) SELECT c.id, 2026, 'Health', 'Vote Health', 1200000000 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO treasury_economic_outlook (citizen_id, forecast_year, gdp_growth_pct, inflation_pct, net_debt_pct) SELECT c.id, 2026, 2.4, 3.1, 42 FROM c
ON CONFLICT DO NOTHING;
