import { fetchMfeDataForCitizen } from "../actions";

export const metadata = { title: "Emissions — Ministry for the Environment — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfeEmissionsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMfeDataForCitizen(did, ["mfe:emissions"]);
  const rows = data?.emissions ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mfe?did=${encodeURIComponent(did)}`}>← Back to MfE case file</Link>
      <h1>Emissions</h1>
      {rows.length === 0 ? (
        <p>No emissions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reportYear</th>
                <th>sector</th>
                <th>tonnesCO2e</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reportYear}</td>
                  <td>{row.sector}</td>
                  <td>{row.tonnesCO2e}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
