CREATE TABLE IF NOT EXISTS benefits (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    benefit_type    TEXT NOT NULL,
    weekly_amount   NUMERIC(10,2) NOT NULL DEFAULT 0,
    start_date      DATE,
    review_date     DATE,
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'suspended', 'closed')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, benefit_type)
);

CREATE INDEX idx_benefits_citizen ON benefits (citizen_id);
