-- Persistent issuer key for the identity server.
--
-- Only one row is ever stored (id = 1). The key is generated on first start
-- and reused on every subsequent start, so DataGrantCredentials issued by this
-- server remain verifiable across restarts and deployments.
CREATE TABLE IF NOT EXISTS issuer_keys (
    id              SMALLINT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    secret_key_b64  TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
