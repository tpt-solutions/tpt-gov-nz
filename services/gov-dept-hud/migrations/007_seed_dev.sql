-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, client_number)
VALUES
    ('did:gov:nz:test-citizen-001', 'HUD-100001'),
    ('did:gov:nz:test-citizen-002', 'HUD-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: waitlisted application + active tenancy + open maintenance request
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO hud_applications (citizen_id, application_number, application_type, status, priority_band, bedrooms_needed, submitted_date)
SELECT c.id, 'HUD-A5001', 'public-housing', 'waitlisted', 'B', 2, '2026-01-15' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO hud_tenancies (citizen_id, tenancy_id, property_address, weekly_rent, income_related_rent, start_date, status)
SELECT c.id, 'HUD-TEN-1', '12 Totara Street, Porirua', 180, true, '2025-11-01', 'active' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO hud_maintenance_requests (citizen_id, request_number, category, status, description, requested_date)
SELECT c.id, 'HUD-M3001', 'plumbing', 'scheduled', 'Leaking kitchen tap', '2026-06-20' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: approved home-ownership application only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO hud_applications (citizen_id, application_number, application_type, status, submitted_date)
SELECT c.id, 'HUD-A6002', 'home-ownership', 'approved', '2025-09-01' FROM c
ON CONFLICT DO NOTHING;
