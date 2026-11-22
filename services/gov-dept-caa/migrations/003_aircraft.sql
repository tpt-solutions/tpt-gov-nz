CREATE TABLE IF NOT EXISTS caa_aircraft (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    registration    TEXT NOT NULL,
    aircraft_type   TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'registered'
                    CHECK (status IN ('registered', 'deregistered', 'suspended')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_caa_aircraft_citizen ON caa_aircraft (citizen_id);
CREATE UNIQUE INDEX idx_caa_aircraft_registration ON caa_aircraft (registration);