-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, client_number)
VALUES
    ('did:gov:nz:test-citizen-001', 'POL-100001'),
    ('did:gov:nz:test-citizen-002', 'POL-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: one unpaid speeding infringement, one filed theft report
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO police_infringements (citizen_id, ticket_number, offense_type, status, amount, issue_date, location, demerit_points, description)
SELECT c.id, 'POL-T5001', 'speeding', 'unpaid', 120, '2026-06-01', 'State Highway 1, Wellington', 20, 'Exceeding speed limit by 15km/h' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO police_reports (citizen_id, report_number, report_type, status, filed_date, description)
SELECT c.id, 'POL-R2001', 'theft', 'under-investigation', '2026-05-20', 'Bicycle stolen from outside address' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: one paid parking infringement
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO police_infringements (citizen_id, ticket_number, offense_type, status, amount, issue_date, location, description)
SELECT c.id, 'POL-T6002', 'parking', 'paid', 60, '2026-03-10', 'Lambton Quay', 'Parked in a loading zone' FROM c
ON CONFLICT DO NOTHING;
