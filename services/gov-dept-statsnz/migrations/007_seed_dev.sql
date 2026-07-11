-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, stats_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'STATS-100001'),
    ('did:gov:nz:test-citizen-002', 'STATS-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: 2023 census record + data profile
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO statsnz_census (citizen_id, census_year, dwelling_type, household_size, region)
SELECT c.id, 2023, 'house', 4, 'Auckland' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO statsnz_profile (citizen_id, data_summary, record_count, last_updated)
SELECT c.id, '2023 Census response recorded.', 1, '2026-06-01' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: profile only (no census response)
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO statsnz_profile (citizen_id, data_summary, record_count, last_updated)
SELECT c.id, 'No census response on file.', 0, '2026-06-01' FROM c
ON CONFLICT DO NOTHING;
