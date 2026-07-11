-- Idempotency keys for the ingester.
ALTER TABLE mch_heritage_sites
    ADD CONSTRAINT mch_heritage_sites_business_key UNIQUE (citizen_id, name);

ALTER TABLE mch_grants
    ADD CONSTRAINT mch_grants_business_key UNIQUE (citizen_id, grant_name);

