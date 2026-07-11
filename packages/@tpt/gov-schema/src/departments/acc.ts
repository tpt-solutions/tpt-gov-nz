import { z } from "zod";

export const AccClaimSchema = z.object({
  claimNumber: z.string(),
  claimType: z.enum(["work", "non-work", "treatment"]),
  status: z.enum(["open", "approved", "declined", "closed"]),
  injuryDate: z.string(),
  description: z.string(),
  weeklyCompensation: z.number().optional(),
});

export const AccEntitlementSchema = z.object({
  hasEntitlement: z.boolean(),
  type: z.string().optional(),
  weeklyAmount: z.number().optional(),
  remainingWeeks: z.number().optional(),
});

export const AccRehabilitationSchema = z.object({
  planId: z.string(),
  description: z.string(),
  status: z.string(),
  provider: z.string().optional(),
  nextReview: z.string().optional(),
});

export const ACCDataBundleSchema = z.object({
  clientNumber: z.string(),
  claims: z.array(AccClaimSchema).optional(),
  entitlements: AccEntitlementSchema.optional(),
  rehabilitation: z.array(AccRehabilitationSchema).optional(),
});

export type ACCDataBundle = z.infer<typeof ACCDataBundleSchema>;

export const AccActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("lodge-claim"),
    claimType: z.enum(["work", "non-work", "treatment"]),
    injuryDate: z.string(),
    description: z.string().min(1),
  }),
  z.object({
    type: z.literal("request-rehabilitation-review"),
    claimNumber: z.string().optional(),
  }),
]);

export type AccAction = z.infer<typeof AccActionSchema>;
