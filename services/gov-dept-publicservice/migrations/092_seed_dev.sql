-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, psc_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'PSC-100001'),
    ('did:gov:nz:test-citizen-002', 'PSC-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO publicservice_workforce (citizen_id, report_year, agency, headcount) SELECT c.id, 2025, 'Department of Internal Affairs', 4200 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO publicservice_agency_ratings (citizen_id, agency, rating, rating_year) SELECT c.id, 'Department of Internal Affairs', 'Good', 2025 FROM c
ON CONFLICT DO NOTHING;
