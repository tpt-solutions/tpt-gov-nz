-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, crownlaw_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'CL-100001'),
    ('did:gov:nz:test-citizen-002', 'CL-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO crownlaw_legal_opinions (citizen_id, reference, topic, issued_date, status) SELECT c.id, 'CL-OP-2026-001', 'Treaty settlement wording', '2026-02-18', 'final' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO crownlaw_litigation (citizen_id, case_name, crown_role, status) SELECT c.id, 'Re Crown assets', 'Defendant', 'ongoing' FROM c
ON CONFLICT DO NOTHING;
