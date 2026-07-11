import { fetchMfatDataForCitizen } from "../actions";

export const metadata = { title: "Overseas missions — Ministry of Foreign Affairs and Trade — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfatOverseasMissionsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMfatDataForCitizen(did, ["mfat:overseas-missions"]);
  const rows = data?.overseas_missions ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mfat?did=${encodeURIComponent(did)}`}>← Back to MFAT case file</Link>
      <h1>Overseas missions</h1>
      {rows.length === 0 ? (
        <p>No overseas missions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>country</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.country}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
