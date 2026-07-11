CREATE TABLE IF NOT EXISTS msd_case_history (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id   UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    event_id     TEXT NOT NULL,
    event_date   DATE NOT NULL,
    service_line TEXT NOT NULL,
    summary      TEXT NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_msd_case_history_citizen ON msd_case_history (citizen_id);
