-- Idempotency keys for the ingester.
ALTER TABLE sfo_investigations
    ADD CONSTRAINT sfo_investigations_business_key UNIQUE (citizen_id, reference);

ALTER TABLE sfo_outcomes
    ADD CONSTRAINT sfo_outcomes_business_key UNIQUE (citizen_id, reference);

