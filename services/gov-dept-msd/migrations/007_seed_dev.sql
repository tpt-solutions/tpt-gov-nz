-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, client_number)
VALUES
    ('did:gov:nz:test-citizen-001', 'MSD-100001'),
    ('did:gov:nz:test-citizen-002', 'MSD-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: StudyLink loan + allowance + cross-service case history
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO msd_studylink (citizen_id, has_student_loan, loan_balance, repayment_plan, has_allowance, allowance_type, next_payment_date, weekly_amount)
SELECT c.id, true, 28450.75, 'standard', true, 'living-allowance', '2026-07-15', 221.48 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO msd_case_history (citizen_id, event_id, event_date, service_line, summary)
SELECT c.id, 'MSD-EVT-001', '2026-05-02', 'Work and Income', 'Applied for Jobseeker Support' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO msd_case_history (citizen_id, event_id, event_date, service_line, summary)
SELECT c.id, 'MSD-EVT-002', '2026-06-10', 'StudyLink', 'Student loan approved for the 2026 academic year' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: allowance only (no loan)
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO msd_studylink (citizen_id, has_student_loan, has_allowance, allowance_type, next_payment_date, weekly_amount)
SELECT c.id, false, true, 'childcare-allowance', '2026-07-20', 188.00 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO msd_case_history (citizen_id, event_id, event_date, service_line, summary)
SELECT c.id, 'MSD-EVT-100', '2026-04-18', 'Work and Income', 'Resolved benefit overpayment' FROM c
ON CONFLICT DO NOTHING;
