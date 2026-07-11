import Link from "next/link";
import { fetchPublicserviceDataForCitizen } from "../actions";

export const metadata = { title: "Agency ratings — Te Kawa Mataaho Public Service Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PublicserviceAgencyRatingsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchPublicserviceDataForCitizen(did, ["publicservice:agency-ratings"]);
  const rows = data?.agency_ratings ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/publicservice?did=${encodeURIComponent(did)}`}>← Back to Public Service case file</Link>
      <h1>Agency ratings</h1>
      {rows.length === 0 ? (
        <p>No agency ratings on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>agency</th>
                <th>rating</th>
                <th>ratingYear</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.agency}</td>
                  <td>{row.rating}</td>
                  <td>{row.ratingYear}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
