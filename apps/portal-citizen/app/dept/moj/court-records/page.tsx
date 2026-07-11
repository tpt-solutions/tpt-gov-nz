import Link from "next/link";
import { fetchMojData } from "../actions";

export const metadata = { title: "Court Records — My Gov NZ" };

export default async function MojCourtRecordsPage() {
  const data = await fetchMojData(["moj:court-records"]);
  const records = data?.courtRecords ?? [];

  return (
    <main>
      <Link href="/dept/moj">← Back to Ministry of Justice</Link>
      <h1>Court Records</h1>

      {records.length === 0 ? (
        <p>No court records on file.</p>
      ) : (
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
