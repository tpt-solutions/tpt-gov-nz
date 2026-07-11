import { z } from "zod";

export const PublicserviceWorkforceSchema = z.object({
  reportYear: z.number(),
  agency: z.string(),
  headcount: z.number(),
});

export const PublicserviceAgencyRatingsSchema = z.object({
  agency: z.string(),
  rating: z.string(),
  ratingYear: z.number(),
});

export const PublicserviceDataBundleSchema = z.object({
  publicserviceId: z.string(),
  workforce: z.array(PublicserviceWorkforceSchema).optional(),
  agency_ratings: z.array(PublicserviceAgencyRatingsSchema).optional(),
});

export type PublicserviceDataBundle = z.infer<typeof PublicserviceDataBundleSchema>;

export const PublicserviceActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-workforce-report"),
    agency: z.string().min(1),
  }),
]);

export type PublicserviceAction = z.infer<typeof PublicserviceActionSchema>;
