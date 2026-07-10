import type {
  DeptAdapter,
  CitizenRef,
  DeptAction,
  ActionResult,
  Service,
  Scope,
  AiContextChunk,
  DIADataBundle,
} from "@tpt/gov-schema";
import { DIADataBundleSchema } from "@tpt/gov-schema";

export class DIAAdapter implements DeptAdapter {
  readonly deptId = "dia";
  readonly displayName = "Department of Internal Affairs (DIA)";

  constructor(private readonly baseUrl: string, private readonly apiKey: string) {}

  async resolveCitizen(did: string): Promise<CitizenRef> {
    const res = await fetch(`${this.baseUrl}/citizen/resolve`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did }),
    });
    if (!res.ok) throw new Error(`DIA resolve failed: ${res.status}`);
    return res.json() as Promise<CitizenRef>;
  }

  async fetchConsentedData(did: string, scopes: Scope[]): Promise<DIADataBundle> {
    const diaScopes = scopes.filter((s) => s.startsWith("dia:"));
    if (diaScopes.length === 0) throw new Error("No DIA scopes granted");
    const res = await fetch(`${this.baseUrl}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did, scopes: diaScopes }),
    });
    if (!res.ok) throw new Error(`DIA data fetch failed: ${res.status}`);
    return DIADataBundleSchema.parse(await res.json());
  }

  async submitAction(_did: string, action: DeptAction): Promise<ActionResult> {
    const res = await fetch(`${this.baseUrl}/citizen/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify(action),
    });
    if (!res.ok) return { success: false, message: `DIA action failed: ${res.status}` };
    return { success: true };
  }

  async listServices(): Promise<Service[]> {
    return [
      {
        id: "dia:passport",
        name: "Passport",
        description: "View passport status and renew",
        requiredScopes: ["dia:passport"],
      },
      {
        id: "dia:documents",
        name: "Birth Certificate",
        description: "Request a copy of your birth certificate",
        requiredScopes: ["dia:documents"],
      },
    ];
  }

  produceAiContext(bundle: DIADataBundle): AiContextChunk[] {
    const lines: string[] = [];
    if (bundle.citizenship) lines.push(`Citizenship: ${bundle.citizenship.status}`);
    if (bundle.passport) {
      lines.push(`Passport expires: ${bundle.passport.expiryDate}`);
      if (bundle.passport.renewable) lines.push("Passport is eligible for renewal");
    }
    return [{ deptId: this.deptId, content: lines.join("\n") }];
  }
}
