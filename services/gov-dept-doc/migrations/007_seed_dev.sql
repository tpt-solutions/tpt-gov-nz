-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, doc_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'DOC-100001'),
    ('did:gov:nz:test-citizen-002', 'DOC-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: active permit + concession
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO doc_permits (citizen_id, permit_number, activity, location, status, expires_date)
SELECT c.id, 'DOC-P1001', 'Recreational hunting', 'Whakarewarewa Forest', 'active', '2027-05-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO doc_concessions (citizen_id, concession_id, type, holder, start_date, end_date)
SELECT c.id, 'DOC-C2001', 'guided-tour', 'Alex Tane', '2026-01-01', '2026-12-31' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: pending permit only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO doc_permits (citizen_id, permit_number, activity, location, status, expires_date)
SELECT c.id, 'DOC-P2002', 'Fishing', 'Lake Taupō', 'pending', '2027-03-15' FROM c
ON CONFLICT DO NOTHING;
