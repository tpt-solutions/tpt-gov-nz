import Link from "next/link";
import { fetchDocDataForCitizen } from "../actions";

export const metadata = { title: "Concessions — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DocStaffConcessionsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Concessions — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchDocDataForCitizen(did, ["doc:concessions"]);
  const concessions = data?.concessions;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/doc?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Concessions — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load concessions.</p>}
      {data && (!concessions || concessions.length === 0) && <p>No conservation concessions on record.</p>}

      {concessions && concessions.length > 0 && (
        <ul>
          {concessions.map((c) => (
            <li key={c.concessionId}>
              <strong>{c.concessionId}</strong> — {c.type} ({c.holder})
              <br />
              <small>{c.startDate} to {c.endDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
