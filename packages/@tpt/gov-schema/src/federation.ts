import { z } from "zod";
import { DataGrantCredentialSchema } from "./identity.js";

export const FederationMessageTypeSchema = z.enum([
  "DATA_REQUEST",
  "DATA_RESPONSE",
  "DATA_DENIED",
  "AUDIT_ACK",
]);

export type FederationMessageType = z.infer<typeof FederationMessageTypeSchema>;

export const FederationEnvelopeSchema = z.object({
  messageId: z.string().uuid(),
  correlationId: z.string().uuid().optional(),
  type: FederationMessageTypeSchema,
  fromDeptId: z.string(),
  toDeptId: z.string(),
  timestamp: z.number(),
  consentGrants: z.array(DataGrantCredentialSchema),
  payloadEncrypted: z.string(),
  signature: z.string(),
});

export type FederationEnvelope = z.infer<typeof FederationEnvelopeSchema>;

export const AuditLogEntrySchema = z.object({
  id: z.string().uuid(),
  messageId: z.string().uuid(),
  citizenDid: z.string(),
  action: FederationMessageTypeSchema,
  fromDeptId: z.string(),
  toDeptId: z.string(),
  scopesAccessed: z.array(z.string()),
  timestamp: z.number(),
  signature: z.string(),
});

export type AuditLogEntry = z.infer<typeof AuditLogEntrySchema>;
