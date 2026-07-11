-- Idempotency keys for the ingester.
ALTER TABLE oranga_care_placements
    ADD CONSTRAINT oranga_care_placements_business_key UNIQUE (citizen_id, placement_type);

ALTER TABLE oranga_support_services
    ADD CONSTRAINT oranga_support_services_business_key UNIQUE (citizen_id, service);

