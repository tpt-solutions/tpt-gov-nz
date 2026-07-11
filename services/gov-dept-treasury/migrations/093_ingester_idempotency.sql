-- Idempotency keys for the ingester.
ALTER TABLE treasury_budget
    ADD CONSTRAINT treasury_budget_business_key UNIQUE (citizen_id, fiscal_year);

ALTER TABLE treasury_economic_outlook
    ADD CONSTRAINT treasury_economic_outlook_business_key UNIQUE (citizen_id);

