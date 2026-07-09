import { describe, it, expect } from "vitest";
import {
  AiLevelSchema,
  AiProviderSchema,
  AiConfigSchema,
  AiContextChunkSchema,
  AiActionSchema,
} from "../ai.js";

describe("AiLevelSchema", () => {
  it("accepts all valid levels", () => {
    expect(AiLevelSchema.parse("none")).toBe("none");
    expect(AiLevelSchema.parse("advisory")).toBe("advisory");
    expect(AiLevelSchema.parse("assisted")).toBe("assisted");
    expect(AiLevelSchema.parse("automated")).toBe("automated");
  });

  it("rejects invalid levels", () => {
    expect(() => AiLevelSchema.parse("full")).toThrow();
    expect(() => AiLevelSchema.parse("")).toThrow();
  });
});

describe("AiProviderSchema", () => {
  it("accepts valid providers", () => {
    expect(AiProviderSchema.parse("openrouter")).toBe("openrouter");
    expect(AiProviderSchema.parse("ollama")).toBe("ollama");
  });

  it("rejects invalid providers", () => {
    expect(() => AiProviderSchema.parse("openai")).toThrow();
  });
});

describe("AiConfigSchema", () => {
  it("accepts minimal config (level only)", () => {
    const result = AiConfigSchema.parse({ level: "none" });
    expect(result.level).toBe("none");
    expect(result.provider).toBeUndefined();
  });

  it("accepts full config", () => {
    const result = AiConfigSchema.parse({
      level: "advisory",
      provider: "openrouter",
      model: "anthropic/claude-sonnet-4-6",
      baseUrl: "https://openrouter.ai/api/v1",
      apiKey: "sk-or-...",
    });
    expect(result.provider).toBe("openrouter");
    expect(result.model).toBe("anthropic/claude-sonnet-4-6");
  });

  it("rejects config with invalid baseUrl", () => {
    expect(() =>
      AiConfigSchema.parse({
        level: "advisory",
        baseUrl: "not-a-url",
      })
    ).toThrow();
  });
});

describe("AiContextChunkSchema", () => {
  it("accepts valid chunk", () => {
    const result = AiContextChunkSchema.parse({
      deptId: "ird",
      content: "Tax year 2024 summary",
    });
    expect(result.deptId).toBe("ird");
  });

  it("accepts chunk with metadata", () => {
    const result = AiContextChunkSchema.parse({
      deptId: "ird",
      content: "data",
      metadata: { year: 2024, source: "db" },
    });
    expect(result.metadata?.year).toBe(2024);
  });
});

describe("AiActionSchema", () => {
  const validAction = {
    id: "00000000-0000-0000-0000-000000000001",
    type: "update-kiwisaver-rate",
    deptId: "ird",
    citizenDid: "did:gov:nz:citizen001",
    description: "Update KiwiSaver contribution rate",
    parameters: { newRate: 6 },
    aiLevel: "assisted",
    status: "pending_approval",
    timestamp: Date.now(),
  };

  it("accepts valid action", () => {
    const result = AiActionSchema.parse(validAction);
    expect(result.status).toBe("pending_approval");
  });

  it("accepts action with humanApprovedBy", () => {
    const result = AiActionSchema.parse({
      ...validAction,
      status: "approved",
      humanApprovedBy: "case-worker-001",
    });
    expect(result.humanApprovedBy).toBe("case-worker-001");
  });

  it("rejects action with invalid status", () => {
    expect(() =>
      AiActionSchema.parse({ ...validAction, status: "unknown" })
    ).toThrow();
  });

  it("rejects action with invalid UUID", () => {
    expect(() =>
      AiActionSchema.parse({ ...validAction, id: "bad" })
    ).toThrow();
  });
});
