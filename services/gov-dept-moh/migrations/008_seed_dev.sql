-- Development seed data — test citizen "Alex Tane"
-- Only runs in dev/demo environments

INSERT INTO citizens (did, nhi)
VALUES ('did:gov:nz:test-citizen-001', 'NBA1234')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO gp_enrolments (citizen_id, practice_name, address, phone, enrolled_at)
SELECT c.id, 'Pukekohe Family Health', '12 Queen St, Pukekohe', '09 238 0000', '2023-03-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO prescriptions (citizen_id, medication, dose, repeats_total, repeats_remaining, issued_at)
SELECT c.id, 'Atorvastatin', '20mg once daily', 3, 2, '2026-06-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO appointments (citizen_id, provider, appt_date, type, status)
SELECT c.id, 'Dr. K. Pewhairangi', '2026-07-20T10:30:00+12:00', 'General check-up', 'booked' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO vaccinations (citizen_id, vaccine, vaccine_date, due_for_booster)
SELECT c.id, 'Influenza', '2025-04-15', false FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO vaccinations (citizen_id, vaccine, vaccine_date, due_for_booster)
SELECT c.id, 'COVID-19', '2025-11-02', true FROM c
ON CONFLICT DO NOTHING;
