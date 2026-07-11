export type ClassValue = string | false | null | undefined;

/**
 * Join truthy class names. Accepts plain strings, `false`, `null`, and `undefined`
 * so callers can conditionally apply classes without filtering.
 */
export function cn(...values: ClassValue[]): string {
  return values.filter(Boolean).join(" ");
}
