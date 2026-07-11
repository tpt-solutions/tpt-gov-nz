CREATE TABLE IF NOT EXISTS linz_ownership (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    title_number        TEXT NOT NULL,
    ownership_share     TEXT NOT NULL,
    registered_owners   JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, title_number)
);

CREATE INDEX idx_linz_ownership_citizen ON linz_ownership (citizen_id);
