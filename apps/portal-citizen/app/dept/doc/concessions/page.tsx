import Link from "next/link";
import { fetchDocData } from "../actions";

export const metadata = { title: "Conservation Concessions — My Gov NZ" };

export default async function DocConcessionsPage() {
  const data = await fetchDocData(["doc:concessions"]);
  const concessions = data?.concessions ?? [];

  return (
    <main>
      <Link href="/dept/doc">← Back to conservation</Link>
      <h1>Conservation Concessions</h1>

      {concessions.length === 0 ? (
        <p>No conservation concessions on record.</p>
      ) : (
        <ul>
          {concessions.map((c) => (
            <li key={c.concessionId}>
              <strong>{c.concessionId}</strong> — {c.type} ({c.holder})
              <br />
              <small>
                {c.startDate} to {c.endDate}
              </small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
