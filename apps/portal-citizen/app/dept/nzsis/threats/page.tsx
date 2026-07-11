import Link from "next/link";
import { fetchNzsisData } from "../actions";

export const metadata = { title: "Threats — New Zealand Security Intelligence Service — My Gov NZ" };

export default async function NzsisThreatsPage() {
  const data = await fetchNzsisData(["nzsis:threats"]);
  if (!data) {
    return (
      <main>
        <h1>Threats</h1>
        <p>Unable to load your NZSIS information.</p>
        <Link href={"/consent?dept=nzsis"}>Grant NZSIS access</Link>
      </main>
    );
  }

  const rows = data.threats ?? [];

  return (
    <main>
      <Link href={"/dept/nzsis"}>← Back to NZSIS</Link>
      <h1>Threats</h1>
      {rows.length === 0 ? (
        <p>No threats on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>category</th>
                <th>status</th>
                <th>assessedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.category}</td>
                  <td>{row.status}</td>
                  <td>{row.assessedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
