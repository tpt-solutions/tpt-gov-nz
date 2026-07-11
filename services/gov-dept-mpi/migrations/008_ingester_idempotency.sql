-- Idempotency keys for the ingester.
ALTER TABLE mpi_registrations
    ADD CONSTRAINT mpi_registrations_business_key UNIQUE (citizen_id, nzbn);

ALTER TABLE mpi_certifications
    ADD CONSTRAINT mpi_certifications_business_key UNIQUE (citizen_id, cert_number);
