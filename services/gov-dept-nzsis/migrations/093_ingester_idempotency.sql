-- Idempotency keys for the ingester.
ALTER TABLE nzsis_mandates
    ADD CONSTRAINT nzsis_mandates_business_key UNIQUE (citizen_id, reference);

ALTER TABLE nzsis_threats
    ADD CONSTRAINT nzsis_threats_business_key UNIQUE (citizen_id, reference);

