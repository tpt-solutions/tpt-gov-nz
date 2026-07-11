import type { MOJDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMojAiContext(data: MOJDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "moj",
    content: `MOJ client number: ${data.clientNumber}.`,
    metadata: { area: "client-number" },
  });

  if (data.fines && data.fines.length > 0) {
    for (const f of data.fines) {
      chunks.push({
        deptId: "moj",
        content: `Fine ${f.fineNumber} (${f.fineType}): ${f.status}, $${f.amount}. ${f.description}. Due ${f.dueDate}.`,
        metadata: { area: "fine", fineNumber: f.fineNumber },
      });
    }
  }

  if (data.disputes && data.disputes.length > 0) {
    for (const d of data.disputes) {
      chunks.push({
        deptId: "moj",
        content: `Disputes Tribunal claim ${d.disputeNumber} (${d.claimType}): ${d.status}. ${d.description}.${d.hearingDate ? ` Hearing ${d.hearingDate}.` : ""}`,
        metadata: { area: "dispute", disputeNumber: d.disputeNumber },
      });
    }
  }

  if (data.courtRecords && data.courtRecords.length > 0) {
    for (const c of data.courtRecords) {
      chunks.push({
        deptId: "moj",
        content: `Court case ${c.caseNumber} (${c.caseType}): ${c.status}. ${c.description}.${c.nextHearingDate ? ` Next hearing ${c.nextHearingDate}.` : ""}`,
        metadata: { area: "court-record", caseNumber: c.caseNumber },
      });
    }
  }

  return chunks;
}
