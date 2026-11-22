-- Idempotency keys for the ingester.
ALTER TABLE caa_licences
    ADD CONSTRAINT caa_licences_business_key UNIQUE (citizen_id, licence_no);

ALTER TABLE caa_aircraft
    ADD CONSTRAINT caa_aircraft_business_key UNIQUE (citizen_id, registration);

