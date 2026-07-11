import type { PoliceDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function producePoliceAiContext(data: PoliceDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "police",
    content: `Police client number: ${data.clientNumber}.`,
    metadata: { area: "client-number" },
  });

  if (data.infringements && data.infringements.length > 0) {
    for (const i of data.infringements) {
      chunks.push({
        deptId: "police",
        content: `Infringement ${i.ticketNumber} (${i.offenseType}): ${i.status}, $${i.amount}. ${i.description}. Issued ${i.issueDate}.${i.demeritPoints != null ? ` ${i.demeritPoints} demerit points.` : ""}`,
        metadata: { area: "infringement", ticketNumber: i.ticketNumber },
      });
    }
  }

  if (data.reports && data.reports.length > 0) {
    for (const r of data.reports) {
      chunks.push({
        deptId: "police",
        content: `Report ${r.reportNumber} (${r.reportType}): ${r.status}. ${r.description}. Filed ${r.filedDate}.`,
        metadata: { area: "report", reportNumber: r.reportNumber },
      });
    }
  }

  return chunks;
}
