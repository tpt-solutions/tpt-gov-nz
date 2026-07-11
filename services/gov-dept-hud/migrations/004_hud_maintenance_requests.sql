CREATE TABLE IF NOT EXISTS hud_maintenance_requests (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    request_number  TEXT NOT NULL,
    category        TEXT NOT NULL CHECK (category IN ('plumbing', 'electrical', 'heating', 'structural', 'other')),
    status          TEXT NOT NULL CHECK (status IN ('submitted', 'scheduled', 'completed')),
    description     TEXT NOT NULL,
    requested_date  DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, request_number)
);

CREATE INDEX idx_hud_maintenance_citizen ON hud_maintenance_requests (citizen_id);
