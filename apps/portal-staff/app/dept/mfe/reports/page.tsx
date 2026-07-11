import Link from "next/link";
import { fetchMfeDataForCitizen } from "../actions";

export const metadata = { title: "Reports — Ministry for the Environment — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfeReportsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMfeDataForCitizen(did, ["mfe:reports"]);
  const rows = data?.reports ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mfe?did=${encodeURIComponent(did)}`}>← Back to MfE case file</Link>
      <h1>Reports</h1>
      {rows.length === 0 ? (
        <p>No reports on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>published</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.published}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
