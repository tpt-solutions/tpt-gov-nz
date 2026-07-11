import Link from "next/link";
import { fetchNzdfDataForCitizen } from "../actions";

export const metadata = { title: "Service records — New Zealand Defence Force — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzdfServiceRecordsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchNzdfDataForCitizen(did, ["nzdf:service-records"]);
  const rows = data?.service_records ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzdf?did=${encodeURIComponent(did)}`}>← Back to NZDF case file</Link>
      <h1>Service records</h1>
      {rows.length === 0 ? (
        <p>No service records on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>serviceNo</th>
                <th>branch</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.serviceNo}</td>
                  <td>{row.branch}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
