import { fetchDpmcDataForCitizen } from "../actions";

export const metadata = { title: "Honours — Department of the Prime Minister and Cabinet — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DpmcHonoursStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchDpmcDataForCitizen(did, ["dpmc:honours"]);
  const rows = data?.honours ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/dpmc?did=${encodeURIComponent(did)}`}>← Back to DPMC case file</Link>
      <h1>Honours</h1>
      {rows.length === 0 ? (
        <p>No honours on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>awardYear</th>
                <th>award</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.awardYear}</td>
                  <td>{row.award}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
