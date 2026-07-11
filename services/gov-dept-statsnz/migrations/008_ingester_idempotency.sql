-- Idempotency keys for the ingester.
ALTER TABLE statsnz_census
    ADD CONSTRAINT statsnz_census_business_key UNIQUE (citizen_id, census_year);

ALTER TABLE statsnz_profile
    ADD CONSTRAINT statsnz_profile_business_key UNIQUE (citizen_id);
