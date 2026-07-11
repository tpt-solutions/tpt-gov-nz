-- Idempotency keys for the ingester.
ALTER TABLE nzqa_qualifications
    ADD CONSTRAINT nzqa_qualifications_business_key UNIQUE (citizen_id, qualification_id);

ALTER TABLE nzqa_transcripts
    ADD CONSTRAINT nzqa_transcripts_business_key UNIQUE (citizen_id);
