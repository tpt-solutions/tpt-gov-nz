import type { EroDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceEroAiContext(data: EroDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "ero",
    content: "Education Review Office local ID: " + data.eroId + ".",
    metadata: { area: "ero-id" },
  });

  if (data.reviews && data.reviews.length > 0) {
    for (const x of data.reviews) {
      chunks.push({
        deptId: "ero",
        content: "Reviews: " + x.school + ", " + x.rating + ", " + x.reviewDate + ", " + x.nextReview,
        metadata: { area: "reviews" },
      });
    }
  }

  if (data.reports && data.reports.length > 0) {
    for (const x of data.reports) {
      chunks.push({
        deptId: "ero",
        content: "Reports: " + x.title + ", " + x.published,
        metadata: { area: "reports" },
      });
    }
  }

  return chunks;
}
