import Link from "next/link";
import { fetchEthnicData } from "./actions";
import EthnicAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for Ethnic Communities — My Gov NZ" };

export default async function EthnicOverviewPage() {
  const data = await fetchEthnicData(["ethnic:programmes", "ethnic:community-grants"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Ethnic Communities</h1>
        <p>Unable to load your Ethnic Communities information. Please grant access to continue.</p>
        <Link href={"/consent?dept=ethnic"}>Grant Ethnic Communities access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for Ethnic Communities</h1>
      <p>Ethnic Communities ID: ••••{data.ethnicId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data.programmes ? (
          <Link href={"/dept/ethnic/programmes"}>View programmes →</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Community grants</h2>
        {data.community_grants ? (
          <Link href={"/dept/ethnic/community_grants"}>View community grants →</Link>
        ) : (
          <p>No community grants on file.</p>
        )}
      </section>

      <EthnicAiPrompt />
    </main>
  );
}
