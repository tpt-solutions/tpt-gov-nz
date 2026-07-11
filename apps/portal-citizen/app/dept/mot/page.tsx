import Link from "next/link";
import { fetchMotData } from "./actions";
import MotAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry of Transport — My Gov NZ" };

export default async function MotOverviewPage() {
  const data = await fetchMotData(["mot:strategies", "mot:programmes"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Transport</h1>
        <p>Unable to load your Transport information. Please grant access to continue.</p>
        <Link href={"/consent?dept=mot"}>Grant Transport access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry of Transport</h1>
      <p>Transport ID: ••••{data.motId.slice(-4)}</p>

      <section>
        <h2>Strategies</h2>
        {data.strategies ? (
          <Link href={"/dept/mot/strategies"}>View strategies →</Link>
        ) : (
          <p>No strategies on file.</p>
        )}
      </section>

      <section>
        <h2>Programmes</h2>
        {data.programmes ? (
          <Link href={"/dept/mot/programmes"}>View programmes →</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <MotAiPrompt />
    </main>
  );
}
