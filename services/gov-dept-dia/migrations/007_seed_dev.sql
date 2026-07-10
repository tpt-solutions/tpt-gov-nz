-- Development seed data — test citizen "Alex Tane"
-- Only runs in dev/demo environments

INSERT INTO citizens (did, passport_number)
VALUES ('did:gov:nz:test-citizen-001', 'XA123456')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO passports (citizen_id, passport_number, expiry_date, renewable)
SELECT c.id, 'XA123456', '2028-03-15', true FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO birth_certs (citizen_id, certificate_number, date_of_birth, place_of_birth, parents)
SELECT c.id, 'BC-9001', '1990-05-20', 'Auckland, New Zealand', 'R. Tane and H. Tane' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO citizenship_records (citizen_id, status, certificate_number, granted_at)
SELECT c.id, 'citizen-by-birth', NULL, NULL FROM c
ON CONFLICT DO NOTHING;
