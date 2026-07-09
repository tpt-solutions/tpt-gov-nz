CREATE TABLE IF NOT EXISTS income_records (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id              UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    assessment_year         INTEGER NOT NULL,
    employment_income       NUMERIC(14,2),
    self_employment_income  NUMERIC(14,2),
    rental_income           NUMERIC(14,2),
    other_income            NUMERIC(14,2),
    total_deductions        NUMERIC(14,2),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, assessment_year)
);
