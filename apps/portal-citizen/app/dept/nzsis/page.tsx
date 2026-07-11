import Link from "next/link";
import { fetchNzsisData } from "./actions";
import NzsisAiPrompt from "./ai-prompt";

export const metadata = { title: "New Zealand Security Intelligence Service — My Gov NZ" };

export default async function NzsisOverviewPage() {
  const data = await fetchNzsisData(["nzsis:mandates", "nzsis:threats"]);

  if (!data) {
    return (
      <main>
        <h1>New Zealand Security Intelligence Service</h1>
        <p>Unable to load your NZSIS information. Please grant access to continue.</p>
        <Link href={"/consent?dept=nzsis"}>Grant NZSIS access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>New Zealand Security Intelligence Service</h1>
      <p>NZSIS ID: ••••{data.nzsisId.slice(-4)}</p>

      <section>
        <h2>Mandates</h2>
        {data.mandates ? (
          <Link href={"/dept/nzsis/mandates"}>View mandates →</Link>
        ) : (
          <p>No mandates on file.</p>
        )}
      </section>

      <section>
        <h2>Threats</h2>
        {data.threats ? (
          <Link href={"/dept/nzsis/threats"}>View threats →</Link>
        ) : (
          <p>No threats on file.</p>
        )}
      </section>

      <NzsisAiPrompt />
    </main>
  );
}
