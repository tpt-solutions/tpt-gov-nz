import { describe, expect, it } from "vitest";
import { signJwt } from "./jwt";
import { verifyJwtEdge } from "./jwt-edge";

const SECRET = "edge-test-secret-0123456789";

describe("verifyJwtEdge", () => {
  it("accepts a token signed by the server-side signJwt", async () => {
    const token = signJwt({ staffId: "case-1" }, SECRET, 600);
    expect(await verifyJwtEdge(token, SECRET)).toBe(true);
  });

  it("rejects a token signed with a different secret", async () => {
    const token = signJwt({ staffId: "case-1" }, "another-secret-0123456789", 600);
    expect(await verifyJwtEdge(token, SECRET)).toBe(false);
  });

  it("rejects an expired token", async () => {
    const token = signJwt({ staffId: "case-1" }, SECRET, -10);
    expect(await verifyJwtEdge(token, SECRET)).toBe(false);
  });

  it("rejects a token whose payload was altered after signing", async () => {
    const token = signJwt({ staffId: "case-1", demo: false }, SECRET, 600);
    const [h, , s] = token.split(".");
    const forgedPayload = Buffer.from(JSON.stringify({ staffId: "admin", demo: false, exp: 9999999999 })).toString("base64url");
    expect(await verifyJwtEdge(`${h}.${forgedPayload}.${s}`, SECRET)).toBe(false);
  });

  it("rejects malformed input without throwing", async () => {
    expect(await verifyJwtEdge("", SECRET)).toBe(false);
    expect(await verifyJwtEdge("a.b", SECRET)).toBe(false);
    expect(await verifyJwtEdge("a.b.c", SECRET)).toBe(false);
  });
});
