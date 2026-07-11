-- Idempotency keys for the ingester.
ALTER TABLE mot_strategies
    ADD CONSTRAINT mot_strategies_business_key UNIQUE (citizen_id, title);

ALTER TABLE mot_programmes
    ADD CONSTRAINT mot_programmes_business_key UNIQUE (citizen_id, name);

