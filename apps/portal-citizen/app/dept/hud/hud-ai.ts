import type { HUDDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceHudAiContext(data: HUDDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "hud",
    content: `Housing client number: ${data.clientNumber}.`,
    metadata: { area: "client-number" },
  });

  if (data.applications && data.applications.length > 0) {
    for (const a of data.applications) {
      chunks.push({
        deptId: "hud",
        content: `Application ${a.applicationNumber} (${a.applicationType}): ${a.status}.${a.priorityBand ? ` Priority band ${a.priorityBand}.` : ""}${a.bedroomsNeeded != null ? ` ${a.bedroomsNeeded} bedrooms needed.` : ""} Submitted ${a.submittedDate}.`,
        metadata: { area: "application", applicationNumber: a.applicationNumber },
      });
    }
  }

  if (data.tenancies && data.tenancies.length > 0) {
    for (const t of data.tenancies) {
      chunks.push({
        deptId: "hud",
        content: `Tenancy ${t.tenancyId} at ${t.propertyAddress}: ${t.status}, $${t.weeklyRent}/week${t.incomeRelatedRent ? " (income-related rent)" : ""}. Started ${t.startDate}.`,
        metadata: { area: "tenancy", tenancyId: t.tenancyId },
      });
    }
  }

  if (data.maintenanceRequests && data.maintenanceRequests.length > 0) {
    for (const m of data.maintenanceRequests) {
      chunks.push({
        deptId: "hud",
        content: `Maintenance request ${m.requestNumber} (${m.category}): ${m.status}. ${m.description}.`,
        metadata: { area: "maintenance", requestNumber: m.requestNumber },
      });
    }
  }

  return chunks;
}
