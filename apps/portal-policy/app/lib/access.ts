/**
 * Access control and rate limiting for the policy lab.
 *
 * The lab is an internal analysis tool. It is protected with HTTP Basic auth against
 * `TPT__GOV__POLICY_PASSWORD`. In production the portal refuses every request when no
 * password is configured, so it cannot be opened by accident.
 */

/** Compare two byte arrays without returning early on the first mismatch. */
export function constantTimeEqual(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  let diff = 0;
  // Lengths are equal here, so the ?? 0 fallbacks never change the result.
  for (let i = 0; i < a.length; i++) diff |= (a[i] ?? 0) ^ (b[i] ?? 0);
  return diff === 0;
}

/** Parse an `Authorization: Basic …` header into the password, or null. */
export function basicAuthPassword(header: string | null): string | null {
  if (!header || !header.startsWith("Basic ")) return null;
  try {
    const decoded = atob(header.slice("Basic ".length).trim());
    const sep = decoded.indexOf(":");
    if (sep < 0) return null;
    return decoded.slice(sep + 1);
  } catch {
    return null;
  }
}

/** A fixed-window counter. Process-local: a shared store is needed for several instances. */
export class RateLimiter {
  private readonly windows = new Map<string, { count: number; start: number }>();

  constructor(
    private readonly limit: number,
    private readonly windowMs: number,
  ) {}

  /** Returns true when the call is allowed and counts it; false when the limit is reached. */
  take(key: string, now: number = Date.now()): boolean {
    const entry = this.windows.get(key);
    if (!entry || now - entry.start >= this.windowMs) {
      this.windows.set(key, { count: 1, start: now });
      return true;
    }
    if (entry.count >= this.limit) return false;
    entry.count += 1;
    return true;
  }
}

/** Simulation runs are expensive (AI calls). Allow 20 per 10 minutes across the lab. */
export const simulationLimiter = new RateLimiter(20, 10 * 60 * 1000);

/**
 * Compare a supplied password with the configured one. Both are hashed first, so the
 * comparison takes the same time whatever the lengths are. Runs on the Edge runtime.
 */
export async function passwordMatches(given: string, expected: string): Promise<boolean> {
  const enc = new TextEncoder();
  const [a, b] = await Promise.all([
    crypto.subtle.digest("SHA-256", enc.encode(given)),
    crypto.subtle.digest("SHA-256", enc.encode(expected)),
  ]);
  return constantTimeEqual(new Uint8Array(a), new Uint8Array(b));
}
