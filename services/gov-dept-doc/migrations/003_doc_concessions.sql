CREATE TABLE IF NOT EXISTS doc_concessions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    concession_id   TEXT NOT NULL,
    type            TEXT NOT NULL,
    holder          TEXT NOT NULL,
    start_date      DATE NOT NULL,
    end_date        DATE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, concession_id)
);

CREATE INDEX idx_doc_concessions_citizen ON doc_concessions (citizen_id);
