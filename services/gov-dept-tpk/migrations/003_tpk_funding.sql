CREATE TABLE IF NOT EXISTS tpk_funding (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id  UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    grant_id    TEXT NOT NULL,
    amount      INTEGER NOT NULL,
    purpose     TEXT NOT NULL,
    status      TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, grant_id)
);

CREATE INDEX idx_tpk_funding_citizen ON tpk_funding (citizen_id);
