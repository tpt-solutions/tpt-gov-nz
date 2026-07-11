import Link from "next/link";
import { fetchFenzData } from "./actions";
import FenzAiPrompt from "./ai-prompt";

export const metadata = { title: "Fire and Emergency New Zealand — My Gov NZ" };

export default async function FenzOverviewPage() {
  const data = await fetchFenzData(["fenz:fire-safety", "fenz:incidents"]);

  if (!data) {
    return (
      <main>
        <h1>Fire and Emergency New Zealand</h1>
        <p>Unable to load your FENZ information. Please grant access to continue.</p>
        <Link href={"/consent?dept=fenz"}>Grant FENZ access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Fire and Emergency New Zealand</h1>
      <p>FENZ ID: ••••{data.fenzId.slice(-4)}</p>

      <section>
        <h2>Fire safety</h2>
        {data.fire_safety ? (
          <Link href={"/dept/fenz/fire_safety"}>View fire safety →</Link>
        ) : (
          <p>No fire safety on file.</p>
        )}
      </section>

      <section>
        <h2>Incidents</h2>
        {data.incidents ? (
          <Link href={"/dept/fenz/incidents"}>View incidents →</Link>
        ) : (
          <p>No incidents on file.</p>
        )}
      </section>

      <FenzAiPrompt />
    </main>
  );
}
