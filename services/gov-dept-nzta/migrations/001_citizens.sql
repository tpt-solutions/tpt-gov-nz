CREATE TABLE IF NOT EXISTS citizens (
    id                    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did                   TEXT UNIQUE NOT NULL,
    driver_licence_number TEXT UNIQUE NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_citizens_did ON citizens (did);
