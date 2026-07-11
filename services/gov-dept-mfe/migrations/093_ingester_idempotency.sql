-- Idempotency keys for the ingester.
ALTER TABLE mfe_emissions
    ADD CONSTRAINT mfe_emissions_business_key UNIQUE (citizen_id, report_year);

ALTER TABLE mfe_reports
    ADD CONSTRAINT mfe_reports_business_key UNIQUE (citizen_id, title);

