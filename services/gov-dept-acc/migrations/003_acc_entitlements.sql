CREATE TABLE IF NOT EXISTS acc_entitlements (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    has_entitlement BOOLEAN NOT NULL DEFAULT false,
    type            TEXT,
    weekly_amount   DOUBLE PRECISION,
    remaining_weeks INTEGER,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id)
);

CREATE INDEX idx_acc_entitlements_citizen ON acc_entitlements (citizen_id);
