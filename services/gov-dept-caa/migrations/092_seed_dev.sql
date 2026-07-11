-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, caa_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'CAA-100001'),
    ('did:gov:nz:test-citizen-002', 'CAA-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO caa_licences (citizen_id, licence_no, category, status, expires) SELECT c.id, 'CAA-P-55821', 'Private Pilot', 'current', '2027-06-30' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO caa_aircraft (citizen_id, registration, aircraft_type, status) SELECT c.id, 'ZK-TAN', 'Cessna 172', 'registered' FROM c
ON CONFLICT DO NOTHING;
