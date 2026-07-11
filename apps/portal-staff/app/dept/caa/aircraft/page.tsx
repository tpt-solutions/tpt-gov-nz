import { fetchCaaDataForCitizen } from "../actions";

export const metadata = { title: "Aircraft — Civil Aviation Authority — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CaaAircraftStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchCaaDataForCitizen(did, ["caa:aircraft"]);
  const rows = data?.aircraft ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/caa?did=${encodeURIComponent(did)}`}>← Back to CAA case file</Link>
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
