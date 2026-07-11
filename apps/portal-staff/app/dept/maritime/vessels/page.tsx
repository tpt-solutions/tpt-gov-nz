import { fetchMaritimeDataForCitizen } from "../actions";

export const metadata = { title: "Vessels — Maritime New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MaritimeVesselsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMaritimeDataForCitizen(did, ["maritime:vessels"]);
  const rows = data?.vessels ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/maritime?did=${encodeURIComponent(did)}`}>← Back to Maritime case file</Link>
      <h1>Vessels</h1>
      {rows.length === 0 ? (
        <p>No vessels on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>vesselName</th>
                <th>flag</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.vesselName}</td>
                  <td>{row.flag}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
