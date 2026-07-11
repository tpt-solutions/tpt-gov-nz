import Link from "next/link";
import { fetchDocDataForCitizen } from "../actions";

export const metadata = { title: "Permits — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DocStaffPermitsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Permits — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchDocDataForCitizen(did, ["doc:permits"]);
  const permits = data?.permits;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/doc?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Permits — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load permits.</p>}
      {data && (!permits || permits.length === 0) && <p>No conservation permits on record.</p>}

      {permits && permits.length > 0 && (
        <ul>
          {permits.map((p) => (
            <li key={p.permitNumber}>
              <strong>{p.permitNumber}</strong> — {p.activity} ({p.status})
              <br />
              <small>{p.location} · expires {p.expiresDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
