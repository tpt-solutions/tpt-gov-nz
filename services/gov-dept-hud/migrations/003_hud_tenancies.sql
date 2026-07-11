CREATE TABLE IF NOT EXISTS hud_tenancies (
    id                    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id            UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    tenancy_id            TEXT NOT NULL,
    property_address      TEXT NOT NULL,
    weekly_rent           DOUBLE PRECISION NOT NULL,
    income_related_rent   BOOLEAN NOT NULL DEFAULT false,
    start_date            DATE NOT NULL,
    status                TEXT NOT NULL CHECK (status IN ('active', 'ended')),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, tenancy_id)
);

CREATE INDEX idx_hud_tenancies_citizen ON hud_tenancies (citizen_id);
