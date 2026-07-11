import Link from "next/link";
import { fetchMchDataForCitizen } from "../actions";

export const metadata = { title: "Heritage sites — Ministry for Culture and Heritage — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MchHeritageSitesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMchDataForCitizen(did, ["mch:heritage-sites"]);
  const rows = data?.heritage_sites ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mch?did=${encodeURIComponent(did)}`}>← Back to MCH case file</Link>
      <h1>Heritage sites</h1>
      {rows.length === 0 ? (
        <p>No heritage sites on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>status</th>
                <th>region</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.status}</td>
                  <td>{row.region}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
