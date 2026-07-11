-- Seed a few reference data contracts so the registry is useful out of the box.
-- These mirror (a subset of) the TypeScript types in packages/@tpt/gov-schema.

INSERT INTO schemas (name, version, content, required, checksum, created_at) VALUES
(
    'citizen-identity',
    '1.0.0',
    '{"type":"object","properties":{"did":{"type":"string"},"displayName":{"type":"string"},"scopes":{"type":"array"}},"required":["did"]}'::jsonb,
    ARRAY['did'],
    'seed',
    now()
),
(
    'ird-tax-assessment',
    '1.0.0',
    '{"type":"object","properties":{"citizen_did":{"type":"string"},"tax_year":{"type":"integer"},"total_income":{"type":"number"},"refund":{"type":"number"}},"required":["citizen_did","tax_year"]}'::jsonb,
    ARRAY['citizen_did','tax_year'],
    'seed',
    now()
),
(
    'wff-entitlement',
    '1.0.0',
    '{"type":"object","properties":{"citizen_did":{"type":"string"},"eligible":{"type":"boolean"},"weekly_amount":{"type":"number"}},"required":["citizen_did","eligible"]}'::jsonb,
    ARRAY['citizen_did','eligible'],
    'seed',
    now()
)
ON CONFLICT (name, version) DO NOTHING;
