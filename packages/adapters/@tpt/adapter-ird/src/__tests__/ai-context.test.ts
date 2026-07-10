import { describe, it, expect } from "vitest";
import { IRDAdapter } from "../index.js";
import type { IRDDataBundle } from "@tpt/gov-schema";

function makeBundle(overrides: Partial<IRDDataBundle> = {}): IRDDataBundle {
  return {
    irdNumber: "123-456-789",
    currentTaxYear: {
      assessmentYear: 2025,
      taxCode: "M",
      totalIncome: "66200.00",
      taxableIncome: "66200.00",
      taxLiability: "9870.00",
      taxPaid: "10100.00",
      taxRefundDue: "230.00",
      taxOwing: "0.00",
      assessmentStatus: "final",
    },
    gstRegistered: false,
    workingForFamilies: {
      eligible: true,
      numberOfDependantChildren: 2,
      incomeThreshold: "73944.00",
      currentIncome: "66200.00",
      currentEntitlement: {
        familyTaxCredit: "127.00",
        inWorkTaxCredit: "72.00",
        bestStartPayment: "0.00",
        minimumFamilyTaxCredit: "0.00",
        totalWeeklyEntitlement: "199.00",
      },
      paymentFrequency: "weekly",
    },
    kiwiSaver: {
      membershipStatus: "active",
      contributionRate: "3.00",
      employerContributionRate: "3.00",
      scheme: "Simplicity",
      totalBalance: "18500.00",
      governmentContributionEligible: true,
      firstHomeBuyerEligible: true,
    },
    ...overrides,
  };
}

describe("IRDAdapter.produceAiContext", () => {
  const adapter = new IRDAdapter("http://localhost:8090", "test-key");

  it("produces separate chunks for tax, WFF and KiwiSaver", () => {
    const chunks = adapter.produceAiContext(makeBundle());
    const areas = chunks.map((c) => c.metadata?.area);
    expect(areas).toContain("tax");
    expect(areas).toContain("working-for-families");
    expect(areas).toContain("kiwisaver");
  });

  it("includes WFF entitlement detail when eligible", () => {
    const chunks = adapter.produceAiContext(makeBundle());
    const wff = chunks.find((c) => c.metadata?.area === "working-for-families");
    expect(wff?.content).toContain("Total weekly entitlement: $199.00/wk");
    expect(wff?.content).toContain("Family Tax Credit: $127.00/wk");
  });

  it("includes KiwiSaver rate and balance detail", () => {
    const chunks = adapter.produceAiContext(makeBundle());
    const ks = chunks.find((c) => c.metadata?.area === "kiwisaver");
    expect(ks?.content).toContain("Contribution rate: 3.00%");
    expect(ks?.content).toContain("Estimated balance: $18,500");
  });

  it("reports income headroom when not WFF eligible", () => {
    const chunks = adapter.produceAiContext(
      makeBundle({
        workingForFamilies: {
          eligible: false,
          numberOfDependantChildren: 0,
          incomeThreshold: "73944.00",
          currentIncome: "90000.00",
        },
      }),
    );
    const wff = chunks.find((c) => c.metadata?.area === "working-for-families");
    expect(wff?.content).toContain("above the income threshold");
  });

  it("omits KiwiSaver/WFF chunks when absent from bundle", () => {
    const chunks = adapter.produceAiContext(
      makeBundle({ kiwiSaver: undefined, workingForFamilies: undefined }),
    );
    expect(chunks.map((c) => c.metadata?.area)).toEqual(["tax"]);
  });
});
