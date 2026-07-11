import Link from "next/link";
import { fetchMfeData } from "../actions";

export const metadata = { title: "Reports — Ministry for the Environment — My Gov NZ" };

export default async function MfeReportsPage() {
  const data = await fetchMfeData(["mfe:reports"]);
  if (!data) {
    return (
      <main>
        <h1>Reports</h1>
        <p>Unable to load your MfE information.</p>
        <Link href={"/consent?dept=mfe"}>Grant MfE access</Link>
      </main>
    );
  }

  const rows = data.reports ?? [];

  return (
    <main>
      <Link href={"/dept/mfe"}>← Back to MfE</Link>
      <h1>Reports</h1>
      {rows.length === 0 ? (
        <p>No reports on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>published</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.published}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
