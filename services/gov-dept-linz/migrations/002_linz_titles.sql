CREATE TABLE IF NOT EXISTS linz_titles (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id        UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    title_number      TEXT NOT NULL,
    property_address  TEXT NOT NULL,
    land_area_sqm     DOUBLE PRECISION NOT NULL,
    estate_type       TEXT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, title_number)
);

CREATE INDEX idx_linz_titles_citizen ON linz_titles (citizen_id);
