import type { DOCDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceDocAiContext(data: DOCDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "doc",
    content: `Conservation doc id: ${data.docId}.`,
    metadata: { area: "doc-id" },
  });

  if (data.permits && data.permits.length > 0) {
    for (const p of data.permits) {
      chunks.push({
        deptId: "doc",
        content: `Permit ${p.permitNumber}: ${p.activity} at ${p.location}, ${p.status}. Expires ${p.expiresDate}.`,
        metadata: { area: "permit", permitNumber: p.permitNumber },
      });
    }
  }

  if (data.concessions && data.concessions.length > 0) {
    for (const c of data.concessions) {
      chunks.push({
        deptId: "doc",
        content: `Concession ${c.concessionId} (${c.type}) held by ${c.holder}: ${c.startDate} to ${c.endDate}.`,
        metadata: { area: "concession", concessionId: c.concessionId },
      });
    }
  }

  return chunks;
}
