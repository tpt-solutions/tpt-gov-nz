import Link from "next/link";
import { fetchDefenceData } from "./actions";
import DefenceAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry of Defence — My Gov NZ" };

export default async function DefenceOverviewPage() {
  const data = await fetchDefenceData(["defence:procurements", "defence:bases"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Defence</h1>
        <p>Unable to load your Defence information. Please grant access to continue.</p>
        <Link href={"/consent?dept=defence"}>Grant Defence access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry of Defence</h1>
      <p>Defence ID: ••••{data.defenceId.slice(-4)}</p>

      <section>
        <h2>Procurements</h2>
        {data.procurements ? (
          <Link href={"/dept/defence/procurements"}>View procurements →</Link>
        ) : (
          <p>No procurements on file.</p>
        )}
      </section>

      <section>
        <h2>Bases</h2>
        {data.bases ? (
          <Link href={"/dept/defence/bases"}>View bases →</Link>
        ) : (
          <p>No bases on file.</p>
        )}
      </section>

      <DefenceAiPrompt />
    </main>
  );
}
