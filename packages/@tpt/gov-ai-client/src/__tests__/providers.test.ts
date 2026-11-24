import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { OpenRouterProvider } from "../providers/openrouter.js";
import { OllamaProvider } from "../providers/ollama.js";
import type { ChatMessage } from "../types.js";

const messages: ChatMessage[] = [{ role: "user", content: "Hello" }];

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

describe("OpenRouterProvider", () => {
  const fetchMock = vi.fn();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal("fetch", fetchMock);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("posts to chat/completions with bearer auth and the default model", async () => {
    fetchMock.mockResolvedValue(
      jsonResponse({
        choices: [{ message: { content: "Hi there" } }],
        model: "anthropic/claude-sonnet-4-6",
        usage: { total_tokens: 12 },
      }),
    );
    const provider = new OpenRouterProvider("sk-test");

    const result = await provider.chat(messages);

    const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe("https://openrouter.ai/api/v1/chat/completions");
    expect(init.method).toBe("POST");
    expect((init.headers as Record<string, string>).Authorization).toBe("Bearer sk-test");
    expect(JSON.parse(init.body as string)).toEqual({
      model: "anthropic/claude-sonnet-4-6",
      messages,
    });
    expect(result).toEqual({
      content: "Hi there",
      model: "anthropic/claude-sonnet-4-6",
      provider: "openrouter",
      tokensUsed: 12,
    });
  });

  it("uses an explicit model over the default", async () => {
    fetchMock.mockResolvedValue(
      jsonResponse({ choices: [{ message: { content: "" } }], model: "x/y" }),
    );
    await new OpenRouterProvider("k").chat(messages, "x/y");
    const body = JSON.parse((fetchMock.mock.calls[0] as [string, RequestInit])[1].body as string);
    expect(body.model).toBe("x/y");
  });

  it("returns empty content when no choices are returned", async () => {
    fetchMock.mockResolvedValue(jsonResponse({ choices: [], model: "m" }));
    const result = await new OpenRouterProvider("k").chat(messages);
    expect(result.content).toBe("");
    expect(result.tokensUsed).toBeUndefined();
  });

  it("throws with the status code on an upstream error", async () => {
    fetchMock.mockResolvedValue(new Response("rate limited", { status: 429 }));
    await expect(new OpenRouterProvider("k").chat(messages)).rejects.toThrow(
      "OpenRouter error 429",
    );
  });

  it("reports availability from the models endpoint", async () => {
    fetchMock.mockResolvedValueOnce(new Response("{}", { status: 200 }));
    expect(await new OpenRouterProvider("k").isAvailable()).toBe(true);

    fetchMock.mockResolvedValueOnce(new Response("{}", { status: 401 }));
    expect(await new OpenRouterProvider("k").isAvailable()).toBe(false);
  });

  it("reports unavailable when the network request fails", async () => {
    fetchMock.mockRejectedValue(new TypeError("fetch failed"));
    expect(await new OpenRouterProvider("k").isAvailable()).toBe(false);
  });
});

describe("OllamaProvider", () => {
  const fetchMock = vi.fn();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal("fetch", fetchMock);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("posts to api/chat with streaming disabled and the default model", async () => {
    fetchMock.mockResolvedValue(
      jsonResponse({ message: { content: "Local hi" }, model: "llama3.2", eval_count: 7 }),
    );
    const result = await new OllamaProvider("http://ollama.internal:11434").chat(messages);

    const [url, init] = fetchMock.mock.calls[0] as [string, RequestInit];
    expect(url).toBe("http://ollama.internal:11434/api/chat");
    expect(JSON.parse(init.body as string)).toEqual({
      model: "llama3.2",
      messages,
      stream: false,
    });
    expect(result).toEqual({
      content: "Local hi",
      model: "llama3.2",
      provider: "ollama",
      tokensUsed: 7,
    });
  });

  it("defaults to localhost when no base URL is given", async () => {
    fetchMock.mockResolvedValue(jsonResponse({ message: { content: "" }, model: "m" }));
    await new OllamaProvider().chat(messages);
    expect((fetchMock.mock.calls[0] as [string])[0]).toBe("http://localhost:11434/api/chat");
  });

  it("throws with the status code on an upstream error", async () => {
    fetchMock.mockResolvedValue(new Response("model not found", { status: 404 }));
    await expect(new OllamaProvider().chat(messages)).rejects.toThrow("Ollama error 404");
  });

  it("reports availability from the tags endpoint", async () => {
    fetchMock.mockResolvedValueOnce(new Response("{}", { status: 200 }));
    expect(await new OllamaProvider().isAvailable()).toBe(true);
    expect((fetchMock.mock.calls[0] as [string])[0]).toBe("http://localhost:11434/api/tags");

    fetchMock.mockRejectedValueOnce(new TypeError("connect ECONNREFUSED"));
    expect(await new OllamaProvider().isAvailable()).toBe(false);
  });
});
