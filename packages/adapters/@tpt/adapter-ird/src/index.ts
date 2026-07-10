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
    const chunks: AiContextChunk[] = [];

    // ── Tax summary chunk ──────────────────────────────────────────────────────
    const taxLines: string[] = [];
    if (bundle.currentTaxYear) {
      const t = bundle.currentTaxYear;
      taxLines.push(`Current tax year: ${t.assessmentYear} (${t.assessmentStatus})`);
      taxLines.push(`Total income: $${Number(t.totalIncome).toLocaleString()}`);
      if (t.taxPaid != null) taxLines.push(`Tax paid: $${Number(t.taxPaid).toLocaleString()}`);
      if (Number(t.taxRefundDue) > 0)
        taxLines.push(`Refund due: $${Number(t.taxRefundDue).toLocaleString()}`);
      if (Number(t.taxOwing) > 0)
        taxLines.push(`Tax owing: $${Number(t.taxOwing).toLocaleString()}`);
    }
    if (taxLines.length > 0) {
      chunks.push({ deptId: this.deptId, content: taxLines.join("\n"), metadata: { area: "tax" } });
    }

    // ── Working for Families chunk ─────────────────────────────────────────────
    const wff = bundle.workingForFamilies;
    if (wff) {
      const wffLines: string[] = [];
      wffLines.push(`Working for Families eligible: ${wff.eligible ? "yes" : "no"}`);
      wffLines.push(`Dependant children: ${wff.numberOfDependantChildren}`);
      wffLines.push(`Current family income: $${Number(wff.currentIncome).toLocaleString()}`);
      wffLines.push(`Income threshold: $${Number(wff.incomeThreshold).toLocaleString()}`);
      if (wff.eligible && wff.currentEntitlement) {
        const e = wff.currentEntitlement;
        wffLines.push(`Entitlement — Family Tax Credit: $${Number(e.familyTaxCredit).toFixed(2)}/wk`);
        if (e.inWorkTaxCredit != null)
          wffLines.push(`Entitlement — In-Work Tax Credit: $${Number(e.inWorkTaxCredit).toFixed(2)}/wk`);
        if (e.bestStartPayment != null)
          wffLines.push(`Entitlement — Best Start Payment: $${Number(e.bestStartPayment).toFixed(2)}/wk`);
        if (e.minimumFamilyTaxCredit != null)
          wffLines.push(`Entitlement — Minimum Family Tax Credit: $${Number(e.minimumFamilyTaxCredit).toFixed(2)}/wk`);
        wffLines.push(`Total weekly entitlement: $${Number(e.totalWeeklyEntitlement).toFixed(2)}/wk`);
        wffLines.push(`Payment frequency: ${wff.paymentFrequency ?? "unspecified"}`);
      }
      if (!wff.eligible) {
        const headroom = Number(wff.incomeThreshold) - Number(wff.currentIncome);
        wffLines.push(
          headroom > 0
            ? `Headroom before eligibility threshold: $${headroom.toLocaleString()}`
            : "Currently above the income threshold for entitlement.",
        );
      }
      if (wff.nextReviewDate) wffLines.push(`Next review: ${wff.nextReviewDate}`);
      chunks.push({
        deptId: this.deptId,
        content: wffLines.join("\n"),
        metadata: { area: "working-for-families", eligible: wff.eligible },
      });
    }

    // ── KiwiSaver chunk ────────────────────────────────────────────────────────
    const ks = bundle.kiwiSaver;
    if (ks) {
      const ksLines: string[] = [];
      ksLines.push(`KiwiSaver status: ${ks.membershipStatus}`);
      ksLines.push(`Contribution rate: ${ks.contributionRate}%`);
      if (ks.employerContributionRate != null)
        ksLines.push(`Employer contribution rate: ${ks.employerContributionRate}%`);
      if (ks.scheme) ksLines.push(`Scheme: ${ks.scheme}`);
      if (ks.totalBalance != null)
        ksLines.push(`Estimated balance: $${Number(ks.totalBalance).toLocaleString()}`);
      ksLines.push(`Government contribution eligible: ${ks.governmentContributionEligible ? "yes" : "no"}`);
      if (ks.firstHomeBuyerEligible != null)
        ksLines.push(`First home buyer eligible: ${ks.firstHomeBuyerEligible ? "yes" : "no"}`);
      if (ks.lastContributionDate) ksLines.push(`Last contribution: ${ks.lastContributionDate}`);
      chunks.push({
        deptId: this.deptId,
        content: ksLines.join("\n"),
        metadata: { area: "kiwisaver", membershipStatus: ks.membershipStatus },
      });
    }

    return chunks;
  }
}
