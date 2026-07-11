import { fetchGcsbDataForCitizen } from "../actions";

export const metadata = { title: "Mandates — Government Communications Security Bureau — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function GcsbMandatesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchGcsbDataForCitizen(did, ["gcsb:mandates"]);
  const rows = data?.mandates ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/gcsb?did=${encodeURIComponent(did)}`}>← Back to GCSB case file</Link>
      <h1>Mandates</h1>
      {rows.length === 0 ? (
        <p>No mandates on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>agency</th>
                <th>status</th>
                <th>issuedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.agency}</td>
                  <td>{row.status}</td>
                  <td>{row.issuedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
