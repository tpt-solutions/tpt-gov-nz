-- Idempotency keys for the ingester.
ALTER TABLE hud_applications
    ADD CONSTRAINT hud_applications_business_key UNIQUE (citizen_id, application_number);

ALTER TABLE hud_tenancies
    ADD CONSTRAINT hud_tenancies_business_key UNIQUE (citizen_id, tenancy_id);

ALTER TABLE hud_maintenance_requests
    ADD CONSTRAINT hud_maintenance_requests_business_key UNIQUE (citizen_id, request_number);
