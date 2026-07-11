import Link from "next/link";
import { fetchMpiDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Primary Industries — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MpiStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Primary Industries — Case File</h1>
        <p>No citizen selected. Enter a DID to view their MPI records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMpiDataForCitizen(did, ["mpi:registrations", "mpi:certifications"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Primary Industries — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load MPI information for this citizen.</p>}

      {data && (
        <>
          <p>MPI id: ••••{data.mpiId.slice(-4)}</p>

          <section>
            <h2>Registrations</h2>
            {data.registrations && data.registrations.length > 0 ? (
              <ul>
                {data.registrations.map((r) => (
                  <li key={r.nzbn}>
                    <strong>{r.businessName}</strong> — {r.type} ({r.status})
                  </li>
                ))}
              </ul>
            ) : (
              <p>No registrations on record.</p>
            )}
          </section>

          <section>
            <h2>Certifications</h2>
            {data.certifications && data.certifications.length > 0 ? (
              <ul>
                {data.certifications.map((c) => (
                  <li key={c.certNumber}>
                    <strong>{c.certNumber}</strong> — {c.category}: issued {c.issuedDate}, expires {c.expiresDate}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No certifications on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/mpi/registrations?did=${encodeURIComponent(did)}`}>Registrations</Link>
            {" · "}
            <Link href={`/dept/mpi/certifications?did=${encodeURIComponent(did)}`}>Certifications</Link>
          </nav>
        </>
      )}
    </main>
  );
}
