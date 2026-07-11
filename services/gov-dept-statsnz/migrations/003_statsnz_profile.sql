CREATE TABLE IF NOT EXISTS statsnz_profile (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    data_summary    TEXT NOT NULL,
    record_count    INTEGER NOT NULL,
    last_updated    DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id)
);

CREATE INDEX idx_statsnz_profile_citizen ON statsnz_profile (citizen_id);
