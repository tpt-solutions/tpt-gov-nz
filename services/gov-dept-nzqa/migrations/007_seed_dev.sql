-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, nsn)
VALUES
    ('did:gov:nz:test-citizen-001', 'NSN-100001'),
    ('did:gov:nz:test-citizen-002', 'NSN-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: a completed Bachelor degree + full Record of Achievement
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzqa_qualifications (citizen_id, qualification_id, title, level, awarded_date, provider)
SELECT c.id, 'NZQA-Q1', 'Bachelor of Science', 7, '2024-12-10', 'University of Auckland' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO nzqa_transcripts (citizen_id, record_summary, total_credits, credit_summary)
SELECT c.id, 'Full Record of Achievement on file.', 360, 'Level 7: 360 credits' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: a National Certificate only, no transcript yet
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO nzqa_qualifications (citizen_id, qualification_id, title, level, awarded_date, provider)
SELECT c.id, 'NZQA-Q2', 'National Certificate in Computing', 4, '2023-06-01', 'Te Pukenga' FROM c
ON CONFLICT DO NOTHING;
