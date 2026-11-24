import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  getSession: vi.fn(),
  fetchDeptData: vi.fn(),
  buildAiContext: vi.fn(),
  isEnabled: { value: true },
  chat: vi.fn(),
}));

vi.mock("@/app/lib/session", () => ({ getSession: mocks.getSession }));
vi.mock("@/app/lib/data-access", () => ({ fetchDeptData: mocks.fetchDeptData }));
vi.mock("@/app/lib/ai-context", () => ({ buildAiContext: mocks.buildAiContext }));
vi.mock("@/app/ai/client", () => ({
  buildAiClient: () => ({
    get isEnabled() {
      return mocks.isEnabled.value;
    },
    chat: mocks.chat,
  }),
}));

import { POST } from "./route";

function request(body: unknown): Request {
  return new Request("http://localhost/api/ai", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: typeof body === "string" ? body : JSON.stringify(body),
  });
}

const session = (grantedScopes: string[]) => ({
  did: "did:gov:nz:test",
  sessionId: "s1",
  grantedScopes,
  issuedAt: 0,
  expiresAt: 0,
});

describe("POST /api/ai", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.isEnabled.value = true;
    mocks.getSession.mockResolvedValue(session(["ird:wff"]));
    mocks.fetchDeptData.mockResolvedValue({ workingForFamilies: { eligible: true } });
    mocks.buildAiContext.mockReturnValue([{ source: "ird", content: "eligible" }]);
    mocks.chat.mockResolvedValue({ content: "You may be eligible." });
    vi.spyOn(console, "error").mockImplementation(() => undefined);
  });

  it("rejects unauthenticated callers before reading anything", async () => {
    mocks.getSession.mockResolvedValue(null);
    const res = await POST(request({ question: "Am I eligible?" }));
    expect(res.status).toBe(401);
    expect(mocks.fetchDeptData).not.toHaveBeenCalled();
    expect(mocks.chat).not.toHaveBeenCalled();
  });

  it("rejects malformed JSON", async () => {
    const res = await POST(request("{not json"));
    expect(res.status).toBe(400);
  });

  it("rejects an empty or whitespace-only question", async () => {
    expect((await POST(request({ question: "   " }))).status).toBe(400);
    expect((await POST(request({ question: 42 }))).status).toBe(400);
    expect(mocks.chat).not.toHaveBeenCalled();
  });

  it("rejects a question over the length limit", async () => {
    const res = await POST(request({ question: "q".repeat(2001) }));
    expect(res.status).toBe(400);
    expect(mocks.chat).not.toHaveBeenCalled();
  });

  it("reads only the scopes this session was granted", async () => {
    mocks.getSession.mockResolvedValue(session(["ird:wff"]));
    await POST(request({ question: "Am I eligible?" }));
    expect(mocks.fetchDeptData).toHaveBeenCalledTimes(1);
    expect(mocks.fetchDeptData).toHaveBeenCalledWith("ird", ["ird:wff"]);
  });

  it("reads nothing when the session has no granted scopes", async () => {
    mocks.getSession.mockResolvedValue(session([]));
    const res = await POST(request({ question: "Anything?" }));
    expect(res.status).toBe(200);
    expect(mocks.fetchDeptData).not.toHaveBeenCalled();
  });

  it("returns a disabled answer without calling the model when AI is off", async () => {
    mocks.isEnabled.value = false;
    const res = await POST(request({ question: "Am I eligible?" }));
    const body = await res.json();
    expect(body.enabled).toBe(false);
    expect(mocks.chat).not.toHaveBeenCalled();
  });

  it("redacts identifiers from the context before it reaches the model", async () => {
    mocks.buildAiContext.mockReturnValue([
      { source: "ird", content: "IRD number 123-456-789 is on file" },
    ]);
    await POST(request({ question: "What is my IRD number?" }));
    const [, , context] = mocks.chat.mock.calls[0] as [string, string, { content: string }[]];
    expect(context[0].content).not.toContain("123-456-789");
    expect(context[0].content).toContain("[IRD-REDACTED]");
  });

  it("returns the answer on success", async () => {
    const res = await POST(request({ question: "Am I eligible?" }));
    expect(res.status).toBe(200);
    expect(await res.json()).toEqual({ answer: "You may be eligible.", enabled: true });
  });

  it("does not leak upstream error text to the browser", async () => {
    mocks.chat.mockRejectedValue(new Error("OpenRouter error 500: secret upstream detail"));
    const res = await POST(request({ question: "Am I eligible?" }));
    const text = await res.text();
    expect(res.status).toBe(502);
    expect(text).not.toContain("secret upstream detail");
    expect(text).not.toContain("OpenRouter");
  });
});
