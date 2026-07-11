import { z } from "zod";

export const MbieBusinessSchema = z.object({
  nzbn: z.string(),
  entityName: z.string(),
  entityType: z.enum(["company", "sole-trader", "partnership", "trust"]),
  status: z.string(),
  registeredDate: z.string(),
});

export const MbieDirectorshipSchema = z.object({
  nzbn: z.string(),
  entityName: z.string(),
  role: z.string(),
  appointedDate: z.string(),
});

export const MBIEDataBundleSchema = z.object({
  personId: z.string(),
  businessRegistrations: z.array(MbieBusinessSchema).optional(),
  directorships: z.array(MbieDirectorshipSchema).optional(),
});

export type MBIEDataBundle = z.infer<typeof MBIEDataBundleSchema>;

export const MbieActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("register-business"),
    nzbn: z.string().min(1),
    entityName: z.string().min(1),
    entityType: z.enum(["company", "sole-trader", "partnership", "trust"]),
  }),
  z.object({
    type: z.literal("update-director-details"),
    address: z.string().min(1),
  }),
]);

export type MbieAction = z.infer<typeof MbieActionSchema>;
