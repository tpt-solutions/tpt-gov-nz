import type { CrownlawDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceCrownlawAiContext(data: CrownlawDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "crownlaw",
    content: "Crown Law Office local ID: " + data.crownlawId + ".",
    metadata: { area: "crownlaw-id" },
  });

  if (data.legal_opinions && data.legal_opinions.length > 0) {
    for (const x of data.legal_opinions) {
      chunks.push({
        deptId: "crownlaw",
        content: "Legal opinions: " + x.reference + ", " + x.topic + ", " + x.issuedDate + ", " + x.status,
        metadata: { area: "legal_opinions" },
      });
    }
  }

  if (data.litigation && data.litigation.length > 0) {
    for (const x of data.litigation) {
      chunks.push({
        deptId: "crownlaw",
        content: "Litigation: " + x.caseName + ", " + x.crownRole + ", " + x.status,
        metadata: { area: "litigation" },
      });
    }
  }

  return chunks;
}
