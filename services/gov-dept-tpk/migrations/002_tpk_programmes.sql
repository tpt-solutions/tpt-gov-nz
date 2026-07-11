CREATE TABLE IF NOT EXISTS tpk_programmes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    programme_name  TEXT NOT NULL,
    status          TEXT NOT NULL,
    region          TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, programme_name)
);

CREATE INDEX idx_tpk_programmes_citizen ON tpk_programmes (citizen_id);
