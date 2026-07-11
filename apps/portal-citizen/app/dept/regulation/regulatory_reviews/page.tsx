import Link from "next/link";
import { fetchRegulationData } from "../actions";

export const metadata = { title: "Regulatory reviews — Ministry for Regulation — My Gov NZ" };

export default async function RegulationRegulatoryReviewsPage() {
  const data = await fetchRegulationData(["regulation:regulatory-reviews"]);
  if (!data) {
    return (
      <main>
        <h1>Regulatory reviews</h1>
        <p>Unable to load your Regulation information.</p>
        <Link href={"/consent?dept=regulation"}>Grant Regulation access</Link>
      </main>
    );
  }

  const rows = data.regulatory_reviews ?? [];

  return (
    <main>
      <Link href={"/dept/regulation"}>← Back to Regulation</Link>
      <h1>Regulatory reviews</h1>
      {rows.length === 0 ? (
        <p>No regulatory reviews on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>agency</th>
                <th>status</th>
                <th>reviewYear</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.agency}</td>
                  <td>{row.status}</td>
                  <td>{row.reviewYear}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
