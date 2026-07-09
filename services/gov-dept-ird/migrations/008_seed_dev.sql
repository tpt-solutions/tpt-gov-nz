-- Development seed data — test citizen "Alex Tane"
-- Only runs in dev/demo environments

INSERT INTO citizens (did, ird_number)
VALUES ('did:gov:nz:test-citizen-001', '123-456-789')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO income_records (citizen_id, assessment_year, employment_income, other_income)
SELECT c.id, 2025, 65000.00, 1200.00 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO tax_assessments (citizen_id, assessment_year, tax_code, total_income, taxable_income, tax_liability, tax_paid, tax_refund_due, tax_owing, assessment_status)
SELECT c.id, 2025, 'M', 66200.00, 66200.00, 14833.00, 15000.00, 167.00, 0.00, 'final' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO gst_registrations (citizen_id, registered)
SELECT c.id, FALSE FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO kiwisaver_memberships (citizen_id, membership_status, contribution_rate, employer_contribution_rate, scheme, total_balance, government_contribution_eligible)
SELECT c.id, 'active', 3.0, 3.0, 'ANZ Default KiwiSaver Scheme', 18500.00, TRUE FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO wff_entitlements (citizen_id, eligible, number_of_dependant_children, income_threshold, family_tax_credit, in_work_tax_credit, payment_frequency)
SELECT c.id, TRUE, 2, 42700.00, 127.00, 72.50, 'weekly' FROM c
ON CONFLICT DO NOTHING;
