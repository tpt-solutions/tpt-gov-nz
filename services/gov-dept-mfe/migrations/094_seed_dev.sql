-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, mfe_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MFE-100001'),
    ('did:gov:nz:test-citizen-002', 'MFE-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mfe_emissions (citizen_id, report_year, sector, tonnes_co2e) SELECT c.id, 2025, 'Transport', 3200.5 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mfe_reports (citizen_id, title, published, status) SELECT c.id, 'Aotearoa New Zealand''s Environment 2026', '2026-05-01', 'published' FROM c
ON CONFLICT DO NOTHING;
