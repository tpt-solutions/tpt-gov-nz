//! Cross-department consent verification for the LINZ department node.
//!
//! Mirrors `policies/linz.rego` so the same allow/deny decision can be made locally
//! (and in unit tests) without a running OPA sidecar.

use crate::error::LinzError;
use ed25519_dalek::VerifyingKey;
use gov_identity_core::DataGrantCredential;

pub struct ConsentInput<'a> {
    pub requesting_dept_id: &'a str,
    pub citizen_did: &'a str,
    pub requested_scopes: &'a [String],
    pub consent_grants: &'a [DataGrantCredential],
}

pub fn evaluate_allow(input: &ConsentInput) -> bool {
    input
        .requested_scopes
        .iter()
        .all(|scope| scope_covered(input, scope))
}

fn scope_covered(input: &ConsentInput, scope: &str) -> bool {
    let now = chrono::Utc::now().timestamp();
    input.consent_grants.iter().any(|grant| {
        grant.requesting_dept_id == input.requesting_dept_id
            && grant.providing_dept_id == "linz"
            && grant.citizen_did.as_str() == input.citizen_did
            && grant.expires_at > now
            && grant.scopes.iter().any(|s| s == scope)
    })
}

pub fn denied_scopes(input: &ConsentInput) -> Vec<String> {
    input
        .requested_scopes
        .iter()
        .filter(|scope| !scope_covered(input, scope))
        .cloned()
        .collect()
}

pub fn verify_access(
    input: &ConsentInput,
    issuer_key: Option<&VerifyingKey>,
) -> Result<(), LinzError> {
    if !evaluate_allow(input) {
        return Err(LinzError::ScopeNotGranted(denied_scopes(input).join(",")));
    }

    if let Some(key) = issuer_key {
        let all_signed = input.requested_scopes.iter().all(|scope| {
            input.consent_grants.iter().any(|grant| {
                grant.scopes.iter().any(|s| s == scope)
                    && grant.requesting_dept_id == input.requesting_dept_id
                    && grant.providing_dept_id == "linz"
                    && grant.citizen_did.as_str() == input.citizen_did
                    && grant.expires_at > chrono::Utc::now().timestamp()
                    && grant.verify_signature(key).is_ok()
            })
        });
        if !all_signed {
            return Err(LinzError::ScopeNotGranted("grant signature invalid".into()));
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
        scopes: Vec<String>,
    ) -> DataGrantCredential {
        issuer.issue_data_grant(
            gov_identity_core::GovDid::parse("did:gov:nz:test-citizen-001").unwrap(),
            requesting,
            "linz",
            scopes,
            3600,
        )
    }

    #[test]
    fn linz_grant_allows_linz_scope() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["linz:titles".into()]);
        let scopes = vec!["linz:titles".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(evaluate_allow(&input));
        assert!(verify_access(&input, Some(&i.verifying_key())).is_ok());
    }

    #[test]
    fn wrong_dept_grant_denies() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["ird:income".into()]);
        let scopes = vec!["linz:titles".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g],
        };
        assert!(!evaluate_allow(&input));
    }

    #[test]
    fn expired_grant_is_denied() {
        let i = issuer();
        let g = grant(&i, "ird", vec!["linz:titles".into()]);
        let mut g2 = g;
        g2.expires_at = chrono::Utc::now().timestamp() - 10;
        let scopes = vec!["linz:titles".into()];
        let input = ConsentInput {
            requesting_dept_id: "ird",
            citizen_did: "did:gov:nz:test-citizen-001",
            requested_scopes: &scopes,
            consent_grants: &[g2],
        };
        assert!(!evaluate_allow(&input));
    }
}
