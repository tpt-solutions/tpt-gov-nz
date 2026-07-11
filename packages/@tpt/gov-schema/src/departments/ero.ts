import { z } from "zod";

export const EroReviewsSchema = z.object({
  school: z.string(),
  rating: z.string(),
  reviewDate: z.string(),
  nextReview: z.string(),
});

export const EroReportsSchema = z.object({
  title: z.string(),
  published: z.string(),
});

export const EroDataBundleSchema = z.object({
  eroId: z.string(),
  reviews: z.array(EroReviewsSchema).optional(),
  reports: z.array(EroReportsSchema).optional(),
});

export type EroDataBundle = z.infer<typeof EroDataBundleSchema>;

export const EroActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-report-copy"),
    title: z.string().min(1),
  }),
]);

export type EroAction = z.infer<typeof EroActionSchema>;
