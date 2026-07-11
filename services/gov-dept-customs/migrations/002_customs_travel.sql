CREATE TABLE IF NOT EXISTS customs_travel (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    passport_number     TEXT NOT NULL,
    last_arrival        DATE NOT NULL,
    arrival_port        TEXT NOT NULL,
    frequent_traveller  BOOLEAN NOT NULL DEFAULT false,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id)
);

CREATE INDEX idx_customs_travel_citizen ON customs_travel (citizen_id);
