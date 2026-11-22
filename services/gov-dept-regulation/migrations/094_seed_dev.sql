-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, regulation_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'REG-100001'),
    ('did:gov:nz:test-citizen-002', 'REG-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO regulation_regulatory_reviews (citizen_id, topic, agency, status, review_year) SELECT c.id, 'Building consenting', 'MBIE', 'in-progress', 2026 FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO regulation_proposals (citizen_id, title, status) SELECT c.id, 'Reduce duplicate reporting', 'consultation' FROM c
ON CONFLICT DO NOTHING;
