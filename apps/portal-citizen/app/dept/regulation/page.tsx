import Link from "next/link";
import { fetchRegulationData } from "./actions";
import RegulationAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry for Regulation — My Gov NZ" };

export default async function RegulationOverviewPage() {
  const data = await fetchRegulationData(["regulation:regulatory-reviews", "regulation:proposals"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry for Regulation</h1>
        <p>Unable to load your Regulation information. Please grant access to continue.</p>
        <Link href={"/consent?dept=regulation"}>Grant Regulation access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Ministry for Regulation</h1>
      <p>Regulation ID: ••••{data.regulationId.slice(-4)}</p>

      <section>
        <h2>Regulatory reviews</h2>
        {data.regulatory_reviews ? (
          <Link href={"/dept/regulation/regulatory_reviews"}>View regulatory reviews →</Link>
        ) : (
          <p>No regulatory reviews on file.</p>
        )}
      </section>

      <section>
        <h2>Proposals</h2>
        {data.proposals ? (
          <Link href={"/dept/regulation/proposals"}>View proposals →</Link>
        ) : (
          <p>No proposals on file.</p>
        )}
      </section>

      <RegulationAiPrompt />
    </main>
  );
}
