package example.consent_test

import rego.v1

import data.example.consent

future := 9999999999

request(grants) := {
	"requesting_dept_id": "winz",
	"citizen_did": "did:gov:nz:test-citizen-001",
	"requested_scopes": ["example:records"],
	"consent_grants": grants,
}

grant := {
	"requesting_dept_id": "winz",
	"providing_dept_id": "example",
	"citizen_did": "did:gov:nz:test-citizen-001",
	"scopes": ["example:records"],
	"expires_at": future,
}

test_allows_matching_grant if {
	consent.allow with input as request([grant])
}

test_denies_without_grant if {
	not consent.allow with input as request([])
}

test_denies_expired_grant if {
	not consent.allow with input as request([object.union(grant, {"expires_at": 1})])
}
