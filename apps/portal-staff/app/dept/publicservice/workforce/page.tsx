import { fetchPublicserviceDataForCitizen } from "../actions";

export const metadata = { title: "Workforce — Te Kawa Mataaho Public Service Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PublicserviceWorkforceStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchPublicserviceDataForCitizen(did, ["publicservice:workforce"]);
  const rows = data?.workforce ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/publicservice?did=${encodeURIComponent(did)}`}>← Back to Public Service case file</Link>
      <h1>Workforce</h1>
      {rows.length === 0 ? (
        <p>No workforce on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reportYear</th>
                <th>agency</th>
                <th>headcount</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reportYear}</td>
                  <td>{row.agency}</td>
                  <td>{row.headcount}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
