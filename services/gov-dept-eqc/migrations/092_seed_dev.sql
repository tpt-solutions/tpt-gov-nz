-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, eqc_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'EQC-100001'),
    ('did:gov:nz:test-citizen-002', 'EQC-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO eqc_claims (citizen_id, reference, property, status, lodged_date) SELECT c.id, 'EQC-2026-007', '12 Totara Street, Porirua', 'assessed', '2026-03-02' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO eqc_cover (citizen_id, property, sum_insured, valid_to) SELECT c.id, '12 Totara Street, Porirua', 350000, '2027-01-01' FROM c
ON CONFLICT DO NOTHING;
