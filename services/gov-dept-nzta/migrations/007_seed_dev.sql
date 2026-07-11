-- Development seed data — test citizen "Alex Tane"
-- Only runs in dev/demo environments

INSERT INTO citizens (did, driver_licence_number)
VALUES ('did:gov:nz:test-citizen-001', 'NZ1234567')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO driver_licences (citizen_id, licence_number, full_name, licence_class, expiry_date, conditions)
SELECT c.id, 'NZ1234567', 'Alex Tane', '1 (car)', '2028-09-30', NULL FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO vehicles (citizen_id, registration, make, model, year, fuel_type, registration_expiry)
SELECT c.id, 'ABC123', 'Toyota', 'Corolla', 2021, 'Petrol', '2026-12-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ruc_records (citizen_id, vehicle_rego, licence_type, expiry_date, units_remaining)
SELECT c.id, 'ABC123', 'Heavy vehicle RUC', '2027-06-30', 1500 FROM c
ON CONFLICT DO NOTHING;

-- Second test citizen "Bree Kāre"
INSERT INTO citizens (did, driver_licence_number)
VALUES ('did:gov:nz:test-citizen-002', 'NZ7654321')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO driver_licences (citizen_id, licence_number, full_name, licence_class, expiry_date, conditions)
SELECT c.id, 'NZ7654321', 'Bree Kāre', '6 (motorcycle)', '2027-12-31', NULL FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO vehicles (citizen_id, registration, make, model, year, fuel_type, registration_expiry)
SELECT c.id, 'XYZ789', 'Honda', 'CBR', 2020, 'Petrol', '2027-01-01' FROM c
ON CONFLICT DO NOTHING;
