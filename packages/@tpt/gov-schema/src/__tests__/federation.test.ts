import { describe, it, expect } from "vitest";
import {
  FederationMessageTypeSchema,
  FederationEnvelopeSchema,
  AuditLogEntrySchema,
} from "../federation.js";

describe("FederationMessageTypeSchema", () => {
  it("accepts all valid message types", () => {
    expect(FederationMessageTypeSchema.parse("DATA_REQUEST")).toBe("DATA_REQUEST");
    expect(FederationMessageTypeSchema.parse("DATA_RESPONSE")).toBe("DATA_RESPONSE");
    expect(FederationMessageTypeSchema.parse("DATA_DENIED")).toBe("DATA_DENIED");
    expect(FederationMessageTypeSchema.parse("AUDIT_ACK")).toBe("AUDIT_ACK");
  });

  it("rejects invalid message types", () => {
    expect(() => FederationMessageTypeSchema.parse("INVALID")).toThrow();
    expect(() => FederationMessageTypeSchema.parse("data_request")).toThrow();
  });
});

describe("FederationEnvelopeSchema", () => {
  const validEnvelope = {
    messageId: "00000000-0000-0000-0000-000000000001",
    type: "DATA_REQUEST",
    fromDeptId: "winz",
    toDeptId: "ird",
    timestamp: Date.now(),
    consentGrants: [],
    payloadEncrypted: "encrypted-data",
    signature: "base64sig",
  };

  it("accepts valid envelope", () => {
    const result = FederationEnvelopeSchema.parse(validEnvelope);
    expect(result.fromDeptId).toBe("winz");
    expect(result.toDeptId).toBe("ird");
  });

  it("accepts envelope with optional correlationId", () => {
    const result = FederationEnvelopeSchema.parse({
      ...validEnvelope,
      correlationId: "00000000-0000-0000-0000-000000000002",
    });
    expect(result.correlationId).toBe("00000000-0000-0000-0000-000000000002");
  });

  it("rejects envelope with invalid messageId", () => {
    expect(() =>
      FederationEnvelopeSchema.parse({ ...validEnvelope, messageId: "bad" })
    ).toThrow();
  });

  it("rejects envelope with invalid type", () => {
    expect(() =>
      FederationEnvelopeSchema.parse({ ...validEnvelope, type: "INVALID" })
    ).toThrow();
  });
});

describe("AuditLogEntrySchema", () => {
  const validEntry = {
    id: "00000000-0000-0000-0000-000000000001",
    messageId: "00000000-0000-0000-0000-000000000002",
    citizenDid: "did:gov:nz:citizen001",
    action: "DATA_REQUEST",
    fromDeptId: "winz",
    toDeptId: "ird",
    scopesAccessed: ["ird:income"],
    timestamp: Date.now(),
    signature: "base64sig",
  };

  it("accepts valid audit entry", () => {
    const result = AuditLogEntrySchema.parse(validEntry);
    expect(result.action).toBe("DATA_REQUEST");
    expect(result.scopesAccessed).toEqual(["ird:income"]);
  });

  it("rejects entry with invalid action", () => {
    expect(() =>
      AuditLogEntrySchema.parse({ ...validEntry, action: "INVALID" })
    ).toThrow();
  });

  it("rejects entry with invalid messageId UUID", () => {
    expect(() =>
      AuditLogEntrySchema.parse({ ...validEntry, messageId: "not-a-uuid" })
    ).toThrow();
  });
});
