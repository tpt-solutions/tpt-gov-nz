import Link from "next/link";
import { fetchEthnicData } from "../actions";

export const metadata = { title: "Community grants — Ministry for Ethnic Communities — My Gov NZ" };

export default async function EthnicCommunityGrantsPage() {
  const data = await fetchEthnicData(["ethnic:community-grants"]);
  if (!data) {
    return (
      <main>
        <h1>Community grants</h1>
        <p>Unable to load your Ethnic Communities information.</p>
        <Link href={"/consent?dept=ethnic"}>Grant Ethnic Communities access</Link>
      </main>
    );
  }

  const rows = data.community_grants ?? [];

  return (
    <main>
      <Link href={"/dept/ethnic"}>← Back to Ethnic Communities</Link>
      <h1>Community grants</h1>
      {rows.length === 0 ? (
        <p>No community grants on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>grantName</th>
                <th>amount</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.grantName}</td>
                  <td>{row.amount}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
