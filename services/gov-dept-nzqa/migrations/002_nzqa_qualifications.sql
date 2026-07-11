CREATE TABLE IF NOT EXISTS nzqa_qualifications (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    qualification_id    TEXT NOT NULL,
    title               TEXT NOT NULL,
    level               INTEGER NOT NULL,
    awarded_date        DATE NOT NULL,
    provider            TEXT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, qualification_id)
);

CREATE INDEX idx_nzqa_qualifications_citizen ON nzqa_qualifications (citizen_id);
