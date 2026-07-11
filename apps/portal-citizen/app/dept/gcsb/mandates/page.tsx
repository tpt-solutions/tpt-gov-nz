import Link from "next/link";
import { fetchGcsbData } from "../actions";

export const metadata = { title: "Mandates — Government Communications Security Bureau — My Gov NZ" };

export default async function GcsbMandatesPage() {
  const data = await fetchGcsbData(["gcsb:mandates"]);
  if (!data) {
    return (
      <main>
        <h1>Mandates</h1>
        <p>Unable to load your GCSB information.</p>
        <Link href={"/consent?dept=gcsb"}>Grant GCSB access</Link>
      </main>
    );
  }

  const rows = data.mandates ?? [];

  return (
    <main>
      <Link href={"/dept/gcsb"}>← Back to GCSB</Link>
      <h1>Mandates</h1>
      {rows.length === 0 ? (
        <p>No mandates on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>agency</th>
                <th>status</th>
                <th>issuedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.agency}</td>
                  <td>{row.status}</td>
                  <td>{row.issuedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
