-- IRD mock data for Phase 1 development

CREATE TABLE IF NOT EXISTS citizens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT UNIQUE NOT NULL,
    ird_number TEXT UNIQUE NOT NULL,
    tax_code TEXT NOT NULL DEFAULT 'M',
    employment_income NUMERIC(12,2),
    other_income NUMERIC(12,2),
    tax_paid NUMERIC(12,2),
    tax_refund_due NUMERIC(12,2),
    gst_registered BOOLEAN DEFAULT FALSE,
    working_for_families_eligible BOOLEAN DEFAULT FALSE,
    kiwisaver_rate NUMERIC(4,2) DEFAULT 3.0,
    assessment_year INTEGER NOT NULL DEFAULT 2025,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Seed a test citizen
INSERT INTO citizens (did, ird_number, tax_code, employment_income, tax_paid, working_for_families_eligible, assessment_year)
VALUES ('did:gov:nz:test-citizen-001', '123-456-789', 'M', 65000.00, 13000.00, true, 2025)
ON CONFLICT DO NOTHING;
