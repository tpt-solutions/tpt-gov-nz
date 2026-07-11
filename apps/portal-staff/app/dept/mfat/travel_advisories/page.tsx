import { fetchMfatDataForCitizen } from "../actions";

export const metadata = { title: "Travel advisories — Ministry of Foreign Affairs and Trade — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MfatTravelAdvisoriesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMfatDataForCitizen(did, ["mfat:travel-advisories"]);
  const rows = data?.travel_advisories ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/mfat?did=${encodeURIComponent(did)}`}>← Back to MFAT case file</Link>
      <h1>Travel advisories</h1>
      {rows.length === 0 ? (
        <p>No travel advisories on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>country</th>
                <th>level</th>
                <th>updated</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.country}</td>
                  <td>{row.level}</td>
                  <td>{row.updated}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
