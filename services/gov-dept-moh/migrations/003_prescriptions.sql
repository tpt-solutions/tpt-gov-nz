CREATE TABLE IF NOT EXISTS prescriptions (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id        UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    medication        TEXT NOT NULL,
    dose              TEXT NOT NULL,
    repeats_total     INTEGER NOT NULL DEFAULT 0,
    repeats_remaining INTEGER NOT NULL DEFAULT 0,
    issued_at         DATE NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_prescriptions_citizen ON prescriptions (citizen_id);
