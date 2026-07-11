import Link from "next/link";
import { fetchStatsnzData } from "../actions";

export const metadata = { title: "Stats NZ Profile — My Gov NZ" };

export default async function StatsnzProfilePage() {
  const data = await fetchStatsnzData(["statsnz:profile"]);
  const profile = data?.profile;

  return (
    <main>
      <Link href="/dept/statsnz">← Back to Stats NZ</Link>
      <h1>Stats NZ Profile</h1>

      {!profile ? (
        <p>No data profile on file.</p>
      ) : (
        <section>
          <h2>Your data profile</h2>
          <p>{profile.dataSummary}</p>
          <ul>
            <li>Records: {profile.recordCount}</li>
            <li>Last updated: {profile.lastUpdated}</li>
          </ul>
        </section>
      )}
    </main>
  );
}
