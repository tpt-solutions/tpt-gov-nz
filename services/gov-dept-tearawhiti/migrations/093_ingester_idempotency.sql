-- Idempotency keys for the ingester.
ALTER TABLE tearawhiti_treaty_settlements
    ADD CONSTRAINT tearawhiti_treaty_settlements_business_key UNIQUE (citizen_id, iwi);

ALTER TABLE tearawhiti_engagements
    ADD CONSTRAINT tearawhiti_engagements_business_key UNIQUE (citizen_id, topic);

