import type { AiLevel } from "@tpt/gov-schema";

const ORDER: Record<AiLevel, number> = {
  none: 0,
  advisory: 1,
  assisted: 2,
  automated: 3,
};

export function getAiLevel(): AiLevel {
  const lvl = process.env.TPT__GOV__AI_LEVEL;
  if (lvl === "advisory" || lvl === "assisted" || lvl === "automated") return lvl;
  return "none";
}

export function aiLevelAtLeast(level: AiLevel, minimum: AiLevel): boolean {
  return ORDER[level] >= ORDER[minimum];
}
