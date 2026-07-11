import { z } from "zod";

export const TpkProgrammeSchema = z.object({
  programmeName: z.string(),
  status: z.string(),
  region: z.string(),
});

export const TpkFundingSchema = z.object({
  grantId: z.string(),
  amount: z.number(),
  purpose: z.string(),
  status: z.string(),
});

export const TPKDataBundleSchema = z.object({
  tpkId: z.string(),
  programmes: z.array(TpkProgrammeSchema).optional(),
  funding: z.array(TpkFundingSchema).optional(),
});

export type TPKDataBundle = z.infer<typeof TPKDataBundleSchema>;
