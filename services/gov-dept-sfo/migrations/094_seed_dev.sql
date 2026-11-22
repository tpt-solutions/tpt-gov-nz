-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, sfo_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'SFO-100001'),
    ('did:gov:nz:test-citizen-002', 'SFO-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO sfo_investigations (citizen_id, reference, matter, status, opened_date) SELECT c.id, 'SFO-2026-014', 'Complex investment fraud', 'under-investigation', '2026-01-22' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO sfo_outcomes (citizen_id, reference, result, result_date) SELECT c.id, 'SFO-2025-009', 'Prosecution commenced', '2025-11-03' FROM c
ON CONFLICT DO NOTHING;
