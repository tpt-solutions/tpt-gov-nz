import Link from "next/link";
import { fetchDefenceData } from "../actions";

export const metadata = { title: "Bases — Ministry of Defence — My Gov NZ" };

export default async function DefenceBasesPage() {
  const data = await fetchDefenceData(["defence:bases"]);
  if (!data) {
    return (
      <main>
        <h1>Bases</h1>
        <p>Unable to load your Defence information.</p>
        <Link href={"/consent?dept=defence"}>Grant Defence access</Link>
      </main>
    );
  }

  const rows = data.bases ?? [];

  return (
    <main>
      <Link href={"/dept/defence"}>← Back to Defence</Link>
      <h1>Bases</h1>
      {rows.length === 0 ? (
        <p>No bases on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>location</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.location}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
