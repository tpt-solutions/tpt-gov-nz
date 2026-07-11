-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, pacific_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'PAC-100001'),
    ('did:gov:nz:test-citizen-002', 'PAC-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO pacific_programmes (citizen_id, programme_name, status, year) SELECT c.id, 'Tokelau Language Week', 'enrolled', 2026 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO pacific_language_services (citizen_id, service, region, status) SELECT c.id, 'Gagana Samoa classes', 'Auckland', 'available' FROM c
ON CONFLICT DO NOTHING;
