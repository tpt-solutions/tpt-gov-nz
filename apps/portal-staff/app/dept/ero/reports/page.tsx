import Link from "next/link";
import { fetchEroDataForCitizen } from "../actions";

export const metadata = { title: "Reports — Education Review Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EroReportsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchEroDataForCitizen(did, ["ero:reports"]);
  const rows = data?.reports ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ero?did=${encodeURIComponent(did)}`}>← Back to ERO case file</Link>
      <h1>Reports</h1>
      {rows.length === 0 ? (
        <p>No reports on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>published</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.published}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
