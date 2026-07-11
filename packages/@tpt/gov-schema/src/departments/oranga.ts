import { z } from "zod";

export const OrangaCarePlacementsSchema = z.object({
  placementType: z.string(),
  startDate: z.string(),
  region: z.string(),
});

export const OrangaSupportServicesSchema = z.object({
  service: z.string(),
  status: z.string(),
  nextReview: z.string(),
});

export const OrangaDataBundleSchema = z.object({
  orangaId: z.string(),
  care_placements: z.array(OrangaCarePlacementsSchema).optional(),
  support_services: z.array(OrangaSupportServicesSchema).optional(),
});

export type OrangaDataBundle = z.infer<typeof OrangaDataBundleSchema>;

export const OrangaActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-support-review"),
    service: z.string().min(1),
  }),
]);

export type OrangaAction = z.infer<typeof OrangaActionSchema>;
