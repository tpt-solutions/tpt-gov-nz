# Security Policy

## Status

tpt-gov-nz is pre-pilot software. It has **not** had an external security audit
(see [TODO.md](./TODO.md), "External security audit"). Do not connect it to real
citizen data or live government systems.

## Reporting a vulnerability

Please report vulnerabilities **privately** using GitHub's private vulnerability
reporting: open the repository's **Security** tab and choose **Report a
vulnerability**.

Do not open a public issue for a security problem, and do not include real
personal information, credentials or working exploit code in any report.

Include, where you can:

- the affected component (for example `services/gov-dept-node` or `apps/portal-staff`),
- the version or commit,
- steps to reproduce and the impact you observed,
- whether you have shared the details anywhere else.

We aim to acknowledge reports within 5 working days and to agree a disclosure
timeline with the reporter before anything is made public.

## Scope

In scope: authentication and session handling, consent and grant verification,
audit-log integrity, federation message authentication, and any path that
exposes citizen data without a valid, unrevoked `DataGrantCredential`.

Out of scope: the fictional demo data in `docker/init/` and demo scenarios, and
findings that require a deployment to have disabled the protections described
in `.env.example`.

## Supported versions

Only the `master` branch receives security fixes until the first tagged release.
