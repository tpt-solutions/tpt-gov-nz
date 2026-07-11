import Link from "next/link";
import { fetchOrangaData } from "./actions";
import OrangaAiPrompt from "./ai-prompt";

export const metadata = { title: "Oranga Tamariki — My Gov NZ" };

export default async function OrangaOverviewPage() {
  const data = await fetchOrangaData(["oranga:care-placements", "oranga:support-services"]);

  if (!data) {
    return (
      <main>
        <h1>Oranga Tamariki</h1>
        <p>Unable to load your Oranga Tamariki information. Please grant access to continue.</p>
        <Link href={"/consent?dept=oranga"}>Grant Oranga Tamariki access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Oranga Tamariki</h1>
      <p>Oranga Tamariki ID: ••••{data.orangaId.slice(-4)}</p>

      <section>
        <h2>Care placements</h2>
        {data.care_placements ? (
          <Link href={"/dept/oranga/care_placements"}>View care placements →</Link>
        ) : (
          <p>No care placements on file.</p>
        )}
      </section>

      <section>
        <h2>Support services</h2>
        {data.support_services ? (
          <Link href={"/dept/oranga/support_services"}>View support services →</Link>
        ) : (
          <p>No support services on file.</p>
        )}
      </section>

      <OrangaAiPrompt />
    </main>
  );
}
