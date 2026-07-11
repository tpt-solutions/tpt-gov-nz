import Link from "next/link";
import { fetchPublicserviceData } from "../actions";

export const metadata = { title: "Agency ratings — Te Kawa Mataaho Public Service Commission — My Gov NZ" };

export default async function PublicserviceAgencyRatingsPage() {
  const data = await fetchPublicserviceData(["publicservice:agency-ratings"]);
  if (!data) {
    return (
      <main>
        <h1>Agency ratings</h1>
        <p>Unable to load your Public Service information.</p>
        <Link href={"/consent?dept=publicservice"}>Grant Public Service access</Link>
      </main>
    );
  }

  const rows = data.agency_ratings ?? [];

  return (
    <main>
      <Link href={"/dept/publicservice"}>← Back to Public Service</Link>
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
