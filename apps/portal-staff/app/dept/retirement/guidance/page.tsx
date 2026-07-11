import Link from "next/link";
import { fetchRetirementDataForCitizen } from "../actions";

export const metadata = { title: "Guidance — Retirement Commission (Te Ara Ahunga Ora) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RetirementGuidanceStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchRetirementDataForCitizen(did, ["retirement:guidance"]);
  const rows = data?.guidance ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/retirement?did=${encodeURIComponent(did)}`}>← Back to Retirement case file</Link>
      <h1>Guidance</h1>
      {rows.length === 0 ? (
        <p>No guidance on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>summary</th>
                <th>published</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.summary}</td>
                  <td>{row.published}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
