CREATE TABLE IF NOT EXISTS nzqa_transcripts (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    record_summary      TEXT,
    total_credits       INTEGER,
    credit_summary      TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id)
);

CREATE INDEX idx_nzqa_transcripts_citizen ON nzqa_transcripts (citizen_id);
