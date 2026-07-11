import { z } from "zod";

export const MfeEmissionsSchema = z.object({
  reportYear: z.number(),
  sector: z.string(),
  tonnesCO2e: z.number(),
});

export const MfeReportsSchema = z.object({
  title: z.string(),
  published: z.string(),
  status: z.string(),
});

export const MfeDataBundleSchema = z.object({
  mfeId: z.string(),
  emissions: z.array(MfeEmissionsSchema).optional(),
  reports: z.array(MfeReportsSchema).optional(),
});

export type MfeDataBundle = z.infer<typeof MfeDataBundleSchema>;

export const MfeActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-emissions-report"),
    sector: z.string().min(1),
  }),
]);

export type MfeAction = z.infer<typeof MfeActionSchema>;
