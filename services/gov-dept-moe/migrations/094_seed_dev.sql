-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, moe_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MOE-100001'),
    ('did:gov:nz:test-citizen-002', 'MOE-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO moe_enrolment (citizen_id, school, year_level, status) SELECT c.id, 'Porirua College', 9, 'enrolled' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO moe_student_support (citizen_id, service, status, next_review) SELECT c.id, 'Learning support', 'active', '2026-08-01' FROM c
ON CONFLICT DO NOTHING;
