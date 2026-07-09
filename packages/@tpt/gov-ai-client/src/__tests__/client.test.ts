import { describe, it, expect, vi, beforeEach } from "vitest";
import { GovAiClient } from "../client.js";

// Mock fetch globally
const mockFetch = vi.fn();
vi.stubGlobal("fetch", mockFetch);

beforeEach(() => {
  mockFetch.mockReset();
});

describe("GovAiClient", () => {
  describe("isEnabled", () => {
    it("returns false when level is none", () => {
      const client = new GovAiClient({ level: "none" });
      expect(client.isEnabled).toBe(false);
    });

    it("returns false when level is advisory but no provider configured", () => {
      const client = new GovAiClient({ level: "advisory" });
      expect(client.isEnabled).toBe(false);
    });

    it("returns true when openrouter is configured with apiKey", () => {
      const client = new GovAiClient({
        level: "advisory",
        provider: "openrouter",
        apiKey: "sk-or-test",
      });
      expect(client.isEnabled).toBe(true);
    });

    it("returns true when ollama is configured", () => {
      const client = new GovAiClient({
        level: "advisory",
        provider: "ollama",
        baseUrl: "http://localhost:11434",
      });
      expect(client.isEnabled).toBe(true);
    });

    it("returns false when openrouter has no apiKey", () => {
      const client = new GovAiClient({
        level: "advisory",
        provider: "openrouter",
      });
      expect(client.isEnabled).toBe(false);
    });
  });

  describe("level", () => {
    it("returns the configured level", () => {
      expect(new GovAiClient({ level: "none" }).level).toBe("none");
      expect(new GovAiClient({ level: "automated" }).level).toBe("automated");
    });
  });

  describe("chat", () => {
    it("throws when AI is disabled", async () => {
      const client = new GovAiClient({ level: "none" });
      await expect(client.chat("system", "user")).rejects.toThrow("AI is disabled");
    });

    it("sends redacted user message to openrouter", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          choices: [{ message: { content: "Hello!" } }],
          model: "test-model",
          usage: { total_tokens: 10 },
        }),
      });

      const client = new GovAiClient({
        level: "advisory",
        provider: "openrouter",
        apiKey: "sk-test",
      });

      const result = await client.chat("You are a helper.", "My NHI is ABC1234");

      expect(result.content).toBe("Hello!");
      expect(result.provider).toBe("openrouter");

      // Verify the fetch was called with redacted content
      const body = JSON.parse(mockFetch.mock.calls[0][1].body);
      const userMsg = body.messages.find((m: { role: string }) => m.role === "user");
      expect(userMsg.content).toContain("[NHI-REDACTED]");
      expect(userMsg.content).not.toContain("ABC1234");
    });

    it("includes context chunks in system prompt", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          choices: [{ message: { content: "Response" } }],
          model: "test",
        }),
      });

      const client = new GovAiClient({
        level: "advisory",
        provider: "openrouter",
        apiKey: "sk-test",
      });

      await client.chat("system", "user", [
        { deptId: "ird", content: "Tax year 2024 data" },
      ]);

      const body = JSON.parse(mockFetch.mock.calls[0][1].body);
      const systemMsg = body.messages.find((m: { role: string }) => m.role === "system");
      expect(systemMsg.content).toContain("[IRD]");
      expect(systemMsg.content).toContain("Tax year 2024 data");
    });
  });

  describe("isAvailable", () => {
    it("returns false when provider is null", async () => {
      const client = new GovAiClient({ level: "none" });
      expect(await client.isAvailable()).toBe(false);
    });

    it("returns true when openrouter responds OK", async () => {
      mockFetch.mockResolvedValueOnce({ ok: true });

      const client = new GovAiClient({
        level: "advisory",
        provider: "openrouter",
        apiKey: "sk-test",
      });

      expect(await client.isAvailable()).toBe(true);
    });

    it("returns false when fetch throws", async () => {
      mockFetch.mockRejectedValueOnce(new Error("network error"));

      const client = new GovAiClient({
        level: "advisory",
        provider: "ollama",
      });

      expect(await client.isAvailable()).toBe(false);
    });
  });
});
