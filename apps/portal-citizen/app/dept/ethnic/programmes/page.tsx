import Link from "next/link";
import { fetchEthnicData } from "../actions";

export const metadata = { title: "Programmes — Ministry for Ethnic Communities — My Gov NZ" };

export default async function EthnicProgrammesPage() {
  const data = await fetchEthnicData(["ethnic:programmes"]);
  if (!data) {
    return (
      <main>
        <h1>Programmes</h1>
        <p>Unable to load your Ethnic Communities information.</p>
        <Link href={"/consent?dept=ethnic"}>Grant Ethnic Communities access</Link>
      </main>
    );
  }

  const rows = data.programmes ?? [];

  return (
    <main>
      <Link href={"/dept/ethnic"}>← Back to Ethnic Communities</Link>
      <h1>Programmes</h1>
      {rows.length === 0 ? (
        <p>No programmes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>programmeName</th>
                <th>status</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.programmeName}</td>
                  <td>{row.status}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
