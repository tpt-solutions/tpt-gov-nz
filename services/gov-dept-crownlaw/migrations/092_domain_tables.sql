-- Domain tables referenced by queries but missing from the original migrations.
-- Generated from the row structs and seed data; review column types before production use.

CREATE TABLE IF NOT EXISTS crownlaw_legal_opinions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    reference TEXT NOT NULL,
    topic TEXT NOT NULL,
    issued_date DATE NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crownlaw_litigation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    case_name TEXT NOT NULL,
    crown_role TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
