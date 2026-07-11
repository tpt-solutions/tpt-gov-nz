-- Idempotency keys for the ingester.
ALTER TABLE gcsb_mandates
    ADD CONSTRAINT gcsb_mandates_business_key UNIQUE (citizen_id, reference);

ALTER TABLE gcsb_engagements
    ADD CONSTRAINT gcsb_engagements_business_key UNIQUE (citizen_id, partner);

