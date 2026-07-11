-- Idempotency keys for the ingester.
ALTER TABLE retirement_retirement_plan
    ADD CONSTRAINT retirement_retirement_plan_business_key UNIQUE (citizen_id);

ALTER TABLE retirement_guidance
    ADD CONSTRAINT retirement_guidance_business_key UNIQUE (citizen_id, topic);

