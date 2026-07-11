-- Idempotency keys for the ingester.
ALTER TABLE tec_funding
    ADD CONSTRAINT tec_funding_business_key UNIQUE (citizen_id, provider);

ALTER TABLE tec_courses
    ADD CONSTRAINT tec_courses_business_key UNIQUE (citizen_id, course_name);

