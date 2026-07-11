import { fetchCaaDataForCitizen } from "../actions";

export const metadata = { title: "Licences — Civil Aviation Authority — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CaaLicencesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchCaaDataForCitizen(did, ["caa:licences"]);
  const rows = data?.licences ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/caa?did=${encodeURIComponent(did)}`}>← Back to CAA case file</Link>
      <h1>Licences</h1>
      {rows.length === 0 ? (
        <p>No licences on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>licenceNo</th>
                <th>category</th>
                <th>status</th>
                <th>expires</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.licenceNo}</td>
                  <td>{row.category}</td>
                  <td>{row.status}</td>
                  <td>{row.expires}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
