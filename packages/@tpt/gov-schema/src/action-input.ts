/**
 * Input validation for department server actions (citizen and staff portals).
 *
 * Department actions receive values from the browser. These helpers validate
 * them before they reach a department service or the AI client. They fail
 * closed: an invalid input returns `null` and the caller returns its normal
 * error result.
 */
import { z } from "zod";
import { DidSchema, ScopeSchema } from "./identity.js";

/** Upper bound on the serialised size of an action's parameters. */
export const MAX_ACTION_PARAMETER_BYTES = 8 * 1024;

/** Upper bound on the number of scopes requested in one call. */
export const MAX_SCOPES_PER_REQUEST = 20;

/** Upper bound on a citizen question sent to the AI assistant. */
export const MAX_QUESTION_LENGTH = 1000;

const ScopeListSchema = z.array(ScopeSchema).max(MAX_SCOPES_PER_REQUEST);

// Action types are kebab-case, e.g. "update-kiwisaver-rate".
const ActionTypeSchema = z.string().regex(/^[a-z][a-z0-9-]{0,63}$/);

const ParametersSchema = z
  .record(z.string().min(1).max(64), z.unknown())
  .refine((value) => JSON.stringify(value).length <= MAX_ACTION_PARAMETER_BYTES, {
    message: "parameters too large",
  });

const QuestionSchema = z.string().trim().min(1).max(MAX_QUESTION_LENGTH);

/** Validate a scope list. Returns `null` when any scope is unknown. */
export function parseScopes(input: unknown): string[] | null {
  const result = ScopeListSchema.safeParse(input);
  return result.success ? result.data : null;
}

/** Validate a citizen DID. Returns `null` when it is malformed. */
export function parseDid(input: unknown): string | null {
  const result = DidSchema.safeParse(input);
  return result.success ? result.data : null;
}

/** Validate an action request. Returns `null` when the type or parameters are invalid. */
export function parseActionRequest(
  type: unknown,
  parameters: unknown,
): { type: string; parameters: Record<string, unknown> } | null {
  const typeResult = ActionTypeSchema.safeParse(type);
  const paramsResult = ParametersSchema.safeParse(parameters);
  if (!typeResult.success || !paramsResult.success) return null;
  return { type: typeResult.data, parameters: paramsResult.data };
}

/** Validate and normalise a question. Returns `null` when it is empty or too long. */
export function parseQuestion(input: unknown): string | null {
  const result = QuestionSchema.safeParse(input);
  return result.success ? result.data : null;
}
