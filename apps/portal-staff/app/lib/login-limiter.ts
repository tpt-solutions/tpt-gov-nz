/**
 * Per-account failed-login limiter for the staff passphrase sign-in.
 *
 * This keeps counts in process memory, so it limits guessing against a single
 * server instance. A multi-instance deployment needs a shared store (for example
 * Postgres or Redis) behind the same three functions.
 */

export const LOGIN_WINDOW_MS = 15 * 60 * 1000;
export const MAX_FAILED_ATTEMPTS = 5;

interface Attempts {
  count: number;
  windowStart: number;
}

const attempts = new Map<string, Attempts>();

/** Returns true when the account has used up its failures in the current window. */
export function isLocked(key: string, now: number = Date.now()): boolean {
  const entry = attempts.get(key);
  if (!entry) return false;
  if (now - entry.windowStart >= LOGIN_WINDOW_MS) {
    attempts.delete(key);
    return false;
  }
  return entry.count >= MAX_FAILED_ATTEMPTS;
}

/** Records a failed attempt. Starts a new window when the previous one has expired. */
export function recordFailure(key: string, now: number = Date.now()): void {
  const entry = attempts.get(key);
  if (!entry || now - entry.windowStart >= LOGIN_WINDOW_MS) {
    attempts.set(key, { count: 1, windowStart: now });
    return;
  }
  entry.count += 1;
}

/** Clears the failure count after a successful sign-in. */
export function clearFailures(key: string): void {
  attempts.delete(key);
}
