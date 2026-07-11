CREATE TABLE IF NOT EXISTS moj_court_records (
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id         UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    case_number        TEXT NOT NULL,
    case_type          TEXT NOT NULL CHECK (case_type IN ('traffic', 'civil', 'family')),
    status             TEXT NOT NULL CHECK (status IN ('open', 'closed')),
    next_hearing_date  DATE,
    description        TEXT NOT NULL,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, case_number)
);

CREATE INDEX idx_moj_court_records_citizen ON moj_court_records (citizen_id);
