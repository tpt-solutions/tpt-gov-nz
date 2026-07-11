import { z } from "zod";

export const DidSchema = z
  .string()
  .regex(/^did:gov:nz:[a-zA-Z0-9_-]+$/, "Invalid NZ government DID");

export const ScopeSchema = z.enum([
  "ird:income",
  "ird:tax-summary",
  "ird:gst",
  "ird:gst-history",
  "ird:kiwisaver",
  "ird:wff",
  "winz:benefit-status",
  "winz:payments",
  "moh:nhi",
  "moh:prescriptions",
  "moh:appointments",
  "dia:passport",
  "dia:birth-certificate",
  "nzta:licence",
  "nzta:vehicles",
  "acc:claims",
  "moe:qualifications",
  "msd:work-history",
  "linz:property",
  "moj:fines",
  "moj:disputes",
  "moj:court-records",
  "police:infringements",
  "police:reports",
  "hud:applications",
  "hud:tenancy",
  "hud:maintenance",
]);

export type Scope = z.infer<typeof ScopeSchema>;

export const CitizenIdentityTokenSchema = z.object({
  did: DidSchema,
  sessionId: z.string().uuid(),
  grantedScopes: z.array(ScopeSchema),
  issuedAt: z.number(),
  expiresAt: z.number(),
});

export type CitizenIdentityToken = z.infer<typeof CitizenIdentityTokenSchema>;

export const DataGrantCredentialSchema = z.object({
  id: z.string().uuid(),
  citizenDid: DidSchema,
  requestingDeptId: z.string(),
  providingDeptId: z.string(),
  scopes: z.array(ScopeSchema),
  issuedAt: z.number(),
  expiresAt: z.number(),
  signature: z.string(),
});

export type DataGrantCredential = z.infer<typeof DataGrantCredentialSchema>;
