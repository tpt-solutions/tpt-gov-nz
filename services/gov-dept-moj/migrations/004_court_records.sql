CREATE TABLE IF NOT EXISTS moj_court_records (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    case_number         TEXT NOT NULL,
    case_type           TEXT NOT NULL,
    status              TEXT NOT NULL DEFAULT 'open'
                        CHECK (status IN ('open', 'closed', 'appealed', 'stayed')),
    next_hearing_date   DATE,
    description         TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_moj_court_records_citizen ON moj_court_records (citizen_id);
CREATE UNIQUE INDEX idx_moj_court_records_number ON moj_court_records (case_number);