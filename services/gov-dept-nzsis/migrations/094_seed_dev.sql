-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, nzsis_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'NZSIS-100001'),
    ('did:gov:nz:test-citizen-002', 'NZSIS-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzsis_mandates (citizen_id, reference, agency, status, issued_date) SELECT c.id, 'NZSIS-M-2026-002', 'GCSB', 'active', '2026-01-08' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzsis_threats (citizen_id, reference, category, status, assessed_date) SELECT c.id, 'NZSIS-T-2026-014', 'Foreign interference', 'monitored', '2026-02-11' FROM c
ON CONFLICT DO NOTHING;
