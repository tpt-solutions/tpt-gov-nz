-- DID registration challenges for proof-of-possession flow.
-- Stores ephemeral challenges that clients must sign to prove they control
-- the private key corresponding to their DID.
CREATE TABLE IF NOT EXISTS did_challenges (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did             TEXT NOT NULL,
    public_key_b64  TEXT NOT NULL,
    challenge       TEXT NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_did_challenges_did ON did_challenges (did);
CREATE INDEX idx_did_challenges_expires ON did_challenges (expires_at);

-- Clean up expired challenges periodically (handled by background job).