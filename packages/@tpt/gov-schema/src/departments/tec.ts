import { z } from "zod";

export const TecFundingSchema = z.object({
  provider: z.string(),
  amount: z.number(),
  year: z.number(),
});

export const TecCoursesSchema = z.object({
  courseName: z.string(),
  provider: z.string(),
  status: z.string(),
});

export const TecDataBundleSchema = z.object({
  tecId: z.string(),
  funding: z.array(TecFundingSchema).optional(),
  courses: z.array(TecCoursesSchema).optional(),
});

export type TecDataBundle = z.infer<typeof TecDataBundleSchema>;

export const TecActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-course-info"),
    courseName: z.string().min(1),
  }),
]);

export type TecAction = z.infer<typeof TecActionSchema>;
