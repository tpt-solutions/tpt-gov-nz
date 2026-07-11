import Link from "next/link";
import { fetchStatsnzData } from "./actions";
import StatsnzAiPrompt from "./ai-prompt";

export const metadata = { title: "Stats NZ — My Gov NZ" };

export default async function StatsnzOverviewPage() {
  const data = await fetchStatsnzData(["statsnz:census", "statsnz:profile"]);

  if (!data) {
    return (
      <main>
        <h1>Statistics New Zealand</h1>
        <p>Unable to load your statistics information. Please grant access to continue.</p>
        <Link href="/consent?dept=statsnz">Grant statistics access</Link>
      </main>
    );
  }

  const censusCount = data.census?.length ?? 0;

  return (
    <main>
      <h1>Statistics New Zealand</h1>
      <p>Stats ID: ••••{data.statsId.slice(-4)}</p>

      <section>
        <h2>Census</h2>
        <p>{censusCount} census record(s) on file.</p>
        <Link href="/dept/statsnz/census">View census →</Link>
      </section>

      <section>
        <h2>Profile</h2>
        <p>{data.profile ? "Data profile available." : "No data profile on file."}</p>
        <Link href="/dept/statsnz/profile">View profile →</Link>
      </section>

      <StatsnzAiPrompt />
    </main>
  );
}
