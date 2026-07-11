import { describe, it, expect } from "vitest";
import { IRDDataBundleSchema, IRDActionSchema } from "../departments/ird.js";
import { WINZDataBundleSchema } from "../departments/winz.js";
import { MOHDataBundleSchema } from "../departments/moh.js";
import { DIADataBundleSchema } from "../departments/dia.js";
import { CitizenRefSchema, ServiceSchema, ActionResultSchema } from "../departments/adapter.js";

describe("IRDDataBundleSchema", () => {
  const validBundle = {
    irdNumber: "123-456-789",
    currentTaxYear: {
      assessmentYear: 2024,
      taxCode: "M",
      totalIncome: 75000,
      taxableIncome: 72000,
      taxLiability: 15000,
      taxPaid: 16000,
      taxRefundDue: 1000,
      taxOwing: 0,
      assessmentStatus: "final",
    },
    gstRegistered: false,
  };

  it("accepts minimal valid bundle", () => {
    const result = IRDDataBundleSchema.parse(validBundle);
    expect(result.irdNumber).toBe("123-456-789");
    expect(result.gstRegistered).toBe(false);
  });

  it("accepts bundle with all optional fields", () => {
    const result = IRDDataBundleSchema.parse({
      ...validBundle,
      gstRegistered: true,
      gstPeriods: [
        {
          periodId: "p1",
          periodStart: "2024-04-01",
          periodEnd: "2024-06-30",
          filingDue: "2024-07-28",
          status: "filed",
          salesIncome: 50000,
          gstOnSales: 7500,
          gstOnPurchases: 3000,
          refundOrPayment: -4500,
        },
      ],
      kiwiSaver: {
        membershipStatus: "active",
        contributionRate: 6,
        scheme: "ASB",
        totalBalance: 45000,
        governmentContributionEligible: true,
        firstHomeBuyerEligible: true,
      },
      workingForFamilies: {
        eligible: true,
        currentEntitlement: {
          familyTaxCredit: 120,
          totalWeeklyEntitlement: 150,
        },
        incomeThreshold: 80000,
        currentIncome: 65000,
        numberOfDependantChildren: 2,
        paymentFrequency: "weekly",
      },
      taxHistory: [],
    });
    expect(result.kiwiSaver?.scheme).toBe("ASB");
    expect(result.workingForFamilies?.numberOfDependantChildren).toBe(2);
  });

  it("rejects bundle with invalid assessmentStatus", () => {
    expect(() =>
      IRDDataBundleSchema.parse({
        ...validBundle,
        currentTaxYear: { ...validBundle.currentTaxYear, assessmentStatus: "invalid" },
      })
    ).toThrow();
  });
});

describe("IRDActionSchema", () => {
  it("accepts update-kiwisaver-rate action", () => {
    const result = IRDActionSchema.parse({ type: "update-kiwisaver-rate", newRate: 6 });
    expect(result.type).toBe("update-kiwisaver-rate");
  });

  it("accepts file-gst-return action", () => {
    const result = IRDActionSchema.parse({
      type: "file-gst-return",
      periodId: "p1",
      salesIncome: 50000,
      gstOnSales: 7500,
      gstOnPurchases: 3000,
    });
    expect(result.type).toBe("file-gst-return");
  });

  it("accepts request-tax-summary action", () => {
    const result = IRDActionSchema.parse({
      type: "request-tax-summary",
      assessmentYear: 2024,
    });
    expect(result.type).toBe("request-tax-summary");
  });

  it("rejects invalid action type", () => {
    expect(() =>
      IRDActionSchema.parse({ type: "unknown-action" })
    ).toThrow();
  });
});

describe("WINZDataBundleSchema", () => {
  it("accepts valid bundle", () => {
    const result = WINZDataBundleSchema.parse({
      clientId: "winz-001",
      activeBenefits: [
        { type: "jobseeker", weeklyAmount: "350", startDate: "2024-01-01", status: "active" },
      ],
      totalWeeklyPayment: "350",
    });
    expect(result.activeBenefits).toHaveLength(1);
  });

  it("accepts bundle with optional fields", () => {
    const result = WINZDataBundleSchema.parse({
      clientId: "winz-002",
      activeBenefits: [],
      totalWeeklyPayment: "0",
      caseManagerName: "Jane Smith",
      nextAppointment: "2024-07-15T10:00:00Z",
    });
    expect(result.caseManagerName).toBe("Jane Smith");
  });

  it("rejects invalid benefit type", () => {
    expect(() =>
      WINZDataBundleSchema.parse({
        clientId: "x",
        activeBenefits: [{ type: "invalid", weeklyAmount: "100", startDate: "2024-01-01", status: "active" }],
        totalWeeklyPayment: "100",
      })
    ).toThrow();
  });
});

describe("MOHDataBundleSchema", () => {
  it("accepts valid bundle", () => {
    const result = MOHDataBundleSchema.parse({ nhiNumber: "ABC1234" });
    expect(result.nhiNumber).toBe("ABC1234");
  });

  it("accepts bundle with all optional fields", () => {
    const result = MOHDataBundleSchema.parse({
      nhiNumber: "XYZ5678",
      enrolledGP: { practiceName: "City Medical", address: "1 Main St", phone: "09-555-1234" },
      activePrescriptions: [{ medication: "Paracetamol", dose: "500mg", repeatsRemaining: 2 }],
      upcomingAppointments: [{ provider: "Dr Smith", date: "2024-08-01", type: "checkup" }],
      vaccinations: [{ vaccine: "COVID-19", date: "2024-06-01" }],
    });
    expect(result.enrolledGP?.practiceName).toBe("City Medical");
    expect(result.activePrescriptions).toHaveLength(1);
  });
});

describe("DIADataBundleSchema", () => {
  it("accepts bundle with only the required field", () => {
    const result = DIADataBundleSchema.parse({ passportNumber: "PA000000" });
    expect(result.passportNumber).toBe("PA000000");
    expect(result.passport).toBeUndefined();
  });

  it("accepts bundle with passport data", () => {
    const result = DIADataBundleSchema.parse({
      passportNumber: "PA123456",
      passport: { passportNumber: "PA123456", expiryDate: "2028-12-31", renewable: true },
      citizenship: { status: "citizen-by-birth" },
    });
    expect(result.citizenship?.status).toBe("citizen-by-birth");
  });

  it("rejects invalid citizenship status", () => {
    expect(() =>
      DIADataBundleSchema.parse({ citizenship: { status: "invalid" } })
    ).toThrow();
  });
});

describe("Adapter schemas", () => {
  it("CitizenRefSchema accepts valid ref", () => {
    const result = CitizenRefSchema.parse({
      did: "did:gov:nz:citizen001",
      deptLocalId: "IRD-123",
    });
    expect(result.deptLocalId).toBe("IRD-123");
  });

  it("ServiceSchema accepts valid service", () => {
    const result = ServiceSchema.parse({
      id: "tax-summary",
      name: "Tax Summary",
      description: "View your tax assessment",
      requiredScopes: ["ird:tax-summary"],
    });
    expect(result.requiredScopes).toEqual(["ird:tax-summary"]);
  });

  it("ActionResultSchema accepts success", () => {
    const result = ActionResultSchema.parse({ success: true, message: "Done" });
    expect(result.success).toBe(true);
  });

  it("ActionResultSchema accepts failure", () => {
    const result = ActionResultSchema.parse({ success: false, message: "Failed" });
    expect(result.success).toBe(false);
  });
});
