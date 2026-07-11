import { fetchDpmcDataForCitizen } from "../actions";

export const metadata = { title: "Engagements — Department of the Prime Minister and Cabinet — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DpmcEngagementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchDpmcDataForCitizen(did, ["dpmc:engagements"]);
  const rows = data?.engagements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/dpmc?did=${encodeURIComponent(did)}`}>← Back to DPMC case file</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>eventName</th>
                <th>eventDate</th>
                <th>location</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.eventName}</td>
                  <td>{row.eventDate}</td>
                  <td>{row.location}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
