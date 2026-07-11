//! Lightweight JSON-Schema validation subset used by the registry.
//!
//! This is intentionally small: it enforces `required` top-level properties and
//! `type` checks for declared `properties`. For full Draft-07 validation, front
//! this service with a dedicated validator. It is enough to catch contract
//! regressions between department nodes and their consumers.

use serde_json::Value;

/// Validate `payload` against a JSON-Schema `schema`. Returns the first
/// violation as an `Err` string, or `Ok(())` when the payload conforms.
pub fn validate(schema: &Value, payload: &Value) -> Result<(), String> {
    let obj = payload
        .as_object()
        .ok_or_else(|| "payload must be a JSON object".to_string())?;

    if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
        for r in required {
            if let Some(key) = r.as_str() {
                if !obj.contains_key(key) {
                    return Err(format!("missing required property: {key}"));
                }
            }
        }
    }

    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        for (key, expected) in props {
            if let Some(actual) = obj.get(key) {
                if let Some(t) = expected.get("type").and_then(|v| v.as_str()) {
                    if !matches_type(t, actual) {
                        return Err(format!(
                            "property '{key}' has wrong type, expected {t}"
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn matches_type(t: &str, v: &Value) -> bool {
    match t {
        "string" => v.is_string(),
        "number" => v.is_number(),
        "integer" => v.as_i64().is_some(),
        "boolean" => v.is_boolean(),
        "object" => v.is_object(),
        "array" => v.is_array(),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_conforming_payload() {
        let schema = json!({
            "type": "object",
            "properties": {
                "did": {"type": "string"},
                "tax_year": {"type": "integer"}
            },
            "required": ["did"]
        });
        let payload = json!({"did": "did:gov:1", "tax_year": 2024});
        assert!(validate(&schema, &payload).is_ok());
    }

    #[test]
    fn rejects_missing_required() {
        let schema = json!({"required": ["did"]});
        let payload = json!({"tax_year": 2024});
        assert!(validate(&schema, &payload).is_err());
    }

    #[test]
    fn rejects_wrong_type() {
        let schema = json!({
            "properties": {"tax_year": {"type": "integer"}},
            "required": []
        });
        let payload = json!({"tax_year": "not-a-number"});
        assert!(validate(&schema, &payload).is_err());
    }
}
