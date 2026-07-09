CREATE TABLE IF NOT EXISTS citizens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT UNIQUE NOT NULL,
    nhi_number TEXT UNIQUE NOT NULL,
    gp_practice_name TEXT,
    gp_address TEXT,
    gp_phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS prescriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID REFERENCES citizens(id),
    medication TEXT NOT NULL,
    dose TEXT NOT NULL,
    repeats_remaining INTEGER DEFAULT 0,
    active BOOLEAN DEFAULT TRUE
);

INSERT INTO citizens (did, nhi_number, gp_practice_name, gp_address, gp_phone)
VALUES ('did:gov:nz:test-citizen-001', 'ZZZ9999', 'Auckland City Medical', '1 Queen St, Auckland', '09 000 0000')
ON CONFLICT DO NOTHING;
