package ird.consent_test

# Tests for policies/ird.rego. Run with: opa test policies
#
# Grants use the snake_case field names that the Rust DataGrantCredential
# serialises to (requesting_dept_id, expires_at, ...). A camelCase policy would
# deny every request, so these tests pin the field names.

import rego.v1

import data.ird.consent

future := 9999999999

grant(overrides) := object.union(
	{
		"requesting_dept_id": "winz",
		"providing_dept_id": "ird",
		"citizen_did": "did:gov:nz:test-citizen-001",
		"scopes": ["ird:income"],
		"expires_at": future,
	},
	overrides,
)

request(grants, scopes) := {
	"requesting_dept_id": "winz",
	"providing_dept_id": "ird",
	"citizen_did": "did:gov:nz:test-citizen-001",
	"requested_scopes": scopes,
	"consent_grants": grants,
}

test_allows_when_grant_covers_scope if {
	consent.allow with input as request([grant({})], ["ird:income"])
}

test_denies_when_scope_not_granted if {
	not consent.allow with input as request([grant({})], ["ird:kiwisaver"])
}

test_denies_for_other_requesting_department if {
	not consent.allow with input as request([grant({"requesting_dept_id": "moh"})], ["ird:income"])
}

test_denies_for_other_citizen if {
	not consent.allow with input as request([grant({"citizen_did": "did:gov:nz:someone-else"})], ["ird:income"])
}

test_denies_expired_grant if {
	not consent.allow with input as request([grant({"expires_at": 1})], ["ird:income"])
}

test_denies_when_no_grants if {
	not consent.allow with input as request([], ["ird:income"])
}

test_allows_when_every_scope_covered if {
	consent.allow with input as request([grant({"scopes": ["ird:income", "ird:kiwisaver"]})], ["ird:income", "ird:kiwisaver"])
}

test_reports_uncovered_scopes if {
	denied := consent.denied_scopes with input as request([grant({})], ["ird:income", "ird:kiwisaver"])
	denied == {"ird:kiwisaver"}
}
