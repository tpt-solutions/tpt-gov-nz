import Link from "next/link";
import { fetchGcsbData } from "./actions";
import GcsbAiPrompt from "./ai-prompt";

export const metadata = { title: "Government Communications Security Bureau — My Gov NZ" };

export default async function GcsbOverviewPage() {
  const data = await fetchGcsbData(["gcsb:mandates", "gcsb:engagements"]);

  if (!data) {
    return (
      <main>
        <h1>Government Communications Security Bureau</h1>
        <p>Unable to load your GCSB information. Please grant access to continue.</p>
        <Link href={"/consent?dept=gcsb"}>Grant GCSB access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Government Communications Security Bureau</h1>
      <p>GCSB ID: ••••{data.gcsbId.slice(-4)}</p>

      <section>
        <h2>Mandates</h2>
        {data.mandates ? (
          <Link href={"/dept/gcsb/mandates"}>View mandates →</Link>
        ) : (
          <p>No mandates on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data.engagements ? (
          <Link href={"/dept/gcsb/engagements"}>View engagements →</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>

      <GcsbAiPrompt />
    </main>
  );
}
