# Sovereign Ollama Deployment Guide

This guide describes how to run the AI assistant for **tpt-gov-nz** entirely on
government-owned hardware using [Ollama](https://ollama.com), so that no
citizen PII or prompts ever leave the sovereign boundary.

> The TypeScript side already supports Ollama via `GovAiClient` (see
> `packages/@tpt/gov-ai-client`). This guide is about *operating* that path in
> production.

## 1. Architecture

```
┌────────────────────┐      mTLS      ┌────────────────────────┐
│  portal-citizen    │ ─────────────▶ │  gov-gateway            │
│  (GovAiClient)     │                │  (PII redactor runs     │
└────────────────────┘                │   before any call)      │
                                      └───────────┬────────────┘
                                                  │ internal network only
                                                  ▼
                                      ┌────────────────────────┐
                                      │  Ollama (sovereign)     │
                                      │  model: llama3.1:70b    │
                                      │  bind: 10.0.0.10:11434  │
                                      └────────────────────────┘
```

- The `GovAiClient` provider is selected by `AI_PROVIDER=ollama`.
- The PII redactor (`packages/@tpt/gov-ai-client/src/redactor.ts`) strips NHI,
  IRD numbers, passport numbers and phone numbers **before** the payload leaves
  the portal. Prompts are built from department `produceAiContext()` output,
  which is already scoped to the granted scopes.
- Ollama is bound to a private interface and is **never** exposed to the public
  internet.

## 2. Hardware sizing (reference)

| Model            | Min RAM | Recommended      | Notes                                  |
| ---------------- | ------- | ---------------- | -------------------------------------- |
| `llama3.1:8b`    | 16 GB   | 32 GB            | Advisory tier, fast, lower quality     |
| `llama3.1:70b`   | 140 GB  | 2× A100 80GB     | Production advisory + assisted tiers   |
| `qwen2.5:32b`    | 64 GB   | 96 GB            | Good multilingual (te reo) support     |

Run on air-gapped nodes where possible. Use NVMe-backed models directory
(`OLLAMA_MODELS`) for fast load.

## 3. Install (Debian/bookworm, air-gapped)

```bash
# On a build host with internet, fetch the binary + models, then transfer.
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3.1:70b
ollama pull qwen2.5:32b

# Transfer /usr/local/bin/ollama and ~/.ollama to the sovereign node.
```

## 4. Harden the service

`/etc/systemd/system/ollama.service.d/override.conf`:

```ini
[Service]
# Bind only to the internal management subnet.
Environment="OLLAMA_HOST=10.0.0.10:11434"
# Disable the experimental GPU monitoring endpoint.
Environment="OLLAMA_ORIGINS=http://gov-gateway"
```

Enable the origin allow-list so only `gov-gateway` can call it, and place the
node behind the existing mTLS mesh (`crates/gov-mtls`): terminate mTLS at the
gateway and forward to Ollama over the private link.

## 5. Configure the portal

```bash
# .env (real / sovereign mode)
AI_PROVIDER=ollama
OLLAMA_BASE_URL=http://10.0.0.10:11434
AI_DEFAULT_MODEL=llama3.1:70b
AI_LEVEL=advisory            # advisory | assisted (never automated on sovereign)
```

`NEXT_PUBLIC_DEMO_MODE` must be `false` in production.

## 6. Verification

```bash
# From the gateway node:
curl -k --cert mesh/client.pem --key mesh/client.key \
  https://10.0.0.10:11434/api/tags

# End-to-end (advisory prompt, PII redacted in logs):
curl -X POST https://portal.example.govt.nz/api/ai -d '{"prompt":"Am I eligible for WFF?"}'
grep -R "IRD\|NHI" /var/log/tpt/ai.log   # should be empty (redacted)
```

## 7. Model governance

- Pin model digests (`ollama show --digest`) in your config and alert on drift.
- Keep a signed checksum manifest of `OLLAMA_MODELS` for audit.
- Re-run the load tests in `load-tests/k6/` against the sovereign endpoint
  before each model upgrade.
