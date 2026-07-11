-- Idempotency keys for the ingester.
ALTER TABLE acc_claims
    ADD CONSTRAINT acc_claims_business_key UNIQUE (citizen_id, claim_number);

ALTER TABLE acc_entitlements
    ADD CONSTRAINT acc_entitlements_business_key UNIQUE (citizen_id);

ALTER TABLE acc_rehabilitation
    ADD CONSTRAINT acc_rehabilitation_business_key UNIQUE (citizen_id, plan_id);
