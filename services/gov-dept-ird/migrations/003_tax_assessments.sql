CREATE TABLE IF NOT EXISTS tax_assessments (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    assessment_year     INTEGER NOT NULL,
    tax_code            TEXT NOT NULL DEFAULT 'M',
    total_income        NUMERIC(14,2) NOT NULL DEFAULT 0,
    taxable_income      NUMERIC(14,2) NOT NULL DEFAULT 0,
    tax_liability       NUMERIC(14,2) NOT NULL DEFAULT 0,
    tax_paid            NUMERIC(14,2) NOT NULL DEFAULT 0,
    tax_refund_due      NUMERIC(14,2) NOT NULL DEFAULT 0,
    tax_owing           NUMERIC(14,2) NOT NULL DEFAULT 0,
    assessment_status   TEXT NOT NULL DEFAULT 'estimated'
                        CHECK (assessment_status IN ('final', 'provisional', 'estimated')),
    assessed_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, assessment_year)
);
