import Link from "next/link";
import { fetchSfoData } from "./actions";
import SfoAiPrompt from "./ai-prompt";

export const metadata = { title: "Serious Fraud Office — My Gov NZ" };

export default async function SfoOverviewPage() {
  const data = await fetchSfoData(["sfo:investigations", "sfo:outcomes"]);

  if (!data) {
    return (
      <main>
        <h1>Serious Fraud Office</h1>
        <p>Unable to load your SFO information. Please grant access to continue.</p>
        <Link href={"/consent?dept=sfo"}>Grant SFO access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Serious Fraud Office</h1>
      <p>SFO ID: ••••{data.sfoId.slice(-4)}</p>

      <section>
        <h2>Investigations</h2>
        {data.investigations ? (
          <Link href={"/dept/sfo/investigations"}>View investigations →</Link>
        ) : (
          <p>No investigations on file.</p>
        )}
      </section>

      <section>
        <h2>Outcomes</h2>
        {data.outcomes ? (
          <Link href={"/dept/sfo/outcomes"}>View outcomes →</Link>
        ) : (
          <p>No outcomes on file.</p>
        )}
      </section>

      <SfoAiPrompt />
    </main>
  );
}
