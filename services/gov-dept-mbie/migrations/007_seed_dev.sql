-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, person_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MBIE-P-100001'),
    ('did:gov:nz:test-citizen-002', 'MBIE-P-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: one registered company + one directorship
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mbie_business_registrations (citizen_id, nzbn, entity_name, entity_type, status, registered_date)
SELECT c.id, '9429000000001', 'Tane Consulting Ltd', 'company', 'registered', '2024-03-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mbie_directorships (citizen_id, nzbn, entity_name, role, appointed_date)
SELECT c.id, '9429000000001', 'Tane Consulting Ltd', 'Director', '2024-03-01' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: one registered sole-trader business only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO mbie_business_registrations (citizen_id, nzbn, entity_name, entity_type, status, registered_date)
SELECT c.id, '9429000000002', 'Kare Trading', 'sole-trader', 'registered', '2023-07-15' FROM c
ON CONFLICT DO NOTHING;
