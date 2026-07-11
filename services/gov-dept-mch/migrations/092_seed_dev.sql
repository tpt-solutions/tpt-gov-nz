-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, mch_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MCH-100001'),
    ('did:gov:nz:test-citizen-002', 'MCH-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mch_heritage_sites (citizen_id, name, status, region) SELECT c.id, 'Old St Paul''s', 'Category 1 historic place', 'Wellington' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mch_grants (citizen_id, grant_name, amount, status) SELECT c.id, 'Cultural Innovation Fund', 15000, 'approved' FROM c
ON CONFLICT DO NOTHING;
