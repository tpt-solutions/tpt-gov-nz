-- Idempotency keys for the ingester.
ALTER TABLE crownlaw_legal_opinions
    ADD CONSTRAINT crownlaw_legal_opinions_business_key UNIQUE (citizen_id, reference);

ALTER TABLE crownlaw_litigation
    ADD CONSTRAINT crownlaw_litigation_business_key UNIQUE (citizen_id, case_name);

