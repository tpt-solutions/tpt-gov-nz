import { z } from "zod";

export const DiaPassportSchema = z.object({
  passportNumber: z.string(),
  expiryDate: z.string(),
  renewable: z.boolean(),
});

export const DiaBirthCertificateSchema = z.object({
  certificateNumber: z.string(),
  dateOfBirth: z.string(),
  placeOfBirth: z.string(),
  parents: z.string().optional(),
});

export const DiaCitizenshipSchema = z.object({
  status: z.enum(["citizen-by-birth", "citizen-by-grant", "permanent-resident", "other"]),
  certificateNumber: z.string().optional(),
  grantedAt: z.string().optional(),
});

export const DIADataBundleSchema = z.object({
  passportNumber: z.string(),
  passport: DiaPassportSchema.optional(),
  birthCertificate: DiaBirthCertificateSchema.optional(),
  citizenship: DiaCitizenshipSchema.optional(),
});

export type DIADataBundle = z.infer<typeof DIADataBundleSchema>;
