CREATE TABLE IF NOT EXISTS citizens (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did           TEXT UNIQUE NOT NULL,
    stats_id      TEXT UNIQUE NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_citizens_did ON citizens (did);

-- Stats NZ does not use a separate sequential client number; the department-issued
-- local id (statsId) lives on the citizens table.
