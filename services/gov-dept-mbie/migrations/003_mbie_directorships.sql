CREATE TABLE IF NOT EXISTS mbie_directorships (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    nzbn            TEXT NOT NULL,
    entity_name     TEXT NOT NULL,
    role            TEXT NOT NULL,
    appointed_date  DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, nzbn, appointed_date)
);

CREATE INDEX idx_mbie_directorships_citizen ON mbie_directorships (citizen_id);
