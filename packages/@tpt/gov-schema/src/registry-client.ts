/**
 * Thin client for the gov-schema-registry service.
 *
 * The registry stores versioned JSON-Schema data contracts and can validate a
 * payload against the latest (or a specific) version. Department nodes and
 * portals can use this to assert contract conformance at startup or in tests.
 */

export interface SchemaRow {
  name: string;
  version: string;
  content: unknown;
  required: string[];
  checksum: string;
  created_at: string;
}

export interface ValidateResult {
  valid: boolean;
  name: string;
  version: string;
  error?: string;
}

export class SchemaRegistryClient {
  constructor(
    private readonly baseUrl: string,
    private readonly registryKey?: string,
  ) {}

  private headers(): HeadersInit {
    const h: Record<string, string> = { "content-type": "application/json" };
    if (this.registryKey) h["x-registry-key"] = this.registryKey;
    return h;
  }

  /** List the latest revision of every registered schema. */
  async list(): Promise<SchemaRow[]> {
    const res = await fetch(`${this.baseUrl}/v1/schemas`, {
      headers: this.headers(),
    });
    if (!res.ok) throw new Error(`schema registry list failed: ${res.status}`);
    return res.json();
  }

  /** Fetch the latest version of a named schema. */
  async get(name: string): Promise<SchemaRow> {
    const res = await fetch(
      `${this.baseUrl}/v1/schemas/${encodeURIComponent(name)}`,
      { headers: this.headers() },
    );
    if (!res.ok) throw new Error(`schema not found: ${name}`);
    return res.json();
  }

  /** Register (upsert) a schema version. Requires the registry key. */
  async register(
    name: string,
    version: string,
    content: unknown,
  ): Promise<SchemaRow> {
    const res = await fetch(`${this.baseUrl}/v1/schemas`, {
      method: "POST",
      headers: this.headers(),
      body: JSON.stringify({ name, version, content }),
    });
    if (!res.ok) throw new Error(`schema register failed: ${res.status}`);
    return res.json();
  }

  /** Validate a payload against a schema (latest version if `version` omitted). */
  async validate(
    name: string,
    payload: unknown,
    version?: string,
  ): Promise<ValidateResult> {
    const res = await fetch(`${this.baseUrl}/v1/validate`, {
      method: "POST",
      headers: this.headers(),
      body: JSON.stringify({ name, version, payload }),
    });
    if (!res.ok) throw new Error(`schema validate failed: ${res.status}`);
    return res.json();
  }
}
