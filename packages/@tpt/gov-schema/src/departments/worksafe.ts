import { z } from "zod";

export const WorksafeInspectionsSchema = z.object({
  reference: z.string(),
  site: z.string(),
  inspectionDate: z.string(),
  outcome: z.string(),
});

export const WorksafeInvestigationsSchema = z.object({
  reference: z.string(),
  matter: z.string(),
  status: z.string(),
  openedDate: z.string(),
});

export const WorksafeDataBundleSchema = z.object({
  worksafeId: z.string(),
  inspections: z.array(WorksafeInspectionsSchema).optional(),
  investigations: z.array(WorksafeInvestigationsSchema).optional(),
});

export type WorksafeDataBundle = z.infer<typeof WorksafeDataBundleSchema>;

export const WorksafeActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-investigation-update"),
    reference: z.string().min(1),
  }),
]);

export type WorksafeAction = z.infer<typeof WorksafeActionSchema>;
