import type { LINZDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceLinzAiContext(data: LINZDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "linz",
    content: `LINZ customer ID: ${data.customerId}.`,
    metadata: { area: "customer-id" },
  });

  if (data.titles && data.titles.length > 0) {
    for (const t of data.titles) {
      chunks.push({
        deptId: "linz",
        content: `Title ${t.titleNumber}: ${t.estateType} estate at ${t.propertyAddress}, ${t.landAreaSqm} m².`,
        metadata: { area: "title", titleNumber: t.titleNumber },
      });
    }
  }

  if (data.ownership && data.ownership.length > 0) {
    for (const o of data.ownership) {
      chunks.push({
        deptId: "linz",
        content: `Ownership of ${o.titleNumber}: ${o.ownershipShare} share, registered owners ${o.registeredOwners.join(", ")}.`,
        metadata: { area: "ownership", titleNumber: o.titleNumber },
      });
    }
  }

  return chunks;
}
