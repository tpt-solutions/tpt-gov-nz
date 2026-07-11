import { z } from "zod";

export const FenzFireSafetySchema = z.object({
  property: z.string(),
  grade: z.string(),
  lastInspection: z.string(),
});

export const FenzIncidentsSchema = z.object({
  reference: z.string(),
  incidentType: z.string(),
  incidentDate: z.string(),
  status: z.string(),
});

export const FenzDataBundleSchema = z.object({
  fenzId: z.string(),
  fire_safety: FenzFireSafetySchema.optional(),
  incidents: z.array(FenzIncidentsSchema).optional(),
});

export type FenzDataBundle = z.infer<typeof FenzDataBundleSchema>;

export const FenzActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-safety-check"),
    property: z.string().min(1),
  }),
]);

export type FenzAction = z.infer<typeof FenzActionSchema>;
