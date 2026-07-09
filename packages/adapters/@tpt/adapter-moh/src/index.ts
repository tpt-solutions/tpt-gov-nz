import type {
  DeptAdapter,
  CitizenRef,
  DeptAction,
  ActionResult,
  Service,
  Scope,
  AiContextChunk,
  MOHDataBundle,
} from "@tpt/gov-schema";
import { MOHDataBundleSchema } from "@tpt/gov-schema";

export class MOHAdapter implements DeptAdapter {
  readonly deptId = "moh";
  readonly displayName = "Ministry of Health (MOH)";

  constructor(private readonly baseUrl: string, private readonly apiKey: string) {}

  async resolveCitizen(did: string): Promise<CitizenRef> {
    const res = await fetch(`${this.baseUrl}/citizen/resolve`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did }),
    });
    if (!res.ok) throw new Error(`MOH resolve failed: ${res.status}`);
    return res.json() as Promise<CitizenRef>;
  }

  async fetchConsentedData(did: string, scopes: Scope[]): Promise<MOHDataBundle> {
    const mohScopes = scopes.filter((s) => s.startsWith("moh:"));
    if (mohScopes.length === 0) throw new Error("No MOH scopes granted");
    const res = await fetch(`${this.baseUrl}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did, scopes: mohScopes }),
    });
    if (!res.ok) throw new Error(`MOH data fetch failed: ${res.status}`);
    return MOHDataBundleSchema.parse(await res.json());
  }

  async submitAction(_did: string, action: DeptAction): Promise<ActionResult> {
    const res = await fetch(`${this.baseUrl}/citizen/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify(action),
    });
    if (!res.ok) return { success: false, message: `MOH action failed: ${res.status}` };
    return { success: true };
  }

  async listServices(): Promise<Service[]> {
    return [
      {
        id: "moh:nhi",
        name: "Health Identity (NHI)",
        description: "View your National Health Index number and enrolled GP",
        requiredScopes: ["moh:nhi"],
      },
      {
        id: "moh:prescriptions",
        name: "Prescriptions",
        description: "View and renew active prescriptions",
        requiredScopes: ["moh:prescriptions"],
      },
      {
        id: "moh:appointments",
        name: "Appointments",
        description: "View upcoming health appointments",
        requiredScopes: ["moh:appointments"],
      },
    ];
  }

  produceAiContext(bundle: MOHDataBundle): AiContextChunk[] {
    const lines: string[] = [`NHI: ${bundle.nhiNumber}`];
    if (bundle.enrolledGP) lines.push(`GP: ${bundle.enrolledGP.practiceName}`);
    if (bundle.activePrescriptions?.length) {
      lines.push(`Active prescriptions: ${bundle.activePrescriptions.map((p) => p.medication).join(", ")}`);
    }
    if (bundle.upcomingAppointments?.length) {
      lines.push(`Upcoming appointments: ${bundle.upcomingAppointments.length}`);
    }
    return [{ deptId: this.deptId, content: lines.join("\n") }];
  }
}
