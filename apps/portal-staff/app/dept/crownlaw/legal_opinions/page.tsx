import { fetchCrownlawDataForCitizen } from "../actions";

export const metadata = { title: "Legal opinions — Crown Law Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CrownlawLegalOpinionsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchCrownlawDataForCitizen(did, ["crownlaw:legal-opinions"]);
  const rows = data?.legal_opinions ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/crownlaw?did=${encodeURIComponent(did)}`}>← Back to Crown Law case file</Link>
      <h1>Legal opinions</h1>
      {rows.length === 0 ? (
        <p>No legal opinions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>topic</th>
                <th>issuedDate</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.topic}</td>
                  <td>{row.issuedDate}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
