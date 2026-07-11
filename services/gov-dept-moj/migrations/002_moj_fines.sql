CREATE TABLE IF NOT EXISTS moj_fines (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id    UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    fine_number   TEXT NOT NULL,
    fine_type     TEXT NOT NULL CHECK (fine_type IN ('traffic', 'reserve', 'court')),
    status        TEXT NOT NULL CHECK (status IN ('unpaid', 'paid', 'overdue', 'payment-plan')),
    amount        DOUBLE PRECISION NOT NULL,
    offense_date  DATE NOT NULL,
    due_date      DATE NOT NULL,
    description   TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, fine_number)
);

CREATE INDEX idx_moj_fines_citizen ON moj_fines (citizen_id);
