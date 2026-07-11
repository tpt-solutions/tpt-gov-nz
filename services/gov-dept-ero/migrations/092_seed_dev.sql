-- Development seed data — test citizens. Only runs in dev/demo environments.
INSERT INTO citizens (did, ero_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'ERO-100001'),
    ('did:gov:nz:test-citizen-002', 'ERO-100002')
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ero_reviews (citizen_id, school, rating, review_date, next_review) SELECT c.id, 'Porirua College', 'Developing', '2025-09-01', '2027-09-01' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO ero_reports (citizen_id, title, published) SELECT c.id, 'Porirua College annual report', '2025-10-15' FROM c
ON CONFLICT DO NOTHING;
