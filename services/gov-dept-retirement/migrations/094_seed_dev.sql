-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, retirement_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'RET-100001'),
    ('did:gov:nz:test-citizen-002', 'RET-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO retirement_retirement_plan (citizen_id, has_plan, retirement_age, last_review) SELECT c.id, true, 65, '2025-12-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO retirement_guidance (citizen_id, topic, summary, published) SELECT c.id, 'KiwiSaver contribution rate', 'Consider increasing to 6%.', '2026-02-20' FROM c
ON CONFLICT DO NOTHING;
