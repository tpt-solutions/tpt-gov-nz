import type {
  DeptAdapter,
  CitizenRef,
  DeptAction,
  ActionResult,
  Service,
  Scope,
  AiContextChunk,
  WINZDataBundle,
} from "@tpt/gov-schema";
import { WINZDataBundleSchema } from "@tpt/gov-schema";

export class WINZAdapter implements DeptAdapter {
  readonly deptId = "winz";
  readonly displayName = "Work and Income (WINZ)";

  constructor(private readonly baseUrl: string, private readonly apiKey: string) {}

  async resolveCitizen(did: string): Promise<CitizenRef> {
    const res = await fetch(`${this.baseUrl}/citizen/resolve`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did }),
    });
    if (!res.ok) throw new Error(`WINZ resolve failed: ${res.status}`);
    return res.json() as Promise<CitizenRef>;
  }

  async fetchConsentedData(did: string, scopes: Scope[]): Promise<WINZDataBundle> {
    const winzScopes = scopes.filter((s) => s.startsWith("winz:"));
    if (winzScopes.length === 0) throw new Error("No WINZ scopes granted");
    const res = await fetch(`${this.baseUrl}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did, scopes: winzScopes }),
    });
    if (!res.ok) throw new Error(`WINZ data fetch failed: ${res.status}`);
    return WINZDataBundleSchema.parse(await res.json());
  }

  async submitAction(_did: string, action: DeptAction): Promise<ActionResult> {
    const res = await fetch(`${this.baseUrl}/citizen/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify(action),
    });
    if (!res.ok) return { success: false, message: `WINZ action failed: ${res.status}` };
    return { success: true };
  }

  async listServices(): Promise<Service[]> {
    return [
      {
        id: "winz:benefit-status",
        name: "Benefit Status",
        description: "View your current benefit payments and status",
        requiredScopes: ["winz:benefit-status"],
      },
      {
        id: "winz:payments",
        name: "Payment History",
        description: "View your payment history",
        requiredScopes: ["winz:payments"],
      },
    ];
  }

  produceAiContext(bundle: WINZDataBundle): AiContextChunk[] {
    const lines: string[] = [`Total weekly payment: $${bundle.totalWeeklyPayment}`];
    for (const b of bundle.activeBenefits) {
      lines.push(`${b.type}: $${b.weeklyAmount}/week (from ${b.startDate})`);
    }
    if (bundle.nextAppointment) lines.push(`Next appointment: ${bundle.nextAppointment}`);
    return [{ deptId: this.deptId, content: lines.join("\n") }];
  }
}
