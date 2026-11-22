-- Revocation list for DataGrantCredentials.
-- Each revoked grant is recorded here with its serial number (the grant ID) and
-- revocation timestamp. This allows OPA policies and verifiers to check
-- revocation status without querying the grants table directly.
CREATE TABLE IF NOT EXISTS revocation_list (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grant_id        UUID NOT NULL UNIQUE,
    citizen_did     TEXT NOT NULL,
    revoked_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_by      TEXT NOT NULL, -- 'admin' | 'citizen' | 'ttl_expired' | 'key_rotated'
    reason          TEXT
);

CREATE INDEX idx_revocation_list_citizen ON revocation_list (citizen_did);
CREATE INDEX idx_revocation_list_grant ON revocation_list (grant_id);