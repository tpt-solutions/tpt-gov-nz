import Link from "next/link";
import { fetchDiaDataForCitizen } from "../actions";

export const metadata = { title: "Documents — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DiaStaffDocumentsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Documents — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchDiaDataForCitizen(did, ["dia:documents", "dia:citizenship"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/dia?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Documents — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load documents.</p>}

      {data && (
        <>
          <section>
            <h2>Birth certificate</h2>
            {data.birthCertificate ? (
              <p>
                {data.birthCertificate.certificateNumber} — born{" "}
                {data.birthCertificate.dateOfBirth} in {data.birthCertificate.placeOfBirth}
                {data.birthCertificate.parents ? ` (parents: ${data.birthCertificate.parents})` : ""}
              </p>
            ) : (
              <p>No birth certificate on record.</p>
            )}
          </section>

          <section>
            <h2>Citizenship</h2>
            {data.citizenship ? (
              <p>Status: <strong>{data.citizenship.status}</strong></p>
            ) : (
              <p>No citizenship record.</p>
            )}
          </section>
        </>
      )}
    </main>
  );
}
