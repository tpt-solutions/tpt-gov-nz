import { describe, expect, it } from "vitest";
import {
  LOGIN_WINDOW_MS,
  MAX_FAILED_ATTEMPTS,
  clearFailures,
  isLocked,
  recordFailure,
} from "./login-limiter";

describe("staff login limiter", () => {
  it("locks after the maximum number of failures in one window", () => {
    const key = "limit-test-a";
    const t0 = 1_000_000;
    for (let i = 0; i < MAX_FAILED_ATTEMPTS - 1; i++) recordFailure(key, t0);
    expect(isLocked(key, t0)).toBe(false);
    recordFailure(key, t0);
    expect(isLocked(key, t0)).toBe(true);
  });

  it("unlocks when the window expires", () => {
    const key = "limit-test-b";
    const t0 = 2_000_000;
    for (let i = 0; i < MAX_FAILED_ATTEMPTS; i++) recordFailure(key, t0);
    expect(isLocked(key, t0 + LOGIN_WINDOW_MS - 1)).toBe(true);
    expect(isLocked(key, t0 + LOGIN_WINDOW_MS)).toBe(false);
  });

  it("clears failures after a successful sign-in", () => {
    const key = "limit-test-c";
    const t0 = 3_000_000;
    for (let i = 0; i < MAX_FAILED_ATTEMPTS; i++) recordFailure(key, t0);
    clearFailures(key);
    expect(isLocked(key, t0)).toBe(false);
  });

  it("tracks accounts independently", () => {
    const t0 = 4_000_000;
    for (let i = 0; i < MAX_FAILED_ATTEMPTS; i++) recordFailure("limit-test-d", t0);
    expect(isLocked("limit-test-d", t0)).toBe(true);
    expect(isLocked("limit-test-e", t0)).toBe(false);
  });
});
