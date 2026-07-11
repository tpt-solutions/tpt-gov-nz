import Link from "next/link";
import { fetchCaaData } from "../actions";

export const metadata = { title: "Aircraft — Civil Aviation Authority — My Gov NZ" };

export default async function CaaAircraftPage() {
  const data = await fetchCaaData(["caa:aircraft"]);
  if (!data) {
    return (
      <main>
        <h1>Aircraft</h1>
        <p>Unable to load your CAA information.</p>
        <Link href={"/consent?dept=caa"}>Grant CAA access</Link>
      </main>
    );
  }

  const rows = data.aircraft ?? [];

  return (
    <main>
      <Link href={"/dept/caa"}>← Back to CAA</Link>
      <h1>Aircraft</h1>
      {rows.length === 0 ? (
        <p>No aircraft on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>registration</th>
                <th>aircraftType</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.registration}</td>
                  <td>{row.aircraftType}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
