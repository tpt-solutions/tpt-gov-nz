import { z } from "zod";

export const BenefitTypeSchema = z.enum([
  "jobseeker",
  "sole-parent",
  "supported-living",
  "youth",
  "emergency",
  "accommodation-supplement",
  "disability-allowance",
  "working-for-families",
]);

export type BenefitType = z.infer<typeof BenefitTypeSchema>;

export const WinzBenefitSchema = z.object({
  type: BenefitTypeSchema,
  weeklyAmount: z.string(),
  startDate: z.string().optional(),
  reviewDate: z.string().optional(),
  status: z.string(),
});

export const WinzPaymentSchema = z.object({
  paymentId: z.string(),
  benefitType: z.string(),
  paymentDate: z.string(),
  amount: z.string(),
  method: z.string(),
});

export const WinzCaseNoteSchema = z.object({
  noteId: z.string(),
  noteDate: z.string(),
  author: z.string(),
  note: z.string(),
});

export const WINZDataBundleSchema = z.object({
  clientId: z.string(),
  activeBenefits: z.array(WinzBenefitSchema),
  totalWeeklyPayment: z.string(),
  payments: z.array(WinzPaymentSchema).optional(),
  caseNotes: z.array(WinzCaseNoteSchema).optional(),
  caseManagerName: z.string().optional(),
  nextAppointment: z.string().optional(),
});

export type WINZDataBundle = z.infer<typeof WINZDataBundleSchema>;
