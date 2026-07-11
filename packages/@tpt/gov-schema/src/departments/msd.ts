import { z } from "zod";

export const MsdStudyLinkSchema = z.object({
  hasStudentLoan: z.boolean(),
  loanBalance: z.number().optional(),
  repaymentPlan: z.string().optional(),
  hasAllowance: z.boolean(),
  allowanceType: z.string().optional(),
  nextPaymentDate: z.string().optional(),
  weeklyAmount: z.number().optional(),
});

export const MsdCaseEventSchema = z.object({
  eventDate: z.string(),
  serviceLine: z.string(),
  summary: z.string(),
});

export const MSDDataBundleSchema = z.object({
  clientNumber: z.string(),
  studylink: MsdStudyLinkSchema.optional(),
  caseHistory: z.array(MsdCaseEventSchema).optional(),
});

export type MSDDataBundle = z.infer<typeof MSDDataBundleSchema>;

export const MsdActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("apply-student-allowance"),
    courseOfStudy: z.string().min(1),
    provider: z.string().min(1),
  }),
  z.object({
    type: z.literal("update-loan-repayment-plan"),
    plan: z.string().min(1),
  }),
]);

export type MsdAction = z.infer<typeof MsdActionSchema>;
