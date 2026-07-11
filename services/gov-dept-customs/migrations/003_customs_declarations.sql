CREATE TABLE IF NOT EXISTS customs_declarations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id) ON DELETE CASCADE,
    declaration_id  TEXT NOT NULL,
    date            DATE NOT NULL,
    country_from    TEXT NOT NULL,
    goods_declared  TEXT NOT NULL,
    status          TEXT NOT NULL CHECK (status IN ('submitted', 'assessed', 'cleared', 'referred')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (citizen_id, declaration_id)
);

CREATE INDEX idx_customs_declarations_citizen ON customs_declarations (citizen_id);
