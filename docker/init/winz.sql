CREATE TABLE IF NOT EXISTS citizens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT UNIQUE NOT NULL,
    client_id TEXT UNIQUE NOT NULL,
    case_manager_name TEXT,
    next_appointment TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS benefits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id UUID REFERENCES citizens(id),
    type TEXT NOT NULL,
    weekly_amount NUMERIC(10,2) NOT NULL,
    start_date DATE NOT NULL,
    review_date DATE,
    active BOOLEAN DEFAULT TRUE
);

INSERT INTO citizens (did, client_id, case_manager_name)
VALUES ('did:gov:nz:test-citizen-001', 'WINZ-001', 'Jane Smith')
ON CONFLICT DO NOTHING;
