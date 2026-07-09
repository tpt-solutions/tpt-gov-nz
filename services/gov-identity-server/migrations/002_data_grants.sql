CREATE TABLE IF NOT EXISTS data_grants (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_did         TEXT NOT NULL,
    requesting_dept_id  TEXT NOT NULL,
    providing_dept_id   TEXT NOT NULL,
    scopes              TEXT[] NOT NULL,
    issued_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at          TIMESTAMPTZ NOT NULL,
    revoked_at          TIMESTAMPTZ,
    signature           TEXT NOT NULL
);

CREATE INDEX idx_data_grants_citizen_did ON data_grants (citizen_did);
CREATE INDEX idx_data_grants_requesting ON data_grants (requesting_dept_id);
