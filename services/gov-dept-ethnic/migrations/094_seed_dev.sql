-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, ethnic_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'ETH-100001'),
    ('did:gov:nz:test-citizen-002', 'ETH-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ethnic_programmes (citizen_id, programme_name, status, year) SELECT c.id, 'Ethnic Communities Graduate Programme', 'enrolled', 2026 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ethnic_community_grants (citizen_id, grant_name, amount, status) SELECT c.id, 'Community-led response fund', 5000, 'approved' FROM c
ON CONFLICT DO NOTHING;
