CREATE TABLE IF NOT EXISTS gp_enrolments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    practice_name   TEXT NOT NULL,
    address         TEXT NOT NULL,
    phone           TEXT NOT NULL,
    enrolled_at     DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, practice_name)
);

CREATE INDEX idx_gp_enrolments_citizen ON gp_enrolments (citizen_id);
