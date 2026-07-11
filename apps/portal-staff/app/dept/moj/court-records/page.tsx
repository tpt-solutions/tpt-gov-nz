import Link from "next/link";
import { fetchMojDataForCitizen } from "../actions";

export const metadata = { title: "Court Records — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MojStaffCourtRecordsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Court Records — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMojDataForCitizen(did, ["moj:court-records"]);
  const records = data?.courtRecords;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moj?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Court Records — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load court records.</p>}
      {data && (!records || records.length === 0) && <p>No court records on file.</p>}

      {records && records.length > 0 && (
        <ul>
          {records.map((c) => (
            <li key={c.caseNumber}>
              <strong>{c.caseNumber}</strong> — {c.caseType} ({c.status}): {c.description}
              {c.nextHearingDate ? ` — next hearing ${c.nextHearingDate}` : ""}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
