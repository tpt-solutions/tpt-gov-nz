# Notifiable privacy breach runbook

Draft for the operating agency. Under Part 6 of the Privacy Act 2020, an agency
must notify the Privacy Commissioner and affected individuals as soon as practicable
after it becomes aware of a notifiable privacy breach. Confirm the current obligations
with the agency's privacy officer and legal team before use.

A notifiable breach is one that is likely to cause serious harm. Agencies assess this
when they become aware of the breach.

## 1. Detect

Sources to watch:

- Audit log entries with `Denied` decisions in volume, or unexpected `Allowed` access
- Failed signature or revocation checks in department nodes (`grant signature invalid`, `consent grant revoked or unknown`)
- Staff-portal login lockouts and unusual case-note or referral activity
- Reports from citizens or staff

## 2. Contain (first hour)

1. Revoke affected `DataGrantCredential`s through the identity server revocation endpoint.
2. Rotate `TPT__GOV__IDENTITY_API_KEY` and the issuer key if either may be exposed. Follow the issuer key rotation procedure in `services/gov-identity-server`.
3. Disable the affected staff account or demo mode if they were involved.
4. Preserve the audit log. Do not edit or delete entries.

## 3. Assess

- What information was involved, and how sensitive is it?
- How many people are affected?
- Who could have accessed it, and could they misuse it?
- Is serious harm likely?

Record the assessment and the reasoning in the incident file.

## 4. Notify

- Notify the Privacy Commissioner as soon as practicable where serious harm is likely.
- Notify affected individuals directly, unless an exception applies. Confirm the exception with legal advice.
- Record the time of awareness, the time of each notice, and the content sent.

## 5. Recover and review

- Fix the root cause and add a regression test.
- Update the threat model and the PIA.
- Hold a blameless review within 10 working days, and record the actions.

## Contacts

Fill in before use: privacy officer, system owner, security lead, legal adviser.
Do not put personal contact details in this repository.
