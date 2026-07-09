import { z } from "zod";

export const DIADataBundleSchema = z.object({
  birthCertificateNumber: z.string().optional(),
  passportNumber: z.string().optional(),
  passportExpiry: z.string().optional(),
  passportRenewable: z.boolean().optional(),
  citizenshipStatus: z.enum(["citizen-by-birth", "citizen-by-grant", "permanent-resident", "other"]).optional(),
  marriageCertificateNumber: z.string().optional(),
});

export type DIADataBundle = z.infer<typeof DIADataBundleSchema>;
