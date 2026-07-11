import { z } from "zod";

export const MoeEnrolmentSchema = z.object({
  school: z.string(),
  yearLevel: z.number(),
  status: z.string(),
});

export const MoeStudentSupportSchema = z.object({
  service: z.string(),
  status: z.string(),
  nextReview: z.string(),
});

export const MoeDataBundleSchema = z.object({
  moeId: z.string(),
  enrolment: MoeEnrolmentSchema.optional(),
  student_support: z.array(MoeStudentSupportSchema).optional(),
});

export type MoeDataBundle = z.infer<typeof MoeDataBundleSchema>;

export const MoeActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-support-review"),
    service: z.string().min(1),
  }),
]);

export type MoeAction = z.infer<typeof MoeActionSchema>;
