import Link from "next/link";
import { fetchWomenDataForCitizen } from "../actions";

export const metadata = { title: "Insights — Ministry for Women — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WomenInsightsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchWomenDataForCitizen(did, ["women:insights"]);
  const rows = data?.insights ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/women?did=${encodeURIComponent(did)}`}>← Back to Women case file</Link>
      <h1>Insights</h1>
      {rows.length === 0 ? (
        <p>No insights on file.</p>
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
