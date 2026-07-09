import { describe, it, expect } from "vitest";
import {
  DidSchema,
  ScopeSchema,
  CitizenIdentityTokenSchema,
  DataGrantCredentialSchema,
} from "../identity.js";

describe("DidSchema", () => {
  it("accepts valid NZ government DID", () => {
    expect(DidSchema.parse("did:gov:nz:abc123")).toBe("did:gov:nz:abc123");
    expect(DidSchema.parse("did:gov:nz:ABC-def_012")).toBe("did:gov:nz:ABC-def_012");
  });

  it("rejects non-NZ DIDs", () => {
    expect(() => DidSchema.parse("did:web:example.com")).toThrow();
    expect(() => DidSchema.parse("did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK")).toThrow();
  });

  it("rejects empty string", () => {
    expect(() => DidSchema.parse("")).toThrow();
  });

  it("rejects DID without prefix", () => {
    expect(() => DidSchema.parse("gov:nz:abc123")).toThrow();
  });
});

describe("ScopeSchema", () => {
  it("accepts all valid scopes", () => {
    const scopes = [
      "ird:income", "ird:tax-summary", "ird:gst", "ird:gst-history",
      "ird:kiwisaver", "ird:wff", "winz:benefit-status", "winz:payments",
      "moh:nhi", "moh:prescriptions", "moh:appointments",
      "dia:passport", "dia:birth-certificate",
      "nzta:licence", "nzta:vehicles", "acc:claims",
      "moe:qualifications", "msd:work-history", "linz:property",
    ];
    for (const scope of scopes) {
      expect(ScopeSchema.parse(scope)).toBe(scope);
    }
  });

  it("rejects unknown scopes", () => {
    expect(() => ScopeSchema.parse("ird:unknown")).toThrow();
    expect(() => ScopeSchema.parse("custom:scope")).toThrow();
  });
});

describe("CitizenIdentityTokenSchema", () => {
  const validToken = {
    did: "did:gov:nz:citizen001",
    sessionId: "00000000-0000-0000-0000-000000000001",
    grantedScopes: ["ird:income", "ird:tax-summary"],
    issuedAt: Date.now(),
    expiresAt: Date.now() + 3600_000,
  };

  it("accepts valid token", () => {
    const result = CitizenIdentityTokenSchema.parse(validToken);
    expect(result.did).toBe("did:gov:nz:citizen001");
    expect(result.grantedScopes).toHaveLength(2);
  });

  it("rejects token with invalid DID", () => {
    expect(() =>
      CitizenIdentityTokenSchema.parse({ ...validToken, did: "invalid" })
    ).toThrow();
  });

  it("rejects token with invalid session UUID", () => {
    expect(() =>
      CitizenIdentityTokenSchema.parse({ ...validToken, sessionId: "not-a-uuid" })
    ).toThrow();
  });

  it("rejects token with invalid scope", () => {
    expect(() =>
      CitizenIdentityTokenSchema.parse({
        ...validToken,
        grantedScopes: ["ird:invalid"],
      })
    ).toThrow();
  });
});

describe("DataGrantCredentialSchema", () => {
  const validGrant = {
    id: "00000000-0000-0000-0000-000000000001",
    citizenDid: "did:gov:nz:citizen001",
    requestingDeptId: "winz",
    providingDeptId: "ird",
    scopes: ["ird:income"],
    issuedAt: Date.now(),
    expiresAt: Date.now() + 86400_000,
    signature: "base64signature",
  };

  it("accepts valid grant", () => {
    const result = DataGrantCredentialSchema.parse(validGrant);
    expect(result.requestingDeptId).toBe("winz");
    expect(result.providingDeptId).toBe("ird");
  });

  it("rejects grant with invalid citizen DID", () => {
    expect(() =>
      DataGrantCredentialSchema.parse({ ...validGrant, citizenDid: "bad" })
    ).toThrow();
  });

  it("rejects grant with invalid UUID", () => {
    expect(() =>
      DataGrantCredentialSchema.parse({ ...validGrant, id: "not-a-uuid" })
    ).toThrow();
  });
});
