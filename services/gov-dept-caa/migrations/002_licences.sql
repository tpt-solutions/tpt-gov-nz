CREATE TABLE IF NOT EXISTS caa_licences (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    licence_no      TEXT NOT NULL,
    category        TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'current'
                    CHECK (status IN ('current', 'suspended', 'revoked', 'expired')),
    expires         DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_caa_licences_citizen ON caa_licences (citizen_id);
CREATE UNIQUE INDEX idx_caa_licences_no ON caa_licences (licence_no);