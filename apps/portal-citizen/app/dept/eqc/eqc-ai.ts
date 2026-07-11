import type { EqcDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceEqcAiContext(data: EqcDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "eqc",
    content: "Earthquake Commission (Toka Tū Ake) local ID: " + data.eqcId + ".",
    metadata: { area: "eqc-id" },
  });

  if (data.claims && data.claims.length > 0) {
    for (const x of data.claims) {
      chunks.push({
        deptId: "eqc",
        content: "Claims: " + x.reference + ", " + x.property + ", " + x.status + ", " + x.lodgedDate,
        metadata: { area: "claims" },
      });
    }
  }

  if (data.cover) {
    chunks.push({
      deptId: "eqc",
      content: "Cover: " + data.cover.property + ", " + data.cover.sumInsured + ", " + data.cover.validTo,
      metadata: { area: "cover" },
    });
  }

  return chunks;
}
