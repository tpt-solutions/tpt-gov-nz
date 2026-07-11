-- Idempotency keys for the ingester.
ALTER TABLE worksafe_inspections
    ADD CONSTRAINT worksafe_inspections_business_key UNIQUE (citizen_id, reference);

ALTER TABLE worksafe_investigations
    ADD CONSTRAINT worksafe_investigations_business_key UNIQUE (citizen_id, reference);

