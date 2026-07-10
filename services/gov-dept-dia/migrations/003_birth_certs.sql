CREATE TABLE IF NOT EXISTS birth_certs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    certificate_number  TEXT NOT NULL,
    date_of_birth       DATE NOT NULL,
    place_of_birth      TEXT NOT NULL,
    parents             TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, certificate_number)
);

CREATE INDEX idx_birth_certs_citizen ON birth_certs (citizen_id);
