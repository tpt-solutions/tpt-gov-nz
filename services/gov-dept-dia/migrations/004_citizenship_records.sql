CREATE TABLE IF NOT EXISTS citizenship_records (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    status              TEXT NOT NULL
                        CHECK (status IN ('citizen-by-birth', 'citizen-by-grant', 'permanent-resident', 'other')),
    certificate_number  TEXT,
    granted_at          DATE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, status)
);

CREATE INDEX idx_citizenship_citizen ON citizenship_records (citizen_id);
