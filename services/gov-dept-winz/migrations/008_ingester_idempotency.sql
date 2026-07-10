-- Idempotency key for the ingester: payments are upserted on a natural business
-- key so re-running a batch never duplicates a payment row.
ALTER TABLE payments
    ADD CONSTRAINT payments_business_key
        UNIQUE (citizen_id, benefit_type, payment_date, amount);
