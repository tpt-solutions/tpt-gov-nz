CREATE TABLE IF NOT EXISTS police_reports (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    report_number   TEXT NOT NULL,
    report_type     TEXT NOT NULL CHECK (report_type IN ('theft', 'incident', 'lost-property')),
    status          TEXT NOT NULL CHECK (status IN ('filed', 'under-investigation', 'closed')),
    filed_date      DATE NOT NULL,
    description     TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, report_number)
);

CREATE INDEX idx_police_reports_citizen ON police_reports (citizen_id);
