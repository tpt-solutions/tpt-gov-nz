-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, dpmc_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'DPMC-100001'),
    ('did:gov:nz:test-citizen-002', 'DPMC-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO dpmc_honours (citizen_id, award_year, award, status) SELECT c.id, 2025, 'Queen''s Service Medal', 'nominated' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO dpmc_engagements (citizen_id, event_name, event_date, location) SELECT c.id, 'Citizens'' Honours Reception', '2026-05-12', 'Wellington' FROM c
ON CONFLICT DO NOTHING;
