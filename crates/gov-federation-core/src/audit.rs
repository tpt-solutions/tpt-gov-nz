use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub message_id: Uuid,
    pub citizen_did: String,
    pub action: String,
    pub from_dept_id: String,
    pub to_dept_id: String,
    pub scopes_accessed: Vec<String>,
    pub timestamp_ms: i64,
    pub signature: String,
}

impl AuditLogEntry {
    pub fn new(
        message_id: Uuid,
        citizen_did: impl Into<String>,
        action: impl Into<String>,
        from_dept_id: impl Into<String>,
        to_dept_id: impl Into<String>,
        scopes_accessed: Vec<String>,
        signature: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            message_id,
            citizen_did: citizen_did.into(),
            action: action.into(),
            from_dept_id: from_dept_id.into(),
            to_dept_id: to_dept_id.into(),
            scopes_accessed,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            signature,
        }
    }
}
