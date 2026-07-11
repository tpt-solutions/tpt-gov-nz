import { fetchMotDataForCitizen } from "../actions";

export const metadata = { title: "Strategies — Ministry of Transport — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MotStrategiesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMotDataForCitizen(did, ["mot:strategies"]);
  const rows = data?.strategies ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mot?did=${encodeURIComponent(did)}`}>← Back to Transport case file</Link>
      <h1>Strategies</h1>
      {rows.length === 0 ? (
        <p>No strategies on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>year</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.year}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
