import Link from "next/link";
import { fetchMpiDataForCitizen } from "../actions";

export const metadata = { title: "Registrations — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MpiStaffRegistrationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Registrations — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMpiDataForCitizen(did, ["mpi:registrations"]);
  const registrations = data?.registrations;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mpi?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Registrations — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load registrations.</p>}
      {data && (!registrations || registrations.length === 0) && <p>No registrations on record.</p>}

      {registrations && registrations.length > 0 && (
        <ul>
          {registrations.map((r) => (
            <li key={r.nzbn}>
              <strong>{r.businessName}</strong> — {r.type} ({r.status})
              <br />
              <small>NZBN {r.nzbn} · registered {r.registeredDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
