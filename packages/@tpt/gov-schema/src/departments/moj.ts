import { z } from "zod";

export const MojFineSchema = z.object({
  fineNumber: z.string(),
  fineType: z.enum(["traffic", "reserve", "court"]),
  status: z.enum(["unpaid", "paid", "overdue", "payment-plan"]),
  amount: z.number(),
  offenseDate: z.string(),
  dueDate: z.string(),
  description: z.string(),
});

export const MojDisputeSchema = z.object({
  disputeNumber: z.string(),
  claimType: z.enum(["consumer", "tenancy", "debt"]),
  status: z.enum(["filed", "scheduled", "resolved", "withdrawn"]),
  amountClaimed: z.number().optional(),
  hearingDate: z.string().optional(),
  description: z.string(),
});

export const MojCourtRecordSchema = z.object({
  caseNumber: z.string(),
  caseType: z.enum(["traffic", "civil", "family"]),
  status: z.enum(["open", "closed"]),
  nextHearingDate: z.string().optional(),
  description: z.string(),
});

export const MOJDataBundleSchema = z.object({
  clientNumber: z.string(),
  fines: z.array(MojFineSchema).optional(),
  disputes: z.array(MojDisputeSchema).optional(),
  courtRecords: z.array(MojCourtRecordSchema).optional(),
});

export type MOJDataBundle = z.infer<typeof MOJDataBundleSchema>;

export const MojActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("pay-fine"),
    fineNumber: z.string(),
    amount: z.number().optional(),
  }),
  z.object({
    type: z.literal("file-dispute-claim"),
    claimType: z.enum(["consumer", "tenancy", "debt"]),
    amountClaimed: z.number().optional(),
    description: z.string().min(1),
  }),
  z.object({
    type: z.literal("request-name-change"),
    newName: z.string().min(1),
    reason: z.string().min(1),
  }),
]);

export type MojAction = z.infer<typeof MojActionSchema>;
