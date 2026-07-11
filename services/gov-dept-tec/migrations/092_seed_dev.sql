-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, tec_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'TEC-100001'),
    ('did:gov:nz:test-citizen-002', 'TEC-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tec_funding (citizen_id, provider, amount, year) SELECT c.id, 'Whitireia', 2200000, 2026 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tec_courses (citizen_id, course_name, provider, status) SELECT c.id, 'New Zealand Certificate in IT', 'Whitireia', 'approved' FROM c
ON CONFLICT DO NOTHING;
