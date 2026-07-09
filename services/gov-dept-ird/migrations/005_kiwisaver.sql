CREATE TABLE IF NOT EXISTS kiwisaver_memberships (
    id                              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id                      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE UNIQUE,
    membership_status               TEXT NOT NULL DEFAULT 'not-enrolled'
                                    CHECK (membership_status IN ('active', 'suspended', 'opted-out', 'not-enrolled')),
    contribution_rate               NUMERIC(4,2) NOT NULL DEFAULT 3.0,
    employer_contribution_rate      NUMERIC(4,2),
    scheme                          TEXT,
    total_balance                   NUMERIC(14,2),
    last_contribution_date          DATE,
    government_contribution_eligible BOOLEAN NOT NULL DEFAULT TRUE,
    first_home_buyer_eligible       BOOLEAN,
    enrolled_at                     TIMESTAMPTZ,
    updated_at                      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
