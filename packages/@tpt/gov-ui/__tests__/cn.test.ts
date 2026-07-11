import { describe, expect, it } from "vitest";
import { cn } from "../src/cn";

describe("cn", () => {
  it("joins truthy class names", () => {
    expect(cn("a", "b")).toBe("a b");
  });

  it("ignores false, null, and undefined", () => {
    expect(cn("a", false, null, undefined, "b")).toBe("a b");
  });

  it("returns empty string when only falsy values", () => {
    expect(cn(false, null, undefined)).toBe("");
  });

  it("preserves meaningful ordering", () => {
    expect(cn("govui-btn", "govui-btn--small")).toBe("govui-btn govui-btn--small");
  });
});
