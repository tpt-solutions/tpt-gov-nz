CREATE TABLE IF NOT EXISTS mpi_certifications (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    cert_number     TEXT NOT NULL,
    category        TEXT NOT NULL,
    issued_date     DATE NOT NULL,
    expires_date    DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, cert_number)
);

CREATE INDEX idx_mpi_certifications_citizen ON mpi_certifications (citizen_id);
