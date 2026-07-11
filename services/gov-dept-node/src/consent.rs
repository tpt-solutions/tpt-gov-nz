//! Cross-department consent verification for a generic department node.
//!
//! This is the reusable template behind every department service (IRD, WINZ,
//! MOH, DIA, …). It mirrors the per-department OPA policies so the same
//! allow/deny decision can be made locally (and in unit tests) without a
//! running OPA sidecar. When an OPA endpoint is configured via `OPA_URL`, the
//! route layer can additionally defer to the deployed policy; the local
//! evaluation here is the canonical fallback.
//!
//! The *providing* department is passed in explicitly (the route layer reads it
//! from `TPT__GOV__DEPT_ID` via [`providing_dept_id`]) so the pure decision
//! functions are free of global state and safe to unit-test in parallel.

use ed25519_dalek::VerifyingKey;
use gov_identity_core::DataGrantCredential;

/// The department id this node provides data for (from `TPT__GOV__DEPT_ID`).
pub fn providing_dept_id() -> String {
    std::env::var("TPT__GOV__DEPT_ID").unwrap_or_else(|_| "unknown".into())
}

/// Input to the consent decision, structurally identical to the OPA policy input.
pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

/// Mirrors a department's `allow` rule: every requested scope must be covered.
pub fn evaluate_allow(input: &ConsentInput, providing_dept_id: &str) -> bool {
    input
        .requested_scopes
        .iter()
        .all(|scope| scope_covered(input, scope, providing_dept_id))
}

/// Mirrors a department's `scope_covered` rule.
fn scope_covered(input: &ConsentInput, scope: &str, providing_dept_id: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == providing_dept_id
            && grant.citizen_did.as_str() == input.citizen_did
            && grant.expires_at > now
            && grant.scopes.iter().any(|s| s == scope)
    })
}

/// The scopes in `input` that are NOT covered by any grant (audit helper).
pub fn denied_scopes(input: &ConsentInput, providing_dept_id: &str) -> Vec<String> {
    input
        .requested_scopes
        .iter()
        .filter(|scope| !scope_covered(input, scope, providing_dept_id))
        .cloned()
        .collect()
}

/// Authorise a cross-department request.
///
/// When `issuer_key` is `Some`, each covering grant's Ed25519 signature is
/// cryptographically verified against the identity server's public key before
/// access is granted. Returns an error string if any requested scope is not
/// authorised.
pub fn verify_access(
    input: &ConsentInput,
    providing_dept_id: &str,
    issuer_key: Option<&VerifyingKey>,
) -> Result<(), String> {
    if !evaluate_allow(input, providing_dept_id) {
        return Err(denied_scopes(input, providing_dept_id).join(","));
    }

    if let Some(key) = issuer_key {
        let all_signed = input.requested_scopes.iter().all(|scope| {
            input.consent_grants.iter().any(|grant| {
                grant.scopes.iter().any(|s| s == scope)
                    && grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == providing_dept_id
                    && grant.citizen_did.as_str() == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.verify_signature(key).is_ok()
            })
        });
        if !all_signed {
            return Err("grant signature invalid".into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gov_identity_core::CredentialIssuer;

    fn issuer() -> CredentialIssuer {
        CredentialIssuer::generate()
    }

    fn grant(
        issuer: &CredentialIssuer,
        requesting: &str,
        providing: &str,
        scopes: Vec<String>,
        ttl: i64,
    ) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            providing,
            scopes,
            ttl,
        )
    }

    fn input<'a>(
        requesting: &'a str,
        citizen_did: &'a str,
        scopes: &'a [String],
        grants: &'a [DataGrantCredential],
    ) -> ConsentInput<'a> {
        ConsentInput {
            requesting_dept_id: requesting,
            citizen_did,
            requested_scopes: scopes,
            consent_grants: grants,
        }
    }

    #[test]
    fn winz_income_grant_allows_ird_income() {
        let i = issuer();
        let g = grant(&i, "winz", "ird", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:income".into()];
        let grants = [g];
        let input = input("winz", "did:gov:nz:test-citizen-001", &scopes, &grants);
        assert!(evaluate_allow(&input, "ird"));
        assert!(verify_access(&input, "ird", Some(&i.verifying_key())).is_ok());
    }

    #[test]
    fn missing_scope_is_denied() {
        let i = issuer();
        let g = grant(&i, "winz", "ird", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:kiwisaver".into()];
        let grants = [g];
        let input = input("winz", "did:gov:nz:test-citizen-001", &scopes, &grants);
        assert!(!evaluate_allow(&input, "ird"));
        assert!(verify_access(&input, "ird", Some(&i.verifying_key())).is_err());
    }

    #[test]
    fn wrong_providing_dept_is_denied() {
        // Node believes it is MOH, but the grant was issued by IRD.
        let i = issuer();
        let g = grant(&i, "winz", "ird", vec!["ird:income".into()], 3600);
        let scopes = vec!["ird:income".into()];
        let grants = [g];
        let input = input("winz", "did:gov:nz:test-citizen-001", &scopes, &grants);
        assert!(!evaluate_allow(&input, "moh"));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "winz", "ird", vec!["ird:income".into()], -10);
        let scopes = vec!["ird:income".into()];
        let grants = [g];
        let input = input("winz", "did:gov:nz:test-citizen-001", &scopes, &grants);
        assert!(!evaluate_allow(&input, "ird"));
    }
}
