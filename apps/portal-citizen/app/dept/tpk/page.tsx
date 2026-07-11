import Link from "next/link";
import { fetchTpkData } from "./actions";
import TpkAiPrompt from "./ai-prompt";

export const metadata = { title: "TPK — My Gov NZ" };

export default async function TpkOverviewPage() {
  const data = await fetchTpkData(["tpk:programmes", "tpk:funding"]);

  if (!data) {
    return (
      <main>
        <h1>Te Puni Kōkiri</h1>
        <p>Unable to load your Te Puni Kōkiri information. Please grant access to continue.</p>
        <Link href="/consent?dept=tpk">Grant Te Puni Kōkiri access</Link>
      </main>
    );
  }

  const activeProgrammes =
    data.programmes?.filter((p) => p.status === "enrolled" || p.status === "active") ?? [];
  const activeFunding = data.funding?.filter((f) => f.status !== "closed") ?? [];

  return (
    <main>
      <h1>Te Puni Kōkiri</h1>
      <p>TPK id: ••••{data.tpkId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        <p>{activeProgrammes.length} active programme(s).</p>
        <Link href="/dept/tpk/programmes">View programmes →</Link>
      </section>

      <section>
        <h2>Funding</h2>
        <p>{activeFunding.length} open funding record(s).</p>
        <Link href="/dept/tpk/funding">View funding →</Link>
      </section>

      <TpkAiPrompt />
    </main>
  );
}
