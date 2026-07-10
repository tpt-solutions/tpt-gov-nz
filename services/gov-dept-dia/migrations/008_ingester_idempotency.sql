-- Idempotency keys for the ingester.
ALTER TABLE passports
    ADD CONSTRAINT passports_business_key UNIQUE (citizen_id, passport_number);

ALTER TABLE birth_certs
    ADD CONSTRAINT birth_certs_business_key UNIQUE (citizen_id, certificate_number);

ALTER TABLE citizenship_records
    ADD CONSTRAINT citizenship_business_key UNIQUE (citizen_id, status);
