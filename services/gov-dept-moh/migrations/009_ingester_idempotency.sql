-- Idempotency keys for the ingester: prescription / appointment / vaccination rows
-- are upserted on natural business keys so re-running a batch never duplicates them.
ALTER TABLE prescriptions
    ADD CONSTRAINT prescriptions_business_key
        UNIQUE (citizen_id, medication, issued_at);

ALTER TABLE appointments
    ADD CONSTRAINT appointments_business_key
        UNIQUE (citizen_id, provider, appt_date);

ALTER TABLE vaccinations
    ADD CONSTRAINT vaccinations_business_key
        UNIQUE (citizen_id, vaccine, vaccine_date);
