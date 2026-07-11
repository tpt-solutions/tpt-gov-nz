import Link from "next/link";
import { fetchNzdfData } from "../actions";

export const metadata = { title: "Service records — New Zealand Defence Force — My Gov NZ" };

export default async function NzdfServiceRecordsPage() {
  const data = await fetchNzdfData(["nzdf:service-records"]);
  if (!data) {
    return (
      <main>
        <h1>Service records</h1>
        <p>Unable to load your NZDF information.</p>
        <Link href={"/consent?dept=nzdf"}>Grant NZDF access</Link>
      </main>
    );
  }

  const rows = data.service_records ?? [];

  return (
    <main>
      <Link href={"/dept/nzdf"}>← Back to NZDF</Link>
      <h1>Service records</h1>
      {rows.length === 0 ? (
        <p>No service records on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>serviceNo</th>
                <th>branch</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.serviceNo}</td>
                  <td>{row.branch}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
