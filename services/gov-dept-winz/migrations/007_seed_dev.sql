-- Development seed data — test citizen "Alex Tane"
-- Only runs in dev/demo environments

INSERT INTO citizens (did, client_id)
VALUES ('did:gov:nz:test-citizen-001', 'WINZ-CLIENT-001')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO benefits (citizen_id, benefit_type, weekly_amount, start_date, review_date, status)
SELECT c.id, 'jobseeker', 275.40, '2024-02-01', '2026-02-01', 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO benefits (citizen_id, benefit_type, weekly_amount, start_date, review_date, status)
SELECT c.id, 'accommodation-supplement', 130.00, '2024-02-01', '2026-02-01', 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO payments (citizen_id, benefit_type, payment_date, amount, method)
SELECT c.id, 'jobseeker', '2026-07-06', 275.40, 'bank-deposit' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO payments (citizen_id, benefit_type, payment_date, amount, method)
SELECT c.id, 'accommodation-supplement', '2026-07-06', 130.00, 'bank-deposit' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO case_notes (citizen_id, author, note)
SELECT c.id, 'case.worker', 'Initial appointment completed. Jobseeker and accommodation supplement approved.' FROM c
ON CONFLICT DO NOTHING;
