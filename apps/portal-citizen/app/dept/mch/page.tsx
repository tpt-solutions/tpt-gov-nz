import Link from "next/link";
import { fetchMchData } from "./actions";
import MchAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for Culture and Heritage — My Gov NZ" };

export default async function MchOverviewPage() {
  const data = await fetchMchData(["mch:heritage-sites", "mch:grants"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Culture and Heritage</h1>
        <p>Unable to load your MCH information. Please grant access to continue.</p>
        <Link href={"/consent?dept=mch"}>Grant MCH access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for Culture and Heritage</h1>
      <p>MCH ID: ••••{data.mchId.slice(-4)}</p>

      <section>
        <h2>Heritage sites</h2>
        {data.heritage_sites ? (
          <Link href={"/dept/mch/heritage_sites"}>View heritage sites →</Link>
        ) : (
          <p>No heritage sites on file.</p>
        )}
      </section>

      <section>
        <h2>Grants</h2>
        {data.grants ? (
          <Link href={"/dept/mch/grants"}>View grants →</Link>
        ) : (
          <p>No grants on file.</p>
        )}
      </section>

      <MchAiPrompt />
    </main>
  );
}
