-- Idempotency keys for the ingester.
ALTER TABLE eqc_claims
    ADD CONSTRAINT eqc_claims_business_key UNIQUE (citizen_id, reference);

ALTER TABLE eqc_cover
    ADD CONSTRAINT eqc_cover_business_key UNIQUE (citizen_id);

