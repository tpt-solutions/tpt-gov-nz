import Link from "next/link";
import { fetchMaritimeData } from "./actions";
import MaritimeAiPrompt from "./ai-prompt";

export const metadata = { title: "Maritime New Zealand — My Gov NZ" };

export default async function MaritimeOverviewPage() {
  const data = await fetchMaritimeData(["maritime:vessels", "maritime:incidents"]);

  if (!data) {
    return (
      <main>
        <h1>Maritime New Zealand</h1>
        <p>Unable to load your Maritime information. Please grant access to continue.</p>
        <Link href={"/consent?dept=maritime"}>Grant Maritime access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Maritime New Zealand</h1>
      <p>Maritime ID: ••••{data.maritimeId.slice(-4)}</p>

      <section>
        <h2>Vessels</h2>
        {data.vessels ? (
          <Link href={"/dept/maritime/vessels"}>View vessels →</Link>
        ) : (
          <p>No vessels on file.</p>
        )}
      </section>

      <section>
        <h2>Incidents</h2>
        {data.incidents ? (
          <Link href={"/dept/maritime/incidents"}>View incidents →</Link>
        ) : (
          <p>No incidents on file.</p>
        )}
      </section>

      <MaritimeAiPrompt />
    </main>
  );
}
