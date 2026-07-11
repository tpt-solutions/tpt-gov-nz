import { z } from "zod";

export const MaritimeVesselsSchema = z.object({
  vesselName: z.string(),
  flag: z.string(),
  status: z.string(),
});

export const MaritimeIncidentsSchema = z.object({
  reference: z.string(),
  incidentType: z.string(),
  incidentDate: z.string(),
  status: z.string(),
});

export const MaritimeDataBundleSchema = z.object({
  maritimeId: z.string(),
  vessels: z.array(MaritimeVesselsSchema).optional(),
  incidents: z.array(MaritimeIncidentsSchema).optional(),
});

export type MaritimeDataBundle = z.infer<typeof MaritimeDataBundleSchema>;

export const MaritimeActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("report-incident"),
    incidentType: z.string().min(1),
  }),
]);

export type MaritimeAction = z.infer<typeof MaritimeActionSchema>;
