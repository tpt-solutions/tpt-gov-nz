import { fetchOrangaDataForCitizen } from "../actions";

export const metadata = { title: "Support services — Oranga Tamariki — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function OrangaSupportServicesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchOrangaDataForCitizen(did, ["oranga:support-services"]);
  const rows = data?.support_services ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/oranga?did=${encodeURIComponent(did)}`}>← Back to Oranga Tamariki case file</Link>
      <h1>Support services</h1>
      {rows.length === 0 ? (
        <p>No support services on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>service</th>
                <th>status</th>
                <th>nextReview</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.service}</td>
                  <td>{row.status}</td>
                  <td>{row.nextReview}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
