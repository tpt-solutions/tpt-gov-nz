-- Idempotency keys for the ingester.
ALTER TABLE pacific_programmes
    ADD CONSTRAINT pacific_programmes_business_key UNIQUE (citizen_id, programme_name);

ALTER TABLE pacific_language_services
    ADD CONSTRAINT pacific_language_services_business_key UNIQUE (citizen_id, service);

