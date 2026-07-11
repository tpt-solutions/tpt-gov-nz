import { z } from "zod";

export const RegulationRegulatoryReviewsSchema = z.object({
  topic: z.string(),
  agency: z.string(),
  status: z.string(),
  reviewYear: z.number(),
});

export const RegulationProposalsSchema = z.object({
  title: z.string(),
  status: z.string(),
});

export const RegulationDataBundleSchema = z.object({
  regulationId: z.string(),
  regulatory_reviews: z.array(RegulationRegulatoryReviewsSchema).optional(),
  proposals: z.array(RegulationProposalsSchema).optional(),
});

export type RegulationDataBundle = z.infer<typeof RegulationDataBundleSchema>;

export const RegulationActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-review-summary"),
    topic: z.string().min(1),
  }),
]);

export type RegulationAction = z.infer<typeof RegulationActionSchema>;
