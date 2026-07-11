import Link from "next/link";
import { fetchWomenData } from "./actions";
import WomenAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for Women — My Gov NZ" };

export default async function WomenOverviewPage() {
  const data = await fetchWomenData(["women:programmes", "women:insights"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Women</h1>
        <p>Unable to load your Women information. Please grant access to continue.</p>
        <Link href={"/consent?dept=women"}>Grant Women access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for Women</h1>
      <p>Women ID: ••••{data.womenId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data.programmes ? (
          <Link href={"/dept/women/programmes"}>View programmes →</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Insights</h2>
        {data.insights ? (
          <Link href={"/dept/women/insights"}>View insights →</Link>
        ) : (
          <p>No insights on file.</p>
        )}
      </section>

      <WomenAiPrompt />
    </main>
  );
}
