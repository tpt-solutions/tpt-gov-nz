CREATE TABLE IF NOT EXISTS acc_claims (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    claim_number        TEXT NOT NULL,
    claim_type          TEXT NOT NULL CHECK (claim_type IN ('work', 'non-work', 'treatment')),
    status              TEXT NOT NULL CHECK (status IN ('open', 'approved', 'declined', 'closed')),
    injury_date         DATE NOT NULL,
    description         TEXT NOT NULL,
    weekly_compensation DOUBLE PRECISION,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, claim_number)
);

CREATE INDEX idx_acc_claims_citizen ON acc_claims (citizen_id);
