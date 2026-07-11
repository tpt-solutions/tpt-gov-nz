import type { CUSTOMSDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceCustomsAiContext(data: CUSTOMSDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "customs",
    content: `Customs traveller id: ${data.travellerId}.`,
    metadata: { area: "traveller-id" },
  });

  if (data.travel) {
    const t = data.travel;
    chunks.push({
      deptId: "customs",
      content: `Travel: passport ${t.passportNumber}, last arrival ${t.lastArrival} at ${t.arrivalPort}${t.frequentTraveller ? " (frequent traveller)" : ""}.`,
      metadata: { area: "travel", passportNumber: t.passportNumber },
    });
  }

  if (data.declarations && data.declarations.length > 0) {
    for (const d of data.declarations) {
      chunks.push({
        deptId: "customs",
        content: `Declaration ${d.declarationId} (from ${d.countryFrom}): ${d.status}. Goods declared: ${d.goodsDeclared}. Dated ${d.date}.`,
        metadata: { area: "declaration", declarationId: d.declarationId },
      });
    }
  }

  return chunks;
}
