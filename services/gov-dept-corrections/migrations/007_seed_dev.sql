-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, corrections_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'COR-100001'),
    ('did:gov:nz:test-citizen-002', 'COR-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: active probation + one case
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO corrections_probation (citizen_id, status, officer_name, next_report_date, location)
SELECT c.id, 'active', 'Officer R. Reedy', '2026-08-15', 'Auckland Probation Hub' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO corrections_case (citizen_id, case_number, sentence_type, start_date, end_date, summary)
SELECT c.id, 'COR-C0001', 'supervision', '2025-03-01', '2027-03-01', 'Supervision sentence following guilty plea to careless use of a vehicle.' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: active probation only, no case
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO corrections_probation (citizen_id, status, officer_name, next_report_date, location)
SELECT c.id, 'active', 'Officer M. Wiki', '2026-09-02', 'Wellington Probation Hub' FROM c
ON CONFLICT DO NOTHING;
