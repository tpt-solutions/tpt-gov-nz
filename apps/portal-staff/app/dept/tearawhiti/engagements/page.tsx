import { fetchTearawhitiDataForCitizen } from "../actions";

export const metadata = { title: "Engagements — Te Arawhiti — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TearawhitiEngagementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTearawhitiDataForCitizen(did, ["tearawhiti:engagements"]);
  const rows = data?.engagements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tearawhiti?did=${encodeURIComponent(did)}`}>← Back to Te Arawhiti case file</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>engagementDate</th>
                <th>outcome</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.engagementDate}</td>
                  <td>{row.outcome}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
