CREATE TABLE IF NOT EXISTS appointments (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id  UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    provider    TEXT NOT NULL,
    appt_date   TIMESTAMPTZ NOT NULL,
    type        TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'booked'
                CHECK (status IN ('booked', 'completed', 'cancelled')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_appointments_citizen ON appointments (citizen_id, appt_date);
