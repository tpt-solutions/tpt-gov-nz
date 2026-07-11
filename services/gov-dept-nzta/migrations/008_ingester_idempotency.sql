-- Idempotency keys for the ingester.
ALTER TABLE driver_licences
    ADD CONSTRAINT driver_licences_business_key UNIQUE (citizen_id, licence_number);

ALTER TABLE vehicles
    ADD CONSTRAINT vehicles_business_key UNIQUE (citizen_id, registration);

ALTER TABLE ruc_records
    ADD CONSTRAINT ruc_records_business_key UNIQUE (citizen_id, vehicle_rego);
