-- Idempotency keys for the ingester.
ALTER TABLE corrections_probation
    ADD CONSTRAINT corrections_probation_business_key UNIQUE (citizen_id);

ALTER TABLE corrections_case
    ADD CONSTRAINT corrections_case_business_key UNIQUE (citizen_id, case_number);
