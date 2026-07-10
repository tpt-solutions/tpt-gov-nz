-- Immutable audit log of all actions taken against WINZ data
CREATE TABLE IF NOT EXISTS actions_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id),
    action_type     TEXT NOT NULL,
    parameters      JSONB NOT NULL DEFAULT '{}',
    performed_by    TEXT NOT NULL,   -- 'citizen', 'staff:<id>', 'ai-level-3'
    ai_level        TEXT,            -- null if not AI, else 'advisory'/'assisted'/'automated'
    result_success  BOOLEAN NOT NULL,
    result_message  TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Append-only: no UPDATE or DELETE allowed
CREATE RULE no_update_actions_log AS ON UPDATE TO actions_log DO INSTEAD NOTHING;
CREATE RULE no_delete_actions_log AS ON DELETE TO actions_log DO INSTEAD NOTHING;
