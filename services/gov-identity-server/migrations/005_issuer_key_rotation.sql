-- Issuer key rotation log.
-- Records every rotation of the identity server's issuer keypair.
-- The current active key is always in the issuer_keys table (id=1).
-- This table provides an audit trail for key rotations.
CREATE TABLE IF NOT EXISTS issuer_key_rotation_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    old_key_id      SMALLINT NOT NULL DEFAULT 1,
    new_key_id      SMALLINT NOT NULL DEFAULT 1,
    rotated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    rotated_by      TEXT NOT NULL, -- 'admin' | 'scheduled' | 'emergency'
    reason          TEXT,
    old_did         TEXT NOT NULL,
    new_did         TEXT NOT NULL
);

CREATE INDEX idx_issuer_key_rotation_log_rotated_at ON issuer_key_rotation_log (rotated_at DESC);

-- DID document key history.
-- Tracks the public key history for each DID, enabling verification of
-- credentials issued under previous keys.
CREATE TABLE IF NOT EXISTS did_key_history (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did             TEXT NOT NULL,
    public_key_b64  TEXT NOT NULL,
    activated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deactivated_at  TIMESTAMPTZ,
    is_current      BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX idx_did_key_history_did_current ON did_key_history (did)
    WHERE is_current = TRUE;
CREATE INDEX idx_did_key_history_did ON did_key_history (did);