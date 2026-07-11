import Link from "next/link";
import { fetchDpmcData } from "./actions";
import DpmcAiPrompt from "./ai-prompt";

export const metadata = { title: "Department of the Prime Minister and Cabinet — My Gov NZ" };

export default async function DpmcOverviewPage() {
  const data = await fetchDpmcData(["dpmc:honours", "dpmc:engagements"]);

  if (!data) {
    return (
      <main>
        <h1>Department of the Prime Minister and Cabinet</h1>
        <p>Unable to load your DPMC information. Please grant access to continue.</p>
        <Link href={"/consent?dept=dpmc"}>Grant DPMC access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Department of the Prime Minister and Cabinet</h1>
      <p>DPMC ID: ••••{data.dpmcId.slice(-4)}</p>

      <section>
        <h2>Honours</h2>
        {data.honours ? (
          <Link href={"/dept/dpmc/honours"}>View honours →</Link>
        ) : (
          <p>No honours on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data.engagements ? (
          <Link href={"/dept/dpmc/engagements"}>View engagements →</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>

      <DpmcAiPrompt />
    </main>
  );
}
