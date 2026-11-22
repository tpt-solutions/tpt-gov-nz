-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, fenz_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'FENZ-100001'),
    ('did:gov:nz:test-citizen-002', 'FENZ-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO fenz_fire_safety (citizen_id, property, grade, last_inspection) SELECT c.id, '12 Totara Street, Porirua', 'Compliant', '2025-11-12' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO fenz_incidents (citizen_id, reference, incident_type, incident_date, status) SELECT c.id, 'FENZ-2026-050', 'Structure fire', '2026-01-30', 'closed' FROM c
ON CONFLICT DO NOTHING;
