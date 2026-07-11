import { fetchWorksafeDataForCitizen } from "../actions";

export const metadata = { title: "Inspections — WorkSafe New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WorksafeInspectionsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchWorksafeDataForCitizen(did, ["worksafe:inspections"]);
  const rows = data?.inspections ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/worksafe?did=${encodeURIComponent(did)}`}>← Back to WorkSafe case file</Link>
      <h1>Inspections</h1>
      {rows.length === 0 ? (
        <p>No inspections on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>site</th>
                <th>inspectionDate</th>
                <th>outcome</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.site}</td>
                  <td>{row.inspectionDate}</td>
                  <td>{row.outcome}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
