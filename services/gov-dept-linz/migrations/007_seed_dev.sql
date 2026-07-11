-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, customer_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'LINZ-100001'),
    ('did:gov:nz:test-citizen-002', 'LINZ-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: one title held in full ownership
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO linz_titles (citizen_id, title_number, property_address, land_area_sqm, estate_type)
SELECT c.id, 'LNZ-T-1', '45 Kahu Road, Wellington', 612.5, 'Freehold' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO linz_ownership (citizen_id, title_number, ownership_share, registered_owners)
SELECT c.id, 'LNZ-T-1', '1/1', '["Alex Tane"]'::jsonb FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: one title held as a joint share
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO linz_titles (citizen_id, title_number, property_address, land_area_sqm, estate_type)
SELECT c.id, 'LNZ-T-2', '8 Waiata Place, Hamilton', 845.0, 'Freehold' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO linz_ownership (citizen_id, title_number, ownership_share, registered_owners)
SELECT c.id, 'LNZ-T-2', '1/2', '["Bree Kare", "Sam Kare"]'::jsonb FROM c
ON CONFLICT DO NOTHING;
