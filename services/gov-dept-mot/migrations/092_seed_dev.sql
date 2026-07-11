-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, mot_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MOT-100001'),
    ('did:gov:nz:test-citizen-002', 'MOT-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mot_strategies (citizen_id, title, year, status) SELECT c.id, 'Te Tangi a Te Manu', 2026, 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mot_programmes (citizen_id, name, budget, status) SELECT c.id, 'Road maintenance boost', 800000000, 'funded' FROM c
ON CONFLICT DO NOTHING;
