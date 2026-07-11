import Link from "next/link";
import { fetchWorksafeDataForCitizen } from "../actions";

export const metadata = { title: "Investigations — WorkSafe New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WorksafeInvestigationsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchWorksafeDataForCitizen(did, ["worksafe:investigations"]);
  const rows = data?.investigations ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/worksafe?did=${encodeURIComponent(did)}`}>← Back to WorkSafe case file</Link>
      <h1>Investigations</h1>
      {rows.length === 0 ? (
        <p>No investigations on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>matter</th>
                <th>status</th>
                <th>openedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.matter}</td>
                  <td>{row.status}</td>
                  <td>{row.openedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
