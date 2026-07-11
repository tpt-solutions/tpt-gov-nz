import Link from "next/link";
import { fetchTreasuryData } from "./actions";
import TreasuryAiPrompt from "./ai-prompt";

export const metadata = { title: "The Treasury — My Gov NZ" };

export default async function TreasuryOverviewPage() {
  const data = await fetchTreasuryData(["treasury:budget", "treasury:economic-outlook"]);

  if (!data) {
    return (
      <main>
        <h1>The Treasury</h1>
        <p>Unable to load your Treasury information. Please grant access to continue.</p>
        <Link href={"/consent?dept=treasury"}>Grant Treasury access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>The Treasury</h1>
      <p>Treasury ID: ••••{data.treasuryId.slice(-4)}</p>

      <section>
        <h2>Budget</h2>
        {data.budget ? (
          <Link href={"/dept/treasury/budget"}>View budget →</Link>
        ) : (
          <p>No budget on file.</p>
        )}
      </section>

      <section>
        <h2>Economic outlook</h2>
        {data.economic_outlook ? (
          <Link href={"/dept/treasury/economic_outlook"}>View economic outlook →</Link>
        ) : (
          <p>No economic outlook on file.</p>
        )}
      </section>

      <TreasuryAiPrompt />
    </main>
  );
}
