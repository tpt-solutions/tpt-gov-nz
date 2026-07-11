-- Idempotency keys for the ingester.
ALTER TABLE moj_fines
    ADD CONSTRAINT moj_fines_business_key UNIQUE (citizen_id, fine_number);

ALTER TABLE moj_disputes
    ADD CONSTRAINT moj_disputes_business_key UNIQUE (citizen_id, dispute_number);

ALTER TABLE moj_court_records
    ADD CONSTRAINT moj_court_records_business_key UNIQUE (citizen_id, case_number);
