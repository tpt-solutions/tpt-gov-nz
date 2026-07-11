-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, client_number)
VALUES
    ('did:gov:nz:test-citizen-001', 'ACC-100001'),
    ('did:gov:nz:test-citizen-002', 'ACC-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: one open work claim, entitlement, rehab plan
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO acc_claims (citizen_id, claim_number, claim_type, status, injury_date, description, weekly_compensation)
SELECT c.id, 'ACC-5001', 'work', 'open', '2025-02-10', 'Lower back strain', 420 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO acc_entitlements (citizen_id, has_entitlement, type, weekly_amount, remaining_weeks)
SELECT c.id, true, 'Weekly compensation', 420, 18 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO acc_rehabilitation (citizen_id, plan_id, description, status, provider, next_review)
SELECT c.id, 'PLAN-1', 'Physio + return-to-work', 'active', 'Metro Rehab', '2026-01-15' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: one closed claim
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO acc_claims (citizen_id, claim_number, claim_type, status, injury_date, description)
SELECT c.id, 'ACC-6002', 'non-work', 'closed', '2024-08-01', 'Previous sprain (resolved)' FROM c
ON CONFLICT DO NOTHING;
