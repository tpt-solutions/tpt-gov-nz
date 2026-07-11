-- Idempotency keys for the ingester.
ALTER TABLE tpk_programmes
    ADD CONSTRAINT tpk_programmes_business_key UNIQUE (citizen_id, programme_name);

ALTER TABLE tpk_funding
    ADD CONSTRAINT tpk_funding_business_key UNIQUE (citizen_id, grant_id);
