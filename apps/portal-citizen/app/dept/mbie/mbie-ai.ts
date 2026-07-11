import type { MBIEDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMbieAiContext(data: MBIEDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mbie",
    content: `Person id: ${data.personId}.`,
    metadata: { area: "person-id" },
  });

  if (data.businessRegistrations && data.businessRegistrations.length > 0) {
    for (const b of data.businessRegistrations) {
      chunks.push({
        deptId: "mbie",
        content: `Business ${b.nzbn} (${b.entityName}): ${b.entityType}, ${b.status}. Registered ${b.registeredDate}.`,
        metadata: { area: "business", nzbn: b.nzbn },
      });
    }
  }

  if (data.directorships && data.directorships.length > 0) {
    for (const d of data.directorships) {
      chunks.push({
        deptId: "mbie",
        content: `Directorship at ${d.entityName} (${d.nzbn}): ${d.role}. Appointed ${d.appointedDate}.`,
        metadata: { area: "directorship", nzbn: d.nzbn },
      });
    }
  }

  return chunks;
}
