import Link from "next/link";
import { fetchDefenceData } from "../actions";

export const metadata = { title: "Procurements — Ministry of Defence — My Gov NZ" };

export default async function DefenceProcurementsPage() {
  const data = await fetchDefenceData(["defence:procurements"]);
  if (!data) {
    return (
      <main>
        <h1>Procurements</h1>
        <p>Unable to load your Defence information.</p>
        <Link href={"/consent?dept=defence"}>Grant Defence access</Link>
      </main>
    );
  }

  const rows = data.procurements ?? [];

  return (
    <main>
      <Link href={"/dept/defence"}>← Back to Defence</Link>
      <h1>Procurements</h1>
      {rows.length === 0 ? (
        <p>No procurements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>programme</th>
                <th>value</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.programme}</td>
                  <td>{row.value}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
