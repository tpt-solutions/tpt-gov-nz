import { describe, expect, it } from "vitest";
import { basicAuthPassword, constantTimeEqual, RateLimiter } from "./access";

const enc = (s: string) => new TextEncoder().encode(s);

describe("constantTimeEqual", () => {
  it("is true only for identical bytes of the same length", () => {
    expect(constantTimeEqual(enc("secret"), enc("secret"))).toBe(true);
    expect(constantTimeEqual(enc("secret"), enc("secreT"))).toBe(false);
    expect(constantTimeEqual(enc("secret"), enc("secret1"))).toBe(false);
  });
});

describe("basicAuthPassword", () => {
  const header = (user: string, pass: string) => `Basic ${Buffer.from(`${user}:${pass}`).toString("base64")}`;

  it("extracts the password from a Basic header", () => {
    expect(basicAuthPassword(header("policy", "hunter2"))).toBe("hunter2");
  });

  it("keeps colons that appear in the password", () => {
    expect(basicAuthPassword(header("policy", "a:b:c"))).toBe("a:b:c");
  });

  it("returns null for missing, non-Basic or malformed headers", () => {
    expect(basicAuthPassword(null)).toBeNull();
    expect(basicAuthPassword("Bearer abc")).toBeNull();
    expect(basicAuthPassword("Basic !!!not-base64")).toBeNull();
    expect(basicAuthPassword(`Basic ${Buffer.from("nocolon").toString("base64")}`)).toBeNull();
  });
});

describe("RateLimiter", () => {
  it("allows up to the limit, then refuses until the window passes", () => {
    const limiter = new RateLimiter(2, 1000);
    expect(limiter.take("k", 0)).toBe(true);
    expect(limiter.take("k", 1)).toBe(true);
    expect(limiter.take("k", 2)).toBe(false);
    expect(limiter.take("k", 999)).toBe(false);
    expect(limiter.take("k", 1000)).toBe(true);
  });

  it("counts keys independently", () => {
    const limiter = new RateLimiter(1, 1000);
    expect(limiter.take("a", 0)).toBe(true);
    expect(limiter.take("a", 0)).toBe(false);
    expect(limiter.take("b", 0)).toBe(true);
  });
});
