-- Idempotency keys for the ingester.
ALTER TABLE nzdf_service_records
    ADD CONSTRAINT nzdf_service_records_business_key UNIQUE (citizen_id, service_no);

ALTER TABLE nzdf_deployments
    ADD CONSTRAINT nzdf_deployments_business_key UNIQUE (citizen_id, operation);

