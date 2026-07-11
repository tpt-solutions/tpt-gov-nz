import { fetchEthnicDataForCitizen } from "../actions";

export const metadata = { title: "Community grants — Ministry for Ethnic Communities — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EthnicCommunityGrantsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchEthnicDataForCitizen(did, ["ethnic:community-grants"]);
  const rows = data?.community_grants ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ethnic?did=${encodeURIComponent(did)}`}>← Back to Ethnic Communities case file</Link>
      <h1>Community grants</h1>
      {rows.length === 0 ? (
        <p>No community grants on file.</p>
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
