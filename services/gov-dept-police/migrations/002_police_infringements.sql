CREATE TABLE IF NOT EXISTS police_infringements (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    ticket_number   TEXT NOT NULL,
    offense_type    TEXT NOT NULL CHECK (offense_type IN ('speeding', 'parking', 'other')),
    status          TEXT NOT NULL CHECK (status IN ('unpaid', 'paid', 'disputed')),
    amount          DOUBLE PRECISION NOT NULL,
    issue_date      DATE NOT NULL,
    location        TEXT,
    demerit_points  INTEGER,
    description     TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, ticket_number)
);

CREATE INDEX idx_police_infringements_citizen ON police_infringements (citizen_id);
