CREATE TABLE IF NOT EXISTS statsnz_census (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    census_year     INTEGER NOT NULL,
    dwelling_type   TEXT NOT NULL,
    household_size  INTEGER NOT NULL,
    region          TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, census_year)
);

CREATE INDEX idx_statsnz_census_citizen ON statsnz_census (citizen_id);
