CREATE TABLE IF NOT EXISTS schemas (
    revision BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    content JSONB NOT NULL DEFAULT '{}'::jsonb,
    required TEXT[] NOT NULL DEFAULT '{}',
    checksum TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (name, version)
);

CREATE INDEX IF NOT EXISTS idx_schemas_name ON schemas (name);
CREATE INDEX IF NOT EXISTS idx_schemas_latest ON schemas (name, revision);
