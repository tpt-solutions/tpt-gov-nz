-- Idempotency keys for the ingester.
ALTER TABLE police_infringements
    ADD CONSTRAINT police_infringements_business_key UNIQUE (citizen_id, ticket_number);

ALTER TABLE police_reports
    ADD CONSTRAINT police_reports_business_key UNIQUE (citizen_id, report_number);
