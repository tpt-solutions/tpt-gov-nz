CREATE TABLE IF NOT EXISTS gst_registrations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE UNIQUE,
    registered      BOOLEAN NOT NULL DEFAULT FALSE,
    gst_number      TEXT UNIQUE,
    registered_at   TIMESTAMPTZ,
    filing_frequency TEXT DEFAULT 'two-monthly'
                    CHECK (filing_frequency IN ('monthly', 'two-monthly', 'six-monthly'))
);

CREATE TABLE IF NOT EXISTS gst_periods (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id          UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    period_start        DATE NOT NULL,
    period_end          DATE NOT NULL,
    filing_due          DATE NOT NULL,
    status              TEXT NOT NULL DEFAULT 'due'
                        CHECK (status IN ('filed', 'due', 'overdue', 'not-required')),
    sales_income        NUMERIC(14,2),
    gst_on_sales        NUMERIC(14,2),
    gst_on_purchases    NUMERIC(14,2),
    refund_or_payment   NUMERIC(14,2),
    filed_at            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, period_start)
);
