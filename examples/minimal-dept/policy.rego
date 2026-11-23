package example.consent

# Consent policy for a fictional department, `example`. Copy this file, rename the
# package to your department id, and change `providing_dept_id` below to match.
#
# Grant fields use snake_case because the Rust DataGrantCredential serialises that way.

import rego.v1

providing_dept := "example"

default allow := false

allow if {
    every scope in input.requested_scopes {
        scope_covered(scope)
    }
}

scope_covered(scope) if {
    some grant in input.consent_grants
    grant.requesting_dept_id == input.requesting_dept_id
    grant.providing_dept_id == providing_dept
    grant.citizen_did == input.citizen_did
    grant.expires_at > time.now_ns() / 1000000000
    scope in grant.scopes
}

denied_scopes contains scope if {
    scope := input.requested_scopes[_]
    not scope_covered(scope)
}
