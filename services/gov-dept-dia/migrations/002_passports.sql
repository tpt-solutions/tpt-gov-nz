CREATE TABLE IF NOT EXISTS passports (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    passport_number TEXT NOT NULL,
    expiry_date     DATE NOT NULL,
    renewable       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, passport_number)
);

CREATE INDEX idx_passports_citizen ON passports (citizen_id);
