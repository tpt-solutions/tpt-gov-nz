-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, worksafe_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'WS-100001'),
    ('did:gov:nz:test-citizen-002', 'WS-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO worksafe_inspections (citizen_id, reference, site, inspection_date, outcome) SELECT c.id, 'WS-I-2026-003', 'Tane Construction Ltd', '2026-02-10', 'Compliance order issued' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO worksafe_investigations (citizen_id, reference, matter, status, opened_date) SELECT c.id, 'WS-INV-2026-011', 'Fatality inquiry', 'ongoing', '2026-01-15' FROM c
ON CONFLICT DO NOTHING;
