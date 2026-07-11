CREATE TABLE IF NOT EXISTS doc_permits (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    permit_number   TEXT NOT NULL,
    activity        TEXT NOT NULL,
    location        TEXT NOT NULL,
    status          TEXT NOT NULL CHECK (status IN ('active', 'pending', 'expired', 'revoked')),
    expires_date    DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, permit_number)
);

CREATE INDEX idx_doc_permits_citizen ON doc_permits (citizen_id);
