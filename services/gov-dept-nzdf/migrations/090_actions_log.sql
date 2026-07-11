-- Immutable audit log of all actions taken against New Zealand Defence Force data
CREATE TABLE IF NOT EXISTS actions_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citizen_id      UUID NOT NULL REFERENCES citizens(id),
    action_type     TEXT NOT NULL,
    parameters      JSONB NOT NULL DEFAULT '{}',
    performed_by    TEXT NOT NULL,
    ai_level        TEXT,
    result_success  BOOLEAN NOT NULL,
    result_message  TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE RULE no_update_actions_log AS ON UPDATE TO actions_log DO INSTEAD NOTHING;
CREATE RULE no_delete_actions_log AS ON DELETE TO actions_log DO INSTEAD NOTHING;
