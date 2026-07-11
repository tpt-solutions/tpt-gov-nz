-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, traveller_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'CUST-100001'),
    ('did:gov:nz:test-citizen-002', 'CUST-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: travel record + submitted declaration
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO customs_travel (citizen_id, passport_number, last_arrival, arrival_port, frequent_traveller)
SELECT c.id, 'P1234567', '2026-06-30', 'Auckland', true FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO customs_declarations (citizen_id, declaration_id, date, country_from, goods_declared, status)
SELECT c.id, 'CUST-DCL-1', '2026-06-30', 'Australia', 'Personal effects', 'submitted' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: frequent traveller only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO customs_travel (citizen_id, passport_number, last_arrival, arrival_port, frequent_traveller)
SELECT c.id, 'P7654321', '2026-05-12', 'Wellington', true FROM c
ON CONFLICT DO NOTHING;
