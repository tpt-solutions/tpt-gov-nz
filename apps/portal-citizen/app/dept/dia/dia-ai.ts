import type { DIADataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceDiaAiContext(data: DIADataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "dia",
    content: `Passport number ends in ${data.passportNumber.slice(-4)}.`,
    metadata: { area: "passport-number" },
  });

  if (data.passport) {
    const expiringSoon = new Date(data.passport.expiryDate).getTime() - Date.now() < 1000 * 60 * 60 * 24 * 365;
    chunks.push({
      deptId: "dia",
      content: `Passport expires ${data.passport.expiryDate}.${data.passport.renewable ? " Renewable." : ""}${expiringSoon ? " Expiring within a year." : ""}`,
      metadata: { area: "passport" },
    });
  }

  if (data.citizenship) {
    chunks.push({
      deptId: "dia",
      content: `Citizenship status: ${data.citizenship.status}.`,
      metadata: { area: "citizenship" },
    });
  }

  return chunks;
}
