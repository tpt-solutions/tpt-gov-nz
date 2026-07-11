import Link from "next/link";
import { fetchPublicserviceData } from "../actions";

export const metadata = { title: "Workforce — Te Kawa Mataaho Public Service Commission — My Gov NZ" };

export default async function PublicserviceWorkforcePage() {
  const data = await fetchPublicserviceData(["publicservice:workforce"]);
  if (!data) {
    return (
      <main>
        <h1>Workforce</h1>
        <p>Unable to load your Public Service information.</p>
        <Link href={"/consent?dept=publicservice"}>Grant Public Service access</Link>
      </main>
    );
  }

  const rows = data.workforce ?? [];

  return (
    <main>
      <Link href={"/dept/publicservice"}>← Back to Public Service</Link>
      <h1>Workforce</h1>
      {rows.length === 0 ? (
        <p>No workforce on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reportYear</th>
                <th>agency</th>
                <th>headcount</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reportYear}</td>
                  <td>{row.agency}</td>
                  <td>{row.headcount}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
