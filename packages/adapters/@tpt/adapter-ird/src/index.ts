import type {
  DeptAdapter,
  CitizenRef,
  DeptAction,
  ActionResult,
  Service,
  Scope,
  AiContextChunk,
  IRDDataBundle,
} from "@tpt/gov-schema";
import { IRDDataBundleSchema } from "@tpt/gov-schema";

export class IRDAdapter implements DeptAdapter {
  readonly deptId = "ird";
  readonly displayName = "Inland Revenue (IRD)";

  constructor(private readonly baseUrl: string, private readonly apiKey: string) {}

  async resolveCitizen(did: string): Promise<CitizenRef> {
    const res = await fetch(`${this.baseUrl}/citizen/resolve`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did }),
    });
    if (!res.ok) throw new Error(`IRD resolve failed: ${res.status}`);
    return res.json() as Promise<CitizenRef>;
  }

  async fetchConsentedData(did: string, scopes: Scope[]): Promise<IRDDataBundle> {
    const irdScopes = scopes.filter((s) => s.startsWith("ird:"));
    if (irdScopes.length === 0) {
      throw new Error("No IRD scopes granted");
    }
    const res = await fetch(`${this.baseUrl}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify({ did, scopes: irdScopes }),
    });
    if (!res.ok) throw new Error(`IRD data fetch failed: ${res.status}`);
    const raw = await res.json();
    return IRDDataBundleSchema.parse(raw);
  }

  async submitAction(_did: string, action: DeptAction): Promise<ActionResult> {
    const res = await fetch(`${this.baseUrl}/citizen/action`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Authorization: `Bearer ${this.apiKey}` },
      body: JSON.stringify(action),
    });
    if (!res.ok) return { success: false, message: `IRD action failed: ${res.status}` };
    return { success: true };
  }

  async listServices(): Promise<Service[]> {
    return [
      {
        id: "ird:tax-summary",
        name: "Tax Summary",
        description: "View your annual income tax summary",
        requiredScopes: ["ird:income", "ird:tax-summary"],
      },
      {
        id: "ird:gst",
        name: "GST Returns",
        description: "File and view GST returns",
        requiredScopes: ["ird:gst"],
      },
      {
        id: "ird:wff",
        name: "Working for Families",
        description: "Check eligibility and manage Working for Families payments",
        requiredScopes: ["ird:income"],
      },
    ];
  }

  produceAiContext(bundle: IRDDataBundle): AiContextChunk[] {
    const lines: string[] = [
      `Tax year: ${bundle.assessmentYear}`,
      `Tax code: ${bundle.taxCode}`,
    ];
    if (bundle.employmentIncome != null) lines.push(`Employment income: $${bundle.employmentIncome.toLocaleString()}`);
    if (bundle.taxPaid != null) lines.push(`Tax paid: $${bundle.taxPaid.toLocaleString()}`);
    if (bundle.taxRefundDue != null && bundle.taxRefundDue > 0) lines.push(`Refund due: $${bundle.taxRefundDue.toLocaleString()}`);
    if (bundle.workingForFamiliesEligible === true) lines.push("Potentially eligible for Working for Families");
    if (bundle.kiwisaverRate != null) lines.push(`KiwiSaver contribution rate: ${bundle.kiwisaverRate}%`);

    return [{ deptId: this.deptId, content: lines.join("\n") }];
  }
}
