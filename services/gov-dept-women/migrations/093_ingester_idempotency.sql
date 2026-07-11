-- Idempotency keys for the ingester.
ALTER TABLE women_programmes
    ADD CONSTRAINT women_programmes_business_key UNIQUE (citizen_id, programme_name);

ALTER TABLE women_insights
    ADD CONSTRAINT women_insights_business_key UNIQUE (citizen_id, topic);

