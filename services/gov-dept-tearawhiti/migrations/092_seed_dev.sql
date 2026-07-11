-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, tearawhiti_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'TAW-100001'),
    ('did:gov:nz:test-citizen-002', 'TAW-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tearawhiti_treaty_settlements (citizen_id, iwi, status, settled_date) SELECT c.id, 'Ngāti Toa', 'settled', '2024-07-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tearawhiti_engagements (citizen_id, topic, engagement_date, outcome) SELECT c.id, 'Crown engagement hui', '2026-04-15', 'Recommendation agreed' FROM c
ON CONFLICT DO NOTHING;
