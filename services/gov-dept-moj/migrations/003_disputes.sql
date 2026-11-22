CREATE TABLE IF NOT EXISTS moj_disputes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    dispute_number  TEXT NOT NULL,
    claim_type      TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'filed'
                    CHECK (status IN ('filed', 'in_progress', 'resolved', 'dismissed')),
    amount_claimed  NUMERIC(10,2) NOT NULL,
    hearing_date    DATE,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moj_disputes_citizen ON moj_disputes (citizen_id);
CREATE UNIQUE INDEX idx_moj_disputes_number ON moj_disputes (dispute_number);