-- Development seed data — test citizens.
-- Only runs in dev/demo environments.

INSERT INTO citizens (did, mpi_id)
VALUES
    ('did:gov:nz:test-citizen-001', 'MPI-100001'),
    ('did:gov:nz:test-citizen-002', 'MPI-100002')
ON CONFLICT DO NOTHING;

-- test-citizen-001: a registered food business + an export certificate
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mpi_registrations (citizen_id, nzbn, business_name, type, status, registered_date)
SELECT c.id, '9429046000000', 'Tane Orchards Ltd', 'food-business', 'registered', '2024-03-12' FROM c
ON CONFLICT DO NOTHING;

WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-001')
INSERT INTO mpi_certifications (citizen_id, cert_number, category, issued_date, expires_date)
SELECT c.id, 'MPI-CERT-2026-001', 'export-certificate', '2026-01-10', '2027-01-09' FROM c
ON CONFLICT DO NOTHING;

-- test-citizen-002: an export certificate only
WITH c AS (SELECT id FROM citizens WHERE did = 'did:gov:nz:test-citizen-002')
INSERT INTO mpi_certifications (citizen_id, cert_number, category, issued_date, expires_date)
SELECT c.id, 'MPI-CERT-2026-002', 'export-certificate', '2025-11-05', '2026-11-04' FROM c
ON CONFLICT DO NOTHING;
