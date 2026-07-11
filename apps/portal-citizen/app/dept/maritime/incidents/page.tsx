import Link from "next/link";
import { fetchMaritimeData } from "../actions";

export const metadata = { title: "Incidents — Maritime New Zealand — My Gov NZ" };

export default async function MaritimeIncidentsPage() {
  const data = await fetchMaritimeData(["maritime:incidents"]);
  if (!data) {
    return (
      <main>
        <h1>Incidents</h1>
        <p>Unable to load your Maritime information.</p>
        <Link href={"/consent?dept=maritime"}>Grant Maritime access</Link>
      </main>
    );
  }

  const rows = data.incidents ?? [];

  return (
    <main>
      <Link href={"/dept/maritime"}>← Back to Maritime</Link>
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
