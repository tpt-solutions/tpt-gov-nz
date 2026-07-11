import Link from "next/link";
import { fetchMfatData } from "../actions";

export const metadata = { title: "Overseas missions — Ministry of Foreign Affairs and Trade — My Gov NZ" };

export default async function MfatOverseasMissionsPage() {
  const data = await fetchMfatData(["mfat:overseas-missions"]);
  if (!data) {
    return (
      <main>
        <h1>Overseas missions</h1>
        <p>Unable to load your MFAT information.</p>
        <Link href={"/consent?dept=mfat"}>Grant MFAT access</Link>
      </main>
    );
  }

  const rows = data.overseas_missions ?? [];

  return (
    <main>
      <Link href={"/dept/mfat"}>← Back to MFAT</Link>
      <h1>Overseas missions</h1>
      {rows.length === 0 ? (
        <p>No overseas missions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>country</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.country}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
