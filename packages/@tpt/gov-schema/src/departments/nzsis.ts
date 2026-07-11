import { z } from "zod";

export const NzsisMandatesSchema = z.object({
  reference: z.string(),
  agency: z.string(),
  status: z.string(),
  issuedDate: z.string(),
});

export const NzsisThreatsSchema = z.object({
  reference: z.string(),
  category: z.string(),
  status: z.string(),
  assessedDate: z.string(),
});

export const NzsisDataBundleSchema = z.object({
  nzsisId: z.string(),
  mandates: z.array(NzsisMandatesSchema).optional(),
  threats: z.array(NzsisThreatsSchema).optional(),
});

export type NzsisDataBundle = z.infer<typeof NzsisDataBundleSchema>;

export const NzsisActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-mandate-info"),
    reference: z.string().min(1),
  }),
]);

export type NzsisAction = z.infer<typeof NzsisActionSchema>;
