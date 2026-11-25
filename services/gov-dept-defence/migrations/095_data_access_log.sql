-- Read audit for gov-dept-defence citizen data. One row per /citizen/data call, granted or denied.
CREATE TABLE IF NOT EXISTS data_access_log (
    id                  BIGSERIAL PRIMARY KEY,
    citizen_id          UUID NOT NULL REFERENCES citizens(id),
    requesting_dept_id  TEXT NOT NULL,
    scopes              TEXT[] NOT NULL,
    direct_access       BOOLEAN NOT NULL,
    outcome             TEXT NOT NULL CHECK (outcome IN ('granted', 'denied')),
    accessed_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS data_access_log_citizen_idx ON data_access_log (citizen_id, accessed_at DESC);

CREATE OR REPLACE FUNCTION reject_data_access_log_change() RETURNS trigger AS $$
BEGIN
    RAISE EXCEPTION 'data_access_log is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS data_access_log_append_only ON data_access_log;
CREATE TRIGGER data_access_log_append_only
    BEFORE UPDATE OR DELETE ON data_access_log
    FOR EACH ROW EXECUTE FUNCTION reject_data_access_log_change();
