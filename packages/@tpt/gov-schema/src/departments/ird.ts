import { z } from "zod";

export const IRDTaxAssessmentSchema = z.object({
  assessmentYear: z.number(),
  taxCode: z.string(),
  employmentIncome: z.number().optional(),
  selfEmploymentIncome: z.number().optional(),
  rentalIncome: z.number().optional(),
  otherIncome: z.number().optional(),
  totalIncome: z.number(),
  totalDeductions: z.number().optional(),
  taxableIncome: z.number(),
  taxLiability: z.number(),
  taxPaid: z.number(),
  taxRefundDue: z.number(),
  taxOwing: z.number(),
  assessmentStatus: z.enum(["final", "provisional", "estimated"]),
});

export type IRDTaxAssessment = z.infer<typeof IRDTaxAssessmentSchema>;

export const IRDGstPeriodSchema = z.object({
  periodId: z.string(),
  periodStart: z.string(),
  periodEnd: z.string(),
  filingDue: z.string(),
  status: z.enum(["filed", "due", "overdue", "not-required"]),
  salesIncome: z.number().optional(),
  gstOnSales: z.number().optional(),
  gstOnPurchases: z.number().optional(),
  refundOrPayment: z.number().optional(),
});

export type IRDGstPeriod = z.infer<typeof IRDGstPeriodSchema>;

export const IRDKiwiSaverSchema = z.object({
  membershipStatus: z.enum(["active", "suspended", "opted-out", "not-enrolled"]),
  contributionRate: z.number(),
  employerContributionRate: z.number().optional(),
  scheme: z.string().optional(),
  totalBalance: z.number().optional(),
  lastContributionDate: z.string().optional(),
  governmentContributionEligible: z.boolean(),
  firstHomeBuyerEligible: z.boolean().optional(),
});

export type IRDKiwiSaver = z.infer<typeof IRDKiwiSaverSchema>;

export const IRDWorkingForFamiliesSchema = z.object({
  eligible: z.boolean(),
  currentEntitlement: z.object({
    familyTaxCredit: z.number(),
    inWorkTaxCredit: z.number().optional(),
    bestStartPayment: z.number().optional(),
    minimumFamilyTaxCredit: z.number().optional(),
    totalWeeklyEntitlement: z.number(),
  }).optional(),
  incomeThreshold: z.number(),
  currentIncome: z.number(),
  numberOfDependantChildren: z.number(),
  paymentFrequency: z.enum(["weekly", "fortnightly", "lump-sum"]).optional(),
  nextReviewDate: z.string().optional(),
});

export type IRDWorkingForFamilies = z.infer<typeof IRDWorkingForFamiliesSchema>;

export const IRDDataBundleSchema = z.object({
  irdNumber: z.string(),
  currentTaxYear: IRDTaxAssessmentSchema,
  taxHistory: z.array(IRDTaxAssessmentSchema).optional(),
  gstRegistered: z.boolean(),
  gstPeriods: z.array(IRDGstPeriodSchema).optional(),
  kiwiSaver: IRDKiwiSaverSchema.optional(),
  workingForFamilies: IRDWorkingForFamiliesSchema.optional(),
});

export type IRDDataBundle = z.infer<typeof IRDDataBundleSchema>;

export const IRDActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("update-kiwisaver-rate"),
    newRate: z.union([z.literal(3), z.literal(4), z.literal(6), z.literal(8), z.literal(10)]),
  }),
  z.object({
    type: z.literal("file-gst-return"),
    periodId: z.string(),
    salesIncome: z.number(),
    gstOnSales: z.number(),
    gstOnPurchases: z.number(),
  }),
  z.object({
    type: z.literal("request-tax-summary"),
    assessmentYear: z.number(),
  }),
]);

export type IRDAction = z.infer<typeof IRDActionSchema>;
