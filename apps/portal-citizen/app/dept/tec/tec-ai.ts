import type { TecDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceTecAiContext(data: TecDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "tec",
    content: "Tertiary Education Commission local ID: " + data.tecId + ".",
    metadata: { area: "tec-id" },
  });

  if (data.funding && data.funding.length > 0) {
    for (const x of data.funding) {
      chunks.push({
        deptId: "tec",
        content: "Funding: " + x.provider + ", " + x.amount + ", " + x.year,
        metadata: { area: "funding" },
      });
    }
  }

  if (data.courses && data.courses.length > 0) {
    for (const x of data.courses) {
      chunks.push({
        deptId: "tec",
        content: "Courses: " + x.courseName + ", " + x.provider + ", " + x.status,
        metadata: { area: "courses" },
      });
    }
  }

  return chunks;
}
