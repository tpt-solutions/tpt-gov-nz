-- Idempotency keys for the ingester.
ALTER TABLE defence_procurements
    ADD CONSTRAINT defence_procurements_business_key UNIQUE (citizen_id, programme);

ALTER TABLE defence_bases
    ADD CONSTRAINT defence_bases_business_key UNIQUE (citizen_id, name);

