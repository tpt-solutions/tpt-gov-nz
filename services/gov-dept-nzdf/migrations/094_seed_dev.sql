-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, nzdf_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'NZDF-100001'),
    ('did:gov:nz:test-citizen-002', 'NZDF-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzdf_service_records (citizen_id, service_no, branch, status) SELECT c.id, 'NZDF-55821', 'Army', 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzdf_deployments (citizen_id, operation, country, year) SELECT c.id, 'Burnham readiness', 'NZ', 2025 FROM c
ON CONFLICT DO NOTHING;
