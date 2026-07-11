import { z } from "zod";

export const MotStrategiesSchema = z.object({
  title: z.string(),
  year: z.number(),
  status: z.string(),
});

export const MotProgrammesSchema = z.object({
  name: z.string(),
  budget: z.number(),
  status: z.string(),
});

export const MotDataBundleSchema = z.object({
  motId: z.string(),
  strategies: z.array(MotStrategiesSchema).optional(),
  programmes: z.array(MotProgrammesSchema).optional(),
});

export type MotDataBundle = z.infer<typeof MotDataBundleSchema>;

export const MotActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-programme-info"),
    name: z.string().min(1),
  }),
]);

export type MotAction = z.infer<typeof MotActionSchema>;
