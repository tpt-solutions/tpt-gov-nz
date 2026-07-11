import { z } from "zod";

export const DocPermitSchema = z.object({
  permitNumber: z.string(),
  activity: z.string(),
  location: z.string(),
  status: z.string(),
  expiresDate: z.string(),
});

export const DocConcessionSchema = z.object({
  concessionId: z.string(),
  type: z.string(),
  holder: z.string(),
  startDate: z.string(),
  endDate: z.string(),
});

export const DOCDataBundleSchema = z.object({
  docId: z.string(),
  permits: z.array(DocPermitSchema).optional(),
  concessions: z.array(DocConcessionSchema).optional(),
});

export type DOCDataBundle = z.infer<typeof DOCDataBundleSchema>;
