import Link from "next/link";
import { fetchMfatData } from "./actions";
import MfatAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry of Foreign Affairs and Trade — My Gov NZ" };

export default async function MfatOverviewPage() {
  const data = await fetchMfatData(["mfat:overseas-missions", "mfat:travel-advisories"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Foreign Affairs and Trade</h1>
        <p>Unable to load your MFAT information. Please grant access to continue.</p>
        <Link href={"/consent?dept=mfat"}>Grant MFAT access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry of Foreign Affairs and Trade</h1>
      <p>MFAT ID: ••••{data.mfatId.slice(-4)}</p>

      <section>
        <h2>Overseas missions</h2>
        {data.overseas_missions ? (
          <Link href={"/dept/mfat/overseas_missions"}>View overseas missions →</Link>
        ) : (
          <p>No overseas missions on file.</p>
        )}
      </section>

      <section>
        <h2>Travel advisories</h2>
        {data.travel_advisories ? (
          <Link href={"/dept/mfat/travel_advisories"}>View travel advisories →</Link>
        ) : (
          <p>No travel advisories on file.</p>
        )}
      </section>

      <MfatAiPrompt />
    </main>
  );
}
