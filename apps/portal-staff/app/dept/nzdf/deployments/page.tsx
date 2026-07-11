import { fetchNzdfDataForCitizen } from "../actions";

export const metadata = { title: "Deployments — New Zealand Defence Force — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzdfDeploymentsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchNzdfDataForCitizen(did, ["nzdf:deployments"]);
  const rows = data?.deployments ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzdf?did=${encodeURIComponent(did)}`}>← Back to NZDF case file</Link>
      <h1>Deployments</h1>
      {rows.length === 0 ? (
        <p>No deployments on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>operation</th>
                <th>country</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.operation}</td>
                  <td>{row.country}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
