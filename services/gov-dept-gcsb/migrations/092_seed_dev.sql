-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, gcsb_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'GCSB-100001'),
    ('did:gov:nz:test-citizen-002', 'GCSB-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO gcsb_mandates (citizen_id, reference, agency, status, issued_date) SELECT c.id, 'GCSB-M-2026-001', 'NZSIS', 'active', '2026-01-05' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO gcsb_engagements (citizen_id, partner, engagement_type, engagement_date) SELECT c.id, 'CERT NZ', 'Cyber threat briefing', '2026-02-20' FROM c
ON CONFLICT DO NOTHING;
