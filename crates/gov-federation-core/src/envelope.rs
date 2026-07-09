use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederationEnvelope {
    pub message_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    pub message_type: FederationMessageType,
    pub from_dept_id: String,
    pub to_dept_id: String,
    pub timestamp_ms: i64,
    pub consent_grant_ids: Vec<Uuid>,
    pub payload_encrypted: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FederationMessageType {
    DataRequest,
    DataResponse,
    DataDenied,
    AuditAck,
}

impl FederationEnvelope {
    pub fn new_request(
        from_dept_id: impl Into<String>,
        to_dept_id: impl Into<String>,
        consent_grant_ids: Vec<Uuid>,
        payload_encrypted: String,
        signature: String,
    ) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            correlation_id: None,
            message_type: FederationMessageType::DataRequest,
            from_dept_id: from_dept_id.into(),
            to_dept_id: to_dept_id.into(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            consent_grant_ids,
            payload_encrypted,
            signature,
        }
    }
}
