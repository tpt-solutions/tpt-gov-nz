CREATE TABLE IF NOT EXISTS vaccinations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    vaccine         TEXT NOT NULL,
    vaccine_date    DATE NOT NULL,
    due_for_booster BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vaccinations_citizen ON vaccinations (citizen_id);
