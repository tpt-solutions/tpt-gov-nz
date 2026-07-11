import { fetchWomenDataForCitizen } from "../actions";

export const metadata = { title: "Programmes — Ministry for Women — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WomenProgrammesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchWomenDataForCitizen(did, ["women:programmes"]);
  const rows = data?.programmes ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/women?did=${encodeURIComponent(did)}`}>← Back to Women case file</Link>
      <h1>Programmes</h1>
      {rows.length === 0 ? (
        <p>No programmes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>programmeName</th>
                <th>status</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.programmeName}</td>
                  <td>{row.status}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
