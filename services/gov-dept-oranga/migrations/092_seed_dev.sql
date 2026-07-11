-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, oranga_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'OT-100001'),
    ('did:gov:nz:test-citizen-002', 'OT-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO oranga_care_placements (citizen_id, placement_type, start_date, region) SELECT c.id, 'Whānau placement', '2025-08-01', 'Waikato' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO oranga_support_services (citizen_id, service, status, next_review) SELECT c.id, 'Intensive support', 'active', '2026-09-01' FROM c
ON CONFLICT DO NOTHING;
