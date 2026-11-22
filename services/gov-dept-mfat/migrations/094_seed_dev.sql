-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, mfat_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MFAT-100001'),
    ('did:gov:nz:test-citizen-002', 'MFAT-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mfat_overseas_missions (citizen_id, country, status) SELECT c.id, 'Australia', 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mfat_travel_advisories (citizen_id, country, level, updated) SELECT c.id, 'Indonesia', 'Exercise increased caution', '2026-03-10' FROM c
ON CONFLICT DO NOTHING;
