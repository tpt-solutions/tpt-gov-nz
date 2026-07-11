import { fetchTearawhitiDataForCitizen } from "../actions";

export const metadata = { title: "Treaty settlements — Te Arawhiti — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TearawhitiTreatySettlementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTearawhitiDataForCitizen(did, ["tearawhiti:treaty-settlements"]);
  const rows = data?.treaty_settlements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tearawhiti?did=${encodeURIComponent(did)}`}>← Back to Te Arawhiti case file</Link>
      <h1>Treaty settlements</h1>
      {rows.length === 0 ? (
        <p>No treaty settlements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>iwi</th>
                <th>status</th>
                <th>settledDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.iwi}</td>
                  <td>{row.status}</td>
                  <td>{row.settledDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
