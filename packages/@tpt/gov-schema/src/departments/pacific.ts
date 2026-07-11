import { z } from "zod";

export const PacificProgrammesSchema = z.object({
  programmeName: z.string(),
  status: z.string(),
  year: z.number(),
});

export const PacificLanguageServicesSchema = z.object({
  service: z.string(),
  region: z.string(),
  status: z.string(),
});

export const PacificDataBundleSchema = z.object({
  pacificId: z.string(),
  programmes: z.array(PacificProgrammesSchema).optional(),
  language_services: z.array(PacificLanguageServicesSchema).optional(),
});

export type PacificDataBundle = z.infer<typeof PacificDataBundleSchema>;

export const PacificActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-language-service"),
    service: z.string().min(1),
  }),
]);

export type PacificAction = z.infer<typeof PacificActionSchema>;
