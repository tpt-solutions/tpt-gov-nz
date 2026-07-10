-- Ingestion audit log: records every batch the ingester pulls from the legacy system
-- (mock transport in dev/demo, real WINZ legacy system in production). Immutable.
CREATE TABLE IF NOT EXISTS ingestion_runs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source              TEXT NOT NULL,               -- 'mock' | 'legacy'
    batch_id            TEXT,
    run_started_at      TIMESTAMPTZ NOT NULL,
    run_finished_at     TIMESTAMPTZ,
    citizens_processed  INTEGER NOT NULL DEFAULT 0,
    rows_inserted       INTEGER NOT NULL DEFAULT 0,
    rows_updated        INTEGER NOT NULL DEFAULT 0,
    status              TEXT NOT NULL DEFAULT 'running'
                        CHECK (status IN ('running', 'success', 'failed')),
    error_message       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ingestion_runs_started ON ingestion_runs (run_started_at DESC);
