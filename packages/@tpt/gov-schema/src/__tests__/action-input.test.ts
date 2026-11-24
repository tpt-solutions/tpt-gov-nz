import { describe, it, expect } from "vitest";
import {
  MAX_ACTION_PARAMETER_BYTES,
  MAX_QUESTION_LENGTH,
  MAX_SCOPES_PER_REQUEST,
  parseActionRequest,
  parseDid,
  parseQuestion,
  parseScopes,
} from "../action-input.js";

describe("parseScopes", () => {
  it("accepts known department scopes", () => {
    expect(parseScopes(["ird:wff", "ird:kiwisaver"])).toEqual(["ird:wff", "ird:kiwisaver"]);
  });

  it("rejects an unknown scope", () => {
    expect(parseScopes(["ird:wff", "ird:everything"])).toBeNull();
  });

  it("rejects a non-array", () => {
    expect(parseScopes("ird:wff")).toBeNull();
    expect(parseScopes(undefined)).toBeNull();
  });

  it("rejects more scopes than the per-request limit", () => {
    const many = Array.from({ length: MAX_SCOPES_PER_REQUEST + 1 }, () => "ird:wff");
    expect(parseScopes(many)).toBeNull();
  });
});

describe("parseDid", () => {
  it("accepts a well-formed NZ government DID", () => {
    expect(parseDid("did:gov:nz:abc_DEF-123")).toBe("did:gov:nz:abc_DEF-123");
  });

  it("rejects other DID methods and empty input", () => {
    expect(parseDid("did:web:example.com")).toBeNull();
    expect(parseDid("")).toBeNull();
    expect(parseDid(null)).toBeNull();
  });
});

describe("parseActionRequest", () => {
  it("accepts a kebab-case type with object parameters", () => {
    expect(parseActionRequest("update-kiwisaver-rate", { newRate: 4 })).toEqual({
      type: "update-kiwisaver-rate",
      parameters: { newRate: 4 },
    });
  });

  it("rejects types that are not kebab-case", () => {
    expect(parseActionRequest("Update Rate", {})).toBeNull();
    expect(parseActionRequest("../../etc", {})).toBeNull();
    expect(parseActionRequest("", {})).toBeNull();
  });

  it("rejects non-object parameters", () => {
    expect(parseActionRequest("pay-fine", ["x"])).toBeNull();
    expect(parseActionRequest("pay-fine", "x")).toBeNull();
    expect(parseActionRequest("pay-fine", null)).toBeNull();
  });

  it("rejects oversized parameters", () => {
    const big = { note: "x".repeat(MAX_ACTION_PARAMETER_BYTES) };
    expect(parseActionRequest("file-dispute-claim", big)).toBeNull();
  });

  it("rejects overlong parameter keys", () => {
    expect(parseActionRequest("pay-fine", { ["k".repeat(65)]: 1 })).toBeNull();
  });
});

describe("parseQuestion", () => {
  it("trims and returns a question", () => {
    expect(parseQuestion("  Am I eligible?  ")).toBe("Am I eligible?");
  });

  it("rejects empty and whitespace-only questions", () => {
    expect(parseQuestion("")).toBeNull();
    expect(parseQuestion("   ")).toBeNull();
  });

  it("rejects questions over the length limit and non-strings", () => {
    expect(parseQuestion("q".repeat(MAX_QUESTION_LENGTH + 1))).toBeNull();
    expect(parseQuestion(42)).toBeNull();
  });
});
