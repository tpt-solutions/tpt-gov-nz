import Link from "next/link";
import { fetchTearawhitiData } from "../actions";

export const metadata = { title: "Treaty settlements — Te Arawhiti — My Gov NZ" };

export default async function TearawhitiTreatySettlementsPage() {
  const data = await fetchTearawhitiData(["tearawhiti:treaty-settlements"]);
  if (!data) {
    return (
      <main>
        <h1>Treaty settlements</h1>
        <p>Unable to load your Te Arawhiti information.</p>
        <Link href={"/consent?dept=tearawhiti"}>Grant Te Arawhiti access</Link>
      </main>
    );
  }

  const rows = data.treaty_settlements ?? [];

  return (
    <main>
      <Link href={"/dept/tearawhiti"}>← Back to Te Arawhiti</Link>
      <h1>Treaty settlements</h1>
      {rows.length === 0 ? (
        <p>No treaty settlements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>iwi</th>
                <th>status</th>
                <th>settledDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.iwi}</td>
                  <td>{row.status}</td>
                  <td>{row.settledDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
