import Link from "next/link";
import { fetchGcsbData } from "../actions";

export const metadata = { title: "Engagements — Government Communications Security Bureau — My Gov NZ" };

export default async function GcsbEngagementsPage() {
  const data = await fetchGcsbData(["gcsb:engagements"]);
  if (!data) {
    return (
      <main>
        <h1>Engagements</h1>
        <p>Unable to load your GCSB information.</p>
        <Link href={"/consent?dept=gcsb"}>Grant GCSB access</Link>
      </main>
    );
  }

  const rows = data.engagements ?? [];

  return (
    <main>
      <Link href={"/dept/gcsb"}>← Back to GCSB</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>partner</th>
                <th>engagementType</th>
                <th>engagementDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.partner}</td>
                  <td>{row.engagementType}</td>
                  <td>{row.engagementDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
