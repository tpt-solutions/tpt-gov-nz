-- Idempotency keys for the ingester.
ALTER TABLE moe_enrolment
    ADD CONSTRAINT moe_enrolment_business_key UNIQUE (citizen_id);

ALTER TABLE moe_student_support
    ADD CONSTRAINT moe_student_support_business_key UNIQUE (citizen_id, service);

