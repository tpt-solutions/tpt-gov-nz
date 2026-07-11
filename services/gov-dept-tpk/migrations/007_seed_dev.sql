-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, tpk_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'TPK-100001'),
    ('did:gov:nz:test-citizen-002', 'TPK-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: an enrolled programme + an approved grant
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tpk_programmes (citizen_id, programme_name, status, region)
SELECT c.id, 'Te Hono', 'enrolled', 'Te Tai Tokerau' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tpk_funding (citizen_id, grant_id, amount, purpose, status)
SELECT c.id, 'TPK-G1001', 5000, 'Marae renovations', 'approved' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: a single enrolled programme only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO tpk_programmes (citizen_id, programme_name, status, region)
SELECT c.id, 'Māori Language Fund', 'enrolled', 'Tāmaki Makaurau' FROM c
ON CONFLICT DO NOTHING;
