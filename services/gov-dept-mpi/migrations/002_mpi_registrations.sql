CREATE TABLE IF NOT EXISTS mpi_registrations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    nzbn            TEXT NOT NULL,
    business_name   TEXT NOT NULL,
    type            TEXT NOT NULL,
    status          TEXT NOT NULL,
    registered_date DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, nzbn)
);

CREATE INDEX idx_mpi_registrations_citizen ON mpi_registrations (citizen_id);
