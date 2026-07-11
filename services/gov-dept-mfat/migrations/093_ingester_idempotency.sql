-- Idempotency keys for the ingester.
ALTER TABLE mfat_overseas_missions
    ADD CONSTRAINT mfat_overseas_missions_business_key UNIQUE (citizen_id, country);

ALTER TABLE mfat_travel_advisories
    ADD CONSTRAINT mfat_travel_advisories_business_key UNIQUE (citizen_id, country);

