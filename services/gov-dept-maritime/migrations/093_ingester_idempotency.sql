-- Idempotency keys for the ingester.
ALTER TABLE maritime_vessels
    ADD CONSTRAINT maritime_vessels_business_key UNIQUE (citizen_id, vessel_name);

ALTER TABLE maritime_incidents
    ADD CONSTRAINT maritime_incidents_business_key UNIQUE (citizen_id, reference);

