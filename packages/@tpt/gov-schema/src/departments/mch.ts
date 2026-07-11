import { z } from "zod";

export const MchHeritageSitesSchema = z.object({
  name: z.string(),
  status: z.string(),
  region: z.string(),
});

export const MchGrantsSchema = z.object({
  grantName: z.string(),
  amount: z.number(),
  status: z.string(),
});

export const MchDataBundleSchema = z.object({
  mchId: z.string(),
  heritage_sites: z.array(MchHeritageSitesSchema).optional(),
  grants: z.array(MchGrantsSchema).optional(),
});

export type MchDataBundle = z.infer<typeof MchDataBundleSchema>;

export const MchActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-grant-info"),
    grantName: z.string().min(1),
  }),
]);

export type MchAction = z.infer<typeof MchActionSchema>;
