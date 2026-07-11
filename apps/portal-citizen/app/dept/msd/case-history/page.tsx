import Link from "next/link";
import { fetchMsdData } from "../actions";

export const metadata = { title: "Case History — My Gov NZ" };

export default async function MsdCaseHistoryPage() {
  const data = await fetchMsdData(["msd:case-history"]);
  const events = data?.caseHistory ?? [];

  return (
    <main>
      <Link href="/dept/msd">← Back to MSD</Link>
      <h1>Case History</h1>

      {events.length === 0 ? (
        <p>No case history on record.</p>
      ) : (
        <ul>
          {events.map((e, i) => (
            <li key={`${e.eventDate}-${i}`}>
              <strong>{e.eventDate}</strong> — {e.serviceLine}: {e.summary}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
