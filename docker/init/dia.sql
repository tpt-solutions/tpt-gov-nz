CREATE TABLE IF NOT EXISTS citizens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT UNIQUE NOT NULL,
    birth_certificate_number TEXT,
    passport_number TEXT,
    passport_expiry DATE,
    citizenship_status TEXT DEFAULT 'citizen-by-birth',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO citizens (did, passport_number, passport_expiry, citizenship_status)
VALUES ('did:gov:nz:test-citizen-001', 'LN123456', '2028-06-01', 'citizen-by-birth')
ON CONFLICT DO NOTHING;
