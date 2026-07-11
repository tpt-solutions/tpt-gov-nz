import { fetchCrownlawDataForCitizen } from "../actions";

export const metadata = { title: "Litigation — Crown Law Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CrownlawLitigationStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchCrownlawDataForCitizen(did, ["crownlaw:litigation"]);
  const rows = data?.litigation ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/crownlaw?did=${encodeURIComponent(did)}`}>← Back to Crown Law case file</Link>
      <h1>Litigation</h1>
      {rows.length === 0 ? (
        <p>No litigation on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>caseName</th>
                <th>crownRole</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.caseName}</td>
                  <td>{row.crownRole}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
