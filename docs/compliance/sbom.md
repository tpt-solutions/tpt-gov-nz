# SBOM and supply chain (draft)

A software bill of materials (SBOM) lists every component in a build. This project
has not generated one yet. The steps below are the plan, so the gap is visible.

## Planned steps

1. **Rust:** generate a CycloneDX SBOM per workspace binary with `cargo cyclonedx`,
   run in CI, and attach it to each release.
2. **JavaScript:** generate a CycloneDX SBOM for each portal with
   `@cyclonedx/cyclonedx-npm` against the pnpm lockfile.
3. **Vulnerability checks:** `cargo audit` (or `cargo deny`) on `Cargo.lock`, and
   `pnpm audit` on `pnpm-lock.yaml`. Dependabot already opens update PRs.
4. **Container images:** scan each image with an OCI scanner before publishing.
5. **Provenance:** sign release artefacts and their SBOMs. Record the signing key in
   the release notes, never in the repository.

## Status

| Step | Status |
|---|---|
| Dependabot updates (cargo, npm, GitHub Actions) | Implemented |
| Secret scanning (gitleaks) in CI | Implemented |
| SBOM generation | Planned |
| `cargo audit` / `pnpm audit` in CI | Planned |
| Container image scanning | Planned |
| Signed release artefacts | Planned |
