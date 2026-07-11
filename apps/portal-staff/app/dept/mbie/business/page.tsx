import Link from "next/link";
import { fetchMbieDataForCitizen } from "../actions";

export const metadata = { title: "Business Registrations — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MbieStaffBusinessPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Business Registrations — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMbieDataForCitizen(did, ["mbie:business"]);
  const businesses = data?.businessRegistrations;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mbie?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Business Registrations — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load business registrations.</p>}
      {data && (!businesses || businesses.length === 0) && <p>No business registrations on record.</p>}

      {businesses && businesses.length > 0 && (
        <ul>
          {businesses.map((b) => (
            <li key={b.nzbn}>
              <strong>{b.entityName}</strong> — {b.entityType} ({b.status})
              <br />
              <small>NZBN {b.nzbn} · registered {b.registeredDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
