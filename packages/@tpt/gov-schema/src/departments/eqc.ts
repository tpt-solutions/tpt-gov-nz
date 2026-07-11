import { z } from "zod";

export const EqcClaimsSchema = z.object({
  reference: z.string(),
  property: z.string(),
  status: z.string(),
  lodgedDate: z.string(),
});

export const EqcCoverSchema = z.object({
  property: z.string(),
  sumInsured: z.number(),
  validTo: z.string(),
});

export const EqcDataBundleSchema = z.object({
  eqcId: z.string(),
  claims: z.array(EqcClaimsSchema).optional(),
  cover: EqcCoverSchema.optional(),
});

export type EqcDataBundle = z.infer<typeof EqcDataBundleSchema>;

export const EqcActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-claim-update"),
    reference: z.string().min(1),
  }),
]);

export type EqcAction = z.infer<typeof EqcActionSchema>;
