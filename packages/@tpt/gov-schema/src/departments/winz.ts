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

export const WINZDataBundleSchema = z.object({
  clientId: z.string(),
  activeBenefits: z.array(
    z.object({
      type: BenefitTypeSchema,
      weeklyAmount: z.number(),
      startDate: z.string(),
      reviewDate: z.string().optional(),
    })
  ),
  totalWeeklyPayment: z.number(),
  caseManagerName: z.string().optional(),
  nextAppointment: z.string().optional(),
});

export type WINZDataBundle = z.infer<typeof WINZDataBundleSchema>;
