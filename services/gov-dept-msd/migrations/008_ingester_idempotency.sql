-- Idempotency keys for the ingester.
ALTER TABLE msd_case_history
    ADD CONSTRAINT msd_case_history_business_key UNIQUE (citizen_id, event_id);

ALTER TABLE msd_studylink
    ADD CONSTRAINT msd_studylink_business_key UNIQUE (citizen_id);
