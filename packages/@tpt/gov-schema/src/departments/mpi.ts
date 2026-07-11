import { z } from "zod";

export const MpiRegistrationSchema = z.object({
  nzbn: z.string(),
  businessName: z.string(),
  type: z.string(),
  status: z.string(),
  registeredDate: z.string(),
});

export const MpiCertificationSchema = z.object({
  certNumber: z.string(),
  category: z.string(),
  issuedDate: z.string(),
  expiresDate: z.string(),
});

export const MPIDataBundleSchema = z.object({
  mpiId: z.string(),
  registrations: z.array(MpiRegistrationSchema).optional(),
  certifications: z.array(MpiCertificationSchema).optional(),
});

export type MPIDataBundle = z.infer<typeof MPIDataBundleSchema>;
