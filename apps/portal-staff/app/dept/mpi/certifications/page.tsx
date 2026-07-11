import Link from "next/link";
import { fetchMpiDataForCitizen } from "../actions";

export const metadata = { title: "Certifications — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MpiStaffCertificationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Certifications — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMpiDataForCitizen(did, ["mpi:certifications"]);
  const certifications = data?.certifications;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mpi?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Certifications — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load certifications.</p>}
      {data && (!certifications || certifications.length === 0) && <p>No certifications on record.</p>}

      {certifications && certifications.length > 0 && (
        <ul>
          {certifications.map((c) => (
            <li key={c.certNumber}>
              <strong>{c.certNumber}</strong> — {c.category}
              <br />
              <small>Issued {c.issuedDate} · expires {c.expiresDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
