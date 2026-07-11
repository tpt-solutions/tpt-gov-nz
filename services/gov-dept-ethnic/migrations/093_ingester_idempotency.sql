-- Idempotency keys for the ingester.
ALTER TABLE ethnic_programmes
    ADD CONSTRAINT ethnic_programmes_business_key UNIQUE (citizen_id, programme_name);

ALTER TABLE ethnic_community_grants
    ADD CONSTRAINT ethnic_community_grants_business_key UNIQUE (citizen_id, grant_name);

