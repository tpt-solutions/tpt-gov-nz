import type { TPKDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceTpkAiContext(data: TPKDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "tpk",
    content: `Te Puni Kōkiri (TPK) id: ${data.tpkId}.`,
    metadata: { area: "tpk-id" },
  });

  if (data.programmes && data.programmes.length > 0) {
    for (const p of data.programmes) {
      chunks.push({
        deptId: "tpk",
        content: `Programme ${p.programmeName}: ${p.status} in ${p.region}.`,
        metadata: { area: "programme", programmeName: p.programmeName },
      });
    }
  }

  if (data.funding && data.funding.length > 0) {
    for (const f of data.funding) {
      chunks.push({
        deptId: "tpk",
        content: `Grant ${f.grantId}: $${f.amount} for ${f.purpose}. Status: ${f.status}.`,
        metadata: { area: "funding", grantId: f.grantId },
      });
    }
  }

  return chunks;
}
