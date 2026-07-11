import Link from "next/link";
import { fetchDpmcData } from "../actions";

export const metadata = { title: "Engagements — Department of the Prime Minister and Cabinet — My Gov NZ" };

export default async function DpmcEngagementsPage() {
  const data = await fetchDpmcData(["dpmc:engagements"]);
  if (!data) {
    return (
      <main>
        <h1>Engagements</h1>
        <p>Unable to load your DPMC information.</p>
        <Link href={"/consent?dept=dpmc"}>Grant DPMC access</Link>
      </main>
    );
  }

  const rows = data.engagements ?? [];

  return (
    <main>
      <Link href={"/dept/dpmc"}>← Back to DPMC</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>eventName</th>
                <th>eventDate</th>
                <th>location</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.eventName}</td>
                  <td>{row.eventDate}</td>
                  <td>{row.location}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
