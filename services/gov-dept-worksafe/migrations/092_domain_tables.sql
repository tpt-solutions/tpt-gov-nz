-- Domain tables referenced by queries but missing from the original migrations.
-- Generated from the row structs and seed data; review column types before production use.

CREATE TABLE IF NOT EXISTS worksafe_inspections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    reference TEXT NOT NULL,
    site TEXT NOT NULL,
    inspection_date DATE NOT NULL,
    outcome TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS worksafe_investigations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    reference TEXT NOT NULL,
    matter TEXT NOT NULL,
    status TEXT NOT NULL,
    opened_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
