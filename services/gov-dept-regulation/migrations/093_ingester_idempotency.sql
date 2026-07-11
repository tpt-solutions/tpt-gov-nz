-- Idempotency keys for the ingester.
ALTER TABLE regulation_regulatory_reviews
    ADD CONSTRAINT regulation_regulatory_reviews_business_key UNIQUE (citizen_id, topic);

ALTER TABLE regulation_proposals
    ADD CONSTRAINT regulation_proposals_business_key UNIQUE (citizen_id, title);

