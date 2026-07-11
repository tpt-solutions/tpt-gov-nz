import { fetchRegulationDataForCitizen } from "../actions";

export const metadata = { title: "Regulatory reviews — Ministry for Regulation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RegulationRegulatoryReviewsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchRegulationDataForCitizen(did, ["regulation:regulatory-reviews"]);
  const rows = data?.regulatory_reviews ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/regulation?did=${encodeURIComponent(did)}`}>← Back to Regulation case file</Link>
      <h1>Regulatory reviews</h1>
      {rows.length === 0 ? (
        <p>No regulatory reviews on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>agency</th>
                <th>status</th>
                <th>reviewYear</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.agency}</td>
                  <td>{row.status}</td>
                  <td>{row.reviewYear}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
