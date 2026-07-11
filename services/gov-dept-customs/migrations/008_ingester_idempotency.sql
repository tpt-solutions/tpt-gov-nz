-- Idempotency keys for the ingester.
ALTER TABLE customs_declarations
    ADD CONSTRAINT customs_declarations_business_key UNIQUE (citizen_id, declaration_id);

ALTER TABLE customs_travel
    ADD CONSTRAINT customs_travel_business_key UNIQUE (citizen_id);
