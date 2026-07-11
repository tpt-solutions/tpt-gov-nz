-- Idempotency keys for the ingester.
ALTER TABLE ero_reviews
    ADD CONSTRAINT ero_reviews_business_key UNIQUE (citizen_id, school);

ALTER TABLE ero_reports
    ADD CONSTRAINT ero_reports_business_key UNIQUE (citizen_id, title);

