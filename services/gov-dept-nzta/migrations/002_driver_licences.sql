CREATE TABLE IF NOT EXISTS driver_licences (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    licence_number  TEXT NOT NULL,
    full_name       TEXT NOT NULL,
    licence_class   TEXT NOT NULL,
    expiry_date     DATE NOT NULL,
    conditions      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, licence_number)
);

CREATE INDEX idx_driver_licences_citizen ON driver_licences (citizen_id);
