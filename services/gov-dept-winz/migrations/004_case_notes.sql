CREATE TABLE IF NOT EXISTS case_notes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    note_date       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    author          TEXT NOT NULL,
    note            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_case_notes_citizen ON case_notes (citizen_id, note_date DESC);
