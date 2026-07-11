import { fetchDefenceDataForCitizen } from "../actions";

export const metadata = { title: "Bases — Ministry of Defence — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DefenceBasesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchDefenceDataForCitizen(did, ["defence:bases"]);
  const rows = data?.bases ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/defence?did=${encodeURIComponent(did)}`}>← Back to Defence case file</Link>
      <h1>Bases</h1>
      {rows.length === 0 ? (
        <p>No bases on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>location</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.location}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
