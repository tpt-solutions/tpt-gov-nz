-- Idempotency keys for the ingester.
ALTER TABLE dpmc_honours
    ADD CONSTRAINT dpmc_honours_business_key UNIQUE (citizen_id, award_year);

ALTER TABLE dpmc_engagements
    ADD CONSTRAINT dpmc_engagements_business_key UNIQUE (citizen_id, event_name);

