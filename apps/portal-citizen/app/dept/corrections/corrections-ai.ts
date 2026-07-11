import type { CORRECTIONSDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceCorrectionsAiContext(data: CORRECTIONSDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "corrections",
    content: `Corrections ID: ${data.correctionsId}.`,
    metadata: { area: "corrections-id" },
  });

  if (data.probation) {
    const p = data.probation;
    chunks.push({
      deptId: "corrections",
      content: `Probation: ${p.status}. Officer ${p.officerName} based at ${p.location}. Next report date ${p.nextReportDate}.`,
      metadata: { area: "probation" },
    });
  }

  if (data.case && data.case.length > 0) {
    for (const c of data.case) {
      chunks.push({
        deptId: "corrections",
        content: `Case ${c.caseNumber}: ${c.sentenceType} sentence. Started ${c.startDate}${c.endDate ? `, ends ${c.endDate}` : ""}. ${c.summary}`,
        metadata: { area: "case", caseNumber: c.caseNumber },
      });
    }
  }

  return chunks;
}
