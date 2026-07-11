import type { MoeDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMoeAiContext(data: MoeDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "moe",
    content: "Ministry of Education local ID: " + data.moeId + ".",
    metadata: { area: "moe-id" },
  });

  if (data.enrolment) {
    chunks.push({
      deptId: "moe",
      content: "Enrolment: " + data.enrolment.school + ", " + data.enrolment.yearLevel + ", " + data.enrolment.status,
      metadata: { area: "enrolment" },
    });
  }

  if (data.student_support && data.student_support.length > 0) {
    for (const x of data.student_support) {
      chunks.push({
        deptId: "moe",
        content: "Student support: " + x.service + ", " + x.status + ", " + x.nextReview,
        metadata: { area: "student_support" },
      });
    }
  }

  return chunks;
}
