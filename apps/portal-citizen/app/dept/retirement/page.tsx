import Link from "next/link";
import { fetchRetirementData } from "./actions";
import RetirementAiPrompt from "./ai-prompt";

export const metadata = { title: "Retirement Commission (Te Ara Ahunga Ora) — My Gov NZ" };

export default async function RetirementOverviewPage() {
  const data = await fetchRetirementData(["retirement:retirement-plan", "retirement:guidance"]);

  if (!data) {
    return (
      <main>
        <h1>Retirement Commission (Te Ara Ahunga Ora)</h1>
        <p>Unable to load your Retirement information. Please grant access to continue.</p>
        <Link href={"/consent?dept=retirement"}>Grant Retirement access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Retirement Commission (Te Ara Ahunga Ora)</h1>
      <p>Retirement ID: ••••{data.retirementId.slice(-4)}</p>

      <section>
        <h2>Retirement plan</h2>
        {data.retirement_plan ? (
          <Link href={"/dept/retirement/retirement_plan"}>View retirement plan →</Link>
        ) : (
          <p>No retirement plan on file.</p>
        )}
      </section>

      <section>
        <h2>Guidance</h2>
        {data.guidance ? (
          <Link href={"/dept/retirement/guidance"}>View guidance →</Link>
        ) : (
          <p>No guidance on file.</p>
        )}
      </section>

      <RetirementAiPrompt />
    </main>
  );
}
