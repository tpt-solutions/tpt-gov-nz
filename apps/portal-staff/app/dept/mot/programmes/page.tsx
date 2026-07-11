import { fetchMotDataForCitizen } from "../actions";

export const metadata = { title: "Programmes — Ministry of Transport — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MotProgrammesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMotDataForCitizen(did, ["mot:programmes"]);
  const rows = data?.programmes ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mot?did=${encodeURIComponent(did)}`}>← Back to Transport case file</Link>
      <h1>Programmes</h1>
      {rows.length === 0 ? (
        <p>No programmes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>budget</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.budget}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
