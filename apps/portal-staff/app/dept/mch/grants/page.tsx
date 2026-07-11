import { fetchMchDataForCitizen } from "../actions";

export const metadata = { title: "Grants — Ministry for Culture and Heritage — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MchGrantsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMchDataForCitizen(did, ["mch:grants"]);
  const rows = data?.grants ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mch?did=${encodeURIComponent(did)}`}>← Back to MCH case file</Link>
      <h1>Grants</h1>
      {rows.length === 0 ? (
        <p>No grants on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>grantName</th>
                <th>amount</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.grantName}</td>
                  <td>{row.amount}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
