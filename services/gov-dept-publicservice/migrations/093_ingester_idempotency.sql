-- Idempotency keys for the ingester.
ALTER TABLE publicservice_workforce
    ADD CONSTRAINT publicservice_workforce_business_key UNIQUE (citizen_id, report_year);

ALTER TABLE publicservice_agency_ratings
    ADD CONSTRAINT publicservice_agency_ratings_business_key UNIQUE (citizen_id, agency);

