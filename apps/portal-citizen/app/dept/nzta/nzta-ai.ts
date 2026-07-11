import type { NZTADataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceNztaAiContext(data: NZTADataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "nzta",
    content: `Driver licence number ends in ${data.driverLicenceNumber.slice(-4)}.`,
    metadata: { area: "licence-number" },
  });

  if (data.driverLicence) {
    const expiringSoon =
      new Date(data.driverLicence.expiryDate).getTime() - Date.now() <
      1000 * 60 * 60 * 24 * 365;
    chunks.push({
      deptId: "nzta",
      content: `Driver licence (class ${data.driverLicence.licenceClass}) expires ${data.driverLicence.expiryDate}.${expiringSoon ? " Expiring within a year." : ""}`,
      metadata: { area: "driver-licence" },
    });
  }

  if (data.vehicles && data.vehicles.length > 0) {
    const regos = data.vehicles
      .map((v) => `${v.registration} (${v.make} ${v.model}, ${v.year})`)
      .join("; ");
    chunks.push({
      deptId: "nzta",
      content: `Registered vehicles: ${regos}.`,
      metadata: { area: "vehicles" },
    });
  }

  if (data.ruc && data.ruc.length > 0) {
    const ruc = data.ruc
      .map((r) => `${r.vehicleRego}: ${r.licenceType}, ${r.unitsRemaining} units left, expires ${r.expiryDate}`)
      .join("; ");
    chunks.push({
      deptId: "nzta",
      content: `Road user charges: ${ruc}.`,
      metadata: { area: "ruc" },
    });
  }

  return chunks;
}
