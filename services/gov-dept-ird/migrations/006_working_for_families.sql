CREATE TABLE IF NOT EXISTS wff_entitlements (
    id                              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id                      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE UNIQUE,
    eligible                        BOOLEAN NOT NULL DEFAULT FALSE,
    number_of_dependant_children    INTEGER NOT NULL DEFAULT 0,
    income_threshold                NUMERIC(14,2) NOT NULL DEFAULT 42700,
    family_tax_credit               NUMERIC(10,2),
    in_work_tax_credit              NUMERIC(10,2),
    best_start_payment              NUMERIC(10,2),
    minimum_family_tax_credit       NUMERIC(10,2),
    payment_frequency               TEXT DEFAULT 'weekly'
                                    CHECK (payment_frequency IN ('weekly', 'fortnightly', 'lump-sum')),
    next_review_date                DATE,
    updated_at                      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
