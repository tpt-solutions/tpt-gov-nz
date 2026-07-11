CREATE TABLE IF NOT EXISTS acc_rehabilitation (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    plan_id         TEXT NOT NULL,
    description     TEXT NOT NULL,
    status          TEXT NOT NULL,
    provider        TEXT,
    next_review     DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, plan_id)
);

CREATE INDEX idx_acc_rehab_citizen ON acc_rehabilitation (citizen_id);
