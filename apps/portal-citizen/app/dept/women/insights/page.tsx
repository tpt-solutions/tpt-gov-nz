import Link from "next/link";
import { fetchWomenData } from "../actions";

export const metadata = { title: "Insights — Ministry for Women — My Gov NZ" };

export default async function WomenInsightsPage() {
  const data = await fetchWomenData(["women:insights"]);
  if (!data) {
    return (
      <main>
        <h1>Insights</h1>
        <p>Unable to load your Women information.</p>
        <Link href={"/consent?dept=women"}>Grant Women access</Link>
      </main>
    );
  }

  const rows = data.insights ?? [];

  return (
    <main>
      <Link href={"/dept/women"}>← Back to Women</Link>
      <h1>Insights</h1>
      {rows.length === 0 ? (
        <p>No insights on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>summary</th>
                <th>published</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.summary}</td>
                  <td>{row.published}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
