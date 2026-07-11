CREATE TABLE IF NOT EXISTS msd_studylink (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id        UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    has_student_loan  BOOLEAN NOT NULL DEFAULT false,
    loan_balance      NUMERIC,
    repayment_plan    TEXT,
    has_allowance     BOOLEAN NOT NULL DEFAULT false,
    allowance_type    TEXT,
    next_payment_date DATE,
    weekly_amount     NUMERIC,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id)
);

CREATE INDEX idx_msd_studylink_citizen ON msd_studylink (citizen_id);
