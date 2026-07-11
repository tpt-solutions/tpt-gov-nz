import { z } from "zod";

export const CorrectionsProbationSchema = z.object({
  status: z.string(),
  officerName: z.string(),
  nextReportDate: z.string(),
  location: z.string(),
});

export const CorrectionsCaseSchema = z.object({
  caseNumber: z.string(),
  sentenceType: z.string(),
  startDate: z.string(),
  endDate: z.string().nullable(),
  summary: z.string(),
});

export const CORRECTIONSDataBundleSchema = z.object({
  correctionsId: z.string(),
  probation: CorrectionsProbationSchema.optional(),
  case: z.array(CorrectionsCaseSchema).optional(),
});

export type CORRECTIONSDataBundle = z.infer<typeof CORRECTIONSDataBundleSchema>;
