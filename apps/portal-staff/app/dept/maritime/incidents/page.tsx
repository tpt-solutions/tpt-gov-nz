import Link from "next/link";
import { fetchMaritimeDataForCitizen } from "../actions";

export const metadata = { title: "Incidents — Maritime New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MaritimeIncidentsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMaritimeDataForCitizen(did, ["maritime:incidents"]);
  const rows = data?.incidents ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/maritime?did=${encodeURIComponent(did)}`}>← Back to Maritime case file</Link>
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
