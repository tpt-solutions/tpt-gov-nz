import Link from "next/link";
import { fetchEqcData } from "./actions";
import EqcAiPrompt from "./ai-prompt";

export const metadata = { title: "Earthquake Commission (Toka Tū Ake) — My Gov NZ" };

export default async function EqcOverviewPage() {
  const data = await fetchEqcData(["eqc:claims", "eqc:cover"]);

  if (!data) {
    return (
      <main>
        <h1>Earthquake Commission (Toka Tū Ake)</h1>
        <p>Unable to load your EQC information. Please grant access to continue.</p>
        <Link href={"/consent?dept=eqc"}>Grant EQC access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Earthquake Commission (Toka Tū Ake)</h1>
      <p>EQC ID: ••••{data.eqcId.slice(-4)}</p>

      <section>
        <h2>Claims</h2>
        {data.claims ? (
          <Link href={"/dept/eqc/claims"}>View claims →</Link>
        ) : (
          <p>No claims on file.</p>
        )}
      </section>

      <section>
        <h2>Cover</h2>
        {data.cover ? (
          <Link href={"/dept/eqc/cover"}>View cover →</Link>
        ) : (
          <p>No cover on file.</p>
        )}
      </section>

      <EqcAiPrompt />
    </main>
  );
}
