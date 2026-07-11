-- Idempotency keys for the ingester.
ALTER TABLE doc_permits
    ADD CONSTRAINT doc_permits_business_key UNIQUE (citizen_id, permit_number);

ALTER TABLE doc_concessions
    ADD CONSTRAINT doc_concessions_business_key UNIQUE (citizen_id, concession_id);
