import { fetchDefenceDataForCitizen } from "../actions";

export const metadata = { title: "Procurements — Ministry of Defence — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DefenceProcurementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchDefenceDataForCitizen(did, ["defence:procurements"]);
  const rows = data?.procurements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/defence?did=${encodeURIComponent(did)}`}>← Back to Defence case file</Link>
      <h1>Procurements</h1>
      {rows.length === 0 ? (
        <p>No procurements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>programme</th>
                <th>value</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.programme}</td>
                  <td>{row.value}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
