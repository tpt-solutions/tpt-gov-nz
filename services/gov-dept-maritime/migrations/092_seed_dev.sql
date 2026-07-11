-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, maritime_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MAR-100001'),
    ('did:gov:nz:test-citizen-002', 'MAR-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO maritime_vessels (citizen_id, vessel_name, flag, status) SELECT c.id, 'MV Tane Moana', 'NZ', 'registered' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO maritime_incidents (citizen_id, reference, incident_type, incident_date, status) SELECT c.id, 'MAR-2026-02', 'Pollution', '2026-02-14', 'resolved' FROM c
ON CONFLICT DO NOTHING;
