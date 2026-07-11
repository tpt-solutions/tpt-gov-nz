-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, women_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'WOM-100001'),
    ('did:gov:nz:test-citizen-002', 'WOM-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO women_programmes (citizen_id, programme_name, status, year) SELECT c.id, 'Women in Governance', 'enrolled', 2026 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO women_insights (citizen_id, topic, summary, published) SELECT c.id, 'Pay equity', 'Progress on gender pay gap reporting.', '2026-03-08' FROM c
ON CONFLICT DO NOTHING;
