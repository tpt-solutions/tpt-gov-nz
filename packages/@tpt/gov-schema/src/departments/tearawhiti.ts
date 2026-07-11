import { z } from "zod";

export const TearawhitiTreatySettlementsSchema = z.object({
  iwi: z.string(),
  status: z.string(),
  settledDate: z.string(),
});

export const TearawhitiEngagementsSchema = z.object({
  topic: z.string(),
  engagementDate: z.string(),
  outcome: z.string(),
});

export const TearawhitiDataBundleSchema = z.object({
  tearawhitiId: z.string(),
  treaty_settlements: z.array(TearawhitiTreatySettlementsSchema).optional(),
  engagements: z.array(TearawhitiEngagementsSchema).optional(),
});

export type TearawhitiDataBundle = z.infer<typeof TearawhitiDataBundleSchema>;

export const TearawhitiActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-settlement-info"),
    iwi: z.string().min(1),
  }),
]);

export type TearawhitiAction = z.infer<typeof TearawhitiActionSchema>;
