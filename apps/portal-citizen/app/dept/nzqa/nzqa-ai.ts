import type { NZQADataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceNzqaAiContext(data: NZQADataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "nzqa",
    content: `NSN: ${data.nsn}.`,
    metadata: { area: "nsn" },
  });

  if (data.qualifications && data.qualifications.length > 0) {
    for (const q of data.qualifications) {
      chunks.push({
        deptId: "nzqa",
        content: `Qualification ${q.qualificationId}: ${q.title} (Level ${q.level}), awarded ${q.awardedDate} by ${q.provider}.`,
        metadata: { area: "qualification", qualificationId: q.qualificationId },
      });
    }
  }

  if (data.transcript) {
    const tr = data.transcript;
    const creditInfo = tr.totalCredits != null ? ` ${tr.totalCredits} total credits.` : "";
    const summary = tr.recordSummary ? ` ${tr.recordSummary}` : "";
    chunks.push({
      deptId: "nzqa",
      content: `Record of Achievement:${summary}${creditInfo}${tr.creditSummary ? ` ${tr.creditSummary}` : ""}`,
      metadata: { area: "transcript" },
    });
  }

  return chunks;
}
