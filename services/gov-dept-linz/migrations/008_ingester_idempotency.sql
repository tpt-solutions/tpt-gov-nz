-- Idempotency keys for the ingester.
ALTER TABLE linz_titles
    ADD CONSTRAINT linz_titles_business_key UNIQUE (citizen_id, title_number);

ALTER TABLE linz_ownership
    ADD CONSTRAINT linz_ownership_business_key UNIQUE (citizen_id, title_number);
