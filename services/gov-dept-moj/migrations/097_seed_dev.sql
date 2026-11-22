-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, client_number)
VALUES
    ('did:gov:nz:test-citizen-001', 'MOJ-100001'),
    ('did:gov:nz:test-citizen-002', 'MOJ-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: one unpaid fine, one filed dispute, one open court case
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO moj_fines (citizen_id, fine_number, fine_type, status, amount, offense_date, due_date, description)
SELECT c.id, 'MOJ-F5001', 'traffic', 'unpaid', 150, '2026-05-01', '2026-06-15', 'Speeding 15km/h over limit' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO moj_disputes (citizen_id, dispute_number, claim_type, status, amount_claimed, hearing_date, description)
SELECT c.id, 'MOJ-D2001', 'tenancy', 'filed', 1200, '2026-08-20', 'Bond dispute with former landlord' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO moj_court_records (citizen_id, case_number, case_type, status, next_hearing_date, description)
SELECT c.id, 'MOJ-C3001', 'traffic', 'open', '2026-09-10', 'Careless driving charge' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: one paid fine
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO moj_fines (citizen_id, fine_number, fine_type, status, amount, offense_date, due_date, description)
SELECT c.id, 'MOJ-F6002', 'reserve', 'paid', 80, '2026-01-05', '2026-02-01', 'Reserve parking infringement' FROM c
ON CONFLICT DO NOTHING;
