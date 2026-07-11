CREATE TABLE IF NOT EXISTS moj_disputes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    dispute_number  TEXT NOT NULL,
    claim_type      TEXT NOT NULL CHECK (claim_type IN ('consumer', 'tenancy', 'debt')),
    status          TEXT NOT NULL CHECK (status IN ('filed', 'scheduled', 'resolved', 'withdrawn')),
    amount_claimed  DOUBLE PRECISION,
    hearing_date    DATE,
    description     TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, dispute_number)
);

CREATE INDEX idx_moj_disputes_citizen ON moj_disputes (citizen_id);
