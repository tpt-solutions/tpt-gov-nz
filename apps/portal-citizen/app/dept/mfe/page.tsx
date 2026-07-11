import Link from "next/link";
import { fetchMfeData } from "./actions";
import MfeAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for the Environment — My Gov NZ" };

export default async function MfeOverviewPage() {
  const data = await fetchMfeData(["mfe:emissions", "mfe:reports"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for the Environment</h1>
        <p>Unable to load your MfE information. Please grant access to continue.</p>
        <Link href={"/consent?dept=mfe"}>Grant MfE access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for the Environment</h1>
      <p>MfE ID: ••••{data.mfeId.slice(-4)}</p>

      <section>
        <h2>Emissions</h2>
        {data.emissions ? (
          <Link href={"/dept/mfe/emissions"}>View emissions →</Link>
        ) : (
          <p>No emissions on file.</p>
        )}
      </section>

      <section>
        <h2>Reports</h2>
        {data.reports ? (
          <Link href={"/dept/mfe/reports"}>View reports →</Link>
        ) : (
          <p>No reports on file.</p>
        )}
      </section>

      <MfeAiPrompt />
    </main>
  );
}
