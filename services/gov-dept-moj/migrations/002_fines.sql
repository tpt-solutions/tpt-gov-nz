CREATE TABLE IF NOT EXISTS moj_fines (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    fine_number     TEXT NOT NULL,
    fine_type       TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'unpaid'
                    CHECK (status IN ('unpaid', 'paid', 'overdue', 'waived')),
    amount          NUMERIC(10,2) NOT NULL,
    offense_date    DATE NOT NULL,
    due_date        DATE NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moj_fines_citizen ON moj_fines (citizen_id);
CREATE UNIQUE INDEX idx_moj_fines_number ON moj_fines (fine_number);