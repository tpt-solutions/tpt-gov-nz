-- Idempotency keys for the ingester.
ALTER TABLE mbie_business_registrations
    ADD CONSTRAINT mbie_business_registrations_business_key UNIQUE (citizen_id, nzbn);

ALTER TABLE mbie_directorships
    ADD CONSTRAINT mbie_directorships_business_key UNIQUE (citizen_id, nzbn, appointed_date);
