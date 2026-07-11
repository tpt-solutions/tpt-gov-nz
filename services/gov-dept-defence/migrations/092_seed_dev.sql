-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, defence_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'DEF-100001'),
    ('did:gov:nz:test-citizen-002', 'DEF-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO defence_procurements (citizen_id, programme, value, status) SELECT c.id, 'Frigate sustainment', 450000000, 'ongoing' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO defence_bases (citizen_id, name, location, status) SELECT c.id, 'Trentham Military Camp', 'Upper Hutt', 'operational' FROM c
ON CONFLICT DO NOTHING;
