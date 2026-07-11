import type { MPIDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMpiAiContext(data: MPIDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mpi",
    content: `MPI id: ${data.mpiId}.`,
    metadata: { area: "mpi-id" },
  });

  if (data.registrations && data.registrations.length > 0) {
    for (const r of data.registrations) {
      chunks.push({
        deptId: "mpi",
        content: `Registration ${r.nzbn} (${r.type}): ${r.status}. Business ${r.businessName}. Registered ${r.registeredDate}.`,
        metadata: { area: "registration", nzbn: r.nzbn },
      });
    }
  }

  if (data.certifications && data.certifications.length > 0) {
    for (const c of data.certifications) {
      chunks.push({
        deptId: "mpi",
        content: `Certification ${c.certNumber} (${c.category}): issued ${c.issuedDate}, expires ${c.expiresDate}.`,
        metadata: { area: "certification", certNumber: c.certNumber },
      });
    }
  }

  return chunks;
}
