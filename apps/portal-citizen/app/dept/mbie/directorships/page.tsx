import Link from "next/link";
import { fetchMbieData } from "../actions";

export const metadata = { title: "Directorships — My Gov NZ" };

export default async function MbieDirectorshipsPage() {
  const data = await fetchMbieData(["mbie:directorships"]);
  const directorships = data?.directorships ?? [];

  return (
    <main>
      <Link href="/dept/mbie">← Back to business</Link>
      <h1>Directorships</h1>

      {directorships.length === 0 ? (
        <p>No directorships on record.</p>
      ) : (
        <ul>
          {directorships.map((d) => (
            <li key={`${d.nzbn}-${d.appointedDate}`}>
              <strong>{d.entityName}</strong> — {d.role}
              <br />
              <small>NZBN {d.nzbn} · appointed {d.appointedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
