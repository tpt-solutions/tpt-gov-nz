# My Gov NZ — Demo

A self-contained, fictional demonstration of the **tpt-gov-nz** unified government
services portal. Everything runs on seeded data for a single fictional citizen,
**Alex Tane**, and contacts no real government systems.

The demo shows the full citizen experience:

- **Self-custodied digital identity** — a browser wallet (Ed25519, WebCrypto) derives
  your Decentralised Identifier (DID). In demo mode you sign in with one click.
- **Department modules** — Inland Revenue, Work and Income, Health and Internal Affairs,
  each served from its own data.
- **Consent & audit** — grant/revoke cross-department data sharing; every grant is
  signed and listed in your audit trail.
- **Optional AI assistant** — answers questions using only your consented data.
- **Three scenarios** — Standard, Beneficiary and New-parent, switchable from the banner.

---

## Run with Docker (one command)

```bash
docker compose -f docker/demo.yml up
```

Then open <http://localhost:3000>.

The first `up` installs dependencies and builds the portal, so it may take a minute.

## Run locally (without Docker)

```bash
cp .env.example .env
# ensure NEXT_PUBLIC_DEMO_MODE=true (it is, by default)

pnpm install
pnpm --filter @tpt/portal-citizen dev
```

Open <http://localhost:3000>.

## Enable the AI assistant (optional)

The assistant is off unless an AI provider is configured. In `docker/demo.yml` (or your
`.env`) set one of:

- **Cloud:** `TPT__GOV__AI_PROVIDER=openrouter` + `TPT__GOV__AI_API_KEY=sk-or-...`
  and `TPT__GOV__AI_MODEL=anthropic/claude-sonnet-4-6`
- **Local/air-gapped:** `TPT__GOV__AI_PROVIDER=ollama` + `TPT__GOV__AI_BASE_URL=http://localhost:11434`

Then restart. When `TPT__GOV__AI_LEVEL=advisory`, the assistant surfaces suggestions but
never acts on your behalf.

## Trying the scenarios

Use the **Scenario** selector in the demo banner to switch between Alex Tane's
circumstances:

| Scenario | What you'll see |
|----------|-----------------|
| Standard | Employed, a tax refund, KiwiSaver member, no benefits. |
| Beneficiary | Jobseeker + accommodation supplement, Working for Families with two tamariki. |
| New parent | Recently had a baby; Working for Families with one child, parental leave. |

The **Reset demo** button restores the default scenario.

## Notes

- Demo data is fictional and for illustration only.
- No data leaves your browser/container. The portal sets a local session cookie and, in
  demo mode, stores scenario and consent choices in cookies.
- For the full platform (real department services, federation, identity server), see the
  main `README.md` and `docker/phase1.yml`.
