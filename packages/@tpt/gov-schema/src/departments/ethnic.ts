import { z } from "zod";

export const EthnicProgrammesSchema = z.object({
  programmeName: z.string(),
  status: z.string(),
  year: z.number(),
});

export const EthnicCommunityGrantsSchema = z.object({
  grantName: z.string(),
  amount: z.number(),
  status: z.string(),
});

export const EthnicDataBundleSchema = z.object({
  ethnicId: z.string(),
  programmes: z.array(EthnicProgrammesSchema).optional(),
  community_grants: z.array(EthnicCommunityGrantsSchema).optional(),
});

export type EthnicDataBundle = z.infer<typeof EthnicDataBundleSchema>;

export const EthnicActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-grant-info"),
    grantName: z.string().min(1),
  }),
]);

export type EthnicAction = z.infer<typeof EthnicActionSchema>;
