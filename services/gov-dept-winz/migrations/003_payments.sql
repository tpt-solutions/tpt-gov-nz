CREATE TABLE IF NOT EXISTS payments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    benefit_type    TEXT NOT NULL,
    payment_date    DATE NOT NULL,
    amount          NUMERIC(10,2) NOT NULL,
    method          TEXT DEFAULT 'bank-deposit',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_citizen_date ON payments (citizen_id, payment_date DESC);
