import { fetchSfoDataForCitizen } from "../actions";

export const metadata = { title: "Investigations — Serious Fraud Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function SfoInvestigationsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchSfoDataForCitizen(did, ["sfo:investigations"]);
  const rows = data?.investigations ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/sfo?did=${encodeURIComponent(did)}`}>← Back to SFO case file</Link>
      <h1>Investigations</h1>
      {rows.length === 0 ? (
        <p>No investigations on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>matter</th>
                <th>status</th>
                <th>openedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.matter}</td>
                  <td>{row.status}</td>
                  <td>{row.openedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
