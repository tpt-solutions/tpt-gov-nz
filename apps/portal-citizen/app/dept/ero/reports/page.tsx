import Link from "next/link";
import { fetchEroData } from "../actions";

export const metadata = { title: "Reports — Education Review Office — My Gov NZ" };

export default async function EroReportsPage() {
  const data = await fetchEroData(["ero:reports"]);
  if (!data) {
    return (
      <main>
        <h1>Reports</h1>
        <p>Unable to load your ERO information.</p>
        <Link href={"/consent?dept=ero"}>Grant ERO access</Link>
      </main>
    );
  }

  const rows = data.reports ?? [];

  return (
    <main>
      <Link href={"/dept/ero"}>← Back to ERO</Link>
      <h1>Reports</h1>
      {rows.length === 0 ? (
        <p>No reports on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>published</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.published}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
