CREATE TABLE IF NOT EXISTS ruc_records (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    vehicle_rego    TEXT NOT NULL,
    licence_type    TEXT NOT NULL,
    expiry_date     DATE NOT NULL,
    units_remaining INTEGER NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, vehicle_rego)
);

CREATE INDEX idx_ruc_records_citizen ON ruc_records (citizen_id);
