CREATE TABLE IF NOT EXISTS hud_applications (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    application_number  TEXT NOT NULL,
    application_type    TEXT NOT NULL CHECK (application_type IN ('public-housing', 'emergency-housing', 'home-ownership')),
    status              TEXT NOT NULL CHECK (status IN ('submitted', 'assessed', 'approved', 'declined', 'waitlisted')),
    priority_band       TEXT CHECK (priority_band IN ('A', 'B', 'C', 'D')),
    bedrooms_needed     INTEGER,
    submitted_date      DATE NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, application_number)
);

CREATE INDEX idx_hud_applications_citizen ON hud_applications (citizen_id);
