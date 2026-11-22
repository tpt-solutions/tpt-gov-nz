-- Domain tables referenced by queries but missing from the original migrations.
-- Generated from the row structs and seed data; review column types before production use.

CREATE TABLE IF NOT EXISTS fenz_fire_safety (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    property TEXT NOT NULL,
    grade TEXT NOT NULL,
    last_inspection DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS fenz_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    reference TEXT NOT NULL,
    incident_type TEXT NOT NULL,
    incident_date DATE NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
