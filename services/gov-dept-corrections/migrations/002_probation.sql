CREATE TABLE IF NOT EXISTS corrections_probation (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'completed', 'breached', 'revoked')),
    officer_name    TEXT NOT NULL,
    next_report_date DATE,
    location        TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_corrections_probation_citizen ON corrections_probation (citizen_id);