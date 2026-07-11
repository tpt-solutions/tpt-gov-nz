import Link from "next/link";
import { fetchFenzData } from "../actions";

export const metadata = { title: "Incidents — Fire and Emergency New Zealand — My Gov NZ" };

export default async function FenzIncidentsPage() {
  const data = await fetchFenzData(["fenz:incidents"]);
  if (!data) {
    return (
      <main>
        <h1>Incidents</h1>
        <p>Unable to load your FENZ information.</p>
        <Link href={"/consent?dept=fenz"}>Grant FENZ access</Link>
      </main>
    );
  }

  const rows = data.incidents ?? [];

  return (
    <main>
      <Link href={"/dept/fenz"}>← Back to FENZ</Link>
      <h1>Incidents</h1>
      {rows.length === 0 ? (
        <p>No incidents on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>incidentType</th>
                <th>incidentDate</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.incidentType}</td>
                  <td>{row.incidentDate}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
