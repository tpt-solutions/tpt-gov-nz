CREATE TABLE IF NOT EXISTS did_documents (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did             TEXT UNIQUE NOT NULL,
    public_key_b64  TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_did_documents_did ON did_documents (did);
