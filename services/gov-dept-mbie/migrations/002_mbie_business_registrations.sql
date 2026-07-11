CREATE TABLE IF NOT EXISTS mbie_business_registrations (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id       UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    nzbn             TEXT NOT NULL,
    entity_name      TEXT NOT NULL,
    entity_type      TEXT NOT NULL CHECK (entity_type IN ('company', 'sole-trader', 'partnership', 'trust')),
    status           TEXT NOT NULL,
    registered_date  DATE NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, nzbn)
);

CREATE INDEX idx_mbie_business_registrations_citizen ON mbie_business_registrations (citizen_id);
