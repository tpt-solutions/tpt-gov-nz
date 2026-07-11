-- Idempotency keys for the ingester.
ALTER TABLE fenz_fire_safety
    ADD CONSTRAINT fenz_fire_safety_business_key UNIQUE (citizen_id);

ALTER TABLE fenz_incidents
    ADD CONSTRAINT fenz_incidents_business_key UNIQUE (citizen_id, reference);

