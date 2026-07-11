import Link from "next/link";
import { fetchDpmcData } from "../actions";

export const metadata = { title: "Honours — Department of the Prime Minister and Cabinet — My Gov NZ" };

export default async function DpmcHonoursPage() {
  const data = await fetchDpmcData(["dpmc:honours"]);
  if (!data) {
    return (
      <main>
        <h1>Honours</h1>
        <p>Unable to load your DPMC information.</p>
        <Link href={"/consent?dept=dpmc"}>Grant DPMC access</Link>
      </main>
    );
  }

  const rows = data.honours ?? [];

  return (
    <main>
      <Link href={"/dept/dpmc"}>← Back to DPMC</Link>
      <h1>Honours</h1>
      {rows.length === 0 ? (
        <p>No honours on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>awardYear</th>
                <th>award</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.awardYear}</td>
                  <td>{row.award}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
