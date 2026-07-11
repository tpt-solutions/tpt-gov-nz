CREATE TABLE IF NOT EXISTS corrections_case (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id    UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    case_number   TEXT NOT NULL,
    sentence_type TEXT NOT NULL,
    start_date    DATE NOT NULL,
    end_date      DATE,
    summary       TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_corrections_case_citizen ON corrections_case (citizen_id);
